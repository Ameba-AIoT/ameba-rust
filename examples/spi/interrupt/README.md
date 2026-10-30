# spi/interrupt

## Build

```bash
cargo xtask build example --name spi/interrupt
```

## Wiring

SPI0: PA_7 (CLK), PA_8 (MOSI), PA_9 (MISO), PA_10 (CS0).
MOSI (PA_8) and MISO (PA_9) need a jumper wire for loopback (same as `spi/blocking`/`spi/async`).

## Behavior

Demonstrates registering a bare ISR callback directly on SPI, bypassing async
(`Spi::register_irq` / `SpiIrqHandler`), in contrast to `spi/async`'s waker +
executor scheduling path. `RxCounter::on_irq` runs directly in the SPI0 IRQ
context: it drains whatever data is currently in the RX FIFO using
`read_available` (non-blocking) and prints it, with log lines prefixed
`[SPI-IRQ]` to distinguish them from the main loop's `[SPI]` logs.

The main loop blocking-sends 5 rounds x 4 bytes, 50 ms apart; all reception
happens via the ISR callback (the main loop never reads RX). Reports `[PASS]`
if the total bytes received matches the total sent, `[FAIL]` otherwise.

## Expected Output

```
[SPI interrupt] start
[SPI] [1] sent [41, 42, 43, 01]
[SPI-IRQ] rx 4 byte(s): [41, 42, 43, 01] (from ISR)
[SPI] [2] sent [42, 43, 44, 02]
[SPI-IRQ] rx 4 byte(s): [42, 43, 44, 02] (from ISR)
...
[PASS] SPI interrupt: 20 bytes echoed via ISR OK
```
