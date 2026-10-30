# pbkdf2-bench

## Build

```bash
cargo xtask build example --name pbkdf2-bench
```

## Wiring

No external wiring needed.

## Behavior

Times one PBKDF2-HMAC-SHA1 derivation with 4096 iterations and a 32-byte
output, the computation WPA2-PSK uses to turn a passphrase and SSID into the
pairwise master key. It runs once on the application core at 240 MHz using the
[`pbkdf2`](https://crates.io/crates/pbkdf2) and [`sha1`](https://crates.io/crates/sha1)
crates, then idles.

The result tells you how long a WPA2-PSK join blocks the CPU for this step, and
whether that fits within a watchdog timeout or other timing budget in your
application.

## Expected Output

```
PBKDF2-SHA1-4096 elapsed: 361300 us (361 ms)
```

Prints a single line. The figure above was measured on an RTL8721F with this
example's release profile (`opt-level = "z"`); other build settings give
different results.
