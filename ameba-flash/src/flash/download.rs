//! Download-loop orchestration: erase/write/verify a whole image over the
//! floader protocol in [`transport`], plus the CHKSM-mismatch readback
//! diagnostic and progress reporting. This is the layer `main.rs` calls once
//! per image (`boot.bin`, `app.bin`), after the ROM-stage handshake and
//! floader load have already completed.
//!
//! Only the NOR path is implemented — see [`super`]'s module doc for the
//! fixed scope this port covers (UART + NOR + two fixed images).

use std::time::Duration;

use super::proto::{self, FlashError};
use super::transport::{self, Transport};

/// One image to flash: its raw-flash-bus address range (end-exclusive) and
/// its bytes. `end_address - start_address` is the partition's capacity;
/// `data` may be shorter. [`check_fits_partition`] and
/// [`check_within_flash`] validate the layout before downloading.
pub struct Image<'a> {
    pub start_address: u32,
    pub end_address: u32,
    pub data: &'a [u8],
}

/// `Err(SysParameter)` if the image file is larger than its partition.
pub fn check_fits_partition(image: &Image<'_>) -> Result<(), FlashError> {
    let capacity = image.end_address.saturating_sub(image.start_address) as usize;
    if image.data.len() > capacity {
        return Err(FlashError::SysParameter);
    }
    Ok(())
}

/// `Err(SysOverrange)` if the partition extends past the end of the
/// device's flash, as reported by QUERY.
pub fn check_within_flash(image: &Image<'_>, flash_capacity: u32) -> Result<(), FlashError> {
    let max_addr = u64::from(proto::FLASH_START_ADDRESS) + u64::from(flash_capacity);
    if u64::from(image.start_address) > max_addr || u64::from(image.end_address) > max_addr {
        return Err(FlashError::SysOverrange);
    }
    Ok(())
}

/// Host-side running checksum: LE u32 sum mod 2^32 over `chunk` (which must
/// already be page-sized and 0xFF-padded — the same bytes just transmitted
/// by WRITE), matching the reference's `struct.unpack('<{n}I', chunk_data)`
/// accumulation exactly.
fn accumulate_checksum(checksum: u32, chunk: &[u8]) -> u32 {
    let (words, _) = chunk.as_chunks::<4>();
    words.iter().fold(checksum, |acc, word| {
        acc.wrapping_add(u32::from_le_bytes(*word))
    })
}

/// Progress callback: `percent` only changes value when the caller should
/// act on it — see [`download_image`]'s 1%-granularity note.
pub type ProgressFn<'a> = dyn FnMut(u32) + 'a;

/// Flash one NOR image and verify it with a device-side CHKSM. On a CHKSM
/// mismatch, re-reads the whole image page by page from the device and logs
/// each mismatching page before returning `Err(FlashError::SysChecksum)` —
/// this diagnostic never changes the final error, it only narrows down which
/// pages were affected by a dropped UART frame.
pub fn download_image<T: Transport + ?Sized>(
    t: &mut T,
    image: &Image<'_>,
    mut on_progress: Option<&mut ProgressFn<'_>>,
) -> Result<(), FlashError> {
    let page_size = proto::NOR_PAGE_SIZE;
    let img_length = image.data.len() as u32;
    let aligned_img_length =
        super::rom::page_aligned_size(img_length as usize, page_size as usize) as u32;
    if aligned_img_length == 0 {
        return Ok(());
    }

    let mut addr = image.start_address;
    let mut tx_sum: u32 = 0;
    let mut checksum: u32 = 0;
    let mut write_pages: u32 = 0;
    let mut pages_per_block: u32 = 0;
    let mut last_erase_addr: u32 = 0;
    let mut sense_timeout = Duration::from_millis(proto::SYNC_RESPONSE_TIMEOUT_MS);
    let mut last_percent: u32 = u32::MAX;

    while tx_sum < aligned_img_length {
        if write_pages == 0 {
            let remaining = aligned_img_length - tx_sum;
            let block_size = proto::erase_block_size(addr, remaining);
            pages_per_block = block_size / page_size;
            let erase_addr = addr;
            sense_timeout = Duration::from_millis(proto::nor_write_timeout_ms(
                proto::SENSE_PACKET_COUNT.max(pages_per_block),
                block_size,
            ));

            if erase_addr != last_erase_addr {
                let erase_timeout = Duration::from_millis(proto::nor_erase_timeout_ms(block_size));
                transport::floader_erase(
                    t,
                    proto::MEMORY_TYPE_NOR,
                    erase_addr,
                    erase_addr + block_size,
                    block_size,
                    false,
                    erase_timeout,
                )?;
                last_erase_addr = erase_addr;
            }
        }

        let chunk_start = tx_sum as usize;
        let chunk_end = (chunk_start + page_size as usize).min(image.data.len());
        let mut chunk = if chunk_start < image.data.len() {
            image.data[chunk_start..chunk_end].to_vec()
        } else {
            Vec::new()
        };
        if chunk.len() < page_size as usize {
            chunk.resize(page_size as usize, 0xFF);
        }

        let need_sense = (write_pages + 1).is_multiple_of(proto::SENSE_PACKET_COUNT)
            || (write_pages + 1 >= pages_per_block)
            || (tx_sum + page_size >= aligned_img_length);

        transport::floader_write_page(
            t,
            proto::MEMORY_TYPE_NOR,
            addr,
            &chunk,
            need_sense,
            sense_timeout,
        )?;

        write_pages += 1;
        if write_pages >= pages_per_block {
            write_pages = 0;
        }

        checksum = accumulate_checksum(checksum, &chunk);
        addr += page_size;
        tx_sum += page_size;

        let percent = tx_sum * 100 / aligned_img_length;
        if percent != last_percent {
            last_percent = percent;
            if let Some(cb) = on_progress.as_deref_mut() {
                cb(percent);
            }
        }
    }
    let checksum_timeout =
        Duration::from_millis(proto::nor_checksum_timeout_ms(aligned_img_length));
    // The floader checksums `aligned_img_length` bytes from `start_address`;
    // for NOR, `end_address` is only range-checked, so the partition end is
    // fine.
    let device_checksum = transport::floader_checksum(
        t,
        proto::MEMORY_TYPE_NOR,
        image.start_address,
        image.end_address,
        aligned_img_length,
        checksum_timeout,
    )?;

    if device_checksum != checksum {
        readback_diagnostic(t, image, page_size, aligned_img_length);
        return Err(FlashError::SysChecksum);
    }

    Ok(())
}

