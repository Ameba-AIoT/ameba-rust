//! Per-session orchestration: ROM handshake → floader load → pre-flight
//! checks → one download per image → reset.

use std::time::Duration;

use anyhow::{Context, Result};

use super::{download, floader, proto, transport};

/// One ROM-handshake → floader-load → pre-flight-check → download-both-images
/// → reset session, for the UART + NOR flash case.
pub fn flash_session(
    port: &str,
    floader_bin: &[u8],
    boot_bin: &[u8],
    app_bin: &[u8],
) -> Result<()> {
    let images = [
        ("boot.bin", boot_bin, proto::IMAGE_BOOT_RANGE),
        ("app.bin", app_bin, proto::IMAGE_APP_RANGE),
    ]
    .map(|(label, data, (start, end))| {
        (
            label,
            download::Image {
                start_address: start,
                end_address: end,
                data,
            },
        )
    });

    for (label, image) in &images {
        download::check_fits_partition(image).map_err(|_| {
            anyhow::anyhow!(
                "{label} is {} bytes, too large for its {}-byte partition at {:#010x}; \
                 adjust the memory layout",
                image.data.len(),
                image.end_address - image.start_address,
                image.start_address
            )
        })?;
    }

    let mut serial =
        transport::open_port(port).with_context(|| format!("failed to open {port}"))?;

    println!("[run] ROM handshake ...");
    if transport::rom_handshake(&mut *serial, proto::HANDSHAKE_BAUDRATE, false).is_err() {
        println!("[run] not in download mode, rebooting into UART burn mode ...");
        reset_into_uart_download_mode(&mut *serial)?;
        std::thread::sleep(Duration::from_millis(proto::ROM_BOOT_DELAY_MS));
        transport::rom_handshake(&mut *serial, proto::HANDSHAKE_BAUDRATE, false).map_err(|e| {
            anyhow::anyhow!(
                "ROM handshake failed: {e}; if the board's log UART is not at \
                     {} baud, reset it into download mode manually",
                proto::HANDSHAKE_BAUDRATE
            )
        })?;
    }

    println!("[run] loading floader ...");
    if let Err(e) =
        transport::rom_download_floader(&mut *serial, floader_bin, proto::HANDSHAKE_BAUDRATE, false)
    {
        transport::rom_abort(&mut *serial);
        return Err(anyhow::anyhow!("floader download failed: {e}"));
    }

    std::thread::sleep(Duration::from_millis(proto::FLOADER_BOOT_DELAY_MS));

    println!("[run] floader handshake ...");
    transport::floader_handshake(&mut *serial)
        .map_err(|e| anyhow::anyhow!("floader handshake failed: {e}"))?;

    let device =
        transport::floader_query(&mut *serial).map_err(|e| anyhow::anyhow!("QUERY failed: {e}"))?;

    for (label, image) in &images {
        download::check_within_flash(image, device.flash_capacity).map_err(|_| {
            anyhow::anyhow!(
                "{label} partition {:#010x}..{:#010x} extends past the end of the \
                 {}-byte flash",
                image.start_address,
                image.end_address,
                device.flash_capacity
            )
        })?;
    }

    println!("[run] pre-flight checks ...");
    match transport::check_flash_not_locked(&mut *serial) {
        Ok(()) => {}
        Err(proto::FlashError::FlashLocked) => {
            anyhow::bail!("flash is locked: block-protect bits are set")
        }
        Err(e) => return Err(e).context("flash lock check failed"),
    }
    match transport::check_supported_flash_size(&mut *serial) {
        Ok(()) => {}
        Err(proto::FlashError::UnsupportedFlashSize) => {
            anyhow::bail!("unsupported flash size: eFuse selects 4-byte addressing")
        }
        Err(e) => return Err(e).context("flash size check failed"),
    }

    for (label, image) in &images {
        println!("[run] downloading {label} ...");
        let mut last_percent = u32::MAX;
        let mut on_progress = |p: u32| {
            if p != last_percent {
                last_percent = p;
                print!("\r[run]   {label}: {p}%");
                use std::io::Write;
                let _ = std::io::stdout().flush();
            }
        };
        download::download_image(&mut *serial, image, Some(&mut on_progress))
            .map_err(|e| anyhow::anyhow!("{label} download failed: {e}"))?;
        println!();
    }

    println!("[run] resetting device ...");
    let next_op = transport::floader_next_op(&mut *serial, floader::NextOp::Reset, 0);
    // The DTR/RTS pulse still runs after a failed NEXTOP: boards that wire
    // those lines to RESET can restart without the command.
    transport::run_dtr_rts_timing(&mut *serial, proto::RESET_TIMING)
        .map_err(|e| anyhow::anyhow!("reset timing failed: {e}"))?;
    next_op.map_err(|e| {
        anyhow::anyhow!(
            "images were written and verified, but the reset command failed ({e}); \
             the device may still be in the flash loader — reset or power-cycle it manually"
        )
    })
}

/// Reboot a device running normal firmware into UART download mode by
/// typing `reboot uartburn` on its shell. Assumes the log baud rate already
/// equals the handshake baud rate, so no baud switch is needed around the
/// command.
///
/// The `@` prefix makes the NP core run the command itself; without it the
/// NP forwards it to the AP, which does not handle it.
fn reset_into_uart_download_mode(port: &mut dyn serialport::SerialPort) -> Result<()> {
    port.write_all(b"\x1b\r\n").context("failed to write ESC")?;
    std::thread::sleep(Duration::from_millis(20));
    port.write_all(b"@reboot uartburn\r\n")
        .context("failed to write reboot command")?;
    std::thread::sleep(Duration::from_millis(20));
    port.clear(serialport::ClearBuffer::Input)
        .context("failed to clear input buffer")?;
    std::thread::sleep(Duration::from_millis(50));
    Ok(())
}
