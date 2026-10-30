//! ELF → `km4tz_image2_all.bin` section extraction.

use std::path::Path;

use anyhow::{Context, Result, bail};
use object::elf;
use object::read::elf::{ElfFile32, ProgramHeader, SectionHeader};
use object::{Object, ObjectSection, ObjectSymbol};

use crate::IMG2_SIGN;

fn elf_symbol(elf: &ElfFile32, name: &str) -> Result<u32> {
    let mut defs = elf
        .symbols()
        .filter(|s| s.name() == Ok(name) && !s.is_undefined());
    let sym = defs
        .next()
        .with_context(|| format!("symbol {name:?} not defined in ELF"))?;
    if defs.next().is_some() {
        bail!("symbol {name:?} is defined more than once in ELF");
    }
    u32::try_from(sym.address()).with_context(|| format!("address of {name:?} exceeds 32 bits"))
}

/// One section that occupies bytes in the loaded image.
struct LoadSection<'a> {
    name: &'a str,
    addr: u32,
    data: &'a [u8],
}

/// Every allocated, non-empty section that has file contents (anything but
/// NOBITS, so INIT_ARRAY and friends are included) — what `objcopy -O binary`
/// would emit with no `-j` filter.
fn load_sections<'a>(elf: &'a ElfFile32<'a>) -> Result<Vec<LoadSection<'a>>> {
    let endian = elf.endian();
    let mut out = Vec::new();
    for section in elf.sections() {
        let hdr = section.elf_section_header();
        if hdr.sh_type(endian) == elf::SHT_NOBITS
            || hdr.sh_flags(endian) & elf::SHF_ALLOC == 0
            || hdr.sh_size(endian) == 0
        {
            continue;
        }
        out.push(LoadSection {
            name: section.name()?,
            addr: hdr.sh_addr(endian),
            data: section.data()?,
        });
    }
    Ok(out)
}

/// Lay out the sections that start in `[start, end)` like `objcopy -O
/// binary`: from `start` to the end of the last one, gaps zero-filled.
///
/// Selecting by address rather than by a list of section names is what keeps
/// orphan sections in: a name list silently drops any section the linker
/// places in the range without an explicit rule, zero-filling it in the
/// image — e.g. cortex-m-rt's HardFault handler would be zeros while the
/// vector table still points at it.
fn extract_region(sections: &[LoadSection], start: u32, end: u32) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    for s in sections.iter().filter(|s| (start..end).contains(&s.addr)) {
        let off = (s.addr - start) as usize;
        if s.addr as usize + s.data.len() > end as usize {
            bail!(
                "section {} at {:#010x} runs past region end {end:#010x}",
                s.name,
                s.addr
            );
        }
        if out.len() < off + s.data.len() {
            out.resize(off + s.data.len(), 0);
        }
        out[off..off + s.data.len()].copy_from_slice(s.data);
    }
    Ok(out)
}

fn pad32(mut data: Vec<u8>) -> Vec<u8> {
    if data.is_empty() {
        return data;
    }
    let r = data.len() % 32;
    if r != 0 {
        data.extend(std::iter::repeat_n(0xFF_u8, 32 - r));
    }
    data
}

/// Build one 32-byte-headed sub-image: 8B BE magic, 4B LE size, 4B LE load
/// address, 16B reserved (0xFF), then the payload.
///
/// The size field must be the payload length *as stored in the file* (including
/// any 0xFF filler `pad32` appended), because the boot ROM uses it both to copy
/// the sub-image and to find the next one:
///
/// ```text
/// Len = ImgHdr.image_size + IMAGE_HEADER_LEN;   // boot_ota_km4tz.c
/// BOOT_ImgCopy(DstAddr, StartAddr, Len);
/// StartAddr += Len;                             // walk to next sub-image
/// ```
///
/// A size smaller than the stored payload (e.g. the linker region span, which
/// excludes the filler) makes that walk land short, so the following sub-image's
/// magic fails to match and the ROM reports it as `Invalid`. The size field is
/// therefore the byte length of the whole payload as stored, filler included.
fn make_section_bin(load_addr: u32, payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(32 + payload.len());
    out.extend_from_slice(&IMG2_SIGN.to_be_bytes());
    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    out.extend_from_slice(&load_addr.to_le_bytes());
    out.extend(std::iter::repeat_n(0xFF_u8, 16));
    out.extend_from_slice(payload);
    out
}

