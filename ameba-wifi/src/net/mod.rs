//! smoltcp `Device` implementation backed by the WHC IPC data plane.
//!
//! Also implements `embassy_net_driver::Driver` on the same `WhcDevice` (see
//! the "embassy-net Driver" section near the end) — both coexist; the legacy
//! smoltcp path above is unchanged.

use core::sync::atomic::{AtomicBool, Ordering};
use core::task::{Context as TaskContext, Poll};

use embassy_net_driver::{Capabilities, Driver, HardwareAddress as NetHardwareAddress, LinkState};
use embassy_net_driver::{RxToken as NetRxToken, TxToken as NetTxToken};
use embassy_sync::waitqueue::AtomicWaker;
use smoltcp::phy::{Device, DeviceCapabilities, Medium, RxToken, TxToken};
use smoltcp::time::Instant;

// ── DHCP ─────────────────────────────────────────────────────────────────────

/// Run DHCP and return the assigned IPv4 address.  Also updates `IP_CONFIG`.
///
/// `mac` is the device's Ethernet MAC address, embedded in DHCP DISCOVER.
pub async fn run_dhcp(mac: [u8; 6]) -> [u8; 4] {
    use smoltcp::iface::{Config, Interface, SocketSet, SocketStorage};
    use smoltcp::socket::dhcpv4;
    use smoltcp::wire::{EthernetAddress, HardwareAddress, IpCidr, Ipv4Address, Ipv4Cidr};

    let mut device = WhcDevice::new();
    let mut iface = Interface::new(
        Config::new(HardwareAddress::Ethernet(EthernetAddress(mac))),
        &mut device,
        smoltcp_now(),
    );
    iface.update_ip_addrs(|addrs| {
        let _ = addrs.push(IpCidr::Ipv4(Ipv4Cidr::new(Ipv4Address::UNSPECIFIED, 0)));
    });

    let mut storage = [SocketStorage::EMPTY];
    let mut sockets = SocketSet::new(&mut storage[..]);
    let dhcp_h = sockets.add(dhcpv4::Socket::new());

    loop {
        iface.poll(smoltcp_now(), &mut device, &mut sockets);

        let result = match sockets.get_mut::<dhcpv4::Socket>(dhcp_h).poll() {
            Some(dhcpv4::Event::Configured(ref cfg)) => Some((cfg.address, cfg.router)),
            _ => None,
        };

        if let Some((cidr, router)) = result {
            iface.update_ip_addrs(|addrs| {
                addrs.clear();
                let _ = addrs.push(IpCidr::Ipv4(cidr));
            });
            if let Some(gw) = router {
                let _ = iface.routes_mut().add_default_ipv4_route(gw);
            }
            let ip = cidr.address().0;
            unsafe {
                IP_CONFIG.ip = ip;
                IP_CONFIG.netmask = prefix_to_netmask(cidr.prefix_len());
                IP_CONFIG.gateway = router.map(|g| g.0).unwrap_or([0; 4]);
                IP_CONFIG.mac = mac;
            }
            return ip;
        }

        embassy_time::Timer::after_millis(10).await;
    }
}

fn smoltcp_now() -> Instant {
    Instant::from_millis(embassy_time::Instant::now().as_millis() as i64)
}

fn prefix_to_netmask(prefix: u8) -> [u8; 4] {
    if prefix == 0 {
        return [0u8; 4];
    }
    (!0u32 << (32 - prefix as u32)).to_be_bytes()
}

use crate::cache;
use crate::whc::msg::{EVT_RECV_PKTS, SKB_DATA_MAX, SKB_TX_HEADROOM};
use crate::whc::trx;

/// Run forever, returning every NP RX SKB to NP as soon as it arrives.
///
/// Spawn as a dedicated embassy task after DHCP completes.  Without this,
/// NP's sk_buff pool (skb_num_np = 10) fills up within seconds, preventing
/// beacon and management frame reception → disconnection.
pub async fn run_rx_drain() -> ! {
    loop {
        let evt = trx::RX_CHAN.receive().await;
        if evt.event_num == EVT_RECV_PKTS && evt.msg_addr != 0 {
            trx::recv_done(evt.msg_addr);
        }
    }
}

// ── iperf2 TCP client ─────────────────────────────────────────────────────────

