# board2board/spi-master

Pairs with `spi-slave`. Flash this firmware onto the **master board**.

## Build

```bash
cargo xtask build example --name board2board/spi-master
```

## Wiring

Connect the two boards directly (same-named pins tied together, common ground). The master drives CLK and CS.

| Signal | Pin |
|------|------|
| CLK  | PA7  |
| MOSI | PA8  |
| MISO | PA9  |
| CS   | PA10 |
| GND  | GND  |

## Behavior

Clock ~240 kHz, full-duplex transfer of 16 bytes: the master sends [0x01..0x10] while simultaneously receiving the slave's preloaded [0xA0..0xAF], then verifies the received data.

## Expected Output

```
[SPI-B2B master] start
[SPI-B2B] master rx: a0 a1 a2 ... ae af
[PASS] SPI b2b: full-duplex OK
```
