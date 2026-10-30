//! WiFi events forwarded from NP via WHC_API_WIFI_EVENT.

/// Raw WiFi event delivered by NP.
///
/// `event_code` is the `rtw_event_id` enum value; `evt_info` is a pointer to
/// additional event data in NP SRAM (may be 0).  Invalidate the cache before
/// dereferencing.
#[derive(Clone, Copy, Debug)]
pub struct WifiEvent {
    pub event_code: u32,
    pub evt_info: u32, // *const u8 in NP SRAM; 0 if unused
}

/// Event identifiers matching the SDK's `rtw_event_id` enum (wifi_api_event.h
/// for 0-8; wifi_intf_drv_to_app_internal.h for 104-108, a separate internal
/// enum NP also multiplexes through the same `WHC_API_WIFI_EVENT` path).
pub mod event_code {
    pub const AP_STA_ASSOC: u32 = 0; // SoftAP: station associated
    pub const AP_STA_DISASSOC: u32 = 1; // SoftAP: station disassociated
    pub const JOIN_STATUS: u32 = 2; // STA: connection status change — read join_status::* from evt_info
    pub const CSI_DONE: u32 = 3;
    pub const WPS_START: u32 = 4;
    pub const WPS_FINISH: u32 = 5;
    pub const EAPOL_START: u32 = 6;
    pub const EAPOL_RECVD: u32 = 7;
    pub const DHCP_STATUS: u32 = 8;

    // WPA2-PSK 4-way handshake (STA connecting to a WPA2 AP). `evt_info`
    // points at NP SRAM; see `crate::whc::msg::PskInfo` for the 108 payload,
    // and `rtw_event_report_frame` layout (frame_len: u32, frame: [u8]) for
    // 106/107's raw 802.11 EAPOL frame.
    pub const WPA_STA_4WAY_START: u32 = 104; // STA: 4-way handshake starting, wait for M1
    pub const WPA_AP_4WAY_START: u32 = 105; // SoftAP: 4-way handshake starting (not used until SoftAP-WPA2)
    pub const WPA_STA_4WAY_RECV: u32 = 106; // STA: EAPOL frame received, evt_info -> rtw_event_report_frame
    pub const WPA_AP_4WAY_RECV: u32 = 107; // SoftAP: EAPOL frame received (not used until SoftAP-WPA2)
    pub const WPA_SET_PSK_INFO: u32 = 108; // STA: NP requests PMK — evt_info -> PskInfo, AP must PBKDF2 + wifi_start_join_cmd
}

/// `status` byte in `rtw_event_join_status_info`, mirroring `rtw_join_status`.
///
/// When `event_code == JOIN_STATUS`, read `evt_info + 8` (after the 8-byte priv
/// union) to get this value.
pub mod join_status {
    pub const UNKNOWN: u8 = 0;
    pub const STARTING: u8 = 1;
    pub const SCANNING: u8 = 2;
    pub const AUTHENTICATING: u8 = 3;
    pub const AUTHENTICATED: u8 = 4;
    pub const ASSOCIATING: u8 = 5;
    pub const ASSOCIATED: u8 = 6;
    pub const HANDSHAKING: u8 = 7;
    pub const HANDSHAKE_DONE: u8 = 8;
    pub const SUCCESS: u8 = 9;
    pub const FAIL: u8 = 10;
    pub const DISCONNECT: u8 = 11;
}

/// Read the `status` byte from a `rtw_event_join_status_info` pointer in NP SRAM.
///
/// The `priv` union is 8 bytes (largest member is `fail`: s32+u16 = 6B, padded
/// to 8 by C alignment rules); `status` immediately follows at offset 8.
///
/// # Safety
/// `evt_info` must be a valid NP SRAM address for at least 9 bytes.
pub unsafe fn read_join_status(evt_info: u32) -> u8 {
    unsafe {
        crate::cache::invalidate(evt_info as usize, 16);
        core::ptr::read_volatile((evt_info + 8) as *const u8)
    }
}

/// Read the station MAC from a `rtw_event_ap_sta_assoc` or
/// `rtw_event_ap_sta_disassoc` pointer in NP SRAM (both start with a plain
/// `u8 sta_mac[ETH_ALEN]` at offset 0 — see `wifi_api_event.h`).
///
/// # Safety
/// `evt_info` must be a valid NP SRAM address for at least 6 bytes.
pub unsafe fn read_ap_sta_mac(evt_info: u32) -> [u8; 6] {
    unsafe {
        crate::cache::invalidate(evt_info as usize, 6);
        let mut mac = [0u8; 6];
        for (i, b) in mac.iter_mut().enumerate() {
            *b = core::ptr::read_volatile((evt_info as *const u8).add(i));
        }
        mac
    }
}
