# wifi-ap

## Build

```bash
cargo xtask build example --name wifi-ap
```

## Wiring

No external wiring needed.

## Behavior

Starts an open SoftAP named `ameba-rust-ap` on channel 1, reports station
association events for 30 s, then stops the AP. The SSID, password and channel
are the `AP_SSID`, `AP_PASSWORD` and `AP_CHANNEL` constants in `src/main.rs`.

To see station events, join `ameba-rust-ap` from a phone or laptop within the
30 s window. Only association is reported; the example does not set up an IP
network on the AP side, so the station associates but cannot obtain an IP
address. A station that is still connected when the window ends is
disconnected by `wifi_stop_ap()`; that disconnection happens after the window
and is not printed.

Out of scope: multiple-client management, hidden SSID, WPA3 SoftAP, and running
station and SoftAP at the same time.

## Expected Output

```
[build] <build timestamp>
[wifi] init
[wifi] on: 0
[ap] starting SSID="ameba-rust-ap" channel=1 ...
[ap] start_ap returned: 0
[PASS] softap: start_ap accepted
[ap] observing for 30s ...
[ap] event: code=0 (STA_ASSOC) mac=xx:xx:xx:xx:xx:xx
[ap] stop_ap returned: 0
[PASS] softap: stop_ap accepted
```

The `[ap] event:` line only appears if a station joins during the window. A
phone may associate more than once, each time with a different randomized MAC
address.
