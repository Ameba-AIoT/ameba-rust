//! WHC IPC message structs and constants (mirrors whc_ipc.h / whc_def.h).

// ── IPC TRX event_num values ────────────────────────────────────────────────
pub const MSG_READ_DONE: u32 = 0; // IPC_WIFI_MSG_READ_DONE
pub const MSG_MEMORY_NOT_ENOUGH: u32 = 1; // IPC_WIFI_MSG_MEMORY_NOT_ENOUGH
pub const MSG_RECV_DONE: u32 = 2; // IPC_WIFI_MSG_RECV_DONE (free NP SKB)
pub const CMD_XIMT_PKTS: u32 = 3; // IPC_WIFI_CMD_XIMT_PKTS (AP→NP TX)
pub const EVT_RECV_PKTS: u32 = 4; // IPC_WIFI_EVT_RECV_PKTS (NP→AP RX)

// ── WHC API IDs ─────────────────────────────────────────────────────────────
pub const API_PROCESS_DONE: u32 = 0;

// AP→NP (H2D) — BASIC_API_BASE (0x0000)
pub const API_H2D_WIFI_CONNECT: u32 = 0x0001;
pub const API_H2D_WIFI_DISCONNECT: u32 = 0x0002;
pub const API_H2D_WIFI_SET_USR_CFG: u32 = 0x0007;
pub const API_H2D_WIFI_SET_HOST_RTOS: u32 = 0x0008;
pub const API_H2D_WIFI_ON: u32 = 0x0009;
pub const API_H2D_WIFI_INIT_AP: u32 = 0x000A; // BASIC_API_BASE + 10
pub const API_H2D_WIFI_DEINIT_AP: u32 = 0x000B; // BASIC_API_BASE + 11
pub const API_H2D_WIFI_START_AP: u32 = 0x000C; // BASIC_API_BASE + 12
pub const API_H2D_WIFI_STOP_AP: u32 = 0x000D; // BASIC_API_BASE + 13
pub const API_H2D_WIFI_SCAN: u32 = 0x000E; // BASIC_API_BASE + 14
pub const API_H2D_WIFI_SCAN_ABORT: u32 = 0x0010; // BASIC_API_BASE + 16
pub const API_H2D_WIFI_START_JOIN_CMD: u32 = 0x0017; // BASIC_API_BASE + 23 — resume 802.11 join after PMK is ready

// AP→NP (H2D) — EXT_API_BASE (0x1000)
pub const API_H2D_WIFI_GET_MAC: u32 = 0x1002;
pub const API_H2D_WIFI_SEND_EAPOL: u32 = 0x1016; // EXT_API_BASE + 22

// AP→NP (H2D) — INTER_API_BASE (0x2000)
pub const API_H2D_WIFI_ADD_KEY: u32 = 0x2007; // INTER_API_BASE + 7

// NP→AP (D2H) — BASIC_API_BASE (0x0000)
pub const API_D2H_SCAN_USER_CALLBACK: u32 = 0x0001;
pub const API_D2H_WIFI_EVENT: u32 = 0x0006;
pub const API_D2H_GET_LWIP_INFO: u32 = 0x0008;
pub const API_D2H_SET_NETIF_INFO: u32 = 0x0009;
pub const API_D2H_IP_TABLE_CHK: u32 = 0x000B;

// WHC_WLAN_GET_* subtypes used in GET_LWIP_INFO param_buf[0]
pub const WHC_WLAN_GET_IP: u32 = 0;
pub const WHC_WLAN_GET_GW: u32 = 1;
pub const WHC_WLAN_GET_GWMSK: u32 = 2;
pub const WHC_WLAN_GET_HW_ADDR: u32 = 3;
pub const WHC_WLAN_IS_VALID_IP: u32 = 4;

// ── Security type bit flags (rtw_security_t) ────────────────────────────────
pub const SEC_OPEN: u32 = 0;
pub const SEC_WEP: u32 = 1 << 0;
pub const SEC_TKIP: u32 = 1 << 1;
pub const SEC_AES: u32 = 1 << 2;
pub const SEC_ENTERPRISE: u32 = 1 << 5;
pub const SEC_WPA: u32 = 1 << 21;
pub const SEC_WPA2: u32 = 1 << 22;
pub const SEC_WPA3: u32 = 1 << 23;
pub const SEC_WPA2_AES_PSK: u32 = SEC_WPA2 | SEC_AES;
pub const SEC_WPA3_AES_PSK: u32 = SEC_WPA3 | SEC_AES;
pub const SEC_WPA2_WPA3_MIXED: u32 = SEC_WPA2 | SEC_WPA3 | SEC_AES;

