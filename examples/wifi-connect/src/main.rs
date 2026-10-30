//! WiFi WPA2-AES-PSK connection example.
//!
//! Demonstrates the `embassy-net` path (`embassy_net::Stack` + `TcpSocket`)
//! on top of `ameba_wifi::net::WhcDevice`'s `embassy_net_driver::Driver` impl.
//! The legacy hand-rolled smoltcp path (`ameba_wifi::net::run_dhcp`/
//! `run_iperf_tcp`) still exists in `ameba-wifi` and is not used here.
//!
//! Set [`WIFI_SSID`] and [`WIFI_PASSWORD`] below before flashing.
//!
//! The final TCP check connects to [`TCP_PORT`] on the DHCP gateway, so
//! something must listen there (e.g. an HTTP file server app on the phone that
//! provides the hotspot). Otherwise it ends with `[FAIL] embassy-net: TCP
//! connect failed: ConnectionReset`. Change [`TCP_PORT`] to match your server.
//!
//! Expected serial output:
//!   [build] <build timestamp>
//!   [wifi] init
//!   [wifi] on: 0
//!   [wifi] connecting to "<ssid>" ...
//!   [wifi] connect returned: 0
//!   [wifi] event: code=2
//!   [wifi] join_status=6
//!   [NET] WiFi connected
//!   [NET] MAC: xx:xx:xx:xx:xx:xx
//!   [PASS] noused keep-list: wifi_get_mac_address matches
//!   [PASS] noused keep-list: wifi_scan_abort returned <n> (not stubbed)
//!   [NET] DHCP: IP=192.168.x.x
//!   [NET] TCP: connecting to 192.168.x.1:8080 ...
//!   [NET] TCP: connected
//!   [PASS] embassy-net: TCP connected

#![no_std]
#![no_main]

use ameba_rt as _;
use cortex_m as _;

use ameba_hal::println;
use embassy_executor::Spawner;
use embassy_net::tcp::TcpSocket;
use embassy_net::{Config, Ipv4Address, Runner, StackResources};
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;

use ameba_wifi::WifiEvent;
use ameba_wifi::net::WhcDevice;
use ameba_wifi::wifi::SecurityType;
use ameba_wifi::wifi::events::{event_code, join_status, read_join_status};

// ── Configuration ─────────────────────────────────────────────────────────────

const WIFI_SSID: &str = "your-ssid";
const WIFI_PASSWORD: &str = "your-password";

/// TCP port probed on the DHCP gateway; the check passes only if a server
/// listens on it (8080 is the default of common phone HTTP file-server apps).
const TCP_PORT: u16 = 8080;

// ── Background tasks ──────────────────────────────────────────────────────────

/// Handles blocking NP→AP API calls (GET_LWIP_INFO, WIFI_EVENT, …).
/// Must run throughout the WiFi session.
#[embassy_executor::task]
async fn whc_task() {
    ameba_wifi::run_blk_api().await;
}

/// Drives the embassy-net stack's background poll loop. Never returns.
#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, WhcDevice>) -> ! {
    runner.run().await
}

/// Drains `EVENT_CHAN` for the remainder of the program, once the initial
/// connect loop in `main` hands off to it.
///
/// `run_blk_api` ACKs NP for non-handshake WIFI_EVENTs (e.g. a later
/// disconnect/reconnect JOIN_STATUS) via a blocking `event_tx.send().await`
/// on this same channel (capacity 4). Without a permanent receiver, those
/// events pile up unconsumed and the 5th send blocks forever, stalling NP's
/// ACK and producing endless "Dev api ipc timeout" spam. This task keeps the
/// channel drained so that never happens.
#[embassy_executor::task]
async fn event_task() -> ! {
    loop {
        let evt: WifiEvent = ameba_wifi::EVENT_CHAN.receive().await;
        if evt.event_code == event_code::JOIN_STATUS {
            let status = if evt.evt_info != 0 {
                unsafe { read_join_status(evt.evt_info) }
            } else {
                join_status::UNKNOWN
            };
            println!(
                "[wifi] event: code={} join_status={}",
                evt.event_code, status as u32
            );
            match status {
                join_status::ASSOCIATED | join_status::SUCCESS => {
                    ameba_wifi::net::set_link_up(true);
                    println!("[NET] WiFi (re)connected");
                }
                join_status::DISCONNECT => {
                    ameba_wifi::net::set_link_up(false);
                    println!("[NET] WiFi disconnected");
                }
                _ => {}
            }
        } else {
            println!("[wifi] event: code={}", evt.event_code);
        }
    }
}