static mut IPERF_TX: [u8; 8192] = [0u8; 8192];
static mut IPERF_RX: [u8; 1024] = [0u8; 1024];

fn netmask_to_prefix(nm: [u8; 4]) -> u8 {
    u32::from_be_bytes(nm).leading_ones() as u8
}

/// Run an iperf2-compatible TCP client to `server:port` for `secs` seconds,
/// report throughput, then keep the smoltcp stack alive (acts as RX drain +
/// ARP responder).  Never returns.
///
/// Call after `run_dhcp()` has populated `IP_CONFIG`.
/// Do NOT run `rx_drain_task` concurrently — this function owns `RX_CHAN`.
pub async fn run_iperf_tcp(
    mac: [u8; 6],
    our_ip: [u8; 4],
    server: [u8; 4],
    port: u16,
    secs: u64,
) -> ! {
    use embassy_time::{Duration, Instant, Timer};
    use smoltcp::iface::{Config, Interface, SocketSet, SocketStorage};
    use smoltcp::socket::tcp;
    use smoltcp::wire::{
        EthernetAddress, HardwareAddress, IpAddress, IpCidr, IpEndpoint, Ipv4Address, Ipv4Cidr,
    };

    let mut device = WhcDevice::new();
    let hw = HardwareAddress::Ethernet(EthernetAddress(mac));
    let mut iface = Interface::new(Config::new(hw), &mut device, smoltcp_now());

    let ip_addr = Ipv4Address::from_bytes(&our_ip);
    let prefix = netmask_to_prefix(unsafe { IP_CONFIG.netmask });
    iface.update_ip_addrs(|addrs| {
        let _ = addrs.push(IpCidr::Ipv4(Ipv4Cidr::new(ip_addr, prefix)));
    });
    unsafe {
        let gw_bytes = IP_CONFIG.gateway;
        let gw = Ipv4Address::from_bytes(&gw_bytes);
        if gw != Ipv4Address::UNSPECIFIED {
            let _ = iface.routes_mut().add_default_ipv4_route(gw);
        }
    }

    let rx_buf = unsafe { tcp::SocketBuffer::new(&mut IPERF_RX[..]) };
    let tx_buf = unsafe { tcp::SocketBuffer::new(&mut IPERF_TX[..]) };
    let mut storage = [SocketStorage::EMPTY; 1];
    let mut sockets = SocketSet::new(&mut storage[..]);
    let tcp_h = sockets.add(tcp::Socket::new(rx_buf, tx_buf));

    let remote = IpEndpoint::new(IpAddress::Ipv4(Ipv4Address::from_bytes(&server)), port);

    // iperf2 client_hdr (24 bytes, big-endian):
    //   flags=0, numThreads=1, mPort, bufferlen=0, mWinBand=0, mAmount (10ms units)
    let mut hdr = [0u8; 24];
    hdr[4..8].copy_from_slice(&1i32.to_be_bytes());
    hdr[8..12].copy_from_slice(&(port as i32).to_be_bytes());
    hdr[20..24].copy_from_slice(&((secs * 100) as i32).to_be_bytes());

    const CHUNK: [u8; 1448] = [0u8; 1448];

    let mut phase: u8 = 0; // 0=idle 1=connecting 2=send_hdr 3=sending 4=drain_close 5=done
    let mut total: u64 = 0;
    let mut deadline = Instant::now();

    loop {
        iface.poll(smoltcp_now(), &mut device, &mut sockets);

        match phase {
            0 => {
                let socket = sockets.get_mut::<tcp::Socket>(tcp_h);
                let cx = iface.context();
                if socket.connect(cx, remote, 49152).is_ok() {
                    ameba_hal::println!(
                        "[iperf] connecting to {}.{}.{}.{}:{}",
                        server[0],
                        server[1],
                        server[2],
                        server[3],
                        port,
                    );
                    phase = 1;
                }
            }
            1 => {
                let socket = sockets.get_mut::<tcp::Socket>(tcp_h);
                if socket.may_send() {
                    ameba_hal::println!("[iperf] connected, sending for {}s", secs,);
                    phase = 2;
                } else if socket.state() == tcp::State::Closed {
                    phase = 0; // connection refused / timed out, retry
                }
            }
            2 => {
                let socket = sockets.get_mut::<tcp::Socket>(tcp_h);
                if socket.can_send() && socket.send_slice(&hdr).is_ok() {
                    deadline = Instant::now() + Duration::from_secs(secs);
                    phase = 3;
                }
            }
            3 => {
                let socket = sockets.get_mut::<tcp::Socket>(tcp_h);
                if Instant::now() >= deadline {
                    socket.close();
                    phase = 4;
                } else if socket.can_send()
                    && let Ok(n) = socket.send_slice(&CHUNK)
                {
                    total += n as u64;
                }
            }
            4 => {
                let socket = sockets.get_mut::<tcp::Socket>(tcp_h);
                if !socket.is_active() {
                    let kbps = (total * 8) / (secs * 1000);
                    ameba_hal::println!(
                        "[iperf] done: sent {} KB in {}s = {} kbps",
                        (total >> 10),
                        secs,
                        kbps,
                    );
                    phase = 5;
                }
            }
            _ => {} // idle drain — smoltcp poll loop keeps NP RX SKBs returned
        }

        Timer::after_millis(1).await;
    }
}

