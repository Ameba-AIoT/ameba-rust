#![no_std]

pub use rtl8721f_pac as pac;

#[macro_use]
pub mod macros;

pub mod interrupt;
pub mod mode;
pub use mode::{Async, Blocking, DriverMode};

pub mod cache;
pub mod debugtimer;
pub mod delay;
pub mod dma;
pub mod gpio;
pub mod ipc;
pub mod sys_timer;
pub mod tim;
mod time_driver;

pub mod adc;
pub mod aontimer;
#[cfg(feature = "unstable")]
pub mod audio_clock;
pub mod bor;
pub mod can;
pub mod captouch;
pub mod clk;
#[cfg(feature = "unstable")]
pub mod codec;
pub mod ethernet;
pub mod i2c;
pub mod ir;
#[cfg(feature = "unstable")]
pub mod lcdc;
pub mod ldo;
pub mod logger;
pub mod loguart;
pub mod mpu;
pub mod otp;
pub mod phy;
pub mod pinmux;
pub mod pll;
pub mod pmctimer;
#[cfg(feature = "unstable")]
pub mod ppe;
#[cfg(feature = "unstable")]
pub mod pseudo_i2c;
pub mod psram;
pub mod rcc;
pub mod reset;
pub mod rram;
pub mod rtc;
pub mod sd_host;
pub mod spi;
pub mod sport;
pub mod thermal;
pub mod trng;
pub mod uart;
#[cfg(feature = "unstable")]
pub mod uvc;
pub mod wakepin;
pub mod wdg;

/// Crate-private sealed trait, preventing external types from implementing
/// HAL-internal traits that should not be implementable outside this crate.
pub(crate) mod private {
    pub trait Sealed {}
}

/// Chip-level initialization configuration.
///
/// Pass to [`init()`] as the first call in `main`.  Use
/// `Config::default()` to enable all default drivers.  Individual fields
/// can be set to `false` to skip a driver (e.g. when you bring up your
/// own time source).
///
/// # Extension points
///
/// Add a new `pub` field here (default `true`) for each new peripheral
/// driver that needs one-time hardware setup.  Then add the matching
/// `if config.<field> { <driver>::init(); }` block in [`init()`].
///
/// Example — adding UART:
/// ```
/// pub uart0: bool,   // field in Config
/// // in init():
/// if config.uart0 { uart::uart0::init(); }
/// ```
pub struct Config {
    /// Start the Timer4 (1 MHz) embassy-time driver.
    /// Must be `true` when using `embassy-time` (`Timer::after`, timeouts, …).
    /// (The blocking-delay SysTick free-running counter always starts, in
    /// `chip_pre_init()`, regardless of this flag — `chip_pre_init()` itself
    /// depends on blocking `Delay` for oscillator/clock bring-up.)
    pub sys_timer: bool,
    /// Enable IPC_CPU0_IRQ in NVIC so inter-core messages from km4ns are received.
    /// Must be `true` when using `ameba-wifi` or any IPC-based cross-core protocol.
    pub ipc: bool,
}

impl Config {
    /// All drivers enabled.
    pub const fn default() -> Self {
        Self {
            sys_timer: true,
            ipc: true,
        }
    }
}

/// Initialize all enabled HAL drivers.
///
/// Call this as the very first line of `main` before starting the embassy
/// executor or touching any peripheral.
///
/// ```no_run
/// ameba_hal::init(ameba_hal::Config::default());
/// ```
pub fn init(config: Config) {
    chip_pre_init();
    if config.sys_timer {
        time_driver::init();
    }
    if config.ipc {
        ipc::init();
    }
}

