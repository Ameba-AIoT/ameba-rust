use core::future::Future;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use core::task::{Context, Poll};
use embassy_sync::waitqueue::AtomicWaker;
use rtl8721f_pac::gpio_reg;

// GPIO ports A/B/C. GPIO is a shared, NS-accessible peripheral, so we use the
// Non-Secure base per the HAL-wide S/NS convention (see `crate::TZ_IDAU_SEC_OFFSET`
// doc in lib.rs): every base is the PAC peripheral's own `::PTR` (NS, from the
// SVD) — no Secure offset, no hand-written address.
// The PAC exposes a single GPIO_REG peripheral (= port A); ports B/C are the
// same block at +0x400 / +0x800.
// SDK: hal_platform.h:476-478 : GPIOB_BASE = GPIO_REG_BASE + 0x400, GPIOC_BASE = +0x800
const GPIO_PORT_STRIDE: usize = 0x400;
const GPIOA_PTR: *const gpio_reg::RegisterBlock = rtl8721f_pac::GpioReg::PTR;
const GPIOB_PTR: *const gpio_reg::RegisterBlock =
    rtl8721f_pac::GpioReg::PTR.wrapping_byte_add(GPIO_PORT_STRIDE);
const GPIOC_PTR: *const gpio_reg::RegisterBlock =
    rtl8721f_pac::GpioReg::PTR.wrapping_byte_add(2 * GPIO_PORT_STRIDE);

// ────────────────────────────────────────────────────────────────────────────
// Private: PAC-based hardware access (replaces ROM FFI GPIO_*/Pinmux_*/PAD_*)
//
// Fully migrated to rtl8721f-pac register access (GPIO-PAC-L2/L3). Three
// register blocks are involved, all at their Non-Secure base (per the HAL S/NS
// convention). Each base is the PAC peripheral's own `::PTR` (NS, from the SVD)
// — no address is hand-written here:
//   - gpio_reg    GpioReg     (int / direction / data; ports B/C at +stride)
//   - pinmux_reg  PinmuxReg   (pad pull + function select)
//   - lp_sys      LpSys       (DR/DDR DW-vs-WRAP mode bit @ REG_DW_GPIO_CTRL)
//
// GPIO_Pin encoding matches SDK PinName: bits[6:5]=port (0=A,1=B,2=C), bits[4:0]=pin.
// ────────────────────────────────────────────────────────────────────────────

pub(super) mod hw {
    use super::{GPIOA_PTR, GPIOB_PTR, GPIOC_PTR, Pull, gpio_reg};
    use rtl8721f_pac::{LpSys, lp_sys};

    // GPIO_Mode — SDK: ameba_gpio.h : GPIO_Mode_IN / GPIO_Mode_OUT / GPIO_Mode_INT
    pub const MODE_IN: u32 = 0x0;
    pub const MODE_OUT: u32 = 0x1;
    pub const MODE_INT: u32 = 0x2;

    // GPIO_INT_Trigger — SDK: ameba_gpio.h : GPIO_INT_Trigger_LEVEL / GPIO_INT_Trigger_EDGE / GPIO_INT_Trigger_BOTHEDGE
    pub const INT_TRIGGER_LEVEL: u32 = 0x0;
    pub const INT_TRIGGER_EDGE: u32 = 0x1;
    pub const INT_TRIGGER_BOTHEDGE: u32 = 0x2;

    // GPIO_INT_POLARITY — SDK: ameba_gpio.h : GPIO_INT_POLARITY_LOW / GPIO_INT_POLARITY_HIGH
    pub const INT_POLARITY_LOW: u32 = 0x0;
    pub const INT_POLARITY_HIGH: u32 = 0x1;

    // GPIO_INT_DEBOUNCE — SDK: ameba_gpio.h : GPIO_INT_DEBOUNCE_DISABLE / GPIO_INT_DEBOUNCE_ENABLE
    pub const INT_DEBOUNCE_DISABLE: u32 = 0x0;
    pub const INT_DEBOUNCE_ENABLE: u32 = 0x1;

