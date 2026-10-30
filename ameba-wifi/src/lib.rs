//! `ameba-wifi` — WHC-based WiFi + smoltcp stack for RTL8721F.
//!
//! # Quick-start
//!
//! ```no_run
//! // 1. Initialise IPC handlers (call once before spawning tasks).
//! ameba_wifi::init();
//!
//! // 2. Set static IP config so NP can get routing info.
//! ameba_wifi::net::IpConfig::set(
//!     [192,168,1,100], [192,168,1,1], [255,255,255,0],
//!     [0xAA,0xBB,0xCC,0xDD,0xEE,0xFF],
//! );
//!
//! // 3. In your embassy entry, spawn the background task:
//! //    spawner.spawn(wifi_task(event_channel.sender())).unwrap();
//!
//! // 4. Turn on WiFi and connect.
//! ameba_wifi::wifi::wifi_on().await;
//! ameba_wifi::wifi::wifi_connect("MySSID", "password", SecurityType::Wpa2AesPsk).await;
//!
//! // 5. Build a smoltcp interface using WhcDevice::new().
//! ```

#![no_std]

pub mod cache;
pub mod net;
pub mod whc;
pub mod wifi;

pub use wifi::SecurityType;
pub use wifi::events::WifiEvent;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;

/// Global WiFi event channel (4 slots).  The background task populates this;
/// application code calls `EVENT_CHAN.receive().await` to observe WiFi events.
pub static EVENT_CHAN: Channel<CriticalSectionRawMutex, WifiEvent, 4> = Channel::new();

/// Initialise the WHC IPC handlers.
///
/// Call once, before spawning any embassy tasks that use ameba-wifi.
/// `ameba_hal::ipc::init()` must have been called before this.
pub fn init() {
    whc::trx::register_trx_handler();
    whc::api::register_api_handlers();
}

/// Background task that processes NP→AP blocking API requests.
///
/// Spawn this as an embassy task; it never returns.
///
/// ```no_run
/// #[embassy_executor::task]
/// async fn wifi_task() {
///     ameba_wifi::run_blk_api().await;
/// }
/// ```
pub async fn run_blk_api() -> ! {
    whc::api::run_blk_api(&EVENT_CHAN).await;
    unreachable!()
}
