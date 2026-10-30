//! `ameba-flash` — flash an Ameba Rust firmware ELF over UART.
//!
//! Usable as a cargo runner once installed onto `PATH`
//! (`cargo install --path ameba-flash`): `runner = "ameba-flash flash --monitor"`, then
//! `cargo run --release -- --port <PORT>`, or plain `cargo run --release`
//! with `RTK_SERIAL_PORT` set.

use std::path::{Path, PathBuf};

use ameba_flash::elf::package_image2;
use ameba_flash::flash::session::flash_session;
use ameba_flash::monitor::{LOGUART_BAUD, monitor_serial};
use ameba_flash::{pack, prebuilt};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ameba-flash", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Package an ELF into app.bin and flash it over UART
    Flash {
        /// Firmware ELF to flash
        elf: PathBuf,
        /// Serial port, e.g. COM29 or /dev/ttyUSB0
        #[arg(long, env = "RTK_SERIAL_PORT")]
        port: String,
        /// Monitor serial output after flashing
        #[arg(long)]
        monitor: bool,
        /// Serial monitor baud rate
        #[arg(long, default_value_t = LOGUART_BAUD)]
        baud: u32,
    },
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    match Cli::parse().command {
        Command::Flash {
            elf,
            port,
            monitor,
            baud,
        } => {
            let app_bin = package(&elf)?;
            println!("[run] flashing on {port} ...");
            flash_session(&port, prebuilt::FLOADER_BIN, prebuilt::BOOT_BIN, &app_bin)?;
            println!("[run] flash done.");
            if monitor {
                monitor_serial(&port, baud)?;
            }
            Ok(())
        }
    }
}

/// ELF -> `app.bin` bytes, staged in a fresh, uniquely named temp directory.
fn package(elf: &Path) -> Result<Vec<u8>> {
    let dir = tempfile::Builder::new()
        .prefix("ameba-flash-")
        .tempdir()
        .context("failed to create a temp directory")?;
    let result = package_in(elf, dir.path());
    let path = dir.path().to_path_buf();
    if let Err(e) = dir.close() {
        eprintln!("warning: failed to remove {}: {e}", path.display());
    }
    result
}

fn package_in(elf: &Path, dir: &Path) -> Result<Vec<u8>> {
    let km4ns_bin = dir.join("km4ns_image2_all.bin");
    std::fs::write(&km4ns_bin, prebuilt::NP_IMAGE)?;
    let km4tz_bin = dir.join("km4tz_image2_all.bin");
    package_image2(elf, &km4tz_bin)?;
    let app_bin = dir.join("app.bin");
    pack::fw_pack(&app_bin, &km4ns_bin, &km4tz_bin, dir)?;
    std::fs::read(&app_bin).with_context(|| format!("failed to read {}", app_bin.display()))
}
