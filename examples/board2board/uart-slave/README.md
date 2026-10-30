# board2board/uart-slave

Pairs with `uart-master`. Flash this firmware onto the **slave board**.

## Build

```bash
cargo xtask build example --name board2board/uart-slave
```

## Wiring

Cross-connect TX/RX, common ground:

| This board (B) | Peer (A) |
|-----------|-----------|
| PA23 (TX) | PA24 (RX) |
| PA24 (RX) | PA23 (TX) |
| GND       | GND       |

## Behavior

115200 bps, infinite loop: once all 4 bytes arrive, echoes them back unchanged and prints each round's content.

## Expected Output

```
[UART-B2B slave] start
[UART-B2B] round 1 echo: [11 21 31 01]
[UART-B2B] round 2 echo: [12 22 32 02]
...
```
