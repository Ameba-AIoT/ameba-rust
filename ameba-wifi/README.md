# ameba-wifi

[![License](https://badgen.net/badge/License/Apache%202.0/lightgrey)](../LICENSE)
[![Target](https://badgen.net/badge/target/thumbv8m.main/blue)](https://doc.rust-lang.org/rustc/platform-support/thumbv8m.main-none-eabi.html)

Wi-Fi support for Realtek Ameba series chips: station mode (open and WPA2-PSK
networks), SoftAP, and an [`embassy-net`](https://docs.rs/embassy-net/) driver so
applications open their own sockets.

Part of the [ameba-rust](../README.md) SDK.

## How it works

On these chips the Wi-Fi MAC and PHY are driven by a second core (KM4NS) running
Realtek's pre-built firmware. This crate is the host side of that split: it does
not implement 802.11 itself, but speaks the cross-core WHC protocol over an IPC
mailbox — one channel pair for data frames, another for blocking API calls and
events.

```
  application  ─┐
                │  embassy-net sockets
  WhcDevice  ───┤
                │  WHC protocol over IPC mailbox
  ameba-wifi ───┘
       ╎
  ═════╎══════════  core boundary
       ╎
  KM4NS firmware (802.11 MAC/PHY, pre-built)
```

One consequence worth knowing: WPA2-PSK key derivation happens on *this* core,
not on the network core. When the network core needs a PMK it raises an event,
and this crate answers it by running PBKDF2-HMAC-SHA1 (4096 iterations) locally
and handing the result back. The rest of the 4-way handshake — EAPOL exchange,
PTK/GTK derivation, key installation — is likewise driven from here.

## Usage

```toml
[dependencies]
ameba-wifi = { git = "https://github.com/Ameba-AIoT/ameba-rust" }
```

```rust
use ameba_wifi::net::WhcDevice;
use ameba_wifi::wifi::SecurityType;

/// Services blocking API calls and events raised by the network core.
/// Must keep running for the whole Wi-Fi session.
#[embassy_executor::task]
async fn whc_task() {
    ameba_wifi::run_blk_api().await;
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    ameba_hal::init(ameba_hal::Config::default());   // enables IPC
    ameba_wifi::init();
    spawner.spawn(whc_task()).unwrap();

    ameba_wifi::wifi::wifi_on().await;
    ameba_wifi::wifi::wifi_connect("my-ssid", "my-password", SecurityType::Wpa2AesPsk).await;

    // WhcDevice implements embassy_net_driver::Driver — hand it to
    // embassy_net::new() and use ordinary TcpSocket / UdpSocket from there.
}
```

Ordering matters:

1. `ameba_hal::init()` with `ipc: true` (the default) — the IPC interrupt must
   be live before any cross-core traffic.
2. `ameba_wifi::init()` — registers the WHC data and API handlers.
3. Spawn `run_blk_api()` — the network core busy-waits for acknowledgement on
   some requests, so this task must be running before the first Wi-Fi call.
4. `wifi_on()`, then `wifi_connect()` or `wifi_start_ap()`.

> [!IMPORTANT]
>
> Never block inside an `async fn` without awaiting — a bare `loop {}` or
> `loop { wfi() }` in an async context starves the cooperative executor, which
> stops `run_blk_api()` from running and makes the network core report IPC
> timeouts. Use `Timer::after(...).await` or return from the task instead.

## Modules

| Module | Contents |
|:-------|:---------|
| `wifi` | Station and SoftAP control — `wifi_on`, `wifi_connect`, `wifi_start_ap`, `wifi_stop_ap`, `SecurityType`, event codes and join-status decoding |
| `net` | `WhcDevice` (the `embassy-net-driver` implementation), static IP configuration, and legacy fixed helpers for DHCP and TCP |
| `whc` | Cross-core transport — frame TX/RX, blocking API dispatch, message layout |
| `cache` | Cache maintenance for buffers shared across the core boundary |

Wi-Fi events arrive on the global `EVENT_CHAN` channel; application code
receives from it to observe connect, disconnect and association events.

## Networking

`WhcDevice` implements
[`embassy-net-driver`](https://docs.rs/embassy-net-driver/), so the full
`embassy-net` stack is available — DHCP, TCP, UDP and as many sockets as the
configured `StackResources` allows. See
[`examples/wifi-connect`](../examples/wifi-connect/) for a complete station plus
DHCP plus TCP walk-through, and [`examples/wifi-ap`](../examples/wifi-ap/) for
SoftAP.

An older hand-rolled [smoltcp](https://docs.rs/smoltcp/) path with fixed
`run_dhcp` / `run_iperf_tcp` helpers is still present for backward
compatibility. New code should use the `embassy-net` path.

## Current limitations

- **Station security** — open and WPA2-AES-PSK networks. WPA3/SAE, enterprise
  (802.1X), and 64-hex-digit PSK entry are not implemented.
- **Reconnection** — automatic reconnect after a link loss is not wired up; the
  NP firmware's own auto-reconnect subsystem is inactive in this
  host-driven configuration, so applications that need it should watch for the
  disconnect event and call `wifi_connect()` again.
- **DNS** — not enabled; use IP literals for now.
- **SoftAP** — start and stop work; multi-client management, hidden SSID, WPA3
  AP and concurrent station-plus-AP operation are not implemented.

## License

Apache-2.0 — see [LICENSE](../LICENSE).