// ── IP configuration ─────────────────────────────────────────────────────────

/// Static network configuration consulted by the `GET_LWIP_INFO` handler.
pub static mut IP_CONFIG: IpConfig = IpConfig::zero();

#[derive(Clone, Copy)]
pub struct IpConfig {
    pub ip: [u8; 4],
    pub gateway: [u8; 4],
    pub netmask: [u8; 4],
    pub mac: [u8; 6],
}

impl IpConfig {
    pub const fn zero() -> Self {
        Self {
            ip: [0; 4],
            gateway: [0; 4],
            netmask: [0; 4],
            mac: [0; 6],
        }
    }

    /// Configure static IP parameters. Call before `wifi_connect`.
    pub fn set(ip: [u8; 4], gateway: [u8; 4], netmask: [u8; 4], mac: [u8; 6]) {
        unsafe {
            IP_CONFIG.ip = ip;
            IP_CONFIG.gateway = gateway;
            IP_CONFIG.netmask = netmask;
            IP_CONFIG.mac = mac;
        }
    }
}

// ── Static single-packet RX buffer ───────────────────────────────────────────

static mut RX_PKT_BUF: [u8; 1600] = [0u8; 1600];
static mut RX_PKT_LEN: usize = 0;

// ── WhcDevice ─────────────────────────────────────────────────────────────────

/// smoltcp device backed by the WHC IPC channel 6 (TRX).
///
/// For the smoltcp path, create with `WhcDevice::new()` and pass to
/// `smoltcp::iface::Interface`.
///
/// For the `embassy_net::new()` path (via the `embassy_net_driver::Driver` impl
/// below), use [`WhcDevice::with_mac`] — **not** `new()`. `Driver` reports the
/// MAC synchronously through `hardware_address()`, and `new()` leaves it all
/// zero, which breaks DHCP: the OFFER comes back addressed to the radio's real
/// MAC and NP's receive filter drops it, so the stack waits forever while every
/// log line still looks healthy.
pub struct WhcDevice {
    wlan_idx: u8,
    mac: [u8; 6],
}

impl Default for WhcDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl WhcDevice {
    pub const fn new() -> Self {
        Self {
            wlan_idx: 0,
            mac: [0; 6],
        }
    }

    /// Create a device that reports `mac` from `hardware_address()`.
    ///
    /// Required for the `embassy_net_driver::Driver` path: `hardware_address()`
    /// is synchronous, so the MAC must be known before the stack is built.
    /// Get it from [`crate::wifi::netif_mac`], which returns the address NP
    /// pushed via `SET_NETIF_INFO` during `wifi_on`.
    ///
    /// Prefer it over [`crate::wifi::wifi_get_mac_address`], which needs a round
    /// trip to NP.
    pub const fn with_mac(mac: [u8; 6]) -> Self {
        Self { wlan_idx: 0, mac }
    }
}

/// Set by the `JOIN_STATUS` event handler (see the example) to drive
/// `Driver::link_state()`. Not touched by this crate itself — application
/// code owns updating it as WiFi association state changes.
pub static LINK_UP: AtomicBool = AtomicBool::new(false);

