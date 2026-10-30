# tim

## Build

```bash
cargo xtask build example --name tim
```

## Behavior

Demonstrates the full usage flow of `ameba-hal`'s hardware timer (Timer0, basic mode):

- Clock source: SDM32K, 32768 Hz (basic timers Timer0-3 always use this clock, no prescaler)
- Period: ARR = 32767, i.e. one overflow per second (32768 ticks)
- Each overflow triggers `TIMER0_IRQ`; the ISR clears the SR flag (W1C + 32kHz cross-domain sync) and increments `IRQ_COUNT`
- The main task waits 5 seconds, checking `IRQ_COUNT` every second, and finally verifies it falls within a reasonable range (4-6)

Corresponding SDK example: `example/peripheral/raw/Timer/raw_gtimer/example_raw_gtimer.c`

## Expected Output

```
[TIM] hardware timer test start
[TIM] timer0 started (32kHz clock, 1s period)
[TIM] irq_count=1 expected=1
[TIM] irq_count=2 expected=2
[TIM] irq_count=3 expected=3
[TIM] irq_count=4 expected=4
[TIM] irq_count=5 expected=5
[PASS] timer0: irq_count=5
```

Prints `[FAIL]` if the count deviates by more than +/-1.