    // LP_SYS pointer, the PAC's own Non-Secure base (never hand-written), per the
    // HAL S/NS convention. NOTE: the DW_GPIO_CTRL register the SDK reads at
    // SYSTEM_CTRL_BASE+0x2C4 is modeled by the PAC under the LP_SYS peripheral
    // (base 0x4080_A200, reg offset 0xC4) — same absolute 0x4080_A2C4 — so we
    // must start from `LpSys::PTR`, not SYSTEM_CTRL's base, or the offset lands
    // 0x200 short. (Pinmux/PAD register access now lives in `crate::pinmux`.)
    const LP_SYS_PTR: *const lp_sys::RegisterBlock = LpSys::PTR;

    #[inline]
    fn port_regs(pin: u32) -> &'static gpio_reg::RegisterBlock {
        // SDK: ameba_pinmux.h : PORT_NUM(pin) = (pin >> 5) & 0x03
        let port = ((pin >> 5) & 0x3) as usize;
        unsafe { &*[GPIOA_PTR, GPIOB_PTR, GPIOC_PTR][port] }
    }

    /// True when DR/DDR use RTK wrapper (toggle) semantics rather than direct write.
    /// SDK: ameba_gpio_rom.c : GPIO_GetDWWrapper — SYSTEM_CTRL REG_LSYS_DW_GPIO_CTRL
    /// bit31 (LSYS_BIT_GPIO_REG_OPT). Reset value 1 = GPIO_DR_DDR_FUNC_WRAP.
    #[inline]
    fn dw_is_wrap() -> bool {
        let sys = unsafe { &*LP_SYS_PTR };
        sys.reg_dw_gpio_ctrl().read().gpio_reg_opt().bit_is_set()
    }

    /// Pinmux a pin to GPIO function — delegates to the single pinmux/PAD
    /// implementation in [`crate::pinmux`] (which also carries the SWD-pin guard
    /// from `ameba_rom_patch.c : Pinmux_Config`). GPIO function ID = 0.
    /// SDK: ameba_pinmux.h : PINMUX_FUNCTION_GPIO = 0.
    fn pinmux_gpio(pin: u32) {
        crate::pinmux::set_raw(pin, 0);
    }

    /// Set the internal pull resistor for a pin — delegates to [`crate::pinmux`].
    /// SDK: ameba_pinmux_rom.c : PAD_PullCtrl. `Pull::Shutdown` is a no-op there.
    fn pad_pull(pin: u32, pull: Pull) {
        crate::pinmux::set_pull_raw(pin, pull);
    }

    /// Set a pin's data direction (input=false, output=true).
    /// SDK: ameba_gpio_rom.c : GPIO_Direction. DW mode: direct set/clear of the DDR bit.
    /// WRAP mode: DDR is a toggle-command register — writing BIT(pin) flips it, writing 0
    /// leaves it — so read the current direction and write BIT only when it differs.
    fn direction(pin: u32, output: bool) {
        let regs = port_regs(pin);
        let bit = pin & 0x1F;
        let cur = regs.gpio_ddr().read().bits();
        let dir = output as u32; // SDK: GPIO_Mode_OUT=1, GPIO_Mode_IN=0
        let new = if dw_is_wrap() {
            if ((cur >> bit) & 0x1) == dir {
                0
            } else {
                1 << bit
            }
        } else if output {
            cur | (1 << bit)
        } else {
            cur & !(1 << bit)
        };
        regs.gpio_ddr().write(|w| unsafe { w.bits(new) });
    }

    /// Drive an output pin high/low, honoring DW/WRAP semantics.
    /// SDK: ameba_gpio_rom.c : GPIO_WriteBit (same DW/WRAP structure as GPIO_Direction).
    pub fn write_bit(pin: u32, high: bool) {
        let regs = port_regs(pin);
        let bit = pin & 0x1F;
        let cur = regs.gpio_dr().read().bits();
        let st = high as u32; // SDK: GPIO_PIN_HIGH=1, GPIO_PIN_LOW=0
        let new = if dw_is_wrap() {
            if ((cur >> bit) & 0x1) == st {
                0
            } else {
                1 << bit
            }
        } else if high {
            cur | (1 << bit)
        } else {
            cur & !(1 << bit)
        };
        regs.gpio_dr().write(|w| unsafe { w.bits(new) });
    }

    /// Read the live pin level from the external port register.
    /// SDK: ameba_gpio_rom.c : GPIO_ReadDataBit — GPIO_EXT_PORT[0].
    pub fn read_bit(pin: u32) -> bool {
        let regs = port_regs(pin);
        let bit = pin & 0x1F;
        (regs.gpio_ext_port().read().bits() >> bit) & 0x1 != 0
    }

    /// Configure (and enable/disable) a pin's interrupt trigger/polarity/debounce.
    /// SDK: ameba_gpio_rom.c : GPIO_INTMode — the trigger/polarity/debounce block only
    /// runs on ENABLE; the INT_EN bit is always updated last.
    pub fn int_mode(pin: u32, enable: bool, trigger: u32, polarity: u32, debounce: u32) {
        let regs = port_regs(pin);
        let bit = pin & 0x1F;
        let m = 1u32 << bit;
        if enable {
            if trigger != INT_TRIGGER_BOTHEDGE {
                // Close both-edge (higher priority), then set level/single-edge + polarity.
                regs.gpio_int_bothedge()
                    .modify(|r, w| unsafe { w.bits(r.bits() & !m) });
                regs.gpio_int_type()
                    .modify(|r, w| unsafe { w.bits((r.bits() & !m) | (trigger << bit)) });
                regs.gpio_int_polarity()
                    .modify(|r, w| unsafe { w.bits((r.bits() & !m) | (polarity << bit)) });
            } else {
                regs.gpio_int_bothedge()
                    .modify(|r, w| unsafe { w.bits(r.bits() | m) });
            }
            regs.gpio_debounce().modify(|r, w| unsafe {
                let v = r.bits() & !m;
                w.bits(if debounce == INT_DEBOUNCE_ENABLE {
                    v | m
                } else {
                    v
                })
            });
        }
        regs.gpio_int_en()
            .modify(|r, w| unsafe { w.bits(if enable { r.bits() | m } else { r.bits() & !m }) });
    }

    /// Mask/unmask a pin's interrupt and clear its pending flag.
    /// SDK: ameba_gpio_rom.c : GPIO_INTConfig — always clears INT_EOI, then sets/clears INT_MASK.
    pub fn int_config(pin: u32, enable: bool) {
        let regs = port_regs(pin);
        let bit = pin & 0x1F;
        let m = 1u32 << bit;
        regs.gpio_int_eoi().write(|w| unsafe { w.bits(m) });
        // DISABLE → mask (set bit); ENABLE → unmask (clear bit)
        regs.gpio_int_mask()
            .modify(|r, w| unsafe { w.bits(if enable { r.bits() & !m } else { r.bits() | m }) });
    }

    /// Full pin init: pinmux to GPIO, then mode-specific direction / pull / interrupt.
    /// SDK: ameba_gpio_rom.c : GPIO_Init.
    ///
    /// Deviation from ROM (fixes GPIO-6): ROM `GPIO_Init(Mode_OUT)` silently drops
    /// the pupd field — output pull requires a separate `PAD_PullCtrl()` call — so
    /// `output().pull(..)` compiled but did nothing. Here `pad_pull` runs for OUTPUT
    /// as well, honoring the builder's pull request for every mode.
    pub fn gpio_init(pin: u32, mode: u32, pull: Pull, trigger: u32, polarity: u32, debounce: u32) {
        pinmux_gpio(pin);
        if mode == MODE_INT {
            direction(pin, false); // input
            pad_pull(pin, pull);
            int_mode(pin, true, trigger, polarity, debounce);
        } else {
            int_mode(pin, false, 0, 0, 0);
            direction(pin, mode == MODE_OUT);
            pad_pull(pin, pull); // GPIO-6: applied for OUTPUT too, unlike ROM
        }
    }

    /// Reset a pin to input / HIGHZ and clear its interrupt.
    /// SDK: ameba_gpio_rom.c : GPIO_DeInit — note the DDR bit is cleared with a *direct*
    /// read-modify-write (not the DW/WRAP toggle path), replicated here exactly.
    pub fn gpio_deinit(pin: u32) {
        let regs = port_regs(pin);
        let bit = pin & 0x1F;
        let m = 1u32 << bit;
        regs.gpio_ddr()
            .modify(|r, w| unsafe { w.bits(r.bits() & !m) });
        pad_pull(pin, Pull::NoPull);
        regs.gpio_int_en()
            .modify(|r, w| unsafe { w.bits(r.bits() & !m) });
        regs.gpio_int_eoi().write(|w| unsafe { w.bits(m) });
    }
}