/// On a CHKSM mismatch, re-read the image from flash page by page (always
/// starting at page 0, regardless of where the write loop above stopped)
/// and compare each page against the original file bytes, logging which
/// pages diverged. Diagnostic only — never changes the caller's result.
fn readback_diagnostic<T: Transport + ?Sized>(
    t: &mut T,
    image: &Image<'_>,
    page_size: u32,
    aligned_img_length: u32,
) {
    let read_timeout = Duration::from_millis(proto::nor_checksum_timeout_ms(page_size));
    let mut rb_addr = image.start_address;
    let mut mismatch_count = 0u32;
    let total_pages = aligned_img_length / page_size;

    for pg in 0..total_pages {
        let start = (pg * page_size) as usize;
        let end = (start + page_size as usize).min(image.data.len());
        let mut expected = if start < image.data.len() {
            image.data[start..end].to_vec()
        } else {
            Vec::new()
        };
        if expected.len() < page_size as usize {
            expected.resize(page_size as usize, 0xFF);
        }

        match transport::floader_read(t, proto::MEMORY_TYPE_NOR, rb_addr, page_size, read_timeout) {
            Ok(got) if got == expected => {}
            Ok(got) => {
                let word = |b: &[u8]| {
                    b.get(..4)
                        .map_or(0, |w| u32::from_le_bytes([w[0], w[1], w[2], w[3]]))
                };
                let (exp_w, got_w) = (word(&expected), word(&got));
                eprintln!(
                    "  page {pg} @ {rb_addr:#010x}: expect={exp_w:#010x} got={got_w:#010x} (len {})",
                    got.len()
                );
                mismatch_count += 1;
            }
            Err(e) => {
                eprintln!("  page {pg} @ {rb_addr:#010x}: read failed: {e:?}");
                mismatch_count += 1;
            }
        }
        rb_addr += page_size;
    }

    eprintln!("readback: {mismatch_count}/{total_pages} pages mismatch");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accumulate_checksum_matches_le_u32_sum() {
        let chunk = [0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00];
        assert_eq!(accumulate_checksum(0, &chunk), 3);
    }

    #[test]
    fn accumulate_checksum_wraps_mod_2_32() {
        let chunk = [0xFF, 0xFF, 0xFF, 0xFF];
        assert_eq!(accumulate_checksum(1, &chunk), 0);
    }

    fn image(start: u32, end: u32, data: &[u8]) -> Image<'_> {
        Image {
            start_address: start,
            end_address: end,
            data,
        }
    }

    #[test]
    fn fits_partition_accepts_exact_fit_rejects_one_byte_over() {
        let data = vec![0u8; 0x1000];
        assert_eq!(
            check_fits_partition(&image(0x0804_0000, 0x0804_1000, &data)),
            Ok(())
        );
        assert_eq!(
            check_fits_partition(&image(0x0804_0000, 0x0804_0FFF, &data)),
            Err(FlashError::SysParameter)
        );
    }

    #[test]
    fn within_flash_checks_partition_end_against_capacity() {
        let (start, end) = proto::IMAGE_APP_RANGE;
        let img = image(start, end, &[]);
        let exact = end - proto::FLASH_START_ADDRESS;
        assert_eq!(check_within_flash(&img, exact), Ok(()));
        assert_eq!(
            check_within_flash(&img, exact - 1),
            Err(FlashError::SysOverrange)
        );
    }
}
