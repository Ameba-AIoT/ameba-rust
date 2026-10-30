# gpio/interrupt

## Build

```bash
cargo xtask build example --name gpio/interrupt
```

## Wiring

Jumper **PA13** and **PA14** together.

- PA14: output, driven by code
- PA13: interrupt input pull-up, falling-edge triggered, follows PA14

## Behavior

Demonstrates registering a bare ISR callback directly, bypassing async
(`GpioPin::register_irq` / `GpioIrqHandler`), in contrast to `gpio/async`'s
waker + executor scheduling path. PA13's interrupt handler
(`EdgeCounter::on_irq`) runs directly in the GPIO IRQ context, printing its
own log lines prefixed with `[GPIO-IRQ]` to distinguish them from the main
loop's `[GPIO]` log lines.

PA14 starts high and toggles 10 times (200 ms apart), which should produce 5
falling edges. Reports `[PASS]` if the count matches, `[FAIL]` otherwise.

## Expected Output

```
[GPIO interrupt] start
[GPIO-IRQ] edge 1 (port=0 pin=13, from ISR)
[GPIO] [0] PA14=L edge_count=1
[GPIO] [1] PA14=H edge_count=1
...
[PASS] GPIO interrupt: 5 edges OK
```
