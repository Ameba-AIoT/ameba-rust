//! WHC control plane — IPC channel 7 (API).
//!
//! AP→NP (ch7 A2N): `send_api_req()` — fills `G_HOST_API_REQ` and polls.
//! NP→AP (ch7 N2A): `api_isr` wakes `run_blk_api()` which handles NP requests.

use core::cell::UnsafeCell;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};

use ameba_hal::ipc::{self, IpcMsg, MSG_TYPE_POINT, a2n, n2a};

use super::msg::*;
use crate::cache;

// Bounded: `api_isr` runs synchronously inside `IPC_CPU0_IRQ` (see
// `ameba_hal::ipc::Ipc::dispatch`) with no mutual exclusion against
// task-context LOGUART writers (e.g. `[ota-dbg]` in `ameba-ota`). If a
// task holds the TX FIFO in a state this ISR's poll never resolves, an
// unbounded spin here would stall every interrupt at this priority
// (Timer4 included) with no output at all.
macro_rules! dbg {
    ($s:literal) => {
        ameba_hal::print_bounded!($s)
    };
    ($fmt:literal, $($arg:expr_2021),+ $(,)?) => {
        ameba_hal::print_bounded!($fmt, $($arg),+)
    };
}
use crate::wifi::events::WifiEvent;

// ── Sync wrapper for UnsafeCell (single-core, no threads) ────────────────────

struct SyncUnsafeCell<T>(UnsafeCell<T>);
unsafe impl<T> Sync for SyncUnsafeCell<T> {}

impl<T> SyncUnsafeCell<T> {
    const fn new(val: T) -> Self {
        Self(UnsafeCell::new(val))
    }
    fn get(&self) -> *mut T {
        self.0.get()
    }
}

// ── AP→NP request buffer (one outstanding request at a time) ─────────────────

static G_HOST_API_REQ: SyncUnsafeCell<WhcIpcHostReqMsg> =
    SyncUnsafeCell::new(WhcIpcHostReqMsg::zeroed());

// api_id starts at API_PROCESS_DONE (0) → first call immediately proceeds.

/// Serialises concurrent callers of `send_api_req`.
static API_SEND_LOCK: Mutex<CriticalSectionRawMutex, ()> = Mutex::new(());

// ── NP→AP request dispatch ────────────────────────────────────────────────────

/// Stores NP's DevReqMsg pointer after the ch7 N2A ISR fires.
static API_WAKE: Signal<CriticalSectionRawMutex, u32> = Signal::new();

// ── STA MAC latched from NP ──────────────────────────────────────────────────

/// STA MAC as reported by NP through `SET_NETIF_INFO`; `None` until that event
/// arrives. Guarded by a critical section rather than an atomic because a MAC is
/// 6 bytes and this target has no 64-bit atomics — splitting it across two
/// atomics would leave a window where a reader sees half of one address.
/// Written only from `run_blk_api`, read by [`netif_mac`].
static NETIF_MAC: critical_section::Mutex<core::cell::Cell<Option<[u8; 6]>>> =
    critical_section::Mutex::new(core::cell::Cell::new(None));

/// Record the 6-byte MAC that `dev_addr` points at.
///
/// # Safety
/// `dev_addr` must be NP's `SET_NETIF_INFO` `param_buf[1]`: a non-null pointer
/// to `ETH_ALEN` (6) readable bytes in shared SRAM.
unsafe fn latch_netif_mac(dev_addr: u32) {
    unsafe {
        cache::invalidate(dev_addr as usize, 6);
        let mut mac = [0u8; 6];
        for (i, b) in mac.iter_mut().enumerate() {
            *b = core::ptr::read_volatile((dev_addr as *const u8).add(i));
        }
        dbg!(
            "[blk_api] netif MAC latched {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}\n",
            mac[0] as u32,
            mac[1] as u32,
            mac[2] as u32,
            mac[3] as u32,
            mac[4] as u32,
            mac[5] as u32
        );
        critical_section::with(|cs| NETIF_MAC.borrow(cs).set(Some(mac)));
    }
}

/// The STA MAC address NP pushed via `SET_NETIF_INFO`, or `None` if that event
/// has not arrived yet (it is sent during `wifi_on`).
///
/// Prefer this over [`crate::wifi::wifi_get_mac_address`]: it needs no round trip
/// to NP, and unlike that query it does not depend on the NP image having been
/// pruned against this firmware's symbol table.
pub fn netif_mac() -> Option<[u8; 6]> {
    critical_section::with(|cs| NETIF_MAC.borrow(cs).get())
}