// ────────────────────────────────────────────────────────────────────────────
// NVIC helpers
// ────────────────────────────────────────────────────────────────────────────

fn enable_gpio_port_irq(port: usize) {
    const GPIO_IRQS: [rtl8721f_pac::Interrupt; 3] = [
        rtl8721f_pac::Interrupt::GPIOA_IRQ, // SDK: ameba_vector_table.h : GPIOA_IRQ = 25
        rtl8721f_pac::Interrupt::GPIOB_IRQ, // SDK: ameba_vector_table.h : GPIOB_IRQ = 26
        rtl8721f_pac::Interrupt::GPIOC_IRQ, // SDK: ameba_vector_table.h : GPIOC_IRQ = 27
    ];
    unsafe { cortex_m::peripheral::NVIC::unmask(GPIO_IRQS[port]) };
}

// ────────────────────────────────────────────────────────────────────────────
// Internal IRQ handler table — one slot per physical pin, no static mut
// ────────────────────────────────────────────────────────────────────────────

struct IrqSlot {
    handler: AtomicUsize,
    data: AtomicUsize,
}

// SAFETY: all accesses are atomic
unsafe impl Sync for IrqSlot {}

impl IrqSlot {
    const fn empty() -> Self {
        IrqSlot {
            handler: AtomicUsize::new(0),
            data: AtomicUsize::new(0),
        }
    }
}

