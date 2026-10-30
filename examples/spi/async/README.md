# spi/async

## Build

```bash
cargo xtask build example --name spi/async
```

## Wiring

SPI0 loopback: jumper **PA8 (MOSI)** and **PA9 (MISO)** together.

| Signal | Pin |
|------|------|
| CLK  | PA7  |
| MOSI | PA8  |
| MISO | PA9  |
| CS   | PA10 |

## Behavior

Master mode, clock ~10 kHz (~0.8 ms per byte), transfers 16 bytes while a heartbeat task runs concurrently (every 5 ms). Verifies the executor isn't blocked during the transfer (heartbeat >= 2) and that the loopback data is correct.

## Expected Output

```
[SPI async] start
[HB] 1
[HB] 2
...
[SPI] heartbeats during transfer: N
[PASS] SPI async: loopback + non-blocking OK
```