/// Woken whenever `LINK_UP` changes, so `Driver::link_state()` re-polls.
static LINK_WAKER: AtomicWaker = AtomicWaker::new();

/// Update the link-up flag and wake anyone polling `Driver::link_state()`.
pub fn set_link_up(up: bool) {
    LINK_UP.store(up, Ordering::Relaxed);
    LINK_WAKER.wake();
}

impl Device for WhcDevice {
    type RxToken<'d>
        = WhcRxToken
    where
        Self: 'd;
    type TxToken<'d>
        = WhcTxToken
    where
        Self: 'd;

    fn receive(&mut self, _ts: Instant) -> Option<(WhcRxToken, WhcTxToken)> {
        loop {
            let evt = trx::RX_CHAN.try_receive().ok()?;
            if evt.event_num == EVT_RECV_PKTS {
                return self.process_rx(evt.msg_addr);
            }
            // Non-packet events (MSG_RECV_DONE etc.) are NP-internal; discard.
        }
    }

    fn transmit(&mut self, _ts: Instant) -> Option<WhcTxToken> {
        let skb_idx = trx::alloc_skb()?;
        Some(WhcTxToken {
            skb_idx,
            wlan_idx: self.wlan_idx,
        })
    }

    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.medium = Medium::Ethernet;
        caps.max_transmission_unit = 1514;
        caps.max_burst_size = Some(1);
        caps
    }
}

impl WhcDevice {
    /// Copy the frame described by `skb_addr` into `RX_PKT_BUF` and hand NP's
    /// SKB straight back, returning whether a frame is now staged for delivery.
    ///
    /// This consumes the RX event irrecoverably, so callers must already hold a
    /// TX slot — see the comment in `Driver::receive`.
    fn stage_rx(&self, skb_addr: u32) -> bool {
        if skb_addr == 0 {
            return false;
        }

        unsafe {
            // NP's sk_buff header is 64 bytes; data ptr at offset 16, len at offset 32.
            cache::invalidate(skb_addr as usize, 64);
            let data_ptr = core::ptr::read_volatile((skb_addr + 16) as *const u32);
            let pkt_len = core::ptr::read_volatile((skb_addr + 32) as *const u32) as usize;

            if pkt_len == 0 || pkt_len > SKB_DATA_MAX || data_ptr == 0 {
                trx::recv_done(skb_addr);
                return false;
            }

            cache::invalidate(data_ptr as usize, pkt_len);
            core::ptr::copy_nonoverlapping(
                data_ptr as *const u8,
                core::ptr::addr_of_mut!(RX_PKT_BUF) as *mut u8,
                pkt_len,
            );
            RX_PKT_LEN = pkt_len;

            // Return NP's SKB immediately after copying.
            trx::recv_done(skb_addr);
        }

        true
    }

    /// `stage_rx` plus the TX token both `Device::receive` and `Driver::receive`
    /// must hand back alongside the RX token.
    ///
    /// Allocates the TX slot *first*: `stage_rx` returns NP's SKB and cannot be
    /// undone, so discovering the TX pool is empty afterwards would silently
    /// drop the frame.
    fn process_rx(&self, skb_addr: u32) -> Option<(WhcRxToken, WhcTxToken)> {
        let skb_idx = trx::alloc_skb()?;
        if !self.stage_rx(skb_addr) {
            return None;
        }
        Some((
            WhcRxToken,
            WhcTxToken {
                skb_idx,
                wlan_idx: self.wlan_idx,
            },
        ))
    }
}

// ── RxToken ───────────────────────────────────────────────────────────────────

pub struct WhcRxToken;

impl RxToken for WhcRxToken {
    fn consume<R, F>(self, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let (len, ptr) = unsafe { (RX_PKT_LEN, core::ptr::addr_of_mut!(RX_PKT_BUF) as *mut u8) };
        let buf = unsafe { core::slice::from_raw_parts_mut(ptr, len) };
        f(buf)
    }
}

// ── TxToken ───────────────────────────────────────────────────────────────────

pub struct WhcTxToken {
    skb_idx: usize,
    wlan_idx: u8,
}

