//! UART flashing protocol, written natively in Rust.
//!
//! - [`proto`] — device profile, error codes, and shared timing constants.
//! - [`rom`] — ROM-stage (xmodem-like) frame codec: loads the floader.
//! - [`floader`] — floader-stage frame codec and command payload builders.
//! - [`transport`] — serial I/O + the retry/timeout state machine driving
//!   both codecs, plus the two read-only pre-flight checks.
//! - [`download`] — the per-image erase/write/verify loop built on top of
//!   [`transport`].
//! - [`session`] — the top-level per-session orchestration (ROM handshake →
//!   floader load → pre-flight checks → one [`download`] call per image →
//!   NEXTOP + DTR/RTS reset pulse).

pub mod download;
pub mod floader;
pub mod proto;
pub mod rom;
pub mod session;
pub mod transport;
