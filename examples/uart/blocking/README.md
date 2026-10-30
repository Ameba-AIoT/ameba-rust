# uart/blocking

## Build

```bash
cargo xtask build example --name uart/blocking
```

## Wiring

No external wiring needed -- uses UART0's internal loopback (`loopback_cmd(true)`).

## Behavior

3 rounds x 4 bytes of blocking send/receive loopback; each round prints the sent/received data and compares it.

## Expected Output

```
[UART BLK] start
[UART BLK] round 1 sent:[41424301] recv:[41424301]
[UART BLK] round 2 sent:[42434402] recv:[42434402]
[UART BLK] round 3 sent:[43444503] recv:[43444503]
[PASS] UART blocking: OK
```