// ── 64-byte IPC messages ─────────────────────────────────────────────────────

// Field order in the three structs below is load-bearing, not cosmetic: each is
// 64-byte aligned and spans two 32-byte Cortex-M33 D-cache lines, and the SDK
// deliberately puts the "ready" flag (`event_num` / `api_id` / `enevt_id`) last
// so `DCache_Clean` — which flushes in ascending address order — publishes the
// payload line before the flag line. The peer therefore never observes a set
// flag alongside stale parameters. See the `Placed last to ensure it is flushed
// last` comments in whc_ipc.h (Jira RSWLANDIOT-15582); reordering these fields
// silently breaks the AP↔NP handshake while `size_of == 64` still holds.

/// Shared 64-byte message on the TRX IPC channel (ch6).
/// msg_addr / wlan_idx / event_num are the live fields; rsvd pads to 64 B.
#[repr(C, align(64))]
#[derive(Clone, Copy)]
pub struct WhcIpcExMsg {
    pub msg_addr: u32,
    pub msg_queue_status: u32,
    pub wlan_idx: u32,
    pub event_num: u32,
    pub rsvd: [u32; 12],
}
const _: () = assert!(core::mem::size_of::<WhcIpcExMsg>() == 64);
const _: () = assert!(core::mem::offset_of!(WhcIpcExMsg, event_num) == 12);

impl WhcIpcExMsg {
    pub const fn zeroed() -> Self {
        Self {
            msg_addr: 0,
            msg_queue_status: 0,
            wlan_idx: 0,
            event_num: MSG_READ_DONE, // ready to accept first send
            rsvd: [0u32; 12],
        }
    }
}

/// 64-byte AP→NP control request (API IPC channel ch7 A2N).
/// HOST_MSG_DUMY_NUM = 64 − (9+2)×4 = 20
#[repr(C, align(64))]
#[derive(Clone, Copy)]
pub struct WhcIpcHostReqMsg {
    pub ret: i32,
    pub param_buf: [u32; 9],
    pub api_id: u32,
    pub dummy: [u8; 20],
}
const _: () = assert!(core::mem::size_of::<WhcIpcHostReqMsg>() == 64);
const _: () = assert!(core::mem::offset_of!(WhcIpcHostReqMsg, api_id) == 40);

impl WhcIpcHostReqMsg {
    pub const fn zeroed() -> Self {
        Self {
            ret: 0,
            param_buf: [0u32; 9],
            api_id: API_PROCESS_DONE,
            dummy: [0u8; 20],
        }
    }
}

/// 64-byte NP→AP control request (API IPC channel ch7 N2A).
/// DEV_MSG_DUMY_NUM = 64 − (8+2)×4 = 24
#[repr(C, align(64))]
#[derive(Clone, Copy)]
pub struct WhcIpcDevReqMsg {
    pub ret: i32,
    pub param_buf: [u32; 8],
    pub enevt_id: u32, // note: "enevt_id" matches the SDK typo
    pub dummy: [u8; 24],
}
const _: () = assert!(core::mem::size_of::<WhcIpcDevReqMsg>() == 64);
const _: () = assert!(core::mem::offset_of!(WhcIpcDevReqMsg, enevt_id) == 36);

// ── WiFi connection parameters ───────────────────────────────────────────────

/// Packed SSID (34 bytes) — mirrors C `#pragma pack(1)` rtw_ssid.
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct RtwSsid {
    pub len: u8,
    pub val: [u8; 33],
}

/// Packed MAC address (6 bytes) — mirrors C `#pragma pack(1)` rtw_mac.
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct RtwMac {
    pub octet: [u8; 6],
}

