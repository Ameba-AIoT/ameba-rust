//! OTA firmware update over HTTP.
//!
//! Downloads an OTA image from a plain-HTTP server on the LAN and programs it
//! into the inactive A/B slot. On success this resets the board so the
//! bootloader picks the new image immediately; on any failure the slot's
//! manifest is never committed, so the currently running image stays
//! selected and the board idles without resetting.
//!
//! Before running, set [`WIFI_SSID`]/[`WIFI_PASSWORD`], [`OTA_SERVER_IP`],
//! [`OTA_SERVER_PORT`] and [`OTA_PATH`] below, and serve the `ota_all.bin`
//! produced by `cargo xtask build`. Any static file server works, as long as
//! the image is reachable at [`OTA_PATH`] on [`OTA_SERVER_PORT`] (by default
//! `http://<OTA_SERVER_IP>:8080/files/ota_all.bin`).
//!
//! The host must be an IPv4 literal — this build has no DNS resolver
//! (`ameba_ota::http::LiteralDns`).
//!
//! Expected serial output (NP debug lines omitted):
//!   [build] <build timestamp>
//!   [marker] <BUILD_MARKER>
//!   [wdg] enabled, timeout=5000ms
//!   [wifi] on: 0
//!   [wifi] connecting to "<ssid>" ...
//!   [wifi] connect returned: 0
//!   [wifi] join_status=6
//!   [NET] WiFi connected
//!   [NET] MAC: xx:xx:xx:xx:xx:xx
//!   [NET] waiting for DHCP ...
//!   [NET] DHCP: IP=192.168.x.x
//!   [ota] GET http://192.168.x.y:8080/files/ota_all.bin
//!   [ota] programmed: fw_ver=... img_id=... body_len=... bytes
//!   [PASS] ota-http: image committed to inactive slot, resetting
//!
//! `fw_ver` is the `version` field of the OTA header and is always
//! `0xFFFFFFFF`; it is not the image version. If the server refuses the
//! connection, `[FAIL] ota-http: connect failed: Network(ConnectionReset)` is
//! printed; if the host is unreachable, the 15 s timeout fires instead.

#![no_std]
#![no_main]

use ameba_rt as _;
use cortex_m as _;

use ameba_hal::{println, wdg::Wdg};
use embassy_executor::Spawner;
use embassy_net::{Config, Ipv4Address, Runner, StackResources};
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;

use ameba_ota::core::{OTA_BUF_SIZE, OtaSession};
use ameba_ota::http::{self, HEADER_BUF_SIZE, LiteralDns};
use ameba_ota::storage::OtaWriteBuf;
use ameba_storage::{Flash, FlashStorage};
use ameba_wifi::WifiEvent;
use ameba_wifi::net::WhcDevice;
use ameba_wifi::wifi::SecurityType;
use ameba_wifi::wifi::events::{event_code, join_status, read_join_status};

// ── Configuration ─────────────────────────────────────────────────────────────

const WIFI_SSID: &str = "your-ssid";
const WIFI_PASSWORD: &str = "your-password";
const WIFI_SECURITY: SecurityType = SecurityType::Wpa2AesPsk;

/// OTA server address. Must be an IPv4 literal — no DNS in this build.
const OTA_SERVER_IP: &str = "192.168.1.100";
const OTA_SERVER_PORT: u16 = 8080;
/// Path to the OTA image on that server.
const OTA_PATH: &str = "/files/ota_all.bin";

/// Build marker, printed at startup.
///
/// The end-to-end test programs an image built from this same example, so the
/// only way to tell from the serial log whether the bootloader actually
/// switched slots after a reset is for the two builds to say something
/// different. Bump this before packaging the image to be downloaded.
const BUILD_MARKER: &str = "A (currently running)";

/// Gate A: whether this boot should attempt an OTA update at all.
///
/// Placeholder for a real trigger signal (button, command, server-pushed
/// flag, ...) — for now a hardcoded per-build constant so the two images in
/// an A/B test can disagree: the currently-running image decides whether the
/// OTA flow runs; set to `false` in the freshly-downloaded image so it does
/// not immediately re-trigger another OTA on its own first boot.
const REQUEST_OTA_UPDATE: bool = true;

