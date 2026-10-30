//! WPA2-PSK 4-way handshake — steps ③-⑦ (STA side only).
//!
//! Driven by `RTW_EVENT_WPA_STA_4WAY_START` (event 104) and
//! `RTW_EVENT_WPA_STA_4WAY_RECV` (event 106), dispatched synchronously from
//! `whc::api::run_blk_api` before that event's ACK is sent to NP (same
//! constraint as `crate::wifi::psk::handle_set_psk_info`: NP frees/reuses the
//! `evt_info` buffer right after the ACK, so it must be fully consumed here).
//!
//! Mirrors the C SDK's `rtw_psk_sta_start_4way` / `rtw_psk_sta_recv_eapol` /
//! `rtw_psk_sta_send_eapol` (`wpa_lite_psk.c`). PMK comes from
//! [`crate::wifi::psk::current_pmk`] (already derived by event 108). GTK is
//! never locally derived — it arrives pre-computed from the AP, AES-Key-Wrap
//! encrypted inside M3's Key Data field.

use core::cell::UnsafeCell;

use aes_kw::{KwAes128, cipher::KeyInit};
use hmac::{Hmac, Mac};
use sha1::Sha1;

use crate::whc::api::send_api_req;
use crate::whc::msg::*;
use crate::wifi::events::{event_code, join_status};

struct SyncUnsafeCell<T>(UnsafeCell<T>);
unsafe impl<T> Sync for SyncUnsafeCell<T> {}
impl<T> SyncUnsafeCell<T> {
    const fn new(v: T) -> Self {
        Self(UnsafeCell::new(v))
    }
    fn get(&self) -> *mut T {
        self.0.get()
    }
}

#[repr(align(64))]
struct Aligned64<const N: usize>([u8; N]);

// ── EAPOL-Key wire layout (802.1X hdr(4) + _LIB1X_EAPOL_KEY hdr(95)) ─────────
//
// Absolute offsets from the start of the buffer (offset 0 = 802.1X
// protocol_version byte). `*Pos` constants from rom_wpa_lite_psk.h are
// relative to the key-descriptor body, which starts right after the 4-byte
// 802.1X header — so ABS = 4 + <name>Pos.
const ABS_DESC_TYPE: usize = 4;
const ABS_KEY_INFO: usize = 5;
const ABS_REPLAY_CNT: usize = 9;
const ABS_KEY_NONCE: usize = 17;
const ABS_KEY_MIC: usize = 81;
const ABS_KEY_DATA_LEN: usize = 97;
const ABS_KEY_DATA: usize = 99;
const EAPOL_HDR_LEN: usize = ABS_KEY_DATA; // 99
const MAX_EAPOLMSG_LEN: usize = 512;

const DESC_TYPE_RSN: u8 = 2;
const EAPOL_KEY_PKT_TYPE: u8 = 3;

// NP's `rtw_event_report_frame`/`wifi_if_send_eapol` buffers are full Ethernet
// frames (dst(6)+src(6)+ethertype(2)), not bare 802.1X frames — see
// `wpa_lite_psk.c`'s `eapol = EAPOLMsgRecvd.Octet + ETH_HLEN` (RX) and
// `wifi_rom_fill_eapol_header` writing `eth_hdr` before the 802.1X header (TX).
// All `ABS_*` offsets above are relative to the 802.1X header, i.e. relative to
// `frame[ETH_HLEN..]` on RX and `buf[ETH_HLEN..]` on TX.
const ETH_HLEN: usize = 14;
const ETHERTYPE_EAPOL: u16 = 0x888E;

const KEYINFO_KEYTYPE_PAIRWISE: u16 = 1 << 3;
const KEYINFO_ACK: u16 = 1 << 7;
const KEYINFO_MIC: u16 = 1 << 8;
const KEYINFO_SECURE: u16 = 1 << 9;
const KEYINFO_ENCDATA: u16 = 1 << 12;
// Key Descriptor Version 2 = HMAC-SHA1-128 + AES key-wrap, used with CCMP.
const KEYINFO_DESC_VER_HMAC_SHA1: u16 = 2;