// ── ch7 N2A ISR ───────────────────────────────────────────────────────────────

/// Registered with `ameba_hal::ipc::Ipc::register_handler(n2a::WIFI_API, ...)`.
///
/// # Safety
/// Must only be invoked by the IPC framework as the `n2a::WIFI_API` channel
/// handler (from IRQ context). The raw `_data`/`_chan` arguments are supplied
/// by that framework and are not dereferenced here; callers must not call this
/// directly.
pub unsafe fn api_isr(_data: *mut (), _ipc_status: u32, _chan: u8) {
    let ipc_msg = ipc::get_message(n2a::WIFI_API);
    dbg!("[api_isr] ch7 N2A fired msg=0x{:08x}\n", ipc_msg.msg);
    // ipc_msg.msg is a pointer to NP's WhcIpcDevReqMsg in shared SRAM.
    API_WAKE.signal(ipc_msg.msg);
}

/// Register both API IPC handlers. Call once during init.
pub fn register_api_handlers() {
    // Flush BSS-zeroed G_HOST_API_REQ to SRAM before use.
    // BSS init writes through D-cache (dirty); if we invalidate before clean, the
    // zero gets discarded and SRAM retains pre-reset garbage. Cleaning first
    // ensures SRAM holds api_id=0 so the first send_api_req poll sees 0 immediately.
    unsafe {
        cache::clean(G_HOST_API_REQ.get() as usize, 64);
        ipc::Ipc::ap().register_handler(n2a::WIFI_API, api_isr, core::ptr::null_mut());
    }
    dbg!("[api] handler registered ch7\n");
}

// ── AP→NP send ───────────────────────────────────────────────────────────────

/// Send an API request to NP and block until NP finishes processing.
///
/// - `api_id`: WHC API ID (e.g. `API_H2D_WIFI_ON`).
/// - `params`: up to 9 u32 parameters; extra slots are zero-padded.
///
/// Returns the `ret` field written by NP (0 = success).
///
/// # Crate-internal
/// Not public: callers need the parameter layout of the specific `api_id` to
/// match what NP's dispatcher expects, which is what the wrappers in this crate
/// encode. Wrap new APIs in `ameba-wifi` rather than reaching for this.
///
/// The `api_id` must be implemented by the prebuilt NP image. An unsupported id
/// compiles cleanly and then hangs the poll loop below on hardware with no
/// diagnostic; `cargo xtask build` rejects such a call by checking every
/// `api_id` against the committed `np_api_ids.txt`.
pub(crate) async fn send_api_req(api_id: u32, params: &[u32]) -> i32 {
    debug_assert!(params.len() <= 9);

    let _lock = API_SEND_LOCK.lock().await;

    dbg!("[api] send_api_req id=0x{:04x}\n", api_id);

    // ── Step 1: wait for NP to have finished the previous request ─────────
    let mut wait_cnt = 0u32;
    loop {
        unsafe {
            cache::invalidate(G_HOST_API_REQ.get() as usize, 64);
        }
        let cur = unsafe { core::ptr::read_volatile(&(*G_HOST_API_REQ.get()).api_id) };
        if cur == API_PROCESS_DONE {
            break;
        }
        wait_cnt += 1;
        if wait_cnt.is_multiple_of(500) {
            dbg!("[api] waiting prev req id=0x{:04x} cnt={}\n", cur, wait_cnt);
        }
        Timer::after(Duration::from_millis(1)).await;
    }

    // ── Step 2: fill request buffer ──────────────────────────────────────
    unsafe {
        let req = &mut *G_HOST_API_REQ.get();
        req.api_id = api_id;
        req.param_buf = [0u32; 9];
        for (i, &p) in params.iter().enumerate() {
            req.param_buf[i] = p;
        }
        req.ret = 0;
        cache::clean(G_HOST_API_REQ.get() as usize, 64);
    }

    // ── Step 3: send IPC A2N ch7 ─────────────────────────────────────────
    let ipc_msg = IpcMsg {
        msg_type: MSG_TYPE_POINT,
        msg: G_HOST_API_REQ.get() as u32,
        msg_len: 0,
        rsvd: 0,
    };
    ipc::send(a2n::WIFI_API, &ipc_msg);

    // ── Step 4: poll until NP clears api_id back to 0 ────────────────────
    let mut poll_cnt = 0u32;
    loop {
        Timer::after(Duration::from_millis(1)).await;
        unsafe {
            cache::invalidate(G_HOST_API_REQ.get() as usize, 64);
        }
        let cur = unsafe { core::ptr::read_volatile(&(*G_HOST_API_REQ.get()).api_id) };
        if cur == API_PROCESS_DONE {
            break;
        }
        poll_cnt += 1;
        if poll_cnt.is_multiple_of(500) {
            dbg!(
                "[api] polling NP for id=0x{:04x} cnt={}\n",
                api_id,
                poll_cnt
            );
        }
    }

    let ret = unsafe { core::ptr::read_volatile(&(*G_HOST_API_REQ.get()).ret) };
    dbg!("[api] send_api_req id=0x{:04x} done ret={}\n", api_id, ret);
    ret
}