/// System watchdog timeout, and the interval its refresh task runs at.
///
/// An OTA download is the longest uninterruptible stretch of work this
/// firmware does, so it is the case a watchdog has to survive. It does, with
/// wide margin: flash access disables interrupts for at most ~7 ms at a time
/// (see [`ameba_ota::storage::CHUNK_LEN`] for the measurements), which cannot
/// delay a 500 ms refresh past a 5 s deadline. Measured end to end on
/// hardware, DHCP-complete to the new image booting takes ~9 s across 96
/// sector erase/write cycles with the watchdog enabled throughout, and it
/// never fires.
const WDG_TIMEOUT_MS: u32 = 5000;
const WDG_REFRESH_MS: u64 = 500;

// ── Background tasks ──────────────────────────────────────────────────────────

/// Handles blocking NP→AP API calls. Must run throughout the WiFi session.
#[embassy_executor::task]
async fn whc_task() {
    ameba_wifi::run_blk_api().await;
}

/// Feeds the system watchdog for as long as the firmware is healthy.
///
/// An independent task rather than a hook threaded through the OTA core, so
/// the OTA code stays unaware of the watchdog and every other code path is
/// covered too.
#[embassy_executor::task]
async fn wdg_task(wdg: Wdg) -> ! {
    loop {
        Timer::after_millis(WDG_REFRESH_MS).await;
        wdg.refresh();
    }
}

/// Drives the embassy-net stack's background poll loop. Never returns.
#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, WhcDevice>) -> ! {
    runner.run().await
}