// ── Main ──────────────────────────────────────────────────────────────────────

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // 1. HAL init: sys_timer (embassy-time) + IPC_CPU0_IRQ.
    ameba_hal::init(ameba_hal::Config::default());

    println!("[build] {}", env!("BUILD_TIMESTAMP"));

    // 2. Register WHC IPC channel handlers (ch6 TRX + ch7 API).
    ameba_wifi::init();
    println!("[wifi] init");

    // 3. Spawn the background task that processes NP→AP requests.
    spawner.spawn(whc_task()).unwrap();

    // 4. Turn on the WiFi NP in STA mode.
    let rc = ameba_wifi::wifi::wifi_on().await;
    println!("[wifi] on: {}", rc);

    // 5. Connect — WPA2-AES-PSK.
    println!("[wifi] connecting to \"{}\" ...", WIFI_SSID);
    let rc =
        ameba_wifi::wifi::wifi_connect(WIFI_SSID, WIFI_PASSWORD, SecurityType::Wpa2AesPsk).await;
    println!("[wifi] connect returned: {}", rc);

    // 6. Wait for the JOIN_STATUS event (event_code 2 = RTW_EVENT_JOIN_STATUS).
    //    NP fires this event for every status transition; we loop until we see
    //    ASSOCIATED/SUCCESS, FAIL (retry), or DISCONNECT.
    loop {
        let evt: WifiEvent = ameba_wifi::EVENT_CHAN.receive().await;
        println!("[wifi] event: code={}", evt.event_code);

        if evt.event_code == event_code::JOIN_STATUS {
            let status = if evt.evt_info != 0 {
                unsafe { read_join_status(evt.evt_info) }
            } else {
                join_status::UNKNOWN
            };
            println!("[wifi] join_status={}", status as u32);

            match status {
                join_status::ASSOCIATED | join_status::SUCCESS => {
                    ameba_wifi::net::set_link_up(true);
                    println!("[NET] WiFi connected");
                    break;
                }
                join_status::FAIL => {
                    println!("[wifi] connect failed, retrying in 3s ...");
                    Timer::after(Duration::from_secs(3)).await;
                    let rc = ameba_wifi::wifi::wifi_connect(
                        WIFI_SSID,
                        WIFI_PASSWORD,
                        SecurityType::Wpa2AesPsk,
                    )
                    .await;
                    println!("[wifi] connect returned: {}", rc);
                }
                join_status::DISCONNECT => {
                    ameba_wifi::net::set_link_up(false);
                    println!("[FAIL] embassy-net: WiFi disconnected before association");
                    return;
                }
                _ => {} // intermediate status (SCANNING, ASSOCIATING, etc.)
            }
        }
    }

    // 6b. Hand off EVENT_CHAN to a permanent consumer so a later disconnect
    // (or any other WIFI_EVENT) never stalls run_blk_api's ACK to NP.
    spawner.spawn(event_task()).unwrap();

    // 7. MAC address, as reported by NP through SET_NETIF_INFO during wifi_on.
    //
    // Bail out rather than substituting a placeholder MAC: the address must match
    // what the radio registered with the AP, and a mismatch fails in the most
    // confusing way possible — association still succeeds and every log line
    // looks healthy, but DHCP quietly never completes because the OFFER comes
    // back addressed to a MAC that NP's hardware receive filter drops.
    let Some(mac) = ameba_wifi::wifi::netif_mac() else {
        println!("[FAIL] embassy-net: no MAC from NP (SET_NETIF_INFO never arrived)");
        return;
    };
    println!(
        "[NET] MAC: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    );

    // 7b. Cross-check the same MAC through `wifi_get_mac_address()`, which queries
    // NP over WHC instead of using the address NP pushed at wifi_on time.
    //
    // This doubles as the acceptance test for the noused keep-list mechanism: this
    // API is in the pruner's risk set, so it only works if the NP image was built
    // from this firmware's symbol table. Against a stale NP image it is a stub —
    // historically a `while(1)` that wedged the core; with the keep-list wired up it
    // must return the real MAC. Comparing the two independent sources also catches a
    // wrong-but-plausible answer, which a bare "did it return?" check would miss.
    match ameba_wifi::wifi::wifi_get_mac_address(0).await {
        Some(q) if q == mac => println!("[PASS] noused keep-list: wifi_get_mac_address matches"),
        Some(q) => println!(
            "[FAIL] noused keep-list: wifi_get_mac_address returned \
             {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}, expected the MAC above",
            q[0], q[1], q[2], q[3], q[4], q[5]
        ),
        None => println!("[FAIL] noused keep-list: wifi_get_mac_address query failed"),
    }

    // 7c. Second risk-set API in the same image. Two pinned C symbols is the case
    // that a single-anchor build cannot exercise: if the linker folded them onto one
    // address, only one name would reach the pruner and the other would come back a
    // stub. No scan is running here, so a non-zero status is the expected answer —
    // reaching this line at all is the point, a stub would have wedged NP instead.
    let rc = ameba_wifi::wifi::wifi_scan_abort().await;
    println!(
        "[PASS] noused keep-list: wifi_scan_abort returned {} (not stubbed)",
        rc
    );

    // 8. Build the embassy-net Stack over WhcDevice and spawn its poll task.
    static RESOURCES: StaticCell<StackResources<4>> = StaticCell::new();

    let trng = unsafe { ameba_hal::trng::Trng::steal() };
    trng.init();
    let seed = trng.read_u32().map(|v| v as u64).unwrap_or(0x1234_5678);

    let device = WhcDevice::with_mac(mac);
    let resources = RESOURCES.init(StackResources::new());
    let (stack, runner) =
        embassy_net::new(device, Config::dhcpv4(Default::default()), resources, seed);

    spawner.spawn(net_task(runner)).unwrap();

    // 9. Wait for DHCP to assign an address.
    //
    // Bound the wait: DISCOVER retransmits indefinitely, so an unreachable
    // DHCP server (or a MAC mismatch causing the OFFER to be dropped — see
    // the netif_mac() comment above) would hang the example with no
    // PASS/FAIL verdict, same rationale as the TCP connect timeout below.
    if embassy_time::with_timeout(Duration::from_secs(15), stack.wait_config_up())
        .await
        .is_err()
    {
        println!("[FAIL] embassy-net: DHCP timed out after 15s");
        // Return rather than spin-looping: `main` is just one task among
        // several under the embassy executor (whc_task/net_task/event_task
        // must keep running). A bare `loop { wfi() }` never yields control
        // back to the executor since it contains no `.await`, so it would
        // starve every other task — including `run_blk_api`, wedging NP's
        // ACK path on the very next WIFI_EVENT.
        return;
    }
    let ip = stack
        .config_v4()
        .map(|c| c.address.address())
        .unwrap_or(Ipv4Address::UNSPECIFIED);
    println!("[NET] DHCP: IP={}", ip);

    // 10. Open a TCP connection to prove the stack works end to end.
    //
    // Target `TCP_PORT` on the gateway: DHCP just proved the gateway is
    // reachable, so this needs no extra host on the LAN. The connect below only
    // succeeds if something on the gateway listens on that port; otherwise it
    // fails with a reset, which cannot distinguish "our stack is broken" from
    // "nobody home".
    let mut rx_buf = [0u8; 2048];
    let mut tx_buf = [0u8; 2048];
    let mut socket = TcpSocket::new(stack, &mut rx_buf, &mut tx_buf);

    let gateway = stack
        .config_v4()
        .and_then(|c| c.gateway)
        .unwrap_or(Ipv4Address::new(192, 168, 1, 1));
    println!("[NET] TCP: connecting to {}:{} ...", gateway, TCP_PORT);

    // Bound the wait: `connect` retransmits SYN indefinitely, so without a
    // timeout an unreachable peer hangs the example with no PASS/FAIL verdict.
    let connect = socket.connect((gateway, TCP_PORT));
    match embassy_time::with_timeout(Duration::from_secs(10), connect).await {
        Ok(Ok(())) => {
            println!("[NET] TCP: connected");
            println!("[PASS] embassy-net: TCP connected");
        }
        Ok(Err(e)) => {
            println!("[FAIL] embassy-net: TCP connect failed: {:?}", e);
        }
        Err(_) => {
            println!("[FAIL] embassy-net: TCP connect timed out after 10s");
        }
    };

    // Return rather than spin-looping — see the DHCP-timeout branch above
    // for why: this task must yield back to the executor so whc_task
    // (run_blk_api), net_task, and event_task keep running for the rest of
    // the WiFi session (reconnects, ongoing traffic, etc.).
}
