# ameba-hal

[![License](https://badgen.net/badge/License/Apache%202.0/lightgrey)](../LICENSE)
[![Target](https://badgen.net/badge/target/thumbv8m.main/blue)](https://doc.rust-lang.org/rustc/platform-support/thumbv8m.main-none-eabi.html)

Peripheral hardware abstraction layer for Realtek Ameba series chips. This is
the crate applications spend most of their time in: it owns chip bring-up,
provides drivers for roughly 45 peripheral blocks, and supplies the
`embassy-time` driver that makes `async` timing work.

Part of the [ameba-rust](../README.md) SDK.

## Usage

Add the dependency and call `init()` as the first statement in `main`:

```toml
[dependencies]
ameba-hal = { git = "https://github.com/Ameba-AIoT/ameba-rust" }
```

```rust
use ameba_hal::gpio::{GpioPin, Output, Pin, Pull};
use embedded_hal::delay::DelayNs;

#[cortex_m_rt::entry]
fn main() -> ! {
    ameba_hal::init(ameba_hal::Config::default());

    let led = GpioPin::<Output>::builder(Pin::PA14).pull(Pull::NoPull).output();
    let mut delay = ameba_hal::delay::Delay::new();

    loop {
        led.toggle();
        delay.delay_ms(500);
    }
}
```

GPIO uses a typestate builder: `GpioPin<Input>`, `GpioPin<Output>` and
`GpioPin<Interrupt>` expose only the methods valid for that direction, so
reading an output-only pin or arming an interrupt on a plain input is a compile
error rather than a silent no-op.

`init()` must run before any peripheral is touched — it clears boot-ROM BSS
residue, brings up oscillators and clocks, starts the free-running SysTick
counter that blocking delays depend on, and optionally starts the
`embassy-time` driver and inter-core IPC.

### Configuration

`Config` selects which chip-level subsystems `init()` starts:

```rust
ameba_hal::init(ameba_hal::Config {
    sys_timer: true,   // embassy-time driver — required for Timer::after etc.
    ipc: false,        // inter-core IPC — required by ameba-wifi
});
```

`Config::default()` enables both. Blocking-only applications that don't use
`embassy-time` can set `sys_timer: false`; the SysTick counter backing
[`delay::Delay`](src/delay.rs) starts unconditionally either way.

## Modules

| Area | Modules |
|:-----|:--------|
| Core / chip | `clk`, `rcc`, `pll`, `reset`, `pinmux`, `interrupt`, `mpu`, `cache`, `otp`, `ldo`, `bor` |
| Timing | `sys_timer`, `delay`, `debugtimer`, `tim`, `aontimer`, `pmctimer`, `rtc`, `wdg` |
| Serial buses | `uart`, `loguart`, `spi`, `i2c`, `pseudo_i2c`\*, `can`, `sport` |
| GPIO & input | `gpio`, `wakepin`, `captouch`, `ir` |
| Analog | `adc`, `thermal`, `trng` |
| Memory & storage | `psram`, `rram`, `sd_host`, `dma` |
| Networking | `ethernet`, `phy` |
| Display & media | `lcdc`\*, `ppe`\*, `codec`\*, `audio_clock`\*, `uvc`\* |
| Cross-core | `ipc` |
| Diagnostics | `logger`, `macros` (`println!` and friends) |

\* Not built by default — these compile but have not been run against real
hardware yet. Enable with the `unstable` Cargo feature.

Each module's rustdoc documents its own API; build it with
`cargo xtask build doc` from the repository root.

## Timing model

- **`embassy-time` driver** — a hardware PWM timer at 1 MHz with a compare
  register, giving 1 µs resolution for `Timer::after`, timeouts and
  `select`/`join` combinators. Which timer backs it is selected by a Cargo
  feature (`time-driver-tim4` by default, `tim5`–`tim7` available so the
  default timer can be freed for user PWM).
- **Blocking delays** — [`delay::Delay`](src/delay.rs) implements both the
  blocking and async `DelayNs` traits, using SysTick as a free-running 24-bit
  counter (no interrupt handler). The blocking path works before the executor
  starts and inside interrupt handlers.

## Register access

Register access goes through [`rtl8721f-pac`](pac/rtl8721f/), a
[`svd2rust`](https://docs.rs/svd2rust/)-generated crate re-exported as
`ameba_hal::pac`. Drivers derive peripheral base addresses from the PAC's
`::PTR` constants rather than hand-written literals, so an SVD correction
propagates automatically.

## Implemented traits

Drivers implement the standard embedded Rust traits wherever they apply, so
third-party driver crates work unmodified:

- [`embedded-hal`](https://docs.rs/embedded-hal/) 1.0 — `SpiBus`, `I2c`,
  `digital::{InputPin, OutputPin, StatefulOutputPin}`, `delay::DelayNs`
- [`embedded-hal-async`](https://docs.rs/embedded-hal-async/) — async
  counterparts where the peripheral supports interrupt-driven operation
- [`embedded-io`](https://docs.rs/embedded-io/) and `embedded-io-async` — UART
  read/write
- [`embassy-time-driver`](https://docs.rs/embassy-time-driver/) — the 1 MHz
  time driver described above

## License

Apache-2.0 — see [LICENSE](../LICENSE).
