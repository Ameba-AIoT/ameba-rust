# ameba-rt

[![License](https://badgen.net/badge/License/Apache%202.0/lightgrey)](../LICENSE)
[![Target](https://badgen.net/badge/target/thumbv8m.main/blue)](https://doc.rust-lang.org/rustc/platform-support/thumbv8m.main-none-eabi.html)

Minimal runtime that gets an Ameba chip from the bootloader's hand-off into
Rust `main`. It provides the boot entry table the bootloader looks for, an early
stack switch, vector table setup, and diagnostic panic and hard-fault handlers.

Part of the [ameba-rust](../README.md) SDK.

## Usage

Depend on it and link it in — that is the entire API surface:

```toml
[dependencies]
ameba-rt = { git = "https://github.com/Ameba-AIoT/ameba-rust" }
```

```rust
#![no_std]
#![no_main]

use ameba_rt as _;    // entry table, panic handler, HardFault handler
use cortex_m as _;    // critical-section implementation

#[cortex_m_rt::entry]
fn main() -> ! {
    ameba_hal::init(ameba_hal::Config::default());
    loop {
        cortex_m::asm::wfi();
    }
}
```

The `use ameba_rt as _;` import exists only to force the linker to pull the
crate in; nothing is called from application code. Without it the entry table
and panic handler would be dropped as unused and the image would not boot.

## What it does

**Boot entry table.** The bootloader looks for a fixed three-word table at the
start of application RAM: a start function pointer, a wake-up function pointer,
and a non-secure vector table pointer. This crate emits that table into a
dedicated linker section, with the start pointer aimed at a trampoline.

**Early stack switch.** The bootloader calls the entry point with a direct
branch, so the main stack pointer is never reloaded from a vector table and
still points at the small stack the bootloader was using — only a couple of
kilobytes below the ROM-fixed stack limit. Runtime startup (BSS clear, `.data`
copy) plus the first function that puts a flash sector buffer on the stack all
overflow that window. The trampoline therefore clears `MSPLIM`, retargets `MSP`
at the application stack top, and only then tail-calls the runtime reset
handler.

**Vector table relocation.** A `#[pre_init]` hook writes `SCB->VTOR` before BSS
is cleared, so the application's own vector table is active before any exception
can fire.

**Panic handler.** Formats the full `PanicInfo` — message plus file, line and
column — and writes it out over LogUART. It talks to the log UART through a
single low-level function rather than depending on `ameba-hal`, which keeps the
panic path free of the HAL's initialisation requirements.

**Hard-fault handler.** A CPU fault does not go through `#[panic_handler]`, and
the default handler is a silent spin at the highest exception priority — it
blocks every other interrupt with no output, which on a two-core part is hard to
tell apart from a stall on the other core. This crate's handler instead dumps
the stacked `pc` and `lr` plus `CFSR`, `HFSR`, `MMFAR` and `BFAR` before
spinning, so the fault class and faulting address are visible on the serial
console.

## Relationship to `cortex-m-rt`

This crate builds on [`cortex-m-rt`](https://docs.rs/cortex-m-rt/) rather than
replacing it: `cortex-m-rt` still owns `Reset`, BSS/`.data` initialisation and
the `#[entry]` attribute. `ameba-rt` adds the chip-specific pieces
`cortex-m-rt` cannot know about — the boot entry table, the stack switch, and
the vector table address.

`cortex-m-rt` is enabled with its `device` feature so the interrupt vector array
comes from `rtl8721f-pac` instead of its own fallback. This must stay consistent
with the PAC's `rt` feature as enabled by `ameba-hal`: turning on only one of
the two either duplicates or omits the vector array at link time.

## License

Apache-2.0 — see [LICENSE](../LICENSE).
