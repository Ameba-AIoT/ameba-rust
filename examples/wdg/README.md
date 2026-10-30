# wdg

## Build

```bash
cargo xtask build example --name wdg
```

## Wiring

No external wiring needed.

## Behavior

Initializes and enables the system watchdog (`Wdg`/`CPU0_S_WDG_DEV`) with a 5 s timeout. Feeds it once every 500 ms (well inside the timeout window), printing 10 times before reporting `[PASS]`, then keeps feeding it to keep the board running -- if the feed logic were wrong, the board would reset within 5 s and the serial log would restart from `[WDG] start` instead of ever reaching `[PASS]`.

`IWDG_DEV` (the independent watchdog) is not covered by this driver: it's a single physical peripheral (unlike the system watchdog, which has separate secure/non-secure instances), and the NP (KM4NS) core's C firmware already takes it over unconditionally at boot (`main/km4ns/src/main.c` : `app_IWDG_init()`, 4095 ms timeout + auto-feed every 500 ms, starting as long as the `APBPeriph_IWDG` fuse bit is set, which is confirmed set on this board). Since NP is already feeding it as a backstop, IWDG can't detect "KM4TZ hung on its own", and reconfiguring it from the KM4TZ side would only risk contending with NP over its registers with no corresponding safety benefit -- so this driver does not provide `Iwdg`.

## Expected Output

```
[WDG] start
[WDG] system watchdog enabled, timeout=5000ms
[WDG] refresh #0
...
[WDG] refresh #9
[PASS] wdg: OK
```