fn chip_pre_init() {
    // Start the SysTick free-running counter first, unconditionally: blocking
    // `Delay` (delay.rs spin_cycles) polls SYST_CVR, and the oscillator/clock
    // bring-up below (rcc::enable, osc131k/osc4m calibration, xtal_aack, ...)
    // all use blocking `Delay`. Without this, SYST_CVR never counts down and
    // every spin_cycles() call loops forever.
    sys_timer::init();

    // Mirror app_start() in ameba_app_start.c (ram_km4tz):
    // cortex-m-rt Reset only zeroes Rust BSS (0x20006020+); NS ROM BSS
    // (0x200014b0–0x20002400) is not touched and retains ROM-boot residuals.
    // A garbage RBSS_UDELAY_CLK causes DelayUs to spin for billions of cycles.
    // Clear the whole NS ROM BSS region first, then set the divider, matching
    // what app_start() does before handing off to FreeRTOS.
    // SDK: ameba_rom_symbol_bcut.ld : RBSS_UDELAY_DIV  (start 0x200014b0)
    // SDK: ameba_layout.ld : KM4TZ_ROMBSS_RAM_NS      (end   0x20002400)
    unsafe {
        core::ptr::write_bytes(0x2000_14b0 as *mut u8, 0, 0x2000_2400 - 0x2000_14b0);
    }
    // RBSS_UDELAY_DIV is used by ROM DelayUs() as a loop-count divider.
    // km4ns bootloader normally sets it to 5 before launching image2, but
    // km4tz image2 runs in its own context and must set it independently.
    // Without this, RCC_PeriphClockCmd (which calls DelayUs internally) stalls
    // ~1.5 s per call because the divider is 0 → max loop iterations.
    // Type is u8: SDK declares `extern u8 RBSS_UDELAY_DIV;` (ameba_delay_rom.c).
    // A u32 write would clobber the 3 adjacent bytes (harmless only because the
    // region was just zeroed above and ARM is little-endian) — use u8 to match.
    const RBSS_UDELAY_DIV: *mut u8 = 0x2000_14b0 as *mut u8; // SDK: ameba_rom_symbol_bcut.ld : RBSS_UDELAY_DIV
    unsafe {
        core::ptr::write_volatile(RBSS_UDELAY_DIV, 5);
    }

    // Enable D-Cache (and I-Cache) on the AP core, matching the SDK's
    // Cache_Enable(ENABLE) call in ameba_app_start.c.
    //
    // With DCache enabled, DCCMVAC/DCIMVAC cache-maintenance instructions
    // (used by the IPC Flash lock protocol in ameba-storage) become effective.
    // Without DCache, those instructions are NOPs and the cross-core
    // Flash-sync protocol in hardware.rs cannot push/pull values to/from
    // physical SRAM reliably.
    //
    // Safety: BSS/data init has already run (cortex-m-rt Reset runs it before
    // calling main → ameba_hal::init).  All existing SRAM writes are clean
    // in physical SRAM.  Enabling DCache here causes cold cache misses on
    // first access, which re-load from that already-correct SRAM.
    cache::dcache_enable();
    cache::icache_enable();

    // Set XTAL_DRV_USB to suppress a 2440 MHz RF spur (Realtek hardware
    // erratum). Matches app_start()'s XTAL_INIT() call order: after
    // Cache_Enable(), before OSC4M_Init/pinmap_init/mpu_init.
    // SDK: ameba_app_start.c : app_start(), ameba_clk.c : XTAL_INIT()
    clk::xtal_init();

    // Configure MPU memory attributes (MAIR) and enable the MPU, then
    // protect the ROM null-pointer range read-only and mark the (currently
    // empty) linker nocache region non-cacheable. Matches the SDK's
    // mpu_init() + app_mpu_nocache_init() calls in app_start().
    // SDK: ameba_app_start.c : app_start()
    mpu::mpu_init();
    mpu::mpu_nocache_init();

    // Enable the SDM32K (32.768 kHz Sigma-Delta) oscillator.
    // Required before any basic timer (Timer0-3) can start; without it the
    // 32kHz clock domain never acknowledges cnt_start and the start loop hangs.
    // SDK: ameba_app_start.c : SDM32K_Enable(), ameba_clk.c : SDM32K_Enable
    clk::sdm32k_enable();

    // One-time cold-boot OSC131K trim, gated by the hardware-latched
    // `first_pon` bit so it only runs once per VBAT power-cycle. Without
    // this the AON 131.072 kHz oscillator (and everything downstream of
    // SDM32K/Timer0-3/RTC) can be off by several percent.
    // SDK: ameba_app_start.c : rtc_irq_init() (first_pon / OSC131K_Calibration
    // portion only — RTC_Enable/Init/SetTime/ClkSource_Select are done by the
    // RTC driver's own init sequence, not here).
    clk::osc131k_cold_boot_init();
}
