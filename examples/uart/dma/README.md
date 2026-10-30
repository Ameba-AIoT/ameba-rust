# uart/dma

## Build

```bash
cargo xtask build example --name uart/dma
```

## Wiring

No external wiring needed -- uses UART0's internal loopback.

## Behavior

GDMA CH0 (TX) + CH1 (RX) DMA-transfer 256 bytes over the UART loopback, then compare the data.

## Expected Output

```
[UART DMA] start
[UART DMA] transfer done
[PASS] UART DMA: 256 bytes loopback OK
```