const M2_KEY_INFO: u16 = KEYINFO_DESC_VER_HMAC_SHA1 | KEYINFO_KEYTYPE_PAIRWISE | KEYINFO_MIC;
const M4_KEY_INFO: u16 =
    KEYINFO_DESC_VER_HMAC_SHA1 | KEYINFO_KEYTYPE_PAIRWISE | KEYINFO_MIC | KEYINFO_SECURE;

fn frame_key_info(f: &[u8]) -> u16 {
    u16::from_be_bytes([f[ABS_KEY_INFO], f[ABS_KEY_INFO + 1]])
}
fn frame_replay_counter(f: &[u8]) -> [u8; 8] {
    f[ABS_REPLAY_CNT..ABS_REPLAY_CNT + 8].try_into().unwrap()
}
fn frame_key_nonce(f: &[u8]) -> [u8; 32] {
    f[ABS_KEY_NONCE..ABS_KEY_NONCE + 32].try_into().unwrap()
}
fn frame_key_mic(f: &[u8]) -> [u8; 16] {
    f[ABS_KEY_MIC..ABS_KEY_MIC + 16].try_into().unwrap()
}
fn frame_key_data_len(f: &[u8]) -> usize {
    u16::from_be_bytes([f[ABS_KEY_DATA_LEN], f[ABS_KEY_DATA_LEN + 1]]) as usize
}

// ── PRF-384 (PRF-SHA1) — IEEE 802.11-2016 §12.7.1.2 ─────────────────────────

const PMK_EXPANSION_CONST: &[u8] = b"Pairwise key expansion"; // 22 bytes

fn prf_sha1(key: &[u8], label: &[u8], data: &[u8], out: &mut [u8]) {
    let mut counter: u8 = 0;
    let mut off = 0;
    while off < out.len() {
        let mut mac = Hmac::<Sha1>::new_from_slice(key).unwrap();
        mac.update(label);
        mac.update(&[0u8]);
        mac.update(data);
        mac.update(core::slice::from_ref(&counter));
        let digest = mac.finalize().into_bytes();
        let n = core::cmp::min(20, out.len() - off);
        out[off..off + n].copy_from_slice(&digest[..n]);
        off += n;
        counter += 1;
    }
}

/// PTK = PRF-384(PMK, "Pairwise key expansion",
///              Min(AA,SPA) || Max(AA,SPA) || Min(ANonce,SNonce) || Max(ANonce,SNonce))
/// Output: `ptk[0..16]`=KCK, `ptk[16..32]`=KEK, `ptk[32..48]`=TK (CCMP-128).
fn calc_ptk(
    pmk: &[u8; 32],
    aa: &[u8; 6],
    spa: &[u8; 6],
    anonce: &[u8; 32],
    snonce: &[u8; 32],
) -> [u8; 48] {
    let (mac_lo, mac_hi) = if aa.as_slice() <= spa.as_slice() {
        (aa, spa)
    } else {
        (spa, aa)
    };
    let (non_lo, non_hi) = if anonce.as_slice() <= snonce.as_slice() {
        (anonce, snonce)
    } else {
        (snonce, anonce)
    };

    let mut data = [0u8; 76];
    data[0..6].copy_from_slice(mac_lo);
    data[6..12].copy_from_slice(mac_hi);
    data[12..44].copy_from_slice(non_lo);
    data[44..76].copy_from_slice(non_hi);

    let mut ptk = [0u8; 48];
    prf_sha1(pmk, PMK_EXPANSION_CONST, &data, &mut ptk);
    ptk
}

/// MIC (Key Descriptor Version 2) = HMAC-SHA1(KCK, frame)[0..16], with the
/// frame's own MIC field zeroed before hashing.
fn calc_mic(kck: &[u8], frame: &[u8]) -> [u8; 16] {
    let mut mac = Hmac::<Sha1>::new_from_slice(kck).unwrap();
    mac.update(frame);
    let digest = mac.finalize().into_bytes();
    let mut mic = [0u8; 16];
    mic.copy_from_slice(&digest[..16]);
    mic
}

