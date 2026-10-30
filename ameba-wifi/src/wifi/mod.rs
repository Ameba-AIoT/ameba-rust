//! High-level WiFi API built on top of WHC.

pub mod events;
pub(crate) mod handshake;
pub(crate) mod psk;

use core::cell::UnsafeCell;

use crate::whc::api::send_api_req;
use crate::whc::msg::*;

// ── Sync wrapper for UnsafeCell (single-core, no threads) ────────────────────

struct SyncUnsafeCell<T>(UnsafeCell<T>);
unsafe impl<T> Sync for SyncUnsafeCell<T> {}

impl<T> SyncUnsafeCell<T> {
    fn get(&self) -> *mut T {
        self.0.get()
    }
}

// ── Security type ─────────────────────────────────────────────────────────────

/// Security type for `wifi_connect`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SecurityType {
    Open = SEC_OPEN,
    Wpa2AesPsk = SEC_WPA2_AES_PSK,
    Wpa3AesPsk = SEC_WPA3_AES_PSK,
    Wpa2Wpa3Mix = SEC_WPA2_WPA3_MIXED,
}

// ── Static connect parameters (allocated once) ────────────────────────────────

/// Static storage for RtwNetworkInfo used by `wifi_connect`.
static NET_INFO: SyncUnsafeCell<RtwNetworkInfo> = SyncUnsafeCell(UnsafeCell::new(
    // SAFETY: zeroed bytes are valid for RtwNetworkInfo (all-zero is open/empty).
    unsafe { core::mem::zeroed() },
));

