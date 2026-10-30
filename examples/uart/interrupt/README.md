# uart/interrupt

## Build

```bash
cargo xtask build example --name uart/interrupt
```

## Wiring

No external wiring needed -- uses UART0's internal loopback (`loopback_cmd(true)`).

## Behavior

Demonstrates registering a bare ISR callback directly on UART, bypassing
async (`Uart::register_irq` / `UartIrqHandler`), in contrast to `uart/async`'s
waker + executor scheduling path. `RxCounter::on_irq` runs directly in the
UART0 IRQ context: it drains whatever data is currently in the Rx FIFO using
`read_available` (non-blocking, unlike `embedded_io::Read`, which busy-waits
when there's no data) and prints it, with log lines prefixed `[UART-IRQ]` to
distinguish them from the main loop's `[UART]` logs.

The main loop blocking-sends 5 rounds x 4 bytes, 50 ms apart; all reception
happens via the ISR callback (the main loop never reads RX). Reports `[PASS]`
if the total bytes received matches the total sent, `[FAIL]` otherwise.

## Expected Output

```
[UART interrupt] start
[UART] [1] sent [41, 42, 43, 01]
[UART-IRQ] rx 4 byte(s): [41, 42, 43, 01] (from ISR)
[UART] [2] sent [42, 43, 44, 02]
[UART-IRQ] rx 4 byte(s): [42, 43, 44, 02] (from ISR)
...
[PASS] UART interrupt: 20 bytes echoed via ISR OK
```
