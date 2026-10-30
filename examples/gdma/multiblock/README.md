# gdma/multiblock

## Build

```bash
cargo xtask build example --name gdma/multiblock
```

## Behavior

Demonstrates GDMA's hardware auto-reload multi-block transfer, along with the
accompanying bare ISR callback (`DmaChannel::register_irq` / `GdmaIrqHandler`),
in contrast to `gdma/basic`'s single-block `mem_to_mem(...).await`.

`GDMA_ReloadDst`: the 128-byte SRC buffer is split into four 32-byte blocks,
while DST reuses a single 32-byte window repeatedly (after each block, DAR
automatically resets to its initial address while the SRC address advances
normally) -- the hardware continues the transfer between blocks without any
CPU intervention. Each completed block fires one interrupt;
`MultiblockHandler::on_irq` performs data verification and clears the reused
DST window directly in the GDMA0_CHANNEL0 IRQ context, and calls
`clean_auto_reload` on the second-to-last block so the final block triggers
`transfer_done` instead of reloading again -- this is a streaming/ping-pong
scenario that a one-shot, completion-only callback mechanism cannot cover.

The main loop's only job is to fill SRC, register the callback, start the
transfer, and `wfi()` while waiting for `done` -- all data movement and
verification happens inside the ISR; `[PASS]` is reported only once all 4
blocks have been verified.

## Expected Output

```
[GDMA multiblock] start
[GDMA-IRQ] block 0 done isr=0x02 (from ISR)
[GDMA-IRQ] block 1 done isr=0x02 (from ISR)
[GDMA-IRQ] block 2 done isr=0x02 (from ISR)
[GDMA-IRQ] block 3 done isr=0x01 (from ISR)
[PASS] GDMA multiblock: 4 blocks of 32 bytes streamed via per-block ISR OK
```
