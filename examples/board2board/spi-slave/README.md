# board2board/spi-slave

Pairs with `spi-master`. Flash this firmware onto the **slave board**.

## Build

```bash
cargo xtask build example --name board2board/spi-slave
```

## Wiring

Connect the two boards directly (same-named pins tied together, common ground). CLK/CS are driven by the master.

| Signal | Pin |
|------|------|
| CLK  | PA7  |
| MOSI | PA8  |
| MISO | PA9  |
| CS   | PA10 |
| GND  | GND  |

## Behavior

Each round preloads MISO with [0xA0..0xAF], waits for the master to initiate a transfer, prints the received master data, and loops forever.

## Expected Output

```
[SPI-B2B slave] start
[SPI-B2B] slave rx (round 1): 01 02 03 ... 0e 0f
[SPI-B2B] slave rx (round 2): 01 02 03 ... 0e 0f
...
```
