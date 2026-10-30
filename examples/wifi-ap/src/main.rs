//! WiFi SoftAP example — basic `wifi_start_ap`/`wifi_stop_ap` flow.
//!
//! Current scope: start an open (or WPA2-PSK) SoftAP, observe station
//! associate/disassociate events for a fixed window, then stop the AP.
//! No multi-client management, hidden SSID, WPA3-AP, or STA+AP concurrency.
//!
//! Expected serial output:
//!   [wifi] init
//!   [ap] start_ap returned: 0
//!   [PASS] softap: start_ap accepted
//!   [ap] event: code=0 (STA_ASSOC) mac=xx:xx:xx:xx:xx:xx   (if a station joins)
//!   [ap] stop_ap returned: 0
//!   [PASS] softap: stop_ap accepted

#![no_std]
#![no_main]

use ameba_rt as _;
use cortex_m as _;

use ameba_hal::println;
use embassy_executor::Spawner;
use embassy_time::Duration;

use ameba_wifi::WifiEvent;
use ameba_wifi::wifi::SecurityType;
use ameba_wifi::wifi::events::{event_code, read_ap_sta_mac};

const AP_SSID: &str = "ameba-rust-ap";
const AP_PASSWORD: &str = "";
const AP_CHANNEL: u8 = 1;

/// How long to sit on the AP and watch for station events before stopping it.
const AP_OBSERVE_WINDOW: Duration = Duration::from_secs(30);

/// Handles blocking NP→AP API calls (GET_LWIP_INFO, WIFI_EVENT, …).
/// Must run throughout the WiFi session.
#[embassy_executor::task]
async fn whc_task() {
    ameba_wifi::run_blk_api().await;
}

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

    // 4. Turn on the WiFi NP. `wifi_on` boots the STA interface, but
    // `wifi_start_ap` below brings up the AP interface independently — the
    // NP's `_wifi_on_ap` runs on first `wifi_start_ap` call.
    let rc = ameba_wifi::wifi::wifi_on().await;
    println!("[wifi] on: {}", rc);

    // 5. Start the SoftAP.
    println!(
        "[ap] starting SSID=\"{}\" channel={} ...",
        AP_SSID, AP_CHANNEL
    );
    let rc =
        ameba_wifi::wifi::wifi_start_ap(AP_SSID, AP_PASSWORD, SecurityType::Open, AP_CHANNEL).await;
    println!("[ap] start_ap returned: {}", rc);

    if rc != 0 {
        println!("[FAIL] softap: start_ap rejected (ret={})", rc);
        loop {
            cortex_m::asm::wfi();
        }
    }
    println!("[PASS] softap: start_ap accepted");

    // 6. Observe station associate/disassociate events for a fixed window.
    // `with_timeout` wraps the whole loop: it cancels the loop (dropping
    // whichever `receive().await` is in flight) once the window elapses,
    // regardless of how many events were handled inside.
    println!("[ap] observing for {}s ...", AP_OBSERVE_WINDOW.as_secs());
    let _ = embassy_time::with_timeout(AP_OBSERVE_WINDOW, async {
        loop {
            let evt: WifiEvent = ameba_wifi::EVENT_CHAN.receive().await;

            match evt.event_code {
                event_code::AP_STA_ASSOC => {
                    let mac = if evt.evt_info != 0 {
                        Some(unsafe { read_ap_sta_mac(evt.evt_info) })
                    } else {
                        None
                    };
                    match mac {
                        Some(m) => println!(
                            "[ap] event: code={} (STA_ASSOC) mac={:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                            evt.event_code, m[0], m[1], m[2], m[3], m[4], m[5]
                        ),
                        None => println!("[ap] event: code={} (STA_ASSOC) mac=?", evt.event_code),
                    }
                }
                event_code::AP_STA_DISASSOC => {
                    let mac = if evt.evt_info != 0 {
                        Some(unsafe { read_ap_sta_mac(evt.evt_info) })
                    } else {
                        None
                    };
                    match mac {
                        Some(m) => println!(
                            "[ap] event: code={} (STA_DISASSOC) mac={:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                            evt.event_code, m[0], m[1], m[2], m[3], m[4], m[5]
                        ),
                        None => println!("[ap] event: code={} (STA_DISASSOC) mac=?", evt.event_code),
                    }
                }
                other => println!("[ap] event: code={} (other)", other),
            }
        }
    })
    .await;

    // 7. Stop the SoftAP.
    let rc = ameba_wifi::wifi::wifi_stop_ap().await;
    println!("[ap] stop_ap returned: {}", rc);
    if rc == 0 {
        println!("[PASS] softap: stop_ap accepted");
    } else {
        println!("[FAIL] softap: stop_ap rejected (ret={})", rc);
    }

    loop {
        cortex_m::asm::wfi();
    }
}
