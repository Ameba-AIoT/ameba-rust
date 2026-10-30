# ameba-flash

Host-side image packaging and UART flashing for the Ameba Rust SDK. Library and
command-line tool in one crate.

- `elf` — extracts the loadable sections of a firmware ELF into the
  `km4tz_image2_all.bin` sub-image.
- `pack` — assembles `app.bin` (certificate, manifest, NP and AP images) and the
  `ota_all.bin` update package.
- `flash` — the UART flashing protocol: ROM-stage handshake, flash-loader
  download, then erase / write / verify of `boot.bin` and `app.bin`.
- `monitor` — serial log monitor.
- `prebuilt` — the bootloader, NP image and flash loader that are flashed with
  every application.

`cargo xtask build` and `cargo xtask run` call into this crate.

## Command line

```bash
cargo install --path ameba-flash

ameba-flash flash <ELF> --port <PORT> [--monitor] [--baud <BAUD>]
```

`<PORT>` is a serial port such as `COM29` or `/dev/ttyUSB0`. Without
`--port`, the `RTK_SERIAL_PORT` environment variable is used. The board is
rebooted into download mode automatically when it is running normal firmware.

Each example's `.cargo/config.toml` sets `ameba-flash` as the cargo runner, so
an example can be flashed and monitored with:

```bash
cd examples/hello-world
cargo run --release -- --port <PORT>
# or, with RTK_SERIAL_PORT set:
cargo run --release
```

## License

Apache-2.0.
