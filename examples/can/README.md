# can

## Build

```bash
cargo xtask build example --name can
```

## Wiring

No external CAN bus needed -- the example only sends a single frame and does not wait for an ACK (TX may not complete with no nodes on the bus, but the driver layer does not block on it).

## Behavior

Initializes CAN0 and sends a data frame with Standard ID 0x123, DLC 8 (data 0x01-0x08).

## Expected Output

```
[CAN] start
[CAN] Sending frame id=0x123
[PASS] CAN TX: done
```