// ── Main ──────────────────────────────────────────────────────────────────────

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    ameba_hal::init(ameba_hal::Config::default());
    println!("[build] {}", env!("BUILD_TIMESTAMP"));
    println!("[marker] {}", BUILD_MARKER);

    // Start the watchdog before anything can branch away, including the gate
    // below. Enabling it is irreversible in software (see [`Wdg::enable`]), so
    // it outlives the image that turned it on: an image reached by OTA finds
    // the watchdog already running and must refresh it from its very first
    // boot, whether or not it does any OTA work itself. Setting it up after
    // the gate instead is a self-inflicted reboot loop — the downloaded image
    // parks at the gate, never feeds, and is reset ~5 s later, forever
    // (observed on hardware as `AP BOOT REASON 40: WDG1_GLB`).
    let wdg = unsafe { Wdg::steal() };
    wdg.init(WDG_TIMEOUT_MS);
    wdg.enable();
    spawner.spawn(wdg_task(wdg)).unwrap();
    println!("[wdg] enabled, timeout={}ms", WDG_TIMEOUT_MS);

    if !REQUEST_OTA_UPDATE {
        println!("[ota] REQUEST_OTA_UPDATE=false, skipping OTA this boot");
        idle().await;
    }

    ameba_wifi::init();
    spawner.spawn(whc_task()).unwrap();

    let rc = ameba_wifi::wifi::wifi_on().await;
    println!("[wifi] on: {}", rc);

    println!("[wifi] connecting to \"{}\" ...", WIFI_SSID);
    let rc = ameba_wifi::wifi::wifi_connect(WIFI_SSID, WIFI_PASSWORD, WIFI_SECURITY).await;
    println!("[wifi] connect returned: {}", rc);

    // Wait for association before bringing the IP stack up.
    //
    // Bounded as a whole: a join can stall in an intermediate state
    // indefinitely (e.g. parked in SCANNING when the target SSID is not on the
    // air, or on a band the radio is not scanning), and NP emits no failure
    // event for that case. Without this timeout the example would sit silently
    // on the event channel forever, printing no verdict.
    let associated = embassy_time::with_timeout(Duration::from_secs(40), async {
        loop {
            let evt: WifiEvent = ameba_wifi::EVENT_CHAN.receive().await;
            if evt.event_code != event_code::JOIN_STATUS {
                println!("[wifi] event: code={} (ignored)", evt.event_code);
                continue;
            }
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
                    return true;
                }
                join_status::FAIL => {
                    println!("[wifi] connect failed, retrying in 3s ...");
                    Timer::after(Duration::from_secs(3)).await;
                    let rc =
                        ameba_wifi::wifi::wifi_connect(WIFI_SSID, WIFI_PASSWORD, WIFI_SECURITY)
                            .await;
                    println!("[wifi] connect returned: {}", rc);
                }
                join_status::DISCONNECT => {
                    ameba_wifi::net::set_link_up(false);
                    println!("[wifi] disconnected before association");
                    return false;
                }
                _ => {}
            }
        }
    })
    .await;

    match associated {
        Ok(true) => {}
        Ok(false) => {
            println!("[FAIL] ota-http: WiFi disconnected before association");
            idle().await;
        }
        Err(_) => {
            println!(
                "[FAIL] ota-http: could not join \"{}\" within 40s \
                 (SSID on air? 2.4GHz? WPA2-PSK?)",
                WIFI_SSID
            );
            idle().await;
        }
    }

    // The MAC must match what the radio registered with the AP, otherwise DHCP
    // silently never completes — NP's receive filter drops the OFFER.
    let Some(mac) = ameba_wifi::wifi::netif_mac() else {
        println!("[FAIL] ota-http: no MAC from NP (SET_NETIF_INFO never arrived)");
        return;
    };

    println!(
        "[NET] MAC: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    );

    static RESOURCES: StaticCell<StackResources<4>> = StaticCell::new();
    let trng = unsafe { ameba_hal::trng::Trng::steal() };
    trng.init();
    let seed = trng.read_u32().map(|v| v as u64).unwrap_or(0x1234_5678);

    let device = WhcDevice::with_mac(mac);
    let resources = RESOURCES.init(StackResources::new());
    let (stack, runner) =
        embassy_net::new(device, Config::dhcpv4(Default::default()), resources, seed);
    spawner.spawn(net_task(runner)).unwrap();

    // Bound the DHCP wait: DISCOVER retransmits forever, so an unreachable
    // server would hang here with no verdict printed. The window is generous
    // because DISCOVER backs off between retries and the first attempt can go
    // out before the freshly associated link passes traffic.
    println!("[NET] waiting for DHCP ...");
    if embassy_time::with_timeout(Duration::from_secs(45), stack.wait_config_up())
        .await
        .is_err()
    {
        println!("[FAIL] ota-http: DHCP timed out after 45s");
        idle().await;
    }
    let ip = stack
        .config_v4()
        .map(|c| c.address.address())
        .unwrap_or(Ipv4Address::UNSPECIFIED);
    println!("[NET] DHCP: IP={}", ip);

    // ── OTA download ─────────────────────────────────────────────────────────
    //
    // Buffers are `static` rather than locals: together they are ~5 KB, which
    // would otherwise land on this task's arena alongside the WiFi/net futures.
    static TCP_STATE: StaticCell<embassy_net::tcp::client::TcpClientState<1, 2048, 2048>> =
        StaticCell::new();
    static HEADER_BUF: StaticCell<[u8; HEADER_BUF_SIZE]> = StaticCell::new();
    static OTA_BUF: StaticCell<[u8; OTA_BUF_SIZE]> = StaticCell::new();
    static WRITE_BUF: StaticCell<OtaWriteBuf> = StaticCell::new();

    let tcp_state = TCP_STATE.init(embassy_net::tcp::client::TcpClientState::new());
    let header_buf = HEADER_BUF.init([0u8; HEADER_BUF_SIZE]);
    let ota_buf = OTA_BUF.init([0u8; OTA_BUF_SIZE]);
    let write_buf = WRITE_BUF.init([0u8; ameba_ota::storage::CHUNK_LEN]);

    let tcp_client = embassy_net::tcp::client::TcpClient::new(stack, tcp_state);
    let dns = LiteralDns;
    let mut client = reqwless::client::HttpClient::new(&tcp_client, &dns);

    // reqwless takes the base URL when opening the connection; the path is
    // passed per request. The URL must outlive the connection, so it is built
    // into a `static` rather than a local.
    static URL: StaticCell<Url> = StaticCell::new();
    let url: &'static Url = URL.init(build_url());
    println!("[ota] GET {}{}", url.as_str(), OTA_PATH);

    // Bound the connect: a TCP SYN to an unreachable server retransmits for
    // minutes with no error, which would hang here printing no verdict at all.
    let mut resource =
        match embassy_time::with_timeout(Duration::from_secs(15), client.resource(url.as_str()))
            .await
        {
            Ok(Ok(r)) => r,
            Ok(Err(e)) => {
                println!("[FAIL] ota-http: connect failed: {:?}", e);
                idle().await;
            }
            Err(_) => {
                println!(
                    "[FAIL] ota-http: connect to {} timed out after 15s \
                 (server unreachable from this network?)",
                    url.as_str()
                );
                idle().await;
            }
        };

    let transport = match embassy_time::with_timeout(
        Duration::from_secs(15),
        http::http_get(&mut resource, OTA_PATH, header_buf),
    )
    .await
    {
        Ok(Ok(t)) => t,
        Ok(Err(e)) => {
            println!("[FAIL] ota-http: GET failed: {:?}", e);
            idle().await;
        }
        Err(_) => {
            println!("[FAIL] ota-http: no response headers within 15s");
            idle().await;
        }
    };

    let mut flash = FlashStorage::new(unsafe { Flash::steal() });
    let session = OtaSession::new(transport);

    match session.run(&mut flash, ota_buf, write_buf).await {
        Ok(report) => {
            println!(
                "[ota] programmed: fw_ver={} img_id={} body_len={} bytes",
                report.fw_ver, report.img_id as u32, report.body_len
            );
            println!("[PASS] ota-http: image committed to inactive slot, resetting");
            // Give LOGUART time to flush before the reset cuts it off.
            Timer::after_millis(20).await;
            cortex_m::peripheral::SCB::sys_reset();
        }
        Err(e) => {
            println!("[FAIL] ota-http: {:?}", e);
        }
    }

    idle().await;
}