/// PSK derivation request delivered via `RTW_EVENT_WPA_SET_PSK_INFO` (event
/// 108, `evt_info` pointer). Layout matches C `struct psk_info`
/// (`wpa_lite_intf.h`, no `#pragma pack`) — verified 208 bytes via
/// `_Static_assert(offsetof(...))` probes compiled with the project's ARM
/// toolchain, same method as [`RtwSoftapInfo`]. `psk_passphrase` is
/// `RTW_MAX_PSK_LEN + 1` = 129 bytes (`RTW_MAX_PSK_LEN` aliases
/// `RTW_WPA3_MAX_PSK_LEN` = 128, not the WPA2-only 64 — see
/// `wifi_api_types.h:70-75`); `security_type` is a plain C `enum` (4-byte
/// int on this ABI), whose values match the `SEC_*` constants above.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PskInfo {
    pub index: u8,
    pub psk_essid: [u8; 33],       // RTW_ESSID_MAX_SIZE + 1
    pub psk_passphrase: [u8; 129], // RTW_MAX_PSK_LEN + 1
    pub wpa_global_psk: [u8; 40],  // A_SHA_DIGEST_LEN * 2 — PMK cache / fast-reconnect
    pub security_type: u32,
}
const _: () = assert!(core::mem::size_of::<PskInfo>() == 208);
const _: () = assert!(core::mem::offset_of!(PskInfo, psk_essid) == 1);
const _: () = assert!(core::mem::offset_of!(PskInfo, psk_passphrase) == 34);
const _: () = assert!(core::mem::offset_of!(PskInfo, wpa_global_psk) == 163);
const _: () = assert!(core::mem::offset_of!(PskInfo, security_type) == 204);

/// WiFi connection parameters passed as `param_buf[0]` address to AP→NP API.
/// Layout matches C rtw_network_info (88 bytes, not packed at struct level).
#[repr(C)]
pub struct RtwNetworkInfo {
    pub ssid: RtwSsid,       // 34B at offset  0
    pub bssid: RtwMac,       //  6B at offset 34
    pub security_type: u32,  //  4B at offset 40
    pub password: u32,       //  4B at offset 44 (pointer)
    pub password_len: i32,   //  4B at offset 48
    pub key_id: i32,         //  4B at offset 52
    pub channel: u8,         //  1B at offset 56
    pub pscan_option: u8,    //  1B at offset 57
    pub is_wps_trigger: u8,  //  1B at offset 58
    pub wpa_supp: [u8; 18],  // 18B at offset 59 (RSNXE_MAX_LEN)
    pub prev_bssid: [u8; 6], //  6B at offset 77
    pub by_reconn: u8,       //  1B at offset 83
    pub rom_rsvd: [u8; 4],   //  4B at offset 84
}
const _: () = assert!(core::mem::size_of::<RtwNetworkInfo>() == 88);

// ── SoftAP parameters ────────────────────────────────────────────────────────

/// SoftAP start config passed as `param_buf[0]` address to `WHC_API_WIFI_START_AP`.
/// Layout matches C rtw_softap_info (48 bytes, not packed at struct level).
/// Verified against `sdk/component/wifi/api/wifi_api_types.h` via
/// `_Static_assert(offsetof(...))` probes compiled with the project's ARM
/// toolchain (arm-elf-xgcc, cortex-m33, thumb) — the C struct's bitfield
/// (`b_no_rsp_to_probereq : 1`) occupies a whole byte on this ABI, so it is
/// modelled here as a plain `u8` with only bit 0 significant.
#[repr(C)]
pub struct RtwSoftapInfo {
    pub ssid: RtwSsid,      // 34B at offset  0
    pub hidden_ssid: u8,    //  1B at offset 34
    pub security_type: u32, //  4B at offset 36
    pub password: u32,      //  4B at offset 40 (pointer)
    pub password_len: u8,   //  1B at offset 44
    pub channel: u8,        //  1B at offset 45
    pub b_no_rsp_to_probereq: u8, //  1B at offset 46 (bit 0 only)
                            // 1 byte tail padding at offset 47 -> size 48
}
const _: () = assert!(core::mem::size_of::<RtwSoftapInfo>() == 48);
const _: () = assert!(core::mem::offset_of!(RtwSoftapInfo, hidden_ssid) == 34);
const _: () = assert!(core::mem::offset_of!(RtwSoftapInfo, security_type) == 36);
const _: () = assert!(core::mem::offset_of!(RtwSoftapInfo, password) == 40);
const _: () = assert!(core::mem::offset_of!(RtwSoftapInfo, password_len) == 44);
const _: () = assert!(core::mem::offset_of!(RtwSoftapInfo, channel) == 45);
const _: () = assert!(core::mem::offset_of!(RtwSoftapInfo, b_no_rsp_to_probereq) == 46);