impl TxToken for WhcTxToken {
    fn consume<R, F>(self, len: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        // Let smoltcp write directly into the SKB's data region.
        let result = unsafe {
            let buf_base = trx::skb_buf_ptr(self.skb_idx);
            let data_ptr = (buf_base + SKB_TX_HEADROOM as u32) as *mut u8;
            f(core::slice::from_raw_parts_mut(data_ptr, len))
        };

        trx::send_pkt_from_skb(self.skb_idx, len, self.wlan_idx);
        result
    }
}

// ── embassy-net Driver ───────────────────────────────────────────────────────
//
// Implements `embassy_net_driver::Driver` on the same `WhcDevice`, so it can
// be handed to `embassy_net::new()` as an alternative to the hand-rolled
// smoltcp `Interface`/`SocketSet` loops above. Both paths coexist; nothing
// above is removed.
//
// `WhcRxToken`/`WhcTxToken` already satisfy embassy-net-driver's `RxToken`/
// `TxToken` traits structurally (same `consume` signatures as smoltcp's), so
// we just add trait impls for the same token types rather than new ones.

impl NetRxToken for WhcRxToken {
    fn consume<R, F>(self, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        RxToken::consume(self, f)
    }
}

impl NetTxToken for WhcTxToken {
    fn consume<R, F>(self, len: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        TxToken::consume(self, len, f)
    }
}

impl Driver for WhcDevice {
    type RxToken<'d>
        = WhcRxToken
    where
        Self: 'd;
    type TxToken<'d>
        = WhcTxToken
    where
        Self: 'd;

    fn receive(&mut self, cx: &mut TaskContext) -> Option<(WhcRxToken, WhcTxToken)> {
        // A TX slot must be in hand *before* touching RX_CHAN: taking an event
        // off the channel stages the frame into RX_PKT_BUF and returns NP's SKB
        // (see `stage_rx`), which cannot be undone, so bailing out afterwards
        // for want of a TX token would drop the packet. Under TX pressure the
        // pool empties, so this is a live path, not a theoretical one.
        //
        // Register on TX_WAKER too — without it, a receive() that bails here
        // would only be re-polled on RX activity, stalling until the next
        // inbound packet even though the real blocker was the TX pool.
        trx::TX_WAKER.register(cx.waker());
        let skb_idx = trx::alloc_skb()?;

        // Register before checking, so a packet arriving between the failed
        // poll_receive and the return of `None` still wakes us (embassy-net
        // driver contract: "if there isn't [a packet], wake cx.waker() when
        // one is ready").
        loop {
            match trx::RX_CHAN.poll_receive(cx) {
                Poll::Ready(evt) if evt.event_num == EVT_RECV_PKTS => {
                    if self.stage_rx(evt.msg_addr) {
                        return Some((
                            WhcRxToken,
                            WhcTxToken {
                                skb_idx,
                                wlan_idx: self.wlan_idx,
                            },
                        ));
                    }
                    // Malformed frame — already returned to NP by stage_rx.
                    // Loop back so the waker stays registered and we pick up
                    // the next queued event; skb_idx is still unclaimed
                    // (alloc_skb only reserves it once TxToken::consume runs).
                }
                Poll::Ready(_) => {
                    // Non-packet events (MSG_RECV_DONE etc.) are NP-internal; discard.
                }
                Poll::Pending => return None,
            }
        }
    }

    fn transmit(&mut self, cx: &mut TaskContext) -> Option<WhcTxToken> {
        // Register before checking (prevents a missed wake if an SKB is freed
        // between `alloc_skb()` and registering — see `crate::whc::trx::TX_WAKER`).
        trx::TX_WAKER.register(cx.waker());
        trx::alloc_skb().map(|skb_idx| WhcTxToken {
            skb_idx,
            wlan_idx: self.wlan_idx,
        })
    }

    fn link_state(&mut self, cx: &mut TaskContext) -> LinkState {
        LINK_WAKER.register(cx.waker());
        if LINK_UP.load(Ordering::Relaxed) {
            LinkState::Up
        } else {
            LinkState::Down
        }
    }

    fn capabilities(&self) -> Capabilities {
        let mut caps = Capabilities::default();
        caps.max_transmission_unit = 1514;
        caps.max_burst_size = Some(1);
        caps
    }

    fn hardware_address(&self) -> NetHardwareAddress {
        NetHardwareAddress::Ethernet(self.mac)
    }
}
