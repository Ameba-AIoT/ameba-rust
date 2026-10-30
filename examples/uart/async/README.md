# uart/async

## Build

```bash
cargo xtask build example --name uart/async
```

## Wiring

No external wiring needed -- uses UART0's internal loopback.

## Behavior

- **Phase 1**: 3 rounds x 4 bytes of asynchronous send/receive loopback, 500 ms timeout.
- **Phase 2**: an asynchronous 256-byte write plus a heartbeat task (every 5 ms), verifying the executor isn't blocked (heartbeat >= 2).

## Expected Output

```
[UART ASYNC] start
[UART ASYNC] round 1 tx=[41 42 43 01] rx=[41 42 43 01] OK
[UART ASYNC] round 2 tx=[42 43 44 02] rx=[42 43 44 02] OK
[UART ASYNC] round 3 tx=[43 44 45 03] rx=[43 44 45 03] OK
[PASS] UART async: echo OK
[UART] concurrent: 256B write start
[UART] heartbeats seen: N
[PASS] UART async: non-blocking write OK
```
