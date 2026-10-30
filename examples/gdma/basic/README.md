# gdma

## Build

```bash
cargo xtask build example --name gdma/basic
```

## Wiring

No external wiring needed.

## Behavior

GDMA CH0 asynchronously copies 64 bytes mem-to-mem, then compares the source and destination buffers.

## Expected Output

```
[GDMA] start
[GDMA] async done in Nms
[PASS] GDMA: mem-to-mem 64 bytes OK
```
