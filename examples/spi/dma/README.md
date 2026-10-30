# spi/dma

## Build

```bash
cargo xtask build example --name spi/dma
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

Master mode, GDMA CH0 (TX) + CH1 (RX) DMA-transfer 1024 bytes in a loopback, then compare the data after the transfer completes.

## Expected Output

```
[SPI DMA] start
[SPI DMA] transfer done
[PASS] SPI DMA: 1024 bytes loopback OK
```
