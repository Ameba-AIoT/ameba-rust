# gpio/async

## Build

```bash
cargo xtask build example --name gpio/async
```

## Wiring

Jumper **PA13** and **PA14** together.

- PA14: output, driven by code
- PA13: input pull-up, follows PA14

## Behavior

- **Phase 1**: PA14 starts high, is pulled low after 100 ms, and PA13 waits up to 200 ms for the falling edge (`wait_for_falling_edge`).
- **Phase 2**: toggle_task flips PA14 every 500 ms for 20 total toggles; edge_task fires `wait_for_any_edge` on each edge and counts it. After 10.5 s, reports whether toggles/edges are both >= 18, with `[PASS]`/`[FAIL]`.

## Expected Output

```
[GPIO async] start
[PASS] GPIO: falling edge detected
[GPIO] PA14 toggle 1
[GPIO] edge 1
[GPIO] PA14 toggle 2
[GPIO] edge 2
...
[PASS] GPIO async: 20 toggles 20 edges OK
```