/// Build `http://<ip>:<port>` without an allocator.
fn build_url() -> Url {
    let mut url = Url::new();
    url.push_str("http://");
    url.push_str(OTA_SERVER_IP);
    url.push_str(":");
    url.push_u16(OTA_SERVER_PORT);
    url
}

/// Minimal fixed-capacity string, just enough for `http://<ipv4>:<port>`.
struct Url {
    buf: [u8; 32],
    len: usize,
}

impl Url {
    fn new() -> Self {
        Self {
            buf: [0; 32],
            len: 0,
        }
    }

    fn push_str(&mut self, s: &str) {
        let end = (self.len + s.len()).min(self.buf.len());
        let n = end - self.len;
        self.buf[self.len..end].copy_from_slice(&s.as_bytes()[..n]);
        self.len = end;
    }

    fn push_u16(&mut self, mut v: u16) {
        let mut digits = [0u8; 5];
        let mut i = digits.len();
        loop {
            i -= 1;
            digits[i] = b'0' + (v % 10) as u8;
            v /= 10;
            if v == 0 {
                break;
            }
        }
        // The slice is ASCII digits by construction.
        self.push_str(core::str::from_utf8(&digits[i..]).unwrap_or("0"));
    }

    fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buf[..self.len]).unwrap_or("")
    }
}

/// Park forever without stalling the executor.
///
/// Yields rather than spinning on `wfi()`: the watchdog refresh task has to
/// keep running after this firmware gives up, otherwise every terminal path
/// below (unreachable server, failed join, bad image) would be chewed up by
/// the watchdog ~5 s later and reboot into the same failure, forever. The
/// watchdog is here to catch an unintended hang, and deliberately parking is
/// not one.
async fn idle() -> ! {
    loop {
        Timer::after_millis(1000).await;
    }
}