// ── NP→AP blk_api task ───────────────────────────────────────────────────────

/// Process NP→AP blocking API requests.
///
/// Spawn this as an embassy task. It loops forever, waiting for the ch7 N2A
/// ISR to signal a new request, then handles it and ACKs NP.
pub async fn run_blk_api(
    event_tx: &embassy_sync::channel::Channel<CriticalSectionRawMutex, WifiEvent, 4>,
) {
    dbg!("[blk_api] task started\n");
    loop {
        let dev_req_ptr = API_WAKE.wait().await;

        if dev_req_ptr == 0 {
            continue;
        }

        let p_msg = dev_req_ptr as *mut WhcIpcDevReqMsg;

        unsafe {
            cache::invalidate(p_msg as usize, core::mem::size_of::<WhcIpcDevReqMsg>());
        }

        let enevt_id = unsafe { core::ptr::read_volatile(&(*p_msg).enevt_id) };
        let params = unsafe { core::ptr::read_volatile(&(*p_msg).param_buf) };

        dbg!(
            "[blk_api] enevt_id=0x{:04x} p[0]={} p[1]=0x{:08x}\n",
            enevt_id,
            params[0],
            params[1]
        );

        match enevt_id {
            API_D2H_GET_LWIP_INFO => {
                dbg!(
                    "[blk_api] GET_LWIP_INFO type={} out=0x{:08x}\n",
                    params[0],
                    params[1]
                );
                unsafe {
                    handle_get_lwip_info(p_msg, &params);
                }
            }
            API_D2H_IP_TABLE_CHK => {
                dbg!("[blk_api] IP_TABLE_CHK -> 0\n");
                // No DHCP server on this side; always return 0.
                unsafe {
                    core::ptr::write_volatile(&mut (*p_msg).ret, 0i32);
                }
            }
            API_D2H_WIFI_EVENT => {
                let event_code = params[0];
                let evt_info = params[1]; // pointer to extra event info (may be 0)
                dbg!(
                    "[blk_api] WIFI_EVENT code={} info=0x{:08x}\n",
                    event_code,
                    evt_info
                );

                if event_code == crate::wifi::events::event_code::WPA_SET_PSK_INFO {
                    // NP allocated the `psk_info` buffer `evt_info` points at
                    // and frees it right after this message is ACKed (see
                    // `handle_set_psk_info`'s doc comment) — it must be read
                    // and fully processed (PBKDF2 + WHC_API_WIFI_START_JOIN_CMD)
                    // before we fall through to the ACK write below, so this
                    // runs here rather than going through `event_tx`.
                    crate::wifi::psk::handle_set_psk_info(evt_info).await;
                } else if event_code == crate::wifi::events::event_code::WPA_STA_4WAY_START {
                    // Same buffer-lifetime constraint as WPA_SET_PSK_INFO above:
                    // NP frees `evt_info` right after this message's ACK.
                    crate::wifi::handshake::handle_4way_start(evt_info, true).await;
                } else if event_code == crate::wifi::events::event_code::WPA_STA_4WAY_RECV {
                    crate::wifi::handshake::handle_4way_recv(evt_info).await;
                } else {
                    // Non-blocking on purpose — mirrors the C SDK's decoupled
                    // design (whc_ipc_host_api.c's whc_ipc_host_blk_api_task_h
                    // only enqueues via rtos_mem_zmalloc + a semaphore-signalled
                    // list and returns immediately; a separate
                    // whc_ipc_host_unblk_api_task_h task does the actual
                    // dispatch/consumption). This task plays the role of that
                    // first task: it must never block waiting on whether the
                    // app has drained a slot, or a stalled/absent consumer
                    // stalls this ACK, and NP times out retrying forever
                    // ("Dev api ipc timeout"). If the channel is full we drop
                    // the event — the app is expected to keep draining
                    // `EVENT_CHAN` for the life of the program; a dropped
                    // status event is recoverable (NP fires JOIN_STATUS again
                    // on the next transition), a wedged ACK path is not.
                    if event_tx
                        .try_send(WifiEvent {
                            event_code,
                            evt_info,
                        })
                        .is_err()
                    {
                        dbg!(
                            "[blk_api] EVENT_CHAN full, dropping event code={}\n",
                            event_code
                        );
                    }
                }
            }
            API_D2H_SET_NETIF_INFO => {
                dbg!("[blk_api] SET_NETIF_INFO idx={}\n", params[0]);
                // param_buf = [wlan_idx, dev_addr], where dev_addr points at
                // ETH_ALEN bytes of NP's real MAC for that interface — see
                // whc_ipc_dev_set_netif_info() in whc_ipc_dev_api.c, which
                // DCache_Clean()s the 6 bytes before sending, and the C host's
                // handler in whc_ipc_host_api.c:383 which copies ETH_ALEN from
                // param_buf[1]. Latch it so `netif_mac()` can hand the driver a
                // real address without asking NP for one.
                if params[0] == 0 && params[1] != 0 {
                    unsafe {
                        latch_netif_mac(params[1]);
                    }
                }
            }
            _ => {
                dbg!("[blk_api] unknown enevt_id=0x{:04x}\n", enevt_id);
            }
        }

        // ACK NP: clear enevt_id to 0 so NP knows we are done.
        unsafe {
            core::ptr::write_volatile(&mut (*p_msg).enevt_id, 0u32);
            cache::clean(p_msg as usize, core::mem::size_of::<WhcIpcDevReqMsg>());
        }
    }
}

