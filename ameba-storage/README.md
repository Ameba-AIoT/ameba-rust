# ameba-storage

[![License](https://badgen.net/badge/License/Apache%202.0/lightgrey)](../LICENSE)
[![Target](https://badgen.net/badge/target/thumbv8m.main/blue)](https://doc.rust-lang.org/rustc/platform-support/thumbv8m.main-none-eabi.html)

NOR flash driver for Realtek Ameba series chips, implementing the
[`embedded-storage`](https://docs.rs/embedded-storage/) traits so any
filesystem or key-value crate built on them works unmodified.

Part of the [ameba-rust](../README.md) SDK.

## Usage

```toml
[dependencies]
ameba-storage = { git = "https://github.com/Ameba-AIoT/ameba-rust" }
```

```rust
use ameba_storage::{Flash, FlashStorage};
use embedded_storage::nor_flash::NorFlash;

let mut storage = FlashStorage::new(Flash::new());
println!("capacity: {} bytes", storage.capacity());

let mut buf = [0u8; 256];
storage.read(0x10_0000, &mut buf)?;

storage.erase(0x10_0000, 0x10_1000)?;   // one 4 KiB sector
storage.write(0x10_0000, b"hello")?;
```

Reads go through the memory-mapped XIP window and need no setup. Writes and
erases briefly take the flash controller out of XIP mode, which is why they
need the safeguards described below.

## Safety model

Two things make writing to this flash different from a typical SPI flash
driver, and both are handled internally:

**Cross-core arbitration.** Both cores share one flash chip, and the network
core fetches its own code from it. Every write and erase therefore acquires a
hardware semaphore and signals the other core to pause flash access before
touching the controller, then releases both afterwards. Without this, a write
issued while the other core is fetching instructions corrupts that fetch.

**Execution from SRAM.** While the flash controller is in user mode, the XIP
window is unusable — code cannot be fetched from flash. All flash-touching
functions are therefore placed in a dedicated SRAM-only linker section, so the
CPU is never executing from flash while flash is busy.

Interrupts are disabled for the duration of a write or erase. Measured worst
case on a 4 KiB sector is about 1.4 ms for an erase and 7 ms for a program,
which is short enough that a watchdog with a sub-second feed interval is never
starved.

## Features

| Feature | Default | Effect |
|:--------|:--------|:-------|
| `embedded-storage` | on | Implements `ReadStorage`, `Storage`, `ReadNorFlash` and `NorFlash` |
| `emulation` | off | Swaps the hardware backend for an in-memory array, so logic above the driver can be unit-tested on the host |

## Architecture

```text
application / filesystem crate
      ↓   embedded-storage traits
storage.rs · nor_flash.rs        trait impls
      ↓
common.rs                        bounds and alignment checks, FlashStorage
      ↓   compile-time backend selection
hardware.rs                      real chip — flash controller + cross-core locking
stub.rs                          in-memory array (feature = "emulation")
```

The backend is chosen at compile time, not dispatched at runtime, so the unused
one costs nothing in the final image.

Geometry follows standard NOR flash: 4 KiB erase sectors, 256-byte program
pages, with the chip identified at startup by its JEDEC ID. Only NOR flash is
supported — these parts can also boot from NAND, but that path is not
implemented here.

## Related

- [`ameba-ota`](../ameba-ota/README.md) builds dual-slot firmware update on top
  of this crate.
- [`examples/ota-http`](../examples/ota-http/) exercises it directly via
  `FlashStorage`.

## License

Apache-2.0 — see [LICENSE](../LICENSE).
