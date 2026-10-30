# board2board/i2c-master

Pairs with `i2c-slave`. Flash this firmware onto the **master board**.

## Build

```bash
cargo xtask build example --name board2board/i2c-master
```

## Wiring

Connect the two boards directly (same-named pins tied together, common ground):

| Signal | Pin | Notes |
|------|------|------|
| SCL  | PA25 | Both ends use their internal pull-up |
| SDA  | PA26 | Same |
| GND  | GND  | Must be common |

## Behavior

1. Wait 1 s (to give the slave time to boot).
2. Master writes 32 bytes (0x01-0x20) to address 0x23.
3. After 100 ms, reads back 32 bytes (the slave's echo) and compares them.

## Expected Output

```
[I2C-B2B master] start
[I2C-B2B] wrote 32B to 0x23
[I2C-B2B] read 32B: 01 02 03 ... 1f 20
[PASS] I2C b2b: write+read OK
```
