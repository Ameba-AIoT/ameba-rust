# rtc

## Build

```bash
cargo xtask build example --name rtc
```

## Wiring

No external wiring needed.

## Behavior

Sets the RTC to 2025-01-01 12:00:00, waits 3 s, reads it back, and checks that the difference is >= 2 seconds (verifying the RTC is ticking).

## Expected Output

```
[RTC] start
[RTC] time set to 2025-01-01 12:00:00, waiting 3s...
[RTC] read-back: 2025-001 12:00:03
[RTC] diff: 3s  (embassy elapsed: 3000ms)
[PASS] RTC: tick OK
```