// ── WPA 4-way handshake session params (event 104) ──────────────────────────

/// `AuthKeyMgmt` values (`wpa_lite_intf.h`) — only the PSK ones are currently supported.
pub const AUTH_KEY_MGMT_PSK: u32 = 1 << 1;
pub const AUTH_KEY_MGMT_PSK2: u32 = 1 << 6;

/// Per-STA/session WPA parameters delivered via `RTW_EVENT_WPA_STA_4WAY_START`
/// (event 104, `evt_info` pointer). Layout matches C `struct wpa_param_t`
/// (`wpa_lite_intf.h`, no `#pragma pack`) — verified 240 bytes via
/// `_Static_assert(offsetof(...))` probes compiled with the project's ARM
/// toolchain, same method as [`PskInfo`]/[`RtwSoftapInfo`]/[`RtwCryptInfo`].
/// `AuthInfoBuf` is the peer's raw RSN/WPA IE (`INFO_ELEMENT_SIZE` = 128,
/// TLV-prefixed: byte 0 = element ID, byte 1 = length, bytes 2.. = value).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WpaParam {
    pub peer_mac: [u8; 6],
    pub self_mac: [u8; 6],
    pub unicast_cipher: u32,
    pub is_enterprise: u8,
    pub rsn_enabled: i32,
    pub dot118021x_grp_privacy: u32,
    pub auth_key_mgmt: u32,
    pub auth_info_buf: [u8; 128],
    pub dot11_wpa_mode: u32,
    pub rsnxe_ie: [u8; 18],
    pub pmksa_id: [u8; 16],
    pub pmk: [u8; 32],
    pub use_pmksa: u8,
    pub ap_gbl_info: u8,
    pub eapol_key_rsnd_limit: u8,
    pub rom_rsvd: u32,
}
const _: () = assert!(core::mem::size_of::<WpaParam>() == 240);
const _: () = assert!(core::mem::offset_of!(WpaParam, self_mac) == 6);
const _: () = assert!(core::mem::offset_of!(WpaParam, unicast_cipher) == 12);
const _: () = assert!(core::mem::offset_of!(WpaParam, is_enterprise) == 16);
const _: () = assert!(core::mem::offset_of!(WpaParam, rsn_enabled) == 20);
const _: () = assert!(core::mem::offset_of!(WpaParam, dot118021x_grp_privacy) == 24);
const _: () = assert!(core::mem::offset_of!(WpaParam, auth_key_mgmt) == 28);
const _: () = assert!(core::mem::offset_of!(WpaParam, auth_info_buf) == 32);
const _: () = assert!(core::mem::offset_of!(WpaParam, dot11_wpa_mode) == 160);
const _: () = assert!(core::mem::offset_of!(WpaParam, rsnxe_ie) == 164);
const _: () = assert!(core::mem::offset_of!(WpaParam, pmksa_id) == 182);
const _: () = assert!(core::mem::offset_of!(WpaParam, pmk) == 198);
const _: () = assert!(core::mem::offset_of!(WpaParam, use_pmksa) == 230);
const _: () = assert!(core::mem::offset_of!(WpaParam, ap_gbl_info) == 231);
const _: () = assert!(core::mem::offset_of!(WpaParam, eapol_key_rsnd_limit) == 232);
const _: () = assert!(core::mem::offset_of!(WpaParam, rom_rsvd) == 236);

// ── Key install (WHC_API_WIFI_ADD_KEY) ──────────────────────────────────────

/// `_AES_` cipher value for `RtwCryptInfo::driver_cipher` (CCMP-128/256).
/// Only cipher currently used by the 4-way handshake (AES-CCMP PSK).
pub const DRIVER_CIPHER_AES: u8 = 0x4;

