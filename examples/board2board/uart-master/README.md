# board2board/uart-master

Pairs with `uart-slave`. Flash this firmware onto the **master board**.

## Build

```bash
cargo xtask build example --name board2board/uart-master
```

## Wiring

Cross-connect TX/RX, common ground:

| This board (A) | Peer (B) | Notes |
|-----------|-----------|------|
| PA23 (TX) | PA24 (RX) | A sends -> B receives |
| PA24 (RX) | PA23 (TX) | B sends -> A receives |
| GND       | GND       |      |

## Behavior

115200 bps, 8 rounds x 4 bytes each: after sending, waits for the slave's echo (500 ms timeout) and compares it.

## Expected Output

```
[UART-B2B master] start
[UART-B2B] round 1 sent: [11 21 31 01] recv: [11 21 31 01] OK
[UART-B2B] round 2 sent: [12 22 32 02] recv: [12 22 32 02] OK
...
[PASS] UART b2b: 8 rounds OK
```
