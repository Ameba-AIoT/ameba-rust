# gpio/basic

## Build

```bash
cargo xtask build example --name gpio/basic
```

## Wiring

Jumper **PA13** and **PA14** together.

- PA14: output, driven by code
- PA13: input pull-up, follows PA14

## Behavior

Toggles PA14 10 times (200 ms apart), reading back PA13's level and comparing it each time. Reports `[PASS]` if all match, `[FAIL]` if any mismatch.

## Expected Output

```
[GPIO basic] start
[GPIO] [0] PA14=L PA13=L
[GPIO] [1] PA14=H PA13=H
...
[PASS] GPIO basic: OK
```
