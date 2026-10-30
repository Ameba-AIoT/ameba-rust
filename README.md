<div align="center">

# ameba-rust

**A bare-metal (`no_std`) Rust SDK for Realtek Ameba series chips, supporting Linux, Windows and macOS.**

[![Language](https://badgen.net/badge/language/Rust/orange)](https://github.com/Ameba-AIoT/ameba-rust/search?l=rust)
[![Target](https://badgen.net/badge/target/thumbv8m.main/blue)](https://doc.rust-lang.org/rustc/platform-support/thumbv8m.main-none-eabi.html)
[![License](https://badgen.net/badge/License/Apache%202.0/lightgrey)](LICENSE)
[![Last Commit](https://badgen.net/github/last-commit/Ameba-AIoT/ameba-rust/main?icon=github)](https://github.com/Ameba-AIoT/ameba-rust/commits/main)

[Documentation](https://aiot.realmcu.com/en/latest/rust/index.html) · [Products](https://aiot.realmcu.com/en/product/index.html)

</div>

ameba-rust is a bare-metal Rust SDK for Realtek Ameba series chips. It needs no
RTOS and no C compiler in your application: peripherals are driven directly
through a generated register layer, concurrency is handled by the
[Embassy](https://embassy.dev/) async executor, and the build produces a single
`app.bin` you flash to the board.

```rust
#![no_std]
#![no_main]

use ameba_rt as _;    // entry table + panic handler
use cortex_m as _;    // critical-section impl
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    ameba_hal::init(ameba_hal::Config::default());
    loop {
        ameba_hal::println!("Hello from Ameba");
        Timer::after(Duration::from_secs(1)).await;
    }
}
```

> [!NOTE]
>
> This SDK is pre-1.0. Peripheral coverage is broad, but APIs are not yet
> stable, the crates are not published to crates.io (use a Git or path
> dependency), and breaking changes land without a deprecation cycle.

## 🔌 Supported Chips

| Chip     | Rust core | Architecture | Target triple |
|:---------|:----------|:-------------|:--------------|
| RTL8721F | KM4TZ | Arm Cortex-M33 (ARMv8-M Mainline) | `thumbv8m.main-none-eabi` |

On RTL8721F, Rust runs on the KM4TZ application core while the KM4NS network
core keeps running Realtek's pre-built Wi-Fi firmware; the two communicate over
an IPC mailbox. This split lets the Rust side own the application without
reimplementing the Wi-Fi MAC/PHY layer.

## 🏗️ Repository Structure

```text
ameba-rust/
├── ameba-hal/            # Peripheral HAL (~45 modules) + embassy-time driver
│   ├── src/              #   GPIO, UART, SPI, I2C, timers, ADC, DMA, Ethernet …
│   ├── pac/rtl8721f/     #   rtl8721f-pac — svd2rust-generated register layer
│   └── ld/               #   Linker scripts
├── ameba-rt/             # Runtime: boot entry table, VTOR setup, panic handler
├── ameba-wifi/           # Wi-Fi: STA (open / WPA2-PSK), SoftAP, embassy-net driver
├── ameba-storage/        # NOR flash driver implementing embedded-storage traits
├── ameba-ota/            # Dual-slot (A/B) OTA update with HTTP transport
├── ameba-rom-sys/        # Boot-ROM symbol table for the linker + prebuilt images
├── ameba-flash/          # Host-side image packaging and UART flashing (lib + CLI)
├── examples/             # 38 runnable example crates, each with its own README
├── xtask/                # Host-side build tool: build / flash / lint / docs
└── scripts/              # Toolchain bootstrap
```

The root Cargo workspace contains **only `xtask`**. Every firmware crate is an
independent mini-workspace with its own `Cargo.lock` and `.cargo/config.toml`,
so each builds for the embedded target without the host tool interfering.

## ✨ Key Features

- **Async-first** — [Embassy](https://embassy.dev/) executor with a 1 MHz
  `embassy-time` driver backed by a hardware timer compare; blocking APIs are
  available where async brings no benefit
- **Peripherals** — GPIO, UART, SPI, I²C, PWM/timers, ADC, DMA, CAN, RTC,
  watchdog, TRNG, Ethernet, PSRAM, SDIO host, cap-touch, IR, audio (SPORT),
  thermal sensor and more
- **Wi-Fi** — STA with open and WPA2-PSK networks, SoftAP, and an
  [`embassy-net`](https://docs.rs/embassy-net/) driver so applications open
  their own sockets
- **Type-safe register access** — [`svd2rust`](https://docs.rs/svd2rust/)
  generated PAC
- **Ecosystem traits** — implements
  [`embedded-hal`](https://docs.rs/embedded-hal/),
  `embedded-hal-async`, [`embedded-storage`](https://docs.rs/embedded-storage/)
  and `embassy-net-driver`, so third-party driver crates work unmodified
- **OTA** — dual-bank A/B update over HTTP, with slot selection and signature
  verification handled by the bootloader
- **Stable Rust only** — no nightly features anywhere in the repository

## 🩺 Peripheral Status

Most drivers below are exercised on real hardware by a matching example in
[`examples/`](examples/), which is how "✔️ Verified" is defined here. A
smaller set compiles and links but has not yet been run on a board — see the
note below before relying on one of those in a product.

| Peripheral | Status | Peripheral | Status |
|:-----------|:------:|:-----------|:------:|
| GPIO | ✔️ | RTC | ✔️ |
| UART | ✔️ | Watchdog | ✔️ |
| SPI | ✔️ | TRNG | ✔️ |
| I²C | ✔️ | Ethernet (MAC) | ✔️ |
| ADC | ✔️ | Ethernet PHY | ✔️ |
| GDMA | ✔️ | PSRAM | ✔️ |
| Timer (basic/PWM) | ✔️ | SDIO host (controller) | ✔️ |
| CAN | ✔️ | CapTouch | ✔️ |
| Thermal sensor | ✔️ | IR (NEC TX) | ⚒️ |
| SPORT (I²S) | ✔️ | LCDC | 🚧 |
| Wi-Fi (STA/SoftAP) | ✔️ | Codec (audio) | 🚧 |
| OTA (A/B, HTTP) | ✔️ | UVC | 🚧 |
| Retention RAM | ⚒️ | PPE | 🚧 |
|  |  | Pseudo I²C | 🚧 |
|  |  | Audio clock | 🚧 |

✔️ **Verified** — has a runnable example with a recorded hardware pass.
⚒️ **Partial** — works but coverage is incomplete: IR's transmit waveform has
not been re-verified against a receiver/analyzer since a NEC-encoding fix;
retention RAM has no example of its own but is exercised indirectly through
`ameba-ota`'s real-hardware update flow.
🚧 **Unverified** — compiles and links; not yet run against real silicon.

> [!NOTE]
>
> This table reflects development status, not a stability or support
> commitment. Following the convention used by other `no_std` HALs, "not yet
> verified" means untested, not broken — treat 🚧 drivers as a starting point
> to validate on your own hardware before shipping.

## 🚀 Quick Start

Full setup instructions — host prerequisites, toolchain install, flashing and
troubleshooting — are in the
[Rust Development Guide](https://aiot.realmcu.com/en/latest/rust/index.html).
The outline:

**1. Clone the SDK**

```bash
git clone https://github.com/Ameba-AIoT/ameba-rust.git
```

The pre-built binaries (`boot.bin`, `km4ns_image2_all.bin` and the UART flash
loader) are committed under `ameba-rom-sys/bin/<chip>/`; image packaging and
UART flashing are implemented in Rust in the `ameba-flash` crate.

**2. Install the Rust toolchain**

The toolchain — stable channel plus the embedded target — is activated
automatically by [`rust-toolchain.toml`](rust-toolchain.toml) on the first cargo
invocation. If you don't have `rustup` yet:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Linking uses the `rust-lld` that ships with Rust, and `ameba-flash` packages and
flashes images.

**3. Build an example**

```bash
cargo xtask build example --name hello-world
```

The image lands in `build/rtl8721f/app.bin`. Nested examples use their
slash-separated path, e.g. `--name gpio/basic`.

**4. Flash and monitor**

```bash
cargo xtask run example --name hello-world --port <serial-port>
```

This flashes over UART and opens a serial monitor. The board is rebooted into
download mode automatically, so there is no need to hold any boot button.

## 📚 Documentation

- [Rust Development Guide](https://aiot.realmcu.com/en/latest/rust/index.html)
  — quick start, SDK download and the full user guide
- Per-crate documentation: [`ameba-hal`](ameba-hal/README.md) ·
  [`ameba-rt`](ameba-rt/README.md) · [`ameba-wifi`](ameba-wifi/README.md) ·
  [`ameba-storage`](ameba-storage/README.md) ·
  [`ameba-ota`](ameba-ota/README.md) ·
  [`ameba-rom-sys`](ameba-rom-sys/README.md) ·
  [`ameba-flash`](ameba-flash/README.md) ·
  [`rtl8721f-pac`](ameba-hal/pac/rtl8721f/README.md) ·
  [`xtask`](xtask/README.md)
- [`examples/`](examples/) — 38 examples, each with a README describing wiring
  and expected output
- For more information on the Ameba series chips, visit the
  [product page](https://aiot.realmcu.com/en/product/index.html)

## 🧩 Examples

| Area | Examples |
|:-----|:---------|
| Getting started | `hello-world`, `delay`, `logger`, `shell` |
| GPIO & timers | `gpio/{basic,async,interrupt}`, `tim`, `rtc`, `wdg` |
| Serial buses | `uart/{blocking,interrupt,async,dma}`, `spi/{blocking,interrupt,async,dma}`, `i2c/{blocking,async}`, `can` |
| DMA | `gdma/{basic,multiblock}` |
| Two-board tests | `board2board/{uart,spi,i2c}-{master,slave}` |
| Analog & sensors | `adc`, `thermal`, `trng` |
| Networking | `wifi-connect`, `wifi-ap`, `ethernet`, `mqtt`, `ota-http` |
| Benchmarks | `pbkdf2-bench` |

Each example directory carries its own README with wiring requirements and the
serial output to expect.

## 🛠️ Development

```bash
cargo xtask check          # cargo check across all crates
cargo xtask lint           # clippy across all crates
cargo xtask fmt            # rustfmt across all crates
cargo xtask build doc      # build rustdoc for all firmware crates
cargo xtask ci             # everything above, as a pre-push gate
```

Because each firmware crate is its own mini-workspace, a plain
`cargo clippy` at the repository root only covers `xtask` — use the
`xtask` subcommands above to cover everything.

## 💬 Feedback

- For questions or suggestions during development, visit the
  [Real-AIOT Forum](https://forum.real-aiot.com/).
- For bugs or feature requests, open a
  [GitHub Issue](https://github.com/Ameba-AIoT/ameba-rust/issues). Please check
  existing issues before opening a new one.

## 📄 License

Licensed under the Apache License, Version 2.0 — see [LICENSE](LICENSE).

The pre-built Realtek binaries under `ameba-rom-sys/bin/` (`boot.bin`,
`km4ns_image2_all.bin`, `floader_rtl8721f.bin`) are built from Realtek's
ameba-rtos and are subject to Realtek's reference-design disclaimer — see
[`Realtek_Disclaimer-2019.pdf`](Realtek_Disclaimer-2019.pdf).