/// Extract the three image2 regions from `elf_path` and write the
/// concatenated, headed sub-images to `out`.
pub fn package_image2(elf_path: &Path, out: &Path) -> Result<()> {
    let data = std::fs::read(elf_path)
        .with_context(|| format!("failed to read {}", elf_path.display()))?;
    let elf = ElfFile32::<object::Endianness>::parse(&*data)
        .with_context(|| format!("failed to parse ELF {}", elf_path.display()))?;

    // Sections are placed by VMA below, which is only what `objcopy -O binary`
    // (placing by LMA) would produce if the two coincide. ameba.x keeps them
    // equal — the bootloader copies each sub-image straight to its run address.
    let endian = elf.endian();
    for ph in elf.elf_program_headers() {
        if ph.p_type(endian) == elf::PT_LOAD && ph.p_paddr(endian) != ph.p_vaddr(endian) {
            bail!(
                "{}: PT_LOAD with LMA {:#010x} != VMA {:#010x} is not supported",
                elf_path.display(),
                ph.p_paddr(endian),
                ph.p_vaddr(endian)
            );
        }
    }

    let region = |start, end| -> Result<(u32, u32)> {
        Ok((elf_symbol(&elf, start)?, elf_symbol(&elf, end)?))
    };
    let regions = [
        region("__flash_text_start__", "__flash_text_end__")?,
        region("__sram_image2_start__", "__sram_image2_end__")?,
        region("__psram_image2_start__", "__psram_image2_end__")?,
    ];

    // Loadable bytes outside all three regions would never reach the device.
    let sections = load_sections(&elf)?;
    if let Some(s) = sections.iter().find(|s| {
        !regions
            .iter()
            .any(|&(start, end)| (start..end).contains(&s.addr))
    }) {
        bail!(
            "{}: loadable section {} at {:#010x} is outside every image2 region",
            elf_path.display(),
            s.name,
            s.addr
        );
    }

    let [xip, sram, psram] = regions;
    let payload = |(start, end)| extract_region(&sections, start, end).map(pad32);
    let (xip_addr, xip_payload) = (xip.0, payload(xip)?);
    let (sram_addr, sram_payload) = (sram.0, payload(sram)?);
    let (psram_addr, psram_payload) = (psram.0, payload(psram)?);

    let mut image2_all = make_section_bin(xip_addr, &xip_payload);
    image2_all.extend(make_section_bin(sram_addr, &sram_payload));
    image2_all.extend(make_section_bin(psram_addr, &psram_payload));

    std::fs::write(out, &image2_all)?;

    println!(
        "[package] xip={}B  sram={}B  psram={}B  total={}B",
        xip_payload.len(),
        sram_payload.len(),
        psram_payload.len(),
        image2_all.len()
    );
    println!("[package] {}", out.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn section<'a>(name: &'a str, addr: u32, data: &'a [u8]) -> LoadSection<'a> {
        LoadSection { name, addr, data }
    }

    #[test]
    fn extract_region_zero_fills_gaps_like_objcopy() {
        let sections = [section(".a", 0x100, &[1, 2]), section(".b", 0x104, &[3])];
        assert_eq!(
            extract_region(&sections, 0x100, 0x200).unwrap(),
            [1, 2, 0, 0, 3]
        );
    }

    #[test]
    fn extract_region_keeps_sections_it_has_no_name_for() {
        // An orphan placed between two named sections must land in the image,
        // not be zero-filled.
        let sections = [
            section(".text", 0x100, &[1]),
            section(".orphan", 0x101, &[0xAA]),
            section(".data", 0x102, &[2]),
        ];
        assert_eq!(
            extract_region(&sections, 0x100, 0x200).unwrap(),
            [1, 0xAA, 2]
        );
    }

    #[test]
    fn extract_region_ignores_sections_outside_range() {
        let sections = [section(".in", 0x100, &[1]), section(".out", 0x300, &[2])];
        assert_eq!(extract_region(&sections, 0x100, 0x200).unwrap(), [1]);
        assert!(extract_region(&sections, 0x400, 0x500).unwrap().is_empty());
    }

    #[test]
    fn extract_region_rejects_section_past_region_end() {
        let sections = [section(".big", 0x1FE, &[1, 2, 3])];
        assert!(extract_region(&sections, 0x100, 0x200).is_err());
    }
}