// ── GTK KDE parsing (RSN Key Data, after AES-KW unwrap) ─────────────────────
//
// KDE: type(1)=0xDD, length(1), OUI(3)=00:0F:AC, data_type(1)=1 (GTK KDE),
// value = KeyID/Tx byte(1) + reserved(1) + GTK(N). Returns (gtk_slice, key_id).
fn parse_gtk_kde(kd: &[u8]) -> Option<(&[u8], u8)> {
    let mut i = 0;
    while i + 2 <= kd.len() {
        let elem_id = kd[i];
        let len = kd[i + 1] as usize;
        if i + 2 + len > kd.len() {
            break;
        }
        if elem_id == 0xDD && len >= 6 && kd[i + 2..i + 5] == [0x00, 0x0F, 0xAC] && kd[i + 5] == 1 {
            let key_id = kd[i + 6] & 0x03;
            return Some((&kd[i + 8..i + 2 + len], key_id));
        }
        i += 2 + len;
    }
    None
}

// ── Handshake state (singleton — one STA interface, one session at a time) ──

struct HandshakeState {
    active: bool,
    ap_mac: [u8; 6],
    self_mac: [u8; 6],
    anonce: [u8; 32],
    snonce: [u8; 32],
    ptk: [u8; 48],
    replay_counter: [u8; 8],
    eapol_version: u8,
    auth_info: [u8; 128],
    auth_info_len: u8,
    triggered_by_join: bool,
}

static HANDSHAKE_STATE: SyncUnsafeCell<HandshakeState> = SyncUnsafeCell::new(HandshakeState {
    active: false,
    ap_mac: [0; 6],
    self_mac: [0; 6],
    anonce: [0; 32],
    snonce: [0; 32],
    ptk: [0; 48],
    replay_counter: [0; 8],
    eapol_version: 1,
    auth_info: [0; 128],
    auth_info_len: 0,
    triggered_by_join: true,
});

static TX_FRAME: SyncUnsafeCell<Aligned64<MAX_EAPOLMSG_LEN>> =
    SyncUnsafeCell::new(Aligned64([0u8; MAX_EAPOLMSG_LEN]));

static CRYPT_INFO: SyncUnsafeCell<RtwCryptInfo> = SyncUnsafeCell::new(RtwCryptInfo {
    pairwise: 0,
    mac_addr: [0; 6],
    wlan_idx: 0,
    key_len: 0,
    key: [0; 32],
    key_idx: 0,
    driver_cipher: 0,
    transition_disable_exist: 0,
    transition_disable_bitmap: 0,
    device_id_flags: 0,
});

#[repr(C, align(8))]
struct JoinStatusInfo {
    _priv: [u8; 8],
    status: u8,
}

static JS_HANDSHAKING: SyncUnsafeCell<JoinStatusInfo> = SyncUnsafeCell::new(JoinStatusInfo {
    _priv: [0; 8],
    status: join_status::HANDSHAKING,
});
static JS_HANDSHAKE_DONE: SyncUnsafeCell<JoinStatusInfo> = SyncUnsafeCell::new(JoinStatusInfo {
    _priv: [0; 8],
    status: join_status::HANDSHAKE_DONE,
});
static JS_SUCCESS: SyncUnsafeCell<JoinStatusInfo> = SyncUnsafeCell::new(JoinStatusInfo {
    _priv: [0; 8],
    status: join_status::SUCCESS,
});

// Non-blocking — same reasoning as `whc::api::run_blk_api`'s WIFI_EVENT
// forwarding (see that function's comment): this runs synchronously inside
// run_blk_api's dispatch, before NP's ACK for the enclosing message is
// written. A blocking send here risks the exact deadlock that pattern was
// meant to avoid — if EVENT_CHAN is already full (e.g. a burst of JOIN_STATUS
// transitions during reconnect outrunning the app's consumer), NP's ACK never
// arrives, run_blk_api never returns to `API_WAKE.wait()`, and every
// subsequent NP->AP message times out too.
fn push_join_status(buf: &SyncUnsafeCell<JoinStatusInfo>) {
    let ptr = buf.get() as u32;
    unsafe {
        crate::cache::clean(ptr as usize, core::mem::size_of::<JoinStatusInfo>());
    }
    if crate::EVENT_CHAN
        .try_send(crate::wifi::events::WifiEvent {
            event_code: event_code::JOIN_STATUS,
            evt_info: ptr,
        })
        .is_err()
    {
        ameba_hal::println!("[handshake] EVENT_CHAN full, dropping join_status push\n");
    }
}

