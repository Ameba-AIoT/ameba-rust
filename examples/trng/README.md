# trng

## Build

```bash
cargo xtask build example --name trng
```

## Wiring

No external wiring needed.

## Behavior

Initializes the TRNG, then reads 10 consecutive 32-bit random numbers and prints them. Reports `[FAIL]` if any read times out or fails the APT check (`read_u32` returns `None`), or if all 10 readings are identical.

## Expected Output

```
[TRNG] start
[TRNG] #0: 0x3a1f9c22
[TRNG] #1: 0x81b40e77
...
[PASS] trng: OK
```
