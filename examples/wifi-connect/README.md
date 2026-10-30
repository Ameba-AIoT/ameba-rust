# wifi-connect

## Build

```bash
cargo xtask build example --name wifi-connect
```

Set `WIFI_SSID` and `WIFI_PASSWORD` at the top of `src/main.rs` before building.

## Wiring

No external wiring needed. The board must be in range of a 2.4 GHz WPA2-PSK
(AES) network.

## Behavior

Brings up WiFi in station mode, joins the configured network, and runs DHCP and
a TCP connection through the [`embassy-net`](https://crates.io/crates/embassy-net)
stack on top of `ameba_wifi::net::WhcDevice`:

1. Turns WiFi on and connects with WPA2-AES-PSK. A failed join is retried every
   3 s.
2. Reads the interface MAC address and cross-checks it against
   `wifi_get_mac_address()`, then calls `wifi_scan_abort()`, confirming both
   network-core APIs respond.
3. Starts an `embassy-net` stack with DHCP and waits up to 15 s for an address.
4. Opens a TCP connection to the DHCP gateway on `TCP_PORT` (`src/main.rs`,
   default 8080), bounded by a 10 s timeout, to show the stack works end to end.

A background task keeps consuming WiFi events after the initial join and prints
later disconnect/reconnect transitions. The example does not reconnect on its
own.

## Expected Output

```
[build] <build timestamp>
[wifi] init
[wifi] on: 0
[wifi] connecting to "<ssid>" ...
[wifi] connect returned: 0
[wifi] join_status=6
[NET] WiFi connected
[NET] MAC: xx:xx:xx:xx:xx:xx
[PASS] noused keep-list: wifi_get_mac_address matches
[PASS] noused keep-list: wifi_scan_abort returned <n> (not stubbed)
[NET] DHCP: IP=192.168.x.x
[NET] TCP: connecting to 192.168.x.1:8080 ...
[NET] TCP: connected
[PASS] embassy-net: TCP connected
```

Intermediate `[wifi] event: code=...` and `[wifi] join_status=...` lines appear
while the join progresses. The TCP step only succeeds if something on the
gateway is listening on `TCP_PORT` (for example an HTTP file server). If nothing
is listening there, the last line is instead a
`[FAIL] embassy-net: TCP connect failed: ...` or
`[FAIL] embassy-net: TCP connect timed out after 10s` line. DHCP has already
succeeded at that point, so the WiFi link and the IP stack are working.