fn gen_nonce() -> [u8; 32] {
    let trng = unsafe { ameba_hal::trng::Trng::steal() };
    trng.init();
    let mut out = [0u8; 32];
    for chunk in out.chunks_mut(4) {
        let v = trng.read_u32().unwrap_or(0xDEAD_BEEF);
        chunk.copy_from_slice(&v.to_le_bytes());
    }
    out
}

// ── Event 104: RTW_EVENT_WPA_STA_4WAY_START ─────────────────────────────────

/// Seed a fresh handshake session from NP's per-session `WpaParam` and push
/// `join_status::HANDSHAKING` so app code sees the same status sequence the
/// C SDK's `rtw_psk_sta_start_4way` produces.
pub(crate) async fn handle_4way_start(evt_info: u32, triggered_by_join: bool) {
    if evt_info == 0 {
        return;
    }

    let param: WpaParam = unsafe {
        crate::cache::invalidate(evt_info as usize, core::mem::size_of::<WpaParam>());
        core::ptr::read_volatile(evt_info as *const WpaParam)
    };

    let snonce = gen_nonce();
    unsafe {
        let s = &mut *HANDSHAKE_STATE.get();
        s.active = true;
        s.ap_mac = param.peer_mac;
        s.self_mac = param.self_mac;
        s.snonce = snonce;
        s.anonce = [0; 32];
        s.ptk = [0; 48];
        s.replay_counter = [0xff; 8];
        s.eapol_version = 1;
        s.triggered_by_join = triggered_by_join;

        // auth_info_buf is TLV-prefixed (id, len, value..); copy exactly the
        // advertised IE so M2's Key Data echoes it back byte-for-byte.
        let ie_len = param.auth_info_buf[1] as usize;
        let total = core::cmp::min(2 + ie_len, 128);
        s.auth_info[..total].copy_from_slice(&param.auth_info_buf[..total]);
        s.auth_info_len = total as u8;
    }

    push_join_status(&JS_HANDSHAKING);
}

// ── Frame construction / IPC send / key install ─────────────────────────────

/// Build an EAPOL-Key frame (M2 or M4) into the static `TX_FRAME` buffer,
/// compute its MIC, and return the frame's total length.
fn build_frame(
    ap_mac: &[u8; 6],
    self_mac: &[u8; 6],
    key_info: u16,
    replay_counter: &[u8; 8],
    key_nonce: &[u8; 32],
    key_data: &[u8],
    kck: &[u8; 16],
) -> usize {
    let total_len = ETH_HLEN + EAPOL_HDR_LEN + key_data.len();
    let buf = unsafe { &mut (*TX_FRAME.get()).0 };
    buf[..total_len].fill(0);

    // Ethernet header: dst=AP, src=self, ethertype=EAPOL.
    buf[0..6].copy_from_slice(ap_mac);
    buf[6..12].copy_from_slice(self_mac);
    buf[12..14].copy_from_slice(&ETHERTYPE_EAPOL.to_be_bytes());

    let f = &mut buf[ETH_HLEN..total_len];
    let body_len = (EAPOL_HDR_LEN - 4) + key_data.len();
    f[0] = 1; // 802.1X ProtocolVersion
    f[1] = EAPOL_KEY_PKT_TYPE;
    f[2..4].copy_from_slice(&(body_len as u16).to_be_bytes());

    f[ABS_DESC_TYPE] = DESC_TYPE_RSN;
    f[ABS_KEY_INFO..ABS_KEY_INFO + 2].copy_from_slice(&key_info.to_be_bytes());
    // Key Length stays 0: only messages 1/3 (AP-originated) set it.
    f[ABS_REPLAY_CNT..ABS_REPLAY_CNT + 8].copy_from_slice(replay_counter);
    f[ABS_KEY_NONCE..ABS_KEY_NONCE + 32].copy_from_slice(key_nonce);
    // Key IV / Key RSC / Key ID: left zeroed for M2/M4.
    f[ABS_KEY_DATA_LEN..ABS_KEY_DATA_LEN + 2]
        .copy_from_slice(&(key_data.len() as u16).to_be_bytes());
    f[ABS_KEY_DATA..EAPOL_HDR_LEN + key_data.len()].copy_from_slice(key_data);

    // MIC (Key Descriptor Version 2) is computed over the 802.1X frame only,
    // not the Ethernet header — mirrors `wifi_rom_calc_mic`'s
    // `eapol = EAPOLMsgSend.Octet + ETH_HLEN` / `data_len = Length - ETH_HLEN`.
    let mic = calc_mic(kck, &buf[ETH_HLEN..total_len]);
    buf[ETH_HLEN + ABS_KEY_MIC..ETH_HLEN + ABS_KEY_MIC + 16].copy_from_slice(&mic);

    total_len
}

