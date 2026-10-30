//! PMK derivation for WPA/WPA2-PSK — handles `RTW_EVENT_WPA_SET_PSK_INFO`
//! (event 108). Mirrors `rtw_psk_set_psk_info()` / `rtw_psk_set_psk_info_evt_hdl()`
//! (`wpa_lite_psk.c:1480-1530`).
//!
//! Must run to completion — including the PBKDF2 pass — before `run_blk_api`
//! ACKs the enclosing `WHC_API_WIFI_EVENT` message: NP allocates `psk_info`
//! with `rtos_mem_zmalloc` and only frees it after that ACK poll resolves
//! (`whc_dev_api_path.c:1136-1148`), so deferring the read to a later
//! consumer task (as `EVENT_CHAN` does for app-facing events) would race a
//! freed/reused buffer.

use core::cell::UnsafeCell;

use pbkdf2::pbkdf2_hmac;
use sha1::Sha1;

use crate::whc::api::send_api_req;
use crate::whc::msg::*;

struct SyncUnsafeCell<T>(UnsafeCell<T>);
unsafe impl<T> Sync for SyncUnsafeCell<T> {}
impl<T> SyncUnsafeCell<T> {
    fn get(&self) -> *mut T {
        self.0.get()
    }
}

// The 9 `rtw_security_t` values for which NP expects a PSK/PMK
// (`RTW_SECURITY_WPA*_PSK` family — WPA/WPA2, TKIP/AES/mixed cipher, no
// enterprise, no SAE/WPA3). Exact-match gate mirroring
// `rtw_psk_set_psk_info()` (`wpa_lite_psk.c:1495-1503`).
const WPA_TKIP_PSK: u32 = SEC_WPA | SEC_TKIP;
const WPA_AES_PSK: u32 = SEC_WPA | SEC_AES;
const WPA_MIXED_PSK: u32 = SEC_WPA | SEC_AES | SEC_TKIP;
const WPA2_TKIP_PSK: u32 = SEC_WPA2 | SEC_TKIP;
const WPA2_MIXED_PSK: u32 = SEC_WPA2 | SEC_AES | SEC_TKIP;
const WPA_WPA2_TKIP_PSK: u32 = SEC_WPA | SEC_WPA2 | SEC_TKIP;
const WPA_WPA2_AES_PSK: u32 = SEC_WPA | SEC_WPA2 | SEC_AES;
const WPA_WPA2_MIXED_PSK: u32 = SEC_WPA | SEC_WPA2 | SEC_AES | SEC_TKIP;

fn is_wpa_psk_security(sec: u32) -> bool {
    matches!(
        sec,
        WPA_TKIP_PSK
            | WPA_AES_PSK
            | WPA_MIXED_PSK
            | WPA2_TKIP_PSK
            | SEC_WPA2_AES_PSK
            | WPA2_MIXED_PSK
            | WPA_WPA2_TKIP_PSK
            | WPA_WPA2_AES_PSK
            | WPA_WPA2_MIXED_PSK
    )
}

/// Cached PMK-derivation state, mirroring C's per-index `wpa_lite[index]`
/// (`global_psk_cfg`/`passphrase_cfg`/`psk_ssid_cfg`/`security_type`). Only
/// wlan index 0 (STA) is supported — SoftAP-WPA2 (index-carrying events
/// 105/107) is not wired up yet.
struct PskState {
    essid: [u8; 33],
    passphrase: [u8; 129],
    security_type: u32,
    /// == C's `global_psk_cfg` (40 B = `A_SHA_DIGEST_LEN*2`); only the first
    /// `PMK_LEN` (32) bytes are the actual PMK fed into PTK derivation (④).
    global_psk: [u8; 40],
}

static PSK_STATE: SyncUnsafeCell<PskState> = SyncUnsafeCell(UnsafeCell::new(PskState {
    essid: [0u8; 33],
    passphrase: [0u8; 129],
    security_type: 0,
    global_psk: [0u8; 40],
}));

fn cstr_len(buf: &[u8]) -> usize {
    buf.iter().position(|&b| b == 0).unwrap_or(buf.len())
}

/// Handle `RTW_EVENT_WPA_SET_PSK_INFO`. `evt_info` points at NP's
/// `struct psk_info` (`crate::whc::msg::PskInfo`); 0 if NP sent none (should
/// not happen per the C source, but guarded defensively).
pub(crate) async fn handle_set_psk_info(evt_info: u32) {
    if evt_info != 0 {
        let info: PskInfo = unsafe {
            crate::cache::invalidate(evt_info as usize, core::mem::size_of::<PskInfo>());
            core::ptr::read_volatile(evt_info as *const PskInfo)
        };

        if is_wpa_psk_security(info.security_type) {
            let state = unsafe { &mut *PSK_STATE.get() };
            let unchanged = state.security_type == info.security_type
                && state.essid == info.psk_essid
                && state.passphrase == info.psk_passphrase;

            if !unchanged {
                if info.wpa_global_psk != [0u8; 40] {
                    // case 2: fast-reconnect — reuse NP-supplied PMK cache verbatim.
                    state.global_psk = info.wpa_global_psk;
                } else {
                    // case 3: derive a fresh PMK (PBKDF2-SHA1-4096).
                    let ssid_len = cstr_len(&info.psk_essid);
                    let pass_len = cstr_len(&info.psk_passphrase);
                    state.global_psk = [0u8; 40];
                    if pass_len <= 63 && ssid_len <= 32 {
                        pbkdf2_hmac::<Sha1>(
                            &info.psk_passphrase[..pass_len],
                            &info.psk_essid[..ssid_len],
                            4096,
                            &mut state.global_psk,
                        );
                    }
                    // 64-hex-char raw-PSK input bypass (C's `strtopsk`) is
                    // not implemented yet.
                }
                state.essid = info.psk_essid;
                state.passphrase = info.psk_passphrase;
                state.security_type = info.security_type;

                ameba_hal::println!(
                    "[psk] PMK derived sec=0x{:08x} fingerprint={:02x}{:02x}",
                    state.security_type,
                    state.global_psk[0],
                    state.global_psk[1],
                );
            }
        }
    }

    // Mirrors rtw_psk_set_psk_info_evt_hdl(): wifi_start_join_cmd() is called
    // unconditionally, regardless of which case above ran (or none).
    send_api_req(API_H2D_WIFI_START_JOIN_CMD, &[]).await;
}

/// The cached PMK (first 32 of the 40-byte `global_psk` cache), for
/// [`crate::wifi::handshake`]'s event-104 handler to seed `HandshakeState`
/// with — mirrors `memcpy(pInfo->PMK, wpa_lite[iface]->global_psk_cfg, PMK_LEN)`
/// in `rtw_psk_set_sta_info()`.
pub(crate) fn current_pmk() -> [u8; 32] {
    let state = unsafe { &*PSK_STATE.get() };
    let mut pmk = [0u8; 32];
    pmk.copy_from_slice(&state.global_psk[..32]);
    pmk
}
