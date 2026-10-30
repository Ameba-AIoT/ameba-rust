# spi/blocking

## Build

```bash
cargo xtask build example --name spi/blocking
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

Master mode, writes 16 bytes (0x01-0x10) to the TX FIFO, loops them back through MOSI->MISO, and reads them back for comparison.

## Expected Output

```
[SPI BLK] start
[PASS] SPI blocking: OK
```