/// Key install request passed as `param_buf[0]` to `WHC_API_WIFI_ADD_KEY`.
/// Layout matches C `struct rtw_crypt_info` (`wifi_intf_drv_to_app_internal.h`,
/// no `#pragma pack`) — verified 48 bytes via `_Static_assert(offsetof(...))`
/// probes compiled with the project's ARM toolchain (arm-none-eabi-gcc,
/// cortex-m33, thumb), same method as [`RtwSoftapInfo`]/[`PskInfo`]. The C
/// struct's trailing bitfields (`device_id:5`, `force_cam_entry:1`,
/// `rpt_mode:1`) pack into a single byte on this ABI, modelled here as a
/// plain `u8` (bit 0..4 = device_id, bit 5 = force_cam_entry, bit 6 = rpt_mode).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RtwCryptInfo {
    pub pairwise: u8,                  //  1B at offset  0 — group(0) or pairwise(1) key
    pub mac_addr: [u8; 6],             //  6B at offset  1
    pub wlan_idx: u8,                  //  1B at offset  7
    pub key_len: u16,                  //  2B at offset  8
    pub key: [u8; 32],                 // 32B at offset 10
    pub key_idx: u8,                   //  1B at offset 42
    pub driver_cipher: u8,             //  1B at offset 43 (DRIVER_CIPHER_AES)
    pub transition_disable_exist: u8,  //  1B at offset 44
    pub transition_disable_bitmap: u8, //  1B at offset 45
    pub device_id_flags: u8,           //  1B at offset 46 (bitfield byte)
                                       // 1 byte tail padding at offset 47 -> size 48
}
const _: () = assert!(core::mem::size_of::<RtwCryptInfo>() == 48);
const _: () = assert!(core::mem::offset_of!(RtwCryptInfo, mac_addr) == 1);
const _: () = assert!(core::mem::offset_of!(RtwCryptInfo, wlan_idx) == 7);
const _: () = assert!(core::mem::offset_of!(RtwCryptInfo, key_len) == 8);
const _: () = assert!(core::mem::offset_of!(RtwCryptInfo, key) == 10);
const _: () = assert!(core::mem::offset_of!(RtwCryptInfo, key_idx) == 42);
const _: () = assert!(core::mem::offset_of!(RtwCryptInfo, driver_cipher) == 43);
const _: () = assert!(core::mem::offset_of!(RtwCryptInfo, transition_disable_exist) == 44);
const _: () = assert!(core::mem::offset_of!(RtwCryptInfo, transition_disable_bitmap) == 45);

// ── sk_buff — AP-side TX pool entry ─────────────────────────────────────────

/// SKB_CACHE_SZ = 32 on RTL8721F (8721F, non-ProX3).
/// SKB_WLAN_TX_EXTRA_LEN = TXDESC_SIZE(40) + HDR_A4_QOS_HTC(36) + IV(8) + SNAP(8) = 92.
pub const SKB_CACHE_SZ: usize = 32;
pub const SKB_TX_HEADROOM: usize = 92; // SKB_DATA_ALIGN(SKB_WLAN_TX_EXTRA_LEN)
pub const SKB_DATA_MAX: usize = 1514; // Ethernet max frame
pub const SKB_BUF_SIZE: usize =
    (SKB_TX_HEADROOM + SKB_DATA_MAX + SKB_CACHE_SZ - 1) & !(SKB_CACHE_SZ - 1); // round up to cache line

/// Number of AP-side TX SKBs in the static pool.
pub const N_SKB_AP: usize = 4;

/// AP-side sk_buff entry (mirrors C struct sk_buff, align(32)).
/// Fields ordered to match C layout exactly.
#[repr(C, align(32))]
pub struct SkBuff {
    pub list: [u32; 2],    //  8B at  0 (list_head.next / .prev)
    pub buf: u32,          //  4B at  8  (base of data buffer)
    pub head: u32,         //  4B at 12
    pub data: u32,         //  4B at 16
    pub tail: u32,         //  4B at 20
    pub end: u32,          //  4B at 24
    pub dev: u32,          //  4B at 28
    pub len: u32,          //  4B at 32
    pub dyalloc_flag: i32, //  4B at 36
    pub busy: u8,          //  1B at 40
    pub no_free: u8,       //  1B at 41
    pub tx_raw: [u8; 13],  // 13B at 42 (skb_raw_para)
    pub _pad1: u8,         //  1B at 55 (aligns ref_count)
    pub ref_count: i32,    //  4B at 56
                           // size = 60B; repr(C, align(32)) → padded to 64B
}
const _: () = assert!(core::mem::size_of::<SkBuff>() == 64);