// [port_index][pin_bit]: port 0=A, 1=B, 2=C. Sized to every physical pin (like
// `GPIO_WAKERS`/`ASYNC_PENDING` below) rather than a small shared pool, so
// `register_irq` can never run out of slots.
static IRQ_TABLE: [[IrqSlot; 32]; 3] = [
    [const { IrqSlot::empty() }; 32],
    [const { IrqSlot::empty() }; 32],
    [const { IrqSlot::empty() }; 32],
];

/// Call from your GPIO interrupt handler, passing the port register block pointer.
/// Dispatches async wakers and registered callback handlers for that port.
pub(crate) fn gpio_int_dispatch(ptr: *const gpio_reg::RegisterBlock) {
    let regs = unsafe { &*ptr };
    // KP-2 exemption: full 32-bit mask stored for per-bit dispatch (state transfer, not bit interpretation).
    let port_status = regs.gpio_int_status().read().gpio_int_status().bits();
    // Clear EOI before any dispatch to prevent new edges from being swallowed.
    // SDK: ameba_gpio.c : GPIO_INTHandler — GPIO_INT_EOI write precedes all callback dispatch.
    regs.gpio_int_eoi()
        .write(|w| unsafe { w.bits(port_status) });
    // Derive port index (0=A, 1=B, 2=C) from pointer offset relative to GPIOA.
    let port_idx = (ptr as usize - GPIOA_PTR as usize) / GPIO_PORT_STRIDE;

    // Async path: wake futures waiting for an edge on this port.
    let async_mask = ASYNC_PENDING[port_idx].load(Ordering::Acquire) & port_status;
    if async_mask != 0 {
        for bit in 0..32u32 {
            if async_mask & (1 << bit) != 0 {
                let full_pin = ((port_idx as u32) << 5) | bit;
                hw::int_config(full_pin, false);
                ASYNC_PENDING[port_idx].fetch_and(!(1 << bit), Ordering::Release);
                GPIO_WAKERS[port_idx][bit as usize].wake();
            }
        }
    }

    // Callback path: dispatch to registered handlers on this port, indexed
    // directly by pin bit (each physical pin owns a fixed slot).
    for pin_bit in 0..32u32 {
        if port_status & (1 << pin_bit) == 0 {
            continue;
        }
        let slot = &IRQ_TABLE[port_idx][pin_bit as usize];
        let handler_ptr = slot.handler.load(Ordering::Acquire);
        if handler_ptr != 0 {
            let data_ptr = slot.data.load(Ordering::Acquire);
            // SAFETY: handler_ptr was written by register_irq from a valid &'static mut H.
            // data_ptr points to the same object, still alive due to 'static bound.
            unsafe {
                let trampoline: unsafe fn(usize, u32, u32) = core::mem::transmute(handler_ptr);
                trampoline(data_ptr, port_idx as u32, pin_bit);
            }
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Async edge-detection (ports A/B/C, 32 pins each)
// ────────────────────────────────────────────────────────────────────────────

// [port_index][pin_bit]: port 0=A, 1=B, 2=C
static GPIO_WAKERS: [[AtomicWaker; 32]; 3] = [
    [const { AtomicWaker::new() }; 32],
    [const { AtomicWaker::new() }; 32],
    [const { AtomicWaker::new() }; 32],
];

/// Per-port bitmask of pins currently waiting for an async edge.
static ASYNC_PENDING: [AtomicU32; 3] = [AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0)];

/// Future returned by `GpioPin<Input>::wait_for_*_edge()`.
pub struct GpioEdgeFuture {
    pin: u32, // full pin encoding (bits[7:5]=port, bits[4:0]=pin_number)
}

impl Future for GpioEdgeFuture {
    type Output = ();
    fn poll(self: core::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let bit = (self.pin & 0x1F) as usize;
        let port = ((self.pin >> 5) & 0x7) as usize;
        // Register waker BEFORE checking condition (prevent missed wake).
        GPIO_WAKERS[port][bit].register(cx.waker());
        if ASYNC_PENDING[port].load(Ordering::Acquire) & (1u32 << bit) == 0 {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

impl Drop for GpioEdgeFuture {
    fn drop(&mut self) {
        let bit = self.pin & 0x1F;
        let port = ((self.pin >> 5) & 0x7) as usize;
        if ASYNC_PENDING[port].load(Ordering::Relaxed) & (1 << bit) != 0 {
            hw::int_config(self.pin, false);
            ASYNC_PENDING[port].fetch_and(!(1 << bit), Ordering::Relaxed);
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Public enums
// ────────────────────────────────────────────────────────────────────────────

// Pin selection and pull configuration share one implementation with the
// peripheral pinmux (SPI/I2C/UART/…): the `gpio` API is typed on `pinmux::Pin`
// and `pinmux::Pull` rather than a raw pin index / a duplicate `Pull` enum.
pub use crate::pinmux::{Pin, Pull};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntTrigger {
    Level,
    Edge,
    BothEdge,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntPolarity {
    ActiveLow,
    ActiveHigh,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntDebounce {
    Disabled,
    Enabled,
}

// ────────────────────────────────────────────────────────────────────────────
// Type states
// ────────────────────────────────────────────────────────────────────────────

pub struct Input;
pub struct Output;
pub struct Interrupt;

// ────────────────────────────────────────────────────────────────────────────
// IRQ handler trait (safe abstraction over C-style void* callback)
// ────────────────────────────────────────────────────────────────────────────

/// GPIO interrupt event passed to callback handlers.
/// Carries port and pin to match SDK `GPIO_IRQ_FUN` Id convention.
/// SDK: ameba_gpio.c : GPIO_INTHandler — Id encodes port_num and pin_num.
#[derive(Clone, Copy, Debug)]
pub struct GpioEvent {
    /// Port index: 0=A, 1=B, 2=C
    pub port: u32,
    /// Pin bit within port: 0–31
    pub pin: u32,
}

pub trait GpioIrqHandler: Send {
    fn on_irq(&mut self, event: GpioEvent);
}

// ────────────────────────────────────────────────────────────────────────────
// GpioPin builder
// ────────────────────────────────────────────────────────────────────────────

pub struct GpioPinBuilder {
    pin: u32,
    pull: Pull,
    it_trigger: IntTrigger,
    it_polarity: IntPolarity,
    it_debounce: IntDebounce,
}

impl GpioPinBuilder {
    pub fn pull(mut self, pull: Pull) -> Self {
        self.pull = pull;
        self
    }

    pub fn trigger(mut self, trigger: IntTrigger) -> Self {
        self.it_trigger = trigger;
        self
    }

    pub fn polarity(mut self, polarity: IntPolarity) -> Self {
        self.it_polarity = polarity;
        self
    }

    pub fn debounce(mut self, debounce: IntDebounce) -> Self {
        self.it_debounce = debounce;
        self
    }

    pub fn input(self) -> GpioPin<Input> {
        hw::gpio_init(self.pin, hw::MODE_IN, self.pull, 0, 0, 0);
        GpioPin {
            pin: self.pin,
            _mode: PhantomData,
        }
    }

    pub fn output(self) -> GpioPin<Output> {
        hw::gpio_init(self.pin, hw::MODE_OUT, self.pull, 0, 0, 0);
        GpioPin {
            pin: self.pin,
            _mode: PhantomData,
        }
    }

    pub fn interrupt(self) -> GpioPin<Interrupt> {
        // Pre-disable before (re-)init matches SDK raw example sequence.
        // SDK: raw_gpio_edge_irq/example_raw_gpio_edge_irq.c : GPIO_INTConfig(DISABLE) before GPIO_Init
        hw::int_config(self.pin, false);
        hw::gpio_init(
            self.pin,
            hw::MODE_INT,
            self.pull,
            trigger_to_u32(self.it_trigger),
            polarity_to_u32(self.it_polarity),
            debounce_to_u32(self.it_debounce),
        );
        GpioPin {
            pin: self.pin,
            _mode: PhantomData,
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// GpioPin struct
// ────────────────────────────────────────────────────────────────────────────

pub struct GpioPin<Mode> {
    pin: u32,
    _mode: PhantomData<Mode>,
}

impl<M> GpioPin<M> {
    /// Create a builder for the given pin (e.g. `Pin::PA14`).
    pub fn builder(pin: Pin) -> GpioPinBuilder {
        GpioPinBuilder {
            // Stored as the raw u32 PinName encoding used throughout `hw` and the
            // IRQ/waker tables; `Pin as u32` == that encoding (bits[7:5]=port,
            // bits[4:0]=pin_number).
            pin: pin as u32,
            pull: Pull::NoPull,
            it_trigger: IntTrigger::Edge,
            it_polarity: IntPolarity::ActiveHigh,
            it_debounce: IntDebounce::Disabled,
        }
    }

    pub fn deinit(self) {
        hw::gpio_deinit(self.pin);
    }
}

impl GpioPin<Input> {
    pub fn read(&self) -> bool {
        hw::read_bit(self.pin)
    }

    /// Wait for a falling edge on this pin (async).
    /// Automatically enables the NVIC IRQ for this pin's GPIO port.
    pub fn wait_for_falling_edge(&mut self) -> GpioEdgeFuture {
        let bit = self.pin & 0x1F;
        let port = ((self.pin >> 5) & 0x7) as usize;
        enable_gpio_port_irq(port);
        ASYNC_PENDING[port].fetch_or(1 << bit, Ordering::Release);
        hw::int_mode(
            self.pin,
            true,
            hw::INT_TRIGGER_EDGE,
            hw::INT_POLARITY_LOW,
            hw::INT_DEBOUNCE_DISABLE,
        );
        hw::int_config(self.pin, true);
        GpioEdgeFuture { pin: self.pin }
    }

    /// Wait for a rising edge on this pin (async).
    /// Automatically enables the NVIC IRQ for this pin's GPIO port.
    pub fn wait_for_rising_edge(&mut self) -> GpioEdgeFuture {
        let bit = self.pin & 0x1F;
        let port = ((self.pin >> 5) & 0x7) as usize;
        enable_gpio_port_irq(port);
        ASYNC_PENDING[port].fetch_or(1 << bit, Ordering::Release);
        hw::int_mode(
            self.pin,
            true,
            hw::INT_TRIGGER_EDGE,
            hw::INT_POLARITY_HIGH,
            hw::INT_DEBOUNCE_DISABLE,
        );
        hw::int_config(self.pin, true);
        GpioEdgeFuture { pin: self.pin }
    }

    /// Wait for either edge on this pin (async).
    /// Automatically enables the NVIC IRQ for this pin's GPIO port.
    pub fn wait_for_any_edge(&mut self) -> GpioEdgeFuture {
        let bit = self.pin & 0x1F;
        let port = ((self.pin >> 5) & 0x7) as usize;
        enable_gpio_port_irq(port);
        ASYNC_PENDING[port].fetch_or(1 << bit, Ordering::Release);
        hw::int_mode(
            self.pin,
            true,
            hw::INT_TRIGGER_BOTHEDGE,
            hw::INT_POLARITY_LOW,
            hw::INT_DEBOUNCE_DISABLE,
        );
        hw::int_config(self.pin, true);
        GpioEdgeFuture { pin: self.pin }
    }
}

impl GpioPin<Output> {
    pub fn set_high(&self) {
        hw::write_bit(self.pin, true)
    }

    pub fn set_low(&self) {
        hw::write_bit(self.pin, false)
    }

    pub fn set(&self, high: bool) {
        hw::write_bit(self.pin, high)
    }

    pub fn toggle(&self) {
        hw::write_bit(self.pin, !hw::read_bit(self.pin))
    }

    pub fn read(&self) -> bool {
        hw::read_bit(self.pin)
    }
}

impl GpioPin<Interrupt> {
    /// Register an IRQ handler for this pin.
    /// The handler must have `'static` lifetime and implement `GpioIrqHandler + Send`.
    pub fn register_irq<H: GpioIrqHandler + 'static>(&self, handler: &'static mut H) {
        let slot = irq_slot(self.pin);
        let handler_ptr: unsafe fn(usize, u32, u32) = handler_trampoline::<H>;
        slot.data
            .store(handler as *mut H as usize, Ordering::Release);
        slot.handler.store(handler_ptr as usize, Ordering::Release);
        enable_gpio_port_irq(((self.pin >> 5) & 0x7) as usize);
        hw::int_config(self.pin, true);
    }

    pub fn unregister_irq(&self) {
        deregister_pin_irq(self.pin);
    }

    pub fn set_mode(&self, trigger: IntTrigger, polarity: IntPolarity, debounce: IntDebounce) {
        hw::int_mode(
            self.pin,
            true,
            trigger_to_u32(trigger),
            polarity_to_u32(polarity),
            debounce_to_u32(debounce),
        )
    }

    pub fn read(&self) -> bool {
        hw::read_bit(self.pin)
    }
}

/// RAII deregistration on scope exit — the counterpart of SDK
/// `GPIO_UserUnRegIrq`: mask the pin + clear pending (`GPIO_INTConfig(DISABLE)`
/// → INT_MASK + INT_EOI), clear `GPIO_INT_EN` (`GPIO_INTMode(DISABLE)`), and
/// free the software `IRQ_TABLE` slot. Without this, dropping an interrupt pin
/// would leak the live interrupt source (INT_EN set) and leave the stale
/// handler pointer registered on this pin's dedicated slot.
///
/// Rust forbids specializing `Drop` on `GpioPin<Interrupt>` alone, so this
/// covers every `GpioPin<M>`. For non-interrupt pins the pin never registered,
/// so every step is a harmless no-op (INT_EN/MASK bits already 0, no matching
/// slot) that leaves direction / level / pad untouched. Deliberately does NOT
/// call `GPIO_DeInit` — resetting the pad is reserved for explicit `deinit`,
/// mirroring SDK's split between `GPIO_UserUnRegIrq` and `GPIO_DeInit`.
impl<M> Drop for GpioPin<M> {
    fn drop(&mut self) {
        deregister_pin_irq(self.pin);
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ────────────────────────────────────────────────────────────────────────────

unsafe fn handler_trampoline<H: GpioIrqHandler>(data: usize, port: u32, pin: u32) {
    let handler = unsafe { &mut *(data as *mut H) };
    handler.on_irq(GpioEvent { port, pin });
}

/// Fully deregister any interrupt on `pin` — the shared body of
/// `GpioPin::<Interrupt>::unregister_irq` and `Drop`. Mirrors SDK
/// `GPIO_UserUnRegIrq`: mask + clear pending, then clear `GPIO_INT_EN`, then
/// free the software `IRQ_TABLE` slot. No-op for pins that never registered
/// (the DISABLE branch of `GPIO_INTMode` ignores the trigger/polarity/debounce
/// args — see ameba_gpio_rom.c:63-96 — so the zeros are inert).
fn deregister_pin_irq(pin: u32) {
    hw::int_config(pin, false); // INT_MASK set + clear INT_EOI
    hw::int_mode(pin, false, 0, 0, 0); // clear GPIO_INT_EN bit
    let slot = irq_slot(pin);
    slot.handler.store(0, Ordering::Release);
    slot.data.store(0, Ordering::Release);
}

fn irq_slot(pin: u32) -> &'static IrqSlot {
    let port = ((pin >> 5) & 0x7) as usize;
    let bit = (pin & 0x1F) as usize;
    &IRQ_TABLE[port][bit]
}

fn trigger_to_u32(t: IntTrigger) -> u32 {
    match t {
        IntTrigger::Level => hw::INT_TRIGGER_LEVEL,
        IntTrigger::Edge => hw::INT_TRIGGER_EDGE,
        IntTrigger::BothEdge => hw::INT_TRIGGER_BOTHEDGE,
    }
}

fn polarity_to_u32(p: IntPolarity) -> u32 {
    match p {
        IntPolarity::ActiveLow => hw::INT_POLARITY_LOW,
        IntPolarity::ActiveHigh => hw::INT_POLARITY_HIGH,
    }
}

fn debounce_to_u32(d: IntDebounce) -> u32 {
    match d {
        IntDebounce::Disabled => hw::INT_DEBOUNCE_DISABLE,
        IntDebounce::Enabled => hw::INT_DEBOUNCE_ENABLE,
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn GPIOA_IRQ() {
    gpio_int_dispatch(GPIOA_PTR);
}
#[unsafe(no_mangle)]
unsafe extern "C" fn GPIOB_IRQ() {
    gpio_int_dispatch(GPIOB_PTR);
}
#[unsafe(no_mangle)]
unsafe extern "C" fn GPIOC_IRQ() {
    gpio_int_dispatch(GPIOC_PTR);
}

pub mod eh;