// ── wifi_user_conf byte buffer ────────────────────────────────────────────────
//
// NP's SET_USR_CFG handler dereferences param_buf[0] as *wifi_user_conf,
// DCache_Invalidates it, then memcpy's into NP's wifi_user_config.
// sizeof(wifi_user_conf) = 92 bytes (from ameba_wificfg_common.h, no pragma pack).
// Must be 64-byte aligned (matching C SDK __attribute__((aligned(64)))).
//
// Field offsets (all non-zero values from wifi_set_user_config() for IPC mode):
//   [ 1]: tx_pwr_table_selection = 2
//   [ 5]: antdiv_mode = RTW_ANTDIV_DISABLE = 0xF
//   [ 6]: ap_sta_num = 5
//   [ 8]: ips_enable = 1
//   [11]: lps_enable = 1
//   [14]: lps_rx_unicast_pkt_timeout = 40
//   [18]: ampdu_rx_enable = 1
//   [19]: ampdu_tx_enable = 1
//   [20]: ap_compatibilty_enabled = 0x07
//   [21]: set_channel_api_do_rfk = 1
//   [30]: freq_band_support = RTW_SUPPORT_BAND_MAX = 4
//   [32]: fast_reconnect_en = 1
//   [33]: auto_reconnect_en = 1
//   [34]: auto_reconnect_count = 10
//   [35]: auto_reconnect_interval = 5 s
//   [36]: no_beacon_disconnect_time = 9 (×2 s = 18 s)
//   [40..43]: skb_num_np = 10 (s32 LE) — NP RX SKB pool count
//   [44..47]: skb_num_ap = 4 (s32 LE) — IPC msg queue size = skb_num_np + skb_num_ap
//   [48..51]: skb_buf_size = 1664 = 0x680 (u32 LE) — each SKB payload size
//   [54]: mcc_force_p1_slot_ratio = 44
//   [55]: tx_shortcut_enable = 1
//   [56]: rx_shortcut_enable = 1
//   [57]: probe_hidden_ap_on_passive_ch = 1
//   [58]: concurrent_enabled = 1
//   [59]: softap_addr_offset_idx = 1
//   [60]: rx_ampdu_num = 4
//   [62]: sgi = 1
//   [63]: he_gi_ltf_cap = RTW_HE_GI_LTF_ALL = 0xFF
//   [64]: tx_ampdu_num = 20
//   [65]: cfg80211 = 0 (RTOS mode; 1 = Linux, uses 4× more NP heap)
//   [66]: wps_retry_count = 4
//   [68..71]: wps_retry_interval = 5000 (u32 LE)
//   [73]: keepalive_interval = 20
//   [76]: wtn_strong_rssi_thresh = -50 (0xCE)
//   [78..79]: wtn_father_refresh_timeout = 3000 (u16 LE)
//   [80..81]: wtn_child_refresh_timeout = 4000 (u16 LE)
//   [86..87]: wtn_max_node_num = 32 (u16 LE)
const RTOS_WIFI_USER_CONF: [u8; 96] = {
    let mut b = [0u8; 96];
    b[1] = 2; // tx_pwr_table_selection
    b[5] = 0x0F; // antdiv_mode = RTW_ANTDIV_DISABLE
    b[6] = 5; // ap_sta_num
    b[8] = 1; // ips_enable
    b[11] = 1; // lps_enable
    b[14] = 40; // lps_rx_unicast_pkt_timeout (ms)
    b[18] = 1; // ampdu_rx_enable
    b[19] = 1; // ampdu_tx_enable
    b[20] = 0x07; // ap_compatibilty_enabled
    b[21] = 1; // set_channel_api_do_rfk
    b[30] = 4; // freq_band_support = RTW_SUPPORT_BAND_MAX
    b[32] = 1; // fast_reconnect_en
    b[33] = 1; // auto_reconnect_en
    b[34] = 10; // auto_reconnect_count
    b[35] = 5; // auto_reconnect_interval (s)
    b[36] = 9; // no_beacon_disconnect_time (×2 s = 18 s)
    b[40] = 10; // skb_num_np (s32 LE, low byte)
    b[44] = 4; // skb_num_ap (s32 LE, low byte)
    b[48] = 0x80; // skb_buf_size = 1664 = 0x0680 (u32 LE, byte 0)
    b[49] = 0x06; // skb_buf_size byte 1
    b[54] = 44; // mcc_force_p1_slot_ratio
    b[55] = 1; // tx_shortcut_enable
    b[56] = 1; // rx_shortcut_enable
    b[57] = 1; // probe_hidden_ap_on_passive_ch
    b[58] = 1; // concurrent_enabled
    b[59] = 1; // softap_addr_offset_idx
    b[60] = 4; // rx_ampdu_num
    b[62] = 1; // sgi
    b[63] = 0xFF; // he_gi_ltf_cap = RTW_HE_GI_LTF_ALL
    b[64] = 20; // tx_ampdu_num
    // b[65] = 0     cfg80211 = 0 (RTOS mode)
    b[66] = 4; // wps_retry_count
    b[68] = 0x88; // wps_retry_interval = 5000 = 0x1388 (u32 LE, byte 0)
    b[69] = 0x13; // wps_retry_interval byte 1
    b[73] = 20; // keepalive_interval (s)
    b[76] = 0xCE; // wtn_strong_rssi_thresh = -50 (s8)
    b[78] = 0xB8; // wtn_father_refresh_timeout = 3000 = 0x0BB8 (u16 LE, byte 0)
    b[79] = 0x0B; // wtn_father_refresh_timeout byte 1
    b[80] = 0xA0; // wtn_child_refresh_timeout = 4000 = 0x0FA0 (u16 LE, byte 0)
    b[81] = 0x0F; // wtn_child_refresh_timeout byte 1
    b[86] = 32; // wtn_max_node_num (u16 LE, low byte)
    b
};

// 64-byte aligned to match C SDK __attribute__((aligned(64))) for DCache ops.
#[repr(align(64))]
struct Aligned64<const N: usize>([u8; N]);

static WIFI_USER_CFG: SyncUnsafeCell<Aligned64<96>> =
    SyncUnsafeCell(UnsafeCell::new(Aligned64(RTOS_WIFI_USER_CONF)));

