# board2board/i2c-slave

Pairs with `i2c-master`. Flash this firmware onto the **slave board**.

## Build

```bash
cargo xtask build example --name board2board/i2c-slave
```

## Wiring

Connect the two boards directly (same-named pins tied together, common ground):

| Signal | Pin |
|------|------|
| SCL  | PA25 |
| SDA  | PA26 |
| GND  | GND  |

## Behavior

Slave address 0x23, infinite loop: wait for the master to write 32 bytes, then echo them back to the master unchanged.

## Expected Output

```
[I2C-B2B slave] start addr=0x23
[I2C-B2B] slave recv 32B: 01 02 03 ... 1f 20
[I2C-B2B] slave sent 32B echo
```
