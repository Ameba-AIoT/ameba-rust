# thermal

## Build

```bash
cargo xtask build example --name thermal
```

## Wiring

No external wiring needed -- this uses the on-chip temperature sensor.

## Behavior

Samples once per second for a total of 5 readings, converts to Celsius, and prints each. Reports `[FAIL]` if the temperature falls outside the -10-110 C range.

## Expected Output

```
[THERMAL] start
[THERMAL] Temp: 35.2 C
[THERMAL] Temp: 35.3 C
...
[PASS] thermal: OK
```
