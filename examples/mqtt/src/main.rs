//! WiFi + MQTT publish/subscribe example.
//!
//! Joins a WPA2-AES-PSK network, then talks to an MQTT broker over an
//! `embassy_net::tcp::TcpSocket` using the `rust-mqtt` client (MQTT v5).
//!
//! The broker is assumed to run on the DHCP gateway (e.g. Mosquitto in Termux
//! on the phone that provides the hotspot) at port 1883, plain TCP. Set
//! [`BROKER_IP`] to override the gateway with a fixed address.
//!
//! Set [`WIFI_SSID`] and [`WIFI_PASSWORD`] below before flashing.
//!
//! Flow:
//!   1. subscribe to `ameba/cmd`
//!   2. publish "hello from ameba" to `ameba/cmd`  (loopback self-check)
//!   3. receive it back within 5 s, ignoring other `ameba/cmd` messages
//!      -> `[PASS] mqtt: loopback`
//!   4. publish a counter to `ameba/status` every 2 s, and print any message
//!      arriving on `ameba/cmd` (e.g. from `mosquitto_pub -t ameba/cmd -m hi`)
//!
//! Expected serial output:
//!   [NET] DHCP: IP=192.168.x.x
//!   [MQTT] connecting to 192.168.x.x:1883 ...
//!   [MQTT] connected
//!   [MQTT] subscribed: ameba/cmd
//!   [MQTT] published: ameba/cmd = "hello from ameba"
//!   [MQTT] received: ameba/cmd = "hello from ameba"
//!   [PASS] mqtt: loopback
//!   [MQTT] published: ameba/status = "count=0"

#![no_std]
#![no_main]

use ameba_rt as _;
use cortex_m as _;

use ameba_hal::println;
use embassy_executor::Spawner;
use embassy_net::tcp::TcpSocket;
use embassy_net::{Config, Ipv4Address, Runner, StackResources};
use embassy_time::{Duration, Timer};
use heapless::String;
use rand_core::{RngCore, impls};
use rust_mqtt::client::client::MqttClient;
use rust_mqtt::client::client_config::{ClientConfig, MqttVersion};
use rust_mqtt::packet::v5::publish_packet::QualityOfService;
use static_cell::StaticCell;

use ameba_wifi::WifiEvent;
use ameba_wifi::net::WhcDevice;
use ameba_wifi::wifi::SecurityType;
use ameba_wifi::wifi::events::{event_code, join_status, read_join_status};

// ── Configuration ─────────────────────────────────────────────────────────────

const WIFI_SSID: &str = "your-ssid";
const WIFI_PASSWORD: &str = "your-password";

/// Broker address. `None` = use the DHCP gateway.
const BROKER_IP: Option<[u8; 4]> = None;
const BROKER_PORT: u16 = 1883;

const TOPIC_CMD: &str = "ameba/cmd";
const TOPIC_STATUS: &str = "ameba/status";
/// The client ID is `CLIENT_ID_PREFIX` plus the last three MAC bytes in hex: a
/// broker drops the older session when a second client connects with the same
/// ID, so boards sharing a broker need distinct IDs.
const CLIENT_ID_PREFIX: &str = "ameba-";

// ── Packet-id source ──────────────────────────────────────────────────────────

/// `rust-mqtt` only uses its RNG for packet identifiers, so a TRNG-seeded
/// counter is enough.
struct PacketIds(u64);

impl RngCore for PacketIds {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(1);
        if self.0 > u16::MAX as u64 || self.0 == 0 {
            self.0 = 1;
        }
        self.0
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }
}

// ── Background tasks ──────────────────────────────────────────────────────────

#[embassy_executor::task]
async fn whc_task() {
    ameba_wifi::run_blk_api().await;
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, WhcDevice>) -> ! {
    runner.run().await
}

/// Keeps `EVENT_CHAN` drained after the initial connect loop; see the same task
/// in `wifi-connect` for why this must never stop.
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
        }
    }
}

