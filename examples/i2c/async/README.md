# i2c/async

## Build

```bash
cargo xtask build example --name i2c/async
```

## Wiring

| Signal | Pin |
|------|------|
| SCL  | PA25 |
| SDA  | PA26 |

No slave device needed.

## Behavior

- **Phase 1**: sends a write command to address 0x55 (no slave present), expecting a NACK within 500 ms.
- **Phase 2**: 30 asynchronous writes plus a heartbeat task (5 ms tick), verifying the executor isn't blocked (heartbeat >= 2).

## Expected Output

```
[I2C ASYNC] start
[PASS] I2C async: NACK OK
[I2C] concurrent: 30 writes start
[HB] 0
[HB] 1
...
[I2C] heartbeats seen: N
[PASS] I2C async: non-blocking OK
```