/// Static storage for the password string used by `wifi_connect`.
static mut PWD_BUF: [u8; 64] = [0u8; 64];

// ── WiFi init ────────────────────────────────────────────────────────────────

// AON_WIFI_IPC register: NP sets bit 31 when it has finished FW init and
// registered its IPC interrupt handler. AP must wait for this before
// sending any IPC messages, otherwise the IPC interrupt fires before NP's
// handler is registered → unhandled IRQ → NP HardFault.
// C SDK equivalent: whc_ipc_host_init() polls AON_BIT_WIFI_INIC_NP_READY.
const REG_AON_WIFI_IPC: *mut u32 = 0x4080_A074 as *mut u32;
const AON_BIT_WIFI_INIC_NP_READY: u32 = 1 << 31;

/// Turn on the WiFi NP in station mode (RTW_MODE_STA = 1).
/// Must be called once before any connection attempt.
pub async fn wifi_on() -> i32 {
    use embassy_time::{Duration, Timer};

    const RTW_MODE_STA: u32 = 1;

    // Wait for NP to signal it has finished initialization and registered its
    // IPC handlers. Without this wait, the very first IPC send triggers an
    // unhandled interrupt on NP → HardFault.
    loop {
        let val = unsafe { core::ptr::read_volatile(REG_AON_WIFI_IPC) };
        if val & AON_BIT_WIFI_INIC_NP_READY != 0 {
            unsafe {
                core::ptr::write_volatile(REG_AON_WIFI_IPC, val & !AON_BIT_WIFI_INIC_NP_READY)
            };
            break;
        }
        Timer::after(Duration::from_millis(1)).await;
    }

    // Mirror the C SDK sequence: SET_HOST_RTOS → SET_USR_CFG → WIFI_ON.
    // NP's whc_ipc_dev_init() does NOT call wifi_set_user_config() — it only
    // creates the API task and signals ready. NP's wifi_user_config starts as
    // all zeros until AP sends SET_USR_CFG. Without this, skb_buf_size=0 and
    // skb_num_ap=0, causing NP heap corruption during wifi_on boot.
    send_api_req(API_H2D_WIFI_SET_HOST_RTOS, &[]).await;

    let cfg_ptr = unsafe {
        let p = (*WIFI_USER_CFG.get()).0.as_ptr() as usize;
        crate::cache::clean(p, core::mem::size_of::<Aligned64<96>>());
        p
    };
    send_api_req(API_H2D_WIFI_SET_USR_CFG, &[cfg_ptr as u32]).await;

    send_api_req(API_H2D_WIFI_ON, &[RTW_MODE_STA]).await
}

// ── WiFi connect ─────────────────────────────────────────────────────────────

/// Connect to a WiFi network.
///
/// `ssid` must be ≤ 32 bytes; `password` must be ≤ 63 bytes.
/// Returns 0 on success (NP accepted the request; the actual 4-way HS result
/// arrives asynchronously as a `WifiEvent`).
pub async fn wifi_connect(ssid: &str, password: &str, security: SecurityType) -> i32 {
    let ssid_b = ssid.as_bytes();
    let pwd_b = password.as_bytes();

    if ssid_b.len() > 32 || pwd_b.len() > 63 {
        return -1;
    }

    unsafe {
        // Copy password to static buffer (NP will read via pointer).
        let pwd_len = pwd_b.len();
        PWD_BUF[..pwd_len].copy_from_slice(pwd_b);
        PWD_BUF[pwd_len] = 0;

        // Fill RtwNetworkInfo.
        let ni = &mut *NET_INFO.get();
        *ni = core::mem::zeroed();

        ni.ssid.len = ssid_b.len() as u8;
        ni.ssid.val[..ssid_b.len()].copy_from_slice(ssid_b);

        ni.security_type = security as u32;
        if pwd_len > 0 {
            ni.password = core::ptr::addr_of!(PWD_BUF) as u32;
            ni.password_len = pwd_len as i32;
        }

        // DCache_Clean so NP can read the struct and password.
        crate::cache::clean(
            NET_INFO.get() as usize,
            core::mem::size_of::<RtwNetworkInfo>(),
        );
        crate::cache::clean(core::ptr::addr_of!(PWD_BUF) as usize, 64);
    }

    // param_buf[0] = pointer to RtwNetworkInfo, param_buf[1] = wlan_idx (0 = STA)
    let net_info_ptr = NET_INFO.get() as u32;
    send_api_req(API_H2D_WIFI_CONNECT, &[net_info_ptr, 0]).await
}

