//! Serial log monitor.

use std::io::{ErrorKind, Read, Write};
use std::time::Duration;

use anyhow::{Context, Result};

/// LogUART baud rate. Selected by the OTP bit `SEC_BIT_LOW_BAUD_LOG_EN`, which
/// resets to 0 = 1.5 Mbaud; use 115200 on parts where it is programmed.
pub const LOGUART_BAUD: u32 = 1_500_000;

/// Print everything the board sends on `port` until the process is killed.
///
/// Ends on Ctrl-C, which terminates the process from under this loop; there is
/// no in-band way to stop. Read timeouts are expected (an idle board sends
/// nothing) and are not errors, so they just restart the read.
pub fn monitor_serial(port: &str, baud: u32) -> Result<()> {
    println!("[run] monitoring {port} @ {baud} (Ctrl-C to stop) ...");

    let mut serial = serialport::new(port, baud)
        .timeout(Duration::from_secs(1))
        .open()
        .with_context(|| format!("failed to open {port} @ {baud}"))?;

    let mut buf = [0u8; 256];
    let mut stdout = std::io::stdout();
    loop {
        match serial.read(&mut buf) {
            Ok(0) => {}
            Ok(n) => {
                stdout.write_all(&buf[..n])?;
                stdout.flush()?;
            }
            Err(err) if err.kind() == ErrorKind::TimedOut => {}
            Err(err) => return Err(err).context("serial read failed"),
        }
    }
}
