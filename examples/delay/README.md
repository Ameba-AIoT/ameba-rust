# delay

## Build

```bash
cargo xtask build example --name delay
```

## Wiring

No external wiring needed.

## Behavior

Verifies the `Delay` driver's accuracy. Tests run in sequence:

| Test | Target | Pass window |
|------|------|---------|
| blocking delay_ms(50) | 50 ms | 47-55 ms |
| blocking delay_us(1000)x100 | 100 ms | 90-120 ms |
| blocking delay_ns(100000)x100 | 10 ms | 9-13 ms |
| blocking delay_ns(500)x10000 | ~5 ms | 4-12 ms |
| async delay_ms(50) | 50 ms | 49-55 ms |
| async delay_us(1000)x100 | 100 ms | 90-120 ms |

## Expected Output

```
[blocking] delay_ms(50) elapsed: 50ms
[PASS]
[blocking] delay_us(1000)*100 elapsed: 100ms
[PASS]
...
[async] delay_ms(50) elapsed: 50ms
[PASS]
...
```