// ── WiFi disconnect ───────────────────────────────────────────────────────────

/// Disconnect from the current WiFi network on interface `wlan_idx` (0 = STA).
pub async fn wifi_disconnect(wlan_idx: u8) -> i32 {
    send_api_req(API_H2D_WIFI_DISCONNECT, &[wlan_idx as u32]).await
}

// ── SoftAP ───────────────────────────────────────────────────────────────────
//
// Current scope: basic start/stop flow only. `wifi_start_ap`/`wifi_stop_ap` on the
// C side (dispatched via WHC_API_WIFI_START_AP/STOP_AP) already call
// `_wifi_on_ap`/`_wifi_off_ap` internally, so no separate init/deinit call is
// needed here. Multi-client management, hidden SSID, WPA3-AP and STA+AP
// concurrent operation are deferred.

/// Static storage for RtwSoftapInfo used by `wifi_start_ap`.
static SOFTAP_INFO: SyncUnsafeCell<RtwSoftapInfo> = SyncUnsafeCell(UnsafeCell::new(
    // SAFETY: zeroed bytes are valid for RtwSoftapInfo (all-zero is open/empty).
    unsafe { core::mem::zeroed() },
));

/// Static storage for the SoftAP password (max WPA2-PSK length, 64 bytes; no
/// null terminator needed — NP reads exactly `password_len` bytes).
static mut AP_PWD_BUF: [u8; 64] = [0u8; 64];

/// Start the SoftAP with the given SSID/password/security/channel.
///
/// Current scope: `security` must be [`SecurityType::Open`] or
/// [`SecurityType::Wpa2AesPsk`] — WPA3-AP is not wired up yet. SSID is always
/// broadcast (no hidden-SSID support yet).
///
/// `ssid` must be 1..=32 bytes. `password` must be empty for `Open`, or
/// 8..=64 bytes for `Wpa2AesPsk` (mirrors the C SDK's `RTW_MIN_PSK_LEN`/
/// `RTW_WPA2_MAX_PSK_LEN` bounds in `wifi_start_ap`).
/// Returns the `ret` NP wrote (0 = success).
pub async fn wifi_start_ap(ssid: &str, password: &str, security: SecurityType, channel: u8) -> i32 {
    if !matches!(security, SecurityType::Open | SecurityType::Wpa2AesPsk) {
        return -1;
    }

    let ssid_b = ssid.as_bytes();
    let pwd_b = password.as_bytes();

    if ssid_b.is_empty() || ssid_b.len() > 32 {
        return -1;
    }
    match security {
        SecurityType::Open => {
            if !pwd_b.is_empty() {
                return -1;
            }
        }
        SecurityType::Wpa2AesPsk => {
            if pwd_b.len() < 8 || pwd_b.len() > 64 {
                return -1;
            }
        }
        _ => unreachable!(),
    }

    unsafe {
        let ap = &mut *SOFTAP_INFO.get();
        *ap = core::mem::zeroed();

        ap.ssid.len = ssid_b.len() as u8;
        ap.ssid.val[..ssid_b.len()].copy_from_slice(ssid_b);
        ap.security_type = security as u32;
        ap.channel = channel;

        if !pwd_b.is_empty() {
            AP_PWD_BUF[..pwd_b.len()].copy_from_slice(pwd_b);
            ap.password = core::ptr::addr_of!(AP_PWD_BUF) as u32;
            ap.password_len = pwd_b.len() as u8;
        }

        // DCache_Clean so NP can read the struct and password (mirrors wifi_connect).
        crate::cache::clean(
            SOFTAP_INFO.get() as usize,
            core::mem::size_of::<RtwSoftapInfo>(),
        );
        crate::cache::clean(core::ptr::addr_of!(AP_PWD_BUF) as usize, 64);
    }

    // param_buf[0] = pointer to RtwSoftapInfo, param_buf[1] = struct size
    // (mirrors whc_ipc_dev_api.c's WHC_API_WIFI_START_AP case: buf/size ->
    // DCache_Invalidate -> cast to struct rtw_softap_info*).
    let info_ptr = SOFTAP_INFO.get() as u32;
    send_api_req(
        API_H2D_WIFI_START_AP,
        &[info_ptr, core::mem::size_of::<RtwSoftapInfo>() as u32],
    )
    .await
}

