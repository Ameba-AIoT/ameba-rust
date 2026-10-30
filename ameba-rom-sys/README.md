# ameba-rom-sys

[![License](https://badgen.net/badge/License/Apache%202.0/lightgrey)](../LICENSE)
[![Target](https://badgen.net/badge/target/thumbv8m.main/blue)](https://doc.rust-lang.org/rustc/platform-support/thumbv8m.main-none-eabi.html)

Boot-ROM symbol table for Realtek Ameba series chips. Supplies the linker with
the addresses of functions burned into chip ROM, and stages the Realtek-provided
pre-built binaries a firmware image is packaged with.

Part of the [ameba-rust](../README.md) SDK.

## What this crate is for

Unusually for a `-sys` crate, `ameba-rom-sys` declares no `extern "C"` bindings.
Its entire contribution is a build-script side effect: adding its linker-script
directory to the link search path, so `ameba.x` can resolve

```
INCLUDE ameba_rom_symbol.x
```

That script assigns roughly 560 ROM addresses via `PROVIDE(...)`, which is a
*weak* assignment — a symbol nobody references simply doesn't appear in the
image, so exposing the whole table costs nothing.

```
ameba-rom-sys/
├── src/lib.rs                     # empty apart from #![no_std]
├── build.rs                       # emits cargo:rustc-link-search
├── ld/<chip>/ameba_rom_symbol.x   # ~560 PROVIDE(symbol = address) entries
└── bin/<chip>/                    # committed pre-built binaries
```

Most crates in this SDK list `ameba-rom-sys` as a dependency purely to trigger
that build script. If a crate did once call into ROM and no longer does, the
dependency still needs to stay — it is what makes the symbol table findable at
link time.

## Why there are no ROM bindings left

Earlier revisions declared a couple of dozen ROM functions here — timers, pinmux,
clock gating, delays, UART character I/O, inter-core semaphores. All of them are
gone: [`ameba-hal`](../ameba-hal/README.md) now drives those peripherals through
[`rtl8721f-pac`](../ameba-hal/pac/rtl8721f/) register access instead.

That was a deliberate direction, for three reasons:

- **ROM addresses drift between silicon revisions.** Comparing two revisions of
  the same part's symbol table, 791 of 798 shared symbols sit at different
  addresses. Anything bound to a raw address is pinned to one revision.
- **ROM code cannot be inspected, stepped through or patched.** A bug in a ROM
  routine is permanent; a bug in a Rust driver is a commit.
- **Register access is type-safe.** The generated PAC makes an invalid field
  write a compile error, where a ROM call takes untyped `u32`s.

The symbol table itself stays, both because it costs nothing and because the
boot and Wi-Fi paths still rely on ROM entry points the linker must be able to
resolve.

## Pre-built binaries

`bin/<chip>/` holds three pre-built binaries and one generated list, committed
to this repository:

| File | Role |
|:-----|:-----|
| `boot.bin` | First-stage bootloader — performs slot selection and signature verification, then enters the Rust image |
| `km4ns_image2_all.bin` | Network-core firmware, containing the Wi-Fi MAC and PHY implementation |
| `floader_<chip>.bin` | RAM-resident flash loader that UART flashing downloads into SRAM first; it is never written to flash |
| `np_api_ids.txt` | The `api_id`s the network-core firmware implements; `cargo xtask build` rejects `ameba-wifi` calls that use any other id |

`cargo xtask build` and `ameba-flash` read them directly from this directory.

> [!NOTE]
>
> These binaries come from the ameba-rtos C SDK and are subject to Realtek's
> reference-design disclaimer. See
> [`Realtek_Disclaimer-2019.pdf`](../Realtek_Disclaimer-2019.pdf).

## License

The crate's own source is Apache-2.0 — see [LICENSE](../LICENSE). The
binaries under `bin/` are subject to Realtek's reference-design disclaimer; see
[`Realtek_Disclaimer-2019.pdf`](../Realtek_Disclaimer-2019.pdf).