/// Clean the cache over `TX_FRAME[..len]` and dispatch it to NP via
/// `WHC_API_WIFI_SEND_EAPOL` (wlan_idx=0/STA, flags=0 — mirrors
/// `whc_ipc_dev_api.c`'s `WHC_API_WIFI_SEND_EAPOL` case).
async fn send_frame(len: usize) -> i32 {
    let ptr = TX_FRAME.get() as u32;
    unsafe {
        crate::cache::clean(ptr as usize, len);
    }
    send_api_req(API_H2D_WIFI_SEND_EAPOL, &[0, ptr, len as u32, 0]).await
}

/// Install a pairwise (TK) or group (GTK) key via `WHC_API_WIFI_ADD_KEY`.
async fn install_key(pairwise: bool, mac: [u8; 6], key: &[u8], key_idx: u8) -> i32 {
    let ptr = CRYPT_INFO.get() as u32;
    unsafe {
        let mut k = [0u8; 32];
        k[..key.len()].copy_from_slice(key);
        *CRYPT_INFO.get() = RtwCryptInfo {
            pairwise: pairwise as u8,
            mac_addr: mac,
            wlan_idx: 0,
            key_len: key.len() as u16,
            key: k,
            key_idx,
            driver_cipher: DRIVER_CIPHER_AES,
            transition_disable_exist: 0,
            transition_disable_bitmap: 0,
            device_id_flags: 0,
        };
        crate::cache::clean(ptr as usize, core::mem::size_of::<RtwCryptInfo>());
    }
    send_api_req(API_H2D_WIFI_ADD_KEY, &[ptr]).await
}

// ── Event 106: RTW_EVENT_WPA_STA_4WAY_RECV ──────────────────────────────────

