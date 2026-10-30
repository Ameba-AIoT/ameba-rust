//! `log` crate facade over the existing LOGUART transport.
//!
//! This is currently a single synchronous, blocking [`log::Log`] implementation —
//! every record is written straight through [`crate::loguart::LogWriter`],
//! the same transport [`crate::println!`] already uses. Level filtering is
//! entirely delegated to the `log` crate's own [`log::set_max_level`] (set
//! via [`init`]); `AmebaLogger::enabled` always returns `true` since the
//! `log!` macros already skip calling [`log::Log::log`] for records above
//! the configured max level. A non-blocking/queued logger is deliberately
//! out of scope until a real use case needs it.

use log::{LevelFilter, Log, Metadata, Record};

struct AmebaLogger;

impl Log for AmebaLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        crate::println!(
            "[{}] {}: {}",
            record.level(),
            record.target(),
            record.args()
        );
    }

    fn flush(&self) {}
}

static LOGGER: AmebaLogger = AmebaLogger;

/// Install the LOGUART-backed logger as the global `log` sink.
///
/// # Panics
/// Panics if a logger has already been installed (`log::set_logger` called
/// more than once) — mirrors the `log` crate's own contract.
pub fn init(level: LevelFilter) {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(level);
}