/// Stop the SoftAP. NP tears down the AP interface (`_wifi_off_ap`) internally.
pub async fn wifi_stop_ap() -> i32 {
    send_api_req(API_H2D_WIFI_STOP_AP, &[]).await
}

// ── WiFi scan ────────────────────────────────────────────────────────────────

/// Trigger a full channel scan. Scan results are delivered as
/// `WifiEvent(event_code::SCAN_DONE)` events.
pub async fn wifi_scan() -> i32 {
    // param_buf[0] = NULL (no filter), param_buf[1] = scan_type (0 = active), param_buf[2..] = 0
    send_api_req(API_H2D_WIFI_SCAN, &[0, 0, 0]).await
}

/// Abort a scan started by [`wifi_scan`].
///
/// Returns NP's status: 0 once the abort is accepted, non-zero if no scan was
/// running. The C SDK's `wifi_scan_abort` reports that second case as success and
/// then blocks until NP confirms; this returns it as-is and does not wait —
/// `SCAN_DONE` still arrives as a `WifiEvent`, so await that if completion matters.
pub async fn wifi_scan_abort() -> i32 {
    send_api_req(API_H2D_WIFI_SCAN_ABORT, &[]).await
}

// ── MAC address query ────────────────────────────────────────────────────────

/// The STA MAC NP reports through `SET_NETIF_INFO` during `wifi_on`.
///
/// This is the preferred way to learn the MAC — see [`netif_mac`] for why it is
/// better than [`wifi_get_mac_address`].
pub use crate::whc::api::netif_mac;

/// Static result buffer for `wifi_get_mac_address` (NP writes via pointer).
static MAC_RESULT: SyncUnsafeCell<RtwMac> =
    SyncUnsafeCell(UnsafeCell::new(RtwMac { octet: [0; 6] }));

/// Query NP for the real MAC address of interface `idx` (0 = STA) via
/// `WHC_API_WIFI_GET_MAC_ADDR` (mirrors C SDK's `wifi_get_mac_address`).
///
/// Prefer [`netif_mac`], which needs no round trip to NP.
///
/// Not safe to call concurrently with itself — shares one static result
/// buffer. Intended for a single call during startup, before constructing an
/// `embassy_net_driver::Driver` (whose `hardware_address()` is synchronous
/// and needs the MAC cached ahead of time).
pub async fn wifi_get_mac_address(idx: u8) -> Option<[u8; 6]> {
    let ptr = MAC_RESULT.get() as u32;
    unsafe {
        crate::cache::clean(ptr as usize, core::mem::size_of::<RtwMac>());
    }

    // param_buf: [idx, &mac_result, efuse=0], mirroring wifi_get_mac_address(idx, mac, 0).
    let ret = send_api_req(API_H2D_WIFI_GET_MAC, &[idx as u32, ptr, 0]).await;
    if ret != 0 {
        return None;
    }

    unsafe {
        crate::cache::invalidate(ptr as usize, core::mem::size_of::<RtwMac>());
        Some((*MAC_RESULT.get()).octet)
    }
}