/// Handle one received EAPOL-Key frame: M1 -> derive PTK, send M2. M3 ->
/// verify MIC, unwrap GTK, install both keys, send M4, report completion.
/// `evt_info` points at NP's `rtw_event_report_frame` (`frame_len: u32` at
/// offset 0, then a full Ethernet frame at offset 4 — dst(6)+src(6)+
/// ethertype(2) followed by the 802.1X/EAPOL-Key body — see the `ETH_HLEN`
/// note above).
pub(crate) async fn handle_4way_recv(evt_info: u32) {
    if evt_info == 0 {
        return;
    }

    let frame_len = unsafe {
        crate::cache::invalidate(evt_info as usize, 4);
        core::ptr::read_volatile(evt_info as *const u32) as usize
    };
    if !((ETH_HLEN + EAPOL_HDR_LEN)..=MAX_EAPOLMSG_LEN).contains(&frame_len) {
        return;
    }

    let mut eth_frame = [0u8; MAX_EAPOLMSG_LEN];
    unsafe {
        crate::cache::invalidate(evt_info as usize + 4, frame_len);
        core::ptr::copy_nonoverlapping(
            (evt_info + 4) as *const u8,
            eth_frame.as_mut_ptr(),
            frame_len,
        );
    }

    let frame_len = frame_len - ETH_HLEN;
    let frame = &mut eth_frame[ETH_HLEN..ETH_HLEN + frame_len];

    if frame[ABS_DESC_TYPE] != DESC_TYPE_RSN {
        return;
    }
    let key_info = frame_key_info(&frame[..frame_len]);

    if unsafe { !(*HANDSHAKE_STATE.get()).active } {
        return; // no session in progress (stray/duplicate frame) — ignore
    }

    if key_info & KEYINFO_MIC == 0 {
        if key_info & KEYINFO_ACK == 0 {
            return; // neither M1 nor a MIC-bearing message — ignore
        }

        // ── M1: derive PTK, reply with M2 ──
        let anonce = frame_key_nonce(&frame[..frame_len]);
        let replay_counter = frame_replay_counter(&frame[..frame_len]);
        let pmk = crate::wifi::psk::current_pmk();

        let (snonce, ap_mac, self_mac) = unsafe {
            let s = &mut *HANDSHAKE_STATE.get();
            s.anonce = anonce;
            s.replay_counter = replay_counter;
            s.ptk = calc_ptk(&pmk, &s.ap_mac, &s.self_mac, &s.anonce, &s.snonce);
            (s.snonce, s.ap_mac, s.self_mac)
        };
        let kck: [u8; 16] = unsafe { (&(*HANDSHAKE_STATE.get()).ptk)[0..16].try_into().unwrap() };
        let (auth_info, auth_info_len) = unsafe {
            let s = &*HANDSHAKE_STATE.get();
            (s.auth_info, s.auth_info_len as usize)
        };

        let len = build_frame(
            &ap_mac,
            &self_mac,
            M2_KEY_INFO,
            &replay_counter,
            &snonce,
            &auth_info[..auth_info_len],
            &kck,
        );
        send_frame(len).await;
        return;
    }

    // MIC bit set: verify against the PTK's KCK before trusting anything else
    // in the frame (mirrors `wifi_rom_check_mic`).
    let kck: [u8; 16] = unsafe { (&(*HANDSHAKE_STATE.get()).ptk)[0..16].try_into().unwrap() };
    let orig_mic = frame_key_mic(&frame[..frame_len]);
    frame[ABS_KEY_MIC..ABS_KEY_MIC + 16].fill(0);
    if calc_mic(&kck, &frame[..frame_len]) != orig_mic {
        ameba_hal::println!("[handshake] MIC verify failed, dropping frame\n");
        return;
    }

    if key_info & KEYINFO_SECURE == 0 {
        return; // MIC-bearing but not Secure -> not M3, nothing else to do here
    }

    // ── M3: unwrap GTK, install keys, reply with M4 ──
    let replay_counter = frame_replay_counter(&frame[..frame_len]);
    let kek: [u8; 16] = unsafe { (&(*HANDSHAKE_STATE.get()).ptk)[16..32].try_into().unwrap() };
    let tk: [u8; 16] = unsafe { (&(*HANDSHAKE_STATE.get()).ptk)[32..48].try_into().unwrap() };
    let ap_mac = unsafe { (*HANDSHAKE_STATE.get()).ap_mac };
    let self_mac = unsafe { (*HANDSHAKE_STATE.get()).self_mac };
    let triggered_by_join = unsafe { (*HANDSHAKE_STATE.get()).triggered_by_join };

    let mut gtk = [0u8; 32];
    let mut gtk_len = 0usize;
    let mut gtk_key_id = 0u8;
    let kd_len = frame_key_data_len(&frame[..frame_len]);
    if key_info & KEYINFO_ENCDATA != 0 && (16..=64).contains(&kd_len) && kd_len.is_multiple_of(8) {
        let kd = &frame[ABS_KEY_DATA..ABS_KEY_DATA + kd_len];
        if let Ok(kw) = KwAes128::new_from_slice(&kek) {
            let mut unwrap_buf = [0u8; 64];
            match kw.unwrap_key(kd, &mut unwrap_buf[..kd_len - 8]) {
                Ok(unwrapped) => {
                    if let Some((g, key_id)) = parse_gtk_kde(unwrapped) {
                        gtk_len = core::cmp::min(g.len(), 32);
                        gtk[..gtk_len].copy_from_slice(&g[..gtk_len]);
                        gtk_key_id = key_id;
                    }
                }
                Err(_) => ameba_hal::println!("[handshake] GTK unwrap failed\n"),
            }
        }
    }

    install_key(true, ap_mac, &tk, 0).await;
    if gtk_len > 0 {
        install_key(false, self_mac, &gtk[..gtk_len], gtk_key_id).await;
    }

    let len = build_frame(
        &ap_mac,
        &self_mac,
        M4_KEY_INFO,
        &replay_counter,
        &[0u8; 32],
        &[],
        &kck,
    );
    send_frame(len).await;

    unsafe {
        (*HANDSHAKE_STATE.get()).active = false;
    }

    push_join_status(&JS_HANDSHAKE_DONE);
    if triggered_by_join {
        push_join_status(&JS_SUCCESS);
    }
}
