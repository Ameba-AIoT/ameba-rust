# ameba-ota

[![License](https://badgen.net/badge/License/Apache%202.0/lightgrey)](../LICENSE)
[![Target](https://badgen.net/badge/target/thumbv8m.main/blue)](https://doc.rust-lang.org/rustc/platform-support/thumbv8m.main-none-eabi.html)

Dual-slot (A/B) over-the-air firmware update for Realtek Ameba series chips,
with an HTTP transport. Downloads a new image into the inactive slot, verifies
it, and commits it so the bootloader picks it up on the next reset.

Part of the [ameba-rust](../README.md) SDK.

## Usage

```toml
[dependencies]
ameba-ota = { git = "https://github.com/Ameba-AIoT/ameba-rust", features = ["http-reqwless"] }
```

```rust
use ameba_ota::core::{OtaSession, OTA_BUF_SIZE};
use ameba_storage::{Flash, FlashStorage};

// Buffers live in statics rather than on the stack — together they are larger
// than a typical task stack, and keeping them out of the future stops the OTA
// state machine from inflating every task that awaits it.
static OTA_BUF: StaticCell<[u8; OTA_BUF_SIZE]> = StaticCell::new();
static WRITE_BUF: StaticCell<OtaWriteBuf> = StaticCell::new();

// `transport` is anything implementing `transport::Transport` — for HTTP,
// obtain one from `http::http_get(...)`.
let session = OtaSession::new(transport);
let mut flash = FlashStorage::new(unsafe { Flash::steal() });

match session.run(&mut flash, ota_buf, write_buf).await {
    Ok(report) => {
        // Image is in the inactive slot and committed. Reset to run it.
        Timer::after_millis(20).await;          // let the log UART drain
        cortex_m::peripheral::SCB::sys_reset();
    }
    Err(e) => { /* active slot is untouched — safe to retry */ }
}
```

See [`examples/ota-http`](../examples/ota-http/) for a complete run: connect to
Wi-Fi, fetch over HTTP, program, verify, commit, reset.

## Update flow

```
  active slot                          inactive slot
  ┌──────────┐                         ┌──────────┐
  │ running  │                         │   old    │
  │ firmware │  ──── download ────▶    │  image   │  ← erased, then programmed
  └──────────┘                         └──────────┘
                                             │
                                       checksum verified
                                             │
                                       manifest written last
                                             ▼
                                       ┌──────────┐
                                       │   new    │  ← bootloader selects this
                                       │ firmware │     on the next reset
                                       └──────────┘
```

The running image is never overwritten, and the manifest that makes a slot
eligible for boot is written only after the payload is fully programmed and
verified. A power loss at any point therefore leaves the device booting the
old firmware — the half-written slot simply never becomes selectable.

## Verification split

This crate does **not** verify the image signature, by design. Slot selection
and ECC signature verification are performed by the bootloader, which runs from
a separate flash region before this image is entered. Re-implementing the check
here would add no security: an application-level check can only be as
trustworthy as the application performing it, and the bootloader already
rejects an unsigned or tampered image before it ever runs.

What this crate verifies is transport integrity — a checksum over the
downloaded payload, catching truncated or corrupted downloads before the slot
is made bootable.

A corollary: because authenticity is established at boot rather than in
transit, plain HTTP is not the security hole it would be in a system without
verified boot. TLS support is deferred for that reason.

## Modules

| Module | Contents |
|:-------|:---------|
| `core` | `OtaSession` — the update state machine, and `OtaReport` describing the result |
| `transport` | The `Transport` trait a download source implements |
| `http` | HTTP transport built on [`reqwless`](https://docs.rs/reqwless/) (feature `http-reqwless`) |
| `slot` | Active/inactive slot resolution |
| `layout` | Flash partition geometry |
| `manifest` | Image header and manifest parsing and writing |
| `storage` | Chunked erase-and-program on top of [`ameba-storage`](../ameba-storage/README.md) |

## Features

| Feature | Default | Effect |
|:--------|:--------|:-------|
| `http-reqwless` | off | Enables the `http` module, pulling in `reqwless` and `embedded-nal-async` |

Without any feature enabled the crate provides the update machinery and the
`Transport` trait, letting an application supply its own download source — over
a serial link, from a filesystem, or from any other channel.

## Design notes

**No heap.** All buffers are caller-supplied fixed-size arrays, so the crate
works without a global allocator. Buffers are borrowed rather than owned so
they can live in statics instead of inflating the returned future.

**Watchdog friendly.** Interrupts are disabled only for the duration of a
single sector erase or page program — worst case about 8.5 ms for one
erase-plus-write cycle. A watchdog feed task running on a 500 ms period is
never starved, and no yield callback needs threading through the API.

> [!IMPORTANT]
>
> A hardware watchdog stays enabled across a reset once started — the enable
> bit is sticky and only a power cycle clears it. New firmware delivered by OTA
> must therefore start feeding the watchdog on every code path from its first
> instruction, including paths that decide to do nothing. Firmware that only
> feeds it after some conditional setup will be reset in a loop.

## Current limitations

- **HTTP only** — HTTPS is not implemented (see the verification split above
  for why it is a lower priority here than it would be elsewhere).
- **IP literals only** — DNS resolution is not wired up.
- **Pull model only** — the device fetches the image. A push model, where a
  caller feeds bytes in over an arbitrary link, is possible today by
  implementing `Transport`, but no such transport ships in the crate.
- **Single-image files** — an update file carrying multiple images is rejected.

## License

Apache-2.0 — see [LICENSE](../LICENSE).
