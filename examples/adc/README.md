# adc

## Build

```bash
cargo xtask build example --name adc
```

## Wiring

Connect **PA17** (ADC CH3) to 3.3V (or GND). The example's pass window is 2.8-3.6V; connecting to GND reports `[FAIL]`.

## Behavior

Software-triggers a single-shot sample of CH3 (PA17) 5 times at 200 ms intervals, converts the raw reading to mV using the EFuse calibration curve, and checks whether the result falls within the 2800-3600 mV range.

## Expected Output

```
[ADC] start
[ADC] raw: NNNN voltage: 3.30V
[ADC] raw: NNNN voltage: 3.30V
...
[PASS] ADC: voltage OK
[ADC] timer-trig raw: NNNN voltage: 3.30V
[PASS] ADC: timer-trigger OK
```