// ── GET_LWIP_INFO handler ─────────────────────────────────────────────────────

/// Respond to NP's query for IP/GW/mask/MAC.
///
/// C SDK protocol (whc_ipc_host_api_lwip_info_handler):
///   param_buf[0] = query type, param_buf[1] = input (unused), param_buf[2] = wlan_idx
///   ret = pointer to the data in AP memory; NP DCache_Invalidates then reads via that pointer.
///   AP must DCache_Clean the data before writing ret so NP sees it after invalidate.
///
/// We return a pointer into the static IP_CONFIG, which is filled by run_dhcp().
/// Before DHCP completes IP_CONFIG is all zeros — NP reads 0.0.0.0 which is acceptable
/// during the association phase (NP only caches it for its own route table).
unsafe fn handle_get_lwip_info(p_msg: *mut WhcIpcDevReqMsg, params: &[u32; 8]) {
    unsafe {
        let query_type = params[0];
        // params[1] = input (unused for GET_IP/GW/GWMSK/HW_ADDR)
        // params[2] = wlan_idx (ignored; we only have one interface)

        match query_type {
            WHC_WLAN_GET_IP => {
                let ptr = core::ptr::addr_of!(crate::net::IP_CONFIG.ip) as u32;
                cache::clean(ptr as usize, 4);
                core::ptr::write_volatile(&mut (*p_msg).ret, ptr as i32);
            }
            WHC_WLAN_GET_GW => {
                let ptr = core::ptr::addr_of!(crate::net::IP_CONFIG.gateway) as u32;
                cache::clean(ptr as usize, 4);
                core::ptr::write_volatile(&mut (*p_msg).ret, ptr as i32);
            }
            WHC_WLAN_GET_GWMSK => {
                let ptr = core::ptr::addr_of!(crate::net::IP_CONFIG.netmask) as u32;
                cache::clean(ptr as usize, 4);
                core::ptr::write_volatile(&mut (*p_msg).ret, ptr as i32);
            }
            WHC_WLAN_GET_HW_ADDR => {
                let ptr = core::ptr::addr_of!(crate::net::IP_CONFIG.mac) as u32;
                cache::clean(ptr as usize, 6);
                core::ptr::write_volatile(&mut (*p_msg).ret, ptr as i32);
            }
            _ => {
                core::ptr::write_volatile(&mut (*p_msg).ret, 0i32);
            }
        }
    }
}
