# xtask

[![License](https://badgen.net/badge/License/Apache%202.0/lightgrey)](../LICENSE)

Host-side build tool for the [ameba-rust](../README.md) SDK. Turns a Rust crate
into a flashable firmware image, programs boards over UART, and runs the lint
and documentation gates.

This is a [`cargo xtask`](https://github.com/matklad/cargo-xtask) — a plain
binary crate in the workspace, invoked as `cargo xtask <command>`, with no
global installation needed.

## Why a custom tool is needed

Producing a bootable image takes more than `cargo build`. The ELF has to be
split into its loadable sections, each section wrapped in a header carrying a
magic value, its size and its load address, and the result packaged together
with the network-core firmware. Flashing then has to program two images — the
bootloader and the application — at different flash addresses. Section
extraction, packaging and UART flashing are implemented in the `ameba-flash`
crate, which `xtask` calls into.

On top of that, every firmware crate is an independent mini-workspace, so a
single `cargo clippy` at the repository root only ever sees `xtask` itself. The
`check`, `lint` and `fmt` subcommands exist to walk all of them.

## Commands

### Build and run

```bash
cargo xtask build example --name hello-world   # one example → app.bin
cargo xtask build examples                     # every example
cargo xtask build doc                          # rustdoc for all firmware crates

cargo xtask run example --name hello-world     # build → flash via UART → monitor
```

Nested examples use their slash-separated path: `--name gpio/basic`. Output
lands in `build/<chip>/app.bin`, alongside `ota_all.bin` for OTA delivery.

### Quality gates

```bash
cargo xtask check              # cargo check, every crate
cargo xtask lint               # clippy, every crate  (--fix to auto-apply)
cargo xtask fmt                # rustfmt, every crate (--check for CI)
cargo xtask ci                 # all of the above plus docs
```

`ci` collects failures and prints a summary instead of stopping at the first
one. Restrict it with `--steps check,lint` when iterating. All of these accept
a package filter to narrow the crate set.

### Clean

```bash
cargo xtask clean              # build artifacts
cargo xtask clean --all        # also cargo target/ directories
```

## External requirements

| Requirement | Needed for | Discovery |
|:------------|:-----------|:----------|
| A serial port | `run example` | `--port`, else `RTK_SERIAL_PORT` |

`build doc`, `check`, `lint` and `fmt` are pure Rust steps and need none of the
above.

## License

Apache-2.0 — see [LICENSE](../LICENSE).
