# i2c/blocking

## Build

```bash
cargo xtask build example --name i2c/blocking
```

## Wiring

| Signal | Pin | Notes |
|------|------|------|
| SCL  | PA25 | Needs an external pull-up (or use the internal pad pull-up) |
| SDA  | PA26 | Same |

No slave device needed -- the example sends a write command to the nonexistent address 0x55 to verify NACK error handling.

## Behavior

I2C0 master writes to address 0x55, expecting a NACK and a returned error.

## Expected Output

```
[I2C BLK] start
[PASS] I2C blocking: NACK OK
```