// ── Main ──────────────────────────────────────────────────────────────────────

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    ameba_hal::init(ameba_hal::Config::default());

    println!("[build] {}", env!("BUILD_TIMESTAMP"));

    ameba_wifi::init();
    spawner.spawn(whc_task()).unwrap();

    let rc = ameba_wifi::wifi::wifi_on().await;
    println!("[wifi] on: {}", rc);

    println!("[wifi] connecting to \"{}\" ...", WIFI_SSID);
    let rc =
        ameba_wifi::wifi::wifi_connect(WIFI_SSID, WIFI_PASSWORD, SecurityType::Wpa2AesPsk).await;
    println!("[wifi] connect returned: {}", rc);

    loop {
        let evt: WifiEvent = ameba_wifi::EVENT_CHAN.receive().await;
        if evt.event_code != event_code::JOIN_STATUS {
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
                println!("[FAIL] mqtt: WiFi disconnected before association");
                return;
            }
            _ => {}
        }
    }

    spawner.spawn(event_task()).unwrap();

    // The address must match what the radio registered with the AP, or the
    // DHCP OFFER is dropped by NP's receive filter (see `wifi-connect`).
    let Some(mac) = ameba_wifi::wifi::netif_mac() else {
        println!("[FAIL] mqtt: no MAC from NP (SET_NETIF_INFO never arrived)");
        return;
    };

    static RESOURCES: StaticCell<StackResources<4>> = StaticCell::new();

    let trng = unsafe { ameba_hal::trng::Trng::steal() };
    trng.init();
    let seed = trng.read_u32().map(|v| v as u64).unwrap_or(0x1234_5678);

    let device = WhcDevice::with_mac(mac);
    let resources = RESOURCES.init(StackResources::new());
    let (stack, runner) =
        embassy_net::new(device, Config::dhcpv4(Default::default()), resources, seed);
    spawner.spawn(net_task(runner)).unwrap();

    if embassy_time::with_timeout(Duration::from_secs(15), stack.wait_config_up())
        .await
        .is_err()
    {
        println!("[FAIL] mqtt: DHCP timed out after 15s");
        return;
    }
    let cfg = stack.config_v4();
    let ip = cfg
        .as_ref()
        .map(|c| c.address.address())
        .unwrap_or(Ipv4Address::UNSPECIFIED);
    println!("[NET] DHCP: IP={}", ip);

    let broker = match BROKER_IP {
        Some(b) => Ipv4Address::new(b[0], b[1], b[2], b[3]),
        None => match cfg.and_then(|c| c.gateway) {
            Some(gw) => gw,
            None => {
                println!("[FAIL] mqtt: no gateway from DHCP and BROKER_IP unset");
                return;
            }
        },
    };

    // TCP.
    let mut rx_buf = [0u8; 1024];
    let mut tx_buf = [0u8; 1024];
    let mut socket = TcpSocket::new(stack, &mut rx_buf, &mut tx_buf);
    socket.set_timeout(Some(Duration::from_secs(30)));

    println!("[MQTT] connecting to {}:{} ...", broker, BROKER_PORT);
    match embassy_time::with_timeout(
        Duration::from_secs(10),
        socket.connect((broker, BROKER_PORT)),
    )
    .await
    {
        Ok(Ok(())) => {}
        Ok(Err(e)) => {
            println!("[FAIL] mqtt: TCP connect failed: {:?}", e);
            return;
        }
        Err(_) => {
            println!("[FAIL] mqtt: TCP connect timed out after 10s");
            return;
        }
    }

    // MQTT session.
    let mut client_id = String::<16>::new();
    let _ = core::fmt::Write::write_fmt(
        &mut client_id,
        format_args!(
            "{}{:02x}{:02x}{:02x}",
            CLIENT_ID_PREFIX, mac[3], mac[4], mac[5]
        ),
    );
    const KEEP_ALIVE_S: u16 = 20;
    let mut config = ClientConfig::<'_, 5, _>::new(MqttVersion::MQTTv5, PacketIds(seed & 0x7fff));
    config.add_client_id(client_id.as_str());
    config.add_max_subscribe_qos(QualityOfService::QoS0);
    config.keep_alive = KEEP_ALIVE_S;
    config.max_packet_size = 256;

    let mut write_buf = [0u8; 256];
    let mut recv_buf = [0u8; 256];
    let mut client =
        MqttClient::<_, 5, _>::new(socket, &mut write_buf, 256, &mut recv_buf, 256, config);

    if let Err(e) = client.connect_to_broker().await {
        println!("[FAIL] mqtt: broker CONNECT rejected: {:?}", e);
        return;
    }
    println!("[MQTT] connected");

    if let Err(e) = client.subscribe_to_topic(TOPIC_CMD).await {
        println!("[FAIL] mqtt: subscribe failed: {:?}", e);
        return;
    }
    println!("[MQTT] subscribed: {}", TOPIC_CMD);

    // Loopback self-check: publish to the topic we just subscribed to.
    const HELLO: &[u8] = b"hello from ameba";
    if let Err(e) = client
        .send_message(TOPIC_CMD, HELLO, QualityOfService::QoS0, false)
        .await
    {
        println!("[FAIL] mqtt: publish failed: {:?}", e);
        return;
    }
    println!(
        "[MQTT] published: {} = \"{}\"",
        TOPIC_CMD,
        core::str::from_utf8(HELLO).unwrap_or("?")
    );

    // `ameba/cmd` is shared, so other messages (retained, or sent by another
    // client) may arrive first: ignore them and keep waiting for our own.
    // Polls with `receive_message_if_ready` rather than cancelling
    // `receive_message` on a timeout, which could drop half a packet.
    let deadline = embassy_time::Instant::now() + Duration::from_secs(5);
    loop {
        match client.receive_message_if_ready().await {
            Ok(Some((topic, payload))) if topic == TOPIC_CMD && payload == HELLO => {
                println!(
                    "[MQTT] received: {} = \"{}\"",
                    topic,
                    core::str::from_utf8(payload).unwrap_or("?")
                );
                println!("[PASS] mqtt: loopback");
                break;
            }
            Ok(Some((topic, payload))) => {
                println!(
                    "[MQTT] ignored during self-check: {} = \"{}\"",
                    topic,
                    core::str::from_utf8(payload).unwrap_or("<binary>")
                );
            }
            Ok(None) => Timer::after_millis(50).await,
            Err(e) => {
                println!("[FAIL] mqtt: loopback receive failed: {:?}", e);
                return;
            }
        }
        // Checked after every iteration, not only when idle: a client that keeps
        // publishing to `ameba/cmd` must not be able to stall the self-check.
        if embassy_time::Instant::now() >= deadline {
            println!("[FAIL] mqtt: loopback timed out after 5s");
            return;
        }
    }

    // Steady state: publish a counter, print incoming commands.
    let mut count: u32 = 0;
    loop {
        match client.receive_message_if_ready().await {
            Ok(Some((topic, payload))) => {
                println!(
                    "[MQTT] received: {} = \"{}\"",
                    topic,
                    core::str::from_utf8(payload).unwrap_or("<binary>")
                );
            }
            Ok(None) => {}
            Err(e) => {
                println!("[FAIL] mqtt: receive failed: {:?}", e);
                return;
            }
        }

        let mut msg = String::<24>::new();
        let _ = core::fmt::Write::write_fmt(&mut msg, format_args!("count={}", count));
        if let Err(e) = client
            .send_message(TOPIC_STATUS, msg.as_bytes(), QualityOfService::QoS0, false)
            .await
        {
            println!("[FAIL] mqtt: publish failed: {:?}", e);
            return;
        }
        println!("[MQTT] published: {} = \"{}\"", TOPIC_STATUS, msg.as_str());
        count = count.wrapping_add(1);

        Timer::after(Duration::from_secs(2)).await;
    }
}
