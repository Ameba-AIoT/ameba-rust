//! RTL8721F SPORT (Serial Port / I2S) driver.
//!
//! Base address: 0x4011_8000 (SPORT0).
//!
//! GDMA-dependent functions (AUDIO_SP_TXGDMA_Init, AUDIO_SP_RXGDMA_Init,
//! AUDIO_SP_TXGDMA_Restart, AUDIO_SP_RXGDMA_Restart, AUDIO_SP_LLPTXGDMA_Init,
//! AUDIO_SP_LLPRXGDMA_Init) are intentionally omitted — they require GDMA and
//! DCache_CleanInvalidate which are out of scope for this crate.
//!
//! No ROM symbols from ameba-rom-sys are used by the SPORT driver.

use bitflags::bitflags;
use rtl8721f_pac::{PinmuxReg, Sport0Reg, sport0_reg::RegisterBlock};

// Bits in REG_I2S_CTRL for SP0 and SP1 DIO mux selection
const PAD_BIT_SP0_DIO0_MUXSEL: u32 = 1 << 0;
const PAD_BIT_SP0_DIO1_MUXSEL: u32 = 1 << 1;
const PAD_BIT_SP0_DIO2_MUXSEL: u32 = 1 << 2;
const PAD_BIT_SP0_DIO3_MUXSEL: u32 = 1 << 3;
const PAD_BIT_SP1_DIO0_MUXSEL: u32 = 1 << 4;
const PAD_BIT_SP1_DIO1_MUXSEL: u32 = 1 << 5;
const PAD_BIT_SP1_DIO2_MUXSEL: u32 = 1 << 6;
const PAD_BIT_SP1_DIO3_MUXSEL: u32 = 1 << 7;

// ---------------------------------------------------------------------------
// Public enumerations
// ---------------------------------------------------------------------------

/// I2S / PCM data format.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DataFormat {
    I2S = 0,
    LeftJustified = 1,
    PcmModeA = 2,
    PcmModeB = 3,
}

/// Word (sample) length in bits.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WordLen {
    Bits16 = 0,
    Bits20 = 1,
    Bits24 = 2,
    Bits32 = 4,
}

/// Channel (slot) length.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ChanLen {
    Bits16 = 0,
    Bits20 = 1,
    Bits24 = 2,
    Bits32 = 4,
}

impl ChanLen {
    fn bits_value(self) -> u32 {
        match self {
            ChanLen::Bits16 => 16,
            ChanLen::Bits20 => 20,
            ChanLen::Bits24 => 24,
            ChanLen::Bits32 => 32,
        }
    }
    fn bclk_div(self) -> u32 {
        match self {
            ChanLen::Bits16 => 32,
            ChanLen::Bits20 => 40,
            ChanLen::Bits24 => 48,
            ChanLen::Bits32 => 64,
        }
    }
}

/// Channel selection (L/R ordering at DAC or ADC path).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ChanSel {
    LR = 0,
    RL = 1,
    LL = 2,
    RR = 3,
}

/// Mono / stereo selection.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MonoStereo {
    Stereo,
    Mono,
}

/// TDM mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TdmMode {
    NoTdm = 0,
    Tdm4 = 1,
    Tdm6 = 2,
    Tdm8 = 3,
}

/// FIFO selection (number of active channels / 2).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FifoSel {
    /// 2 channels — FIFO0_REG0 only
    Fifo2,
    /// 4 channels — FIFO0_REG0 + FIFO0_REG1
    Fifo4,
    /// 6 channels — FIFO0_REG0 + FIFO0_REG1 + FIFO1_REG0
    Fifo6,
    /// 8 channels — all four FIFO sub-registers
    Fifo8,
}

/// Clock source for BCLK/MCLK divider computation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ClkSrc {
    /// 40 MHz crystal
    Xtal40M,
    /// Any other source — caller supplies Hz value
    Other(u32),
}

impl ClkSrc {
    fn hz(self) -> u32 {
        match self {
            ClkSrc::Xtal40M => 40_000_000,
            ClkSrc::Other(v) => v,
        }
    }
}

/// Fixed BCLK divider ratio.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FixedBclkDiv {
    /// dsp_src_clk / 4
    Div4,
    /// dsp_src_clk / 2
    Div2,
}

/// Master / slave mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MasterSlave {
    Master,
    Slave,
}

/// TX channel selector (DOUT0-3 correspond to pairs of channels 0-7).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TxChn {
    Chn0 = 0,
    Chn1 = 1,
    Chn2 = 2,
    Chn3 = 3,
    Chn4 = 4,
    Chn5 = 5,
    Chn6 = 6,
    Chn7 = 7,
}

/// RX channel selector (DIN0-3 correspond to pairs of channels 0-7).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RxChn {
    Chn0 = 0,
    Chn1 = 1,
    Chn2 = 2,
    Chn3 = 3,
    Chn4 = 4,
    Chn5 = 5,
    Chn6 = 6,
    Chn7 = 7,
}

/// TX FIFO slot — which FIFO sub-register contributes to a TX channel.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TxFifoChn {
    Fifo0Reg0L = 0,
    Fifo0Reg0R = 1,
    Fifo0Reg1L = 2,
    Fifo0Reg1R = 3,
    Fifo1Reg0L = 4,
    Fifo1Reg0R = 5,
    Fifo1Reg1L = 6,
    Fifo1Reg1R = 7,
}

/// RX FIFO slot — which RX channel feeds a FIFO sub-register.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RxFifoChn {
    Fifo0Reg0L = 0,
    Fifo0Reg0R = 1,
    Fifo0Reg1L = 2,
    Fifo0Reg1R = 3,
    Fifo1Reg0L = 4,
    Fifo1Reg0R = 5,
    Fifo1Reg1L = 6,
    Fifo1Reg1R = 7,
}

/// I2S interface index (for pin-mux selection).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum I2sSel {
    I2S0,
    I2S1,
}

/// Pin function for I2S pin-mux.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PinFunc {
    Dout0,
    Dout1,
    Dout2,
    Dout3,
    Din0,
    Din1,
    Din2,
    Din3,
}

/// LRCLK edge polarity for start interrupt.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LrclkEdge {
    Rising,
    Falling,
}

// ---------------------------------------------------------------------------
// bitflags! for interrupt flag sets (not yet used externally but available)
// ---------------------------------------------------------------------------

bitflags! {
    /// Interrupt flags visible in SP_INT_CTRL.
    #[derive(Copy, Clone, Debug)]
    pub struct SportIntFlags: u32 {
        const LRC_START = 1 << 14;
        const LRC_STOP  = 1 << 15;
    }
}

// ---------------------------------------------------------------------------
// Initialisation configuration
// ---------------------------------------------------------------------------

/// Direction of a SPORT operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Tx,
    Rx,
}

/// Full initialisation parameters — mirrors SP_InitTypeDef.
#[derive(Clone, Debug)]
pub struct SpInitConfig {
    pub data_format: DataFormat,
    pub word_len: WordLen,
    pub chan_len: ChanLen,
    pub chan_sel: ChanSel,
    pub sample_rate: u32,
    pub tdm: TdmMode,
    pub fifo: FifoSel,
    pub mono_stereo: MonoStereo,
    pub multi_io: bool,
    pub clk_src: ClkSrc,
    /// When true, use fixed BCLK (dsp_src_clk based) instead of fractional divider.
    pub fix_bclk: bool,
}

impl Default for SpInitConfig {
    fn default() -> Self {
        let tdm = TdmMode::NoTdm;
        let sr = 48_000u32;
        Self {
            data_format: DataFormat::I2S,
            word_len: WordLen::Bits16,
            chan_len: ChanLen::Bits32,
            chan_sel: ChanSel::LR,
            sample_rate: sr,
            tdm,
            fifo: FifoSel::Fifo2,
            mono_stereo: MonoStereo::Stereo,
            multi_io: false,
            clk_src: ClkSrc::Xtal40M,
            fix_bclk: false,
        }
    }
}

impl SpInitConfig {
    fn bclk(&self) -> u32 {
        let chn_len = self.chan_len.bits_value();
        let tdm_mult = (self.tdm as u32) + 1;
        chn_len * tdm_mult * 2 * self.sample_rate
    }
}

// ---------------------------------------------------------------------------
// Registration state (replaces the C-level SP_RegFlag array)
// ---------------------------------------------------------------------------

struct RegFlag {
    tx: bool,
    rx: bool,
    bclk: u32,
}

impl RegFlag {
    const fn new() -> Self {
        Self {
            tx: false,
            rx: false,
            bclk: 0,
        }
    }
}

// Only one SPORT instance on RTL8721F.
static mut REG_FLAG: RegFlag = RegFlag::new();

// ---------------------------------------------------------------------------
// Type-state markers
// ---------------------------------------------------------------------------

pub struct Unconfigured;
pub struct TxRunning;
pub struct RxRunning;

// ---------------------------------------------------------------------------
// Main driver struct
// ---------------------------------------------------------------------------

/// SPORT0 driver for RTL8721F.
///
/// Obtain an instance with `Sport::steal()`.  The instance holds an exclusive
/// borrow of the PAC `RegisterBlock` through a raw pointer so that the crate
/// can remain `#![no_std]` without requiring the user to pass the PAC
/// peripheral at call-site.
pub struct Sport<State> {
    rb: *const RegisterBlock,
    _state: core::marker::PhantomData<State>,
}

unsafe impl<S> Send for Sport<S> {}

impl<S> Sport<S> {
    #[inline(always)]
    fn rb(&self) -> &RegisterBlock {
        unsafe { &*self.rb }
    }
}

impl Sport<Unconfigured> {
    /// Create a Sport instance that aliases SPORT0's register block.
    ///
    /// # Safety
    /// Caller must ensure no other code accesses SPORT0 concurrently and that
    /// the peripheral clock is enabled before calling any register-accessing
    /// method.
    pub unsafe fn steal() -> Self {
        Self {
            rb: Sport0Reg::PTR,
            _state: core::marker::PhantomData,
        }
    }

    // -----------------------------------------------------------------------
    // Registration helpers
    // -----------------------------------------------------------------------

    /// Register a direction for this SPORT, checking BCLK consistency.
    /// Returns false if registration conflicts with an existing direction.
    pub fn register(&self, dir: Direction, cfg: &SpInitConfig) -> bool {
        let bclk = cfg.bclk();
        // Safety: single-threaded embedded context; no concurrent access.
        unsafe {
            match dir {
                Direction::Tx => {
                    if REG_FLAG.tx {
                        return false;
                    }
                    if REG_FLAG.rx {
                        if REG_FLAG.bclk == bclk {
                            REG_FLAG.tx = true;
                            return true;
                        }
                        if cfg.fix_bclk {
                            return true;
                        }
                        return false;
                    }
                    REG_FLAG.tx = true;
                    REG_FLAG.bclk = bclk;
                    true
                }
                Direction::Rx => {
                    if REG_FLAG.rx {
                        return false;
                    }
                    if REG_FLAG.tx {
                        if REG_FLAG.bclk == bclk {
                            REG_FLAG.rx = true;
                            return true;
                        }
                        if cfg.fix_bclk {
                            return true;
                        }
                        return false;
                    }
                    REG_FLAG.rx = true;
                    REG_FLAG.bclk = bclk;
                    true
                }
            }
        }
    }

    /// Unregister a direction.
    pub fn unregister(&self, dir: Direction) {
        unsafe {
            match dir {
                Direction::Tx => REG_FLAG.tx = false,
                Direction::Rx => REG_FLAG.rx = false,
            }
        }
    }

    // -----------------------------------------------------------------------
    // Reset
    // -----------------------------------------------------------------------

    /// Assert then de-assert the SPORT module reset.
    pub fn reset(&self) {
        self.rb().sp_ctrl0().modify(|_, w| w.sp_reset().set_bit());
        self.rb().sp_ctrl0().modify(|_, w| w.sp_reset().clear_bit());
    }

    // -----------------------------------------------------------------------
    // Clock configuration
    // -----------------------------------------------------------------------

    /// Compute and program TX BCLK fractional divider (MI/NI) and LRCLK divider.
    pub fn set_tx_clk_div(&self, clk: ClkSrc, sr: u32, tdm: TdmMode, chan_len: ChanLen) {
        let (mi, ni) = compute_mi_ni(clk.hz(), sr, tdm, chan_len);
        self.rb().sr_tx_bclk().modify(|_, w| unsafe {
            w.tx_mi()
                .bits(mi as u16)
                .tx_ni()
                .bits(ni as u16)
                .tx_mi_ni_update()
                .set_bit()
        });
        let bclk_div = chan_len.bclk_div();
        let ratio = match tdm {
            TdmMode::NoTdm => bclk_div - 1,
            TdmMode::Tdm4 => bclk_div * 2 - 1,
            TdmMode::Tdm6 => bclk_div * 3 - 1,
            TdmMode::Tdm8 => bclk_div * 4 - 1,
        };
        self.rb()
            .sp_tx_lrclk()
            .modify(|_, w| unsafe { w.tx_bclk_div_ratio().bits(ratio as u8) });
    }

    /// Compute and program RX BCLK fractional divider (MI/NI) and LRCLK divider.
    pub fn set_rx_clk_div(&self, clk: ClkSrc, sr: u32, tdm: TdmMode, chan_len: ChanLen) {
        let (mi, ni) = compute_mi_ni(clk.hz(), sr, tdm, chan_len);
        self.rb().sp_rx_bclk().modify(|_, w| unsafe {
            w.rx_mi()
                .bits(mi as u16)
                .rx_ni()
                .bits(ni as u16)
                .rx_mi_ni_update()
                .set_bit()
        });
        let bclk_div = chan_len.bclk_div();
        let ratio = match tdm {
            TdmMode::NoTdm => bclk_div - 1,
            TdmMode::Tdm4 => bclk_div * 2 - 1,
            TdmMode::Tdm6 => bclk_div * 3 - 1,
            TdmMode::Tdm8 => bclk_div * 4 - 1,
        };
        self.rb()
            .sp_tx_lrclk()
            .modify(|_, w| unsafe { w.rx_bclk_div_ratio().bits(ratio as u8) });
    }

    /// Enable or disable MCLK output.
    pub fn set_mclk(&self, enable: bool) {
        self.rb()
            .sp_ctrl1()
            .modify(|_, w| w.enable_mclk().bit(enable));
    }

    /// Set MCLK fractional divider. `mclk_out = 40MHz * ni / mi`.
    pub fn set_mclk_div(&self, ni: u16, mi: u16) {
        self.rb().sp_mclk().modify(|_, w| unsafe {
            w.mclk_mi()
                .bits(mi)
                .mclk_ni()
                .bits(ni)
                .mclk_mi_ni_update()
                .set_bit()
        });
    }

    /// Enable or disable fixed BCLK mode (bypasses fractional divider).
    pub fn set_fix_bclk(&self, enable: bool) {
        self.rb()
            .sp_format()
            .modify(|_, w| w.fixed_bclk().bit(enable));
    }

    /// Select fixed BCLK divider ratio.
    pub fn sel_fix_bclk(&self, div: FixedBclkDiv) {
        let sel = matches!(div, FixedBclkDiv::Div2);
        self.rb()
            .sp_format()
            .modify(|_, w| w.fixed_bclk_sel().bit(sel));
    }

    // -----------------------------------------------------------------------
    // TX / RX start / stop
    // -----------------------------------------------------------------------

    /// Start or stop SPORT TX.
    pub fn tx_start(&self, enable: bool) {
        if enable {
            self.rb()
                .sp_ctrl0()
                .modify(|_, w| w.sp_tx_disable().clear_bit().sp_start_tx().set_bit());
        } else {
            self.rb()
                .sp_ctrl0()
                .modify(|_, w| w.sp_tx_disable().set_bit().sp_start_tx().clear_bit());
        }
    }

    /// Start or stop SPORT RX.
    pub fn rx_start(&self, enable: bool) {
        if enable {
            self.rb()
                .sp_ctrl0()
                .modify(|_, w| w.sp_rx_disable().clear_bit().sp_start_rx().set_bit());
        } else {
            self.rb()
                .sp_ctrl0()
                .modify(|_, w| w.sp_rx_disable().set_bit().sp_start_rx().clear_bit());
        }
    }

    // -----------------------------------------------------------------------
    // FIFO enable/disable
    // -----------------------------------------------------------------------

    /// Enable or disable TX FIFOs according to `fifo_sel`.
    pub fn tx_set_fifo(&self, fifo_sel: FifoSel, enable: bool) {
        match (fifo_sel, enable) {
            (FifoSel::Fifo2, true) => self
                .rb()
                .sp_ctrl1()
                .modify(|_, w| w.tx_fifo_0_reg_0_en().set_bit()),
            (FifoSel::Fifo4, true) => self.rb().sp_ctrl1().modify(|_, w| {
                w.tx_fifo_0_reg_0_en()
                    .set_bit()
                    .tx_fifo_0_reg_1_en()
                    .set_bit()
            }),
            (FifoSel::Fifo6, true) => self.rb().sp_ctrl1().modify(|_, w| {
                w.tx_fifo_0_reg_0_en()
                    .set_bit()
                    .tx_fifo_0_reg_1_en()
                    .set_bit()
                    .tx_fifo_1_reg_0_en()
                    .set_bit()
            }),
            (FifoSel::Fifo8, true) => self.rb().sp_ctrl1().modify(|_, w| {
                w.tx_fifo_0_reg_0_en()
                    .set_bit()
                    .tx_fifo_0_reg_1_en()
                    .set_bit()
                    .tx_fifo_1_reg_0_en()
                    .set_bit()
                    .tx_fifo_1_reg_1_en()
                    .set_bit()
            }),
            (FifoSel::Fifo2, false) => self
                .rb()
                .sp_ctrl1()
                .modify(|_, w| w.tx_fifo_0_reg_0_en().clear_bit()),
            (FifoSel::Fifo4, false) => self.rb().sp_ctrl1().modify(|_, w| {
                w.tx_fifo_0_reg_0_en()
                    .clear_bit()
                    .tx_fifo_0_reg_1_en()
                    .clear_bit()
            }),
            (FifoSel::Fifo6, false) => self.rb().sp_ctrl1().modify(|_, w| {
                w.tx_fifo_0_reg_0_en()
                    .clear_bit()
                    .tx_fifo_0_reg_1_en()
                    .clear_bit()
                    .tx_fifo_1_reg_0_en()
                    .clear_bit()
            }),
            (FifoSel::Fifo8, false) => self.rb().sp_ctrl1().modify(|_, w| {
                w.tx_fifo_0_reg_0_en()
                    .clear_bit()
                    .tx_fifo_0_reg_1_en()
                    .clear_bit()
                    .tx_fifo_1_reg_0_en()
                    .clear_bit()
                    .tx_fifo_1_reg_1_en()
                    .clear_bit()
            }),
        };
    }

    /// Enable or disable RX FIFOs according to `fifo_sel`.
    pub fn rx_set_fifo(&self, fifo_sel: FifoSel, enable: bool) {
        match (fifo_sel, enable) {
            (FifoSel::Fifo2, true) => self
                .rb()
                .sp_ctrl1()
                .modify(|_, w| w.rx_fifo_0_reg_0_en().set_bit()),
            (FifoSel::Fifo4, true) => self.rb().sp_ctrl1().modify(|_, w| {
                w.rx_fifo_0_reg_0_en()
                    .set_bit()
                    .rx_fifo_0_reg_1_en()
                    .set_bit()
            }),
            (FifoSel::Fifo6, true) => self.rb().sp_ctrl1().modify(|_, w| {
                w.rx_fifo_0_reg_0_en()
                    .set_bit()
                    .rx_fifo_0_reg_1_en()
                    .set_bit()
                    .rx_fifo_1_reg_0_en()
                    .set_bit()
            }),
            (FifoSel::Fifo8, true) => self.rb().sp_ctrl1().modify(|_, w| {
                w.rx_fifo_0_reg_0_en()
                    .set_bit()
                    .rx_fifo_0_reg_1_en()
                    .set_bit()
                    .rx_fifo_1_reg_0_en()
                    .set_bit()
                    .rx_fifo_1_reg_1_en()
                    .set_bit()
            }),
            (FifoSel::Fifo2, false) => self
                .rb()
                .sp_ctrl1()
                .modify(|_, w| w.rx_fifo_0_reg_0_en().clear_bit()),
            (FifoSel::Fifo4, false) => self.rb().sp_ctrl1().modify(|_, w| {
                w.rx_fifo_0_reg_0_en()
                    .clear_bit()
                    .rx_fifo_0_reg_1_en()
                    .clear_bit()
            }),
            (FifoSel::Fifo6, false) => self.rb().sp_ctrl1().modify(|_, w| {
                w.rx_fifo_0_reg_0_en()
                    .clear_bit()
                    .rx_fifo_0_reg_1_en()
                    .clear_bit()
                    .rx_fifo_1_reg_0_en()
                    .clear_bit()
            }),
            (FifoSel::Fifo8, false) => self.rb().sp_ctrl1().modify(|_, w| {
                w.rx_fifo_0_reg_0_en()
                    .clear_bit()
                    .rx_fifo_0_reg_1_en()
                    .clear_bit()
                    .rx_fifo_1_reg_0_en()
                    .clear_bit()
                    .rx_fifo_1_reg_1_en()
                    .clear_bit()
            }),
        };
    }

    // -----------------------------------------------------------------------
    // TX channel source and RX FIFO source routing
    // -----------------------------------------------------------------------

    /// Map a TX output channel to a FIFO sub-register source.
    pub fn tx_chn_src_sel(&self, tx_chn: TxChn, fifo_chn: TxFifoChn) {
        let v = fifo_chn as u8;
        self.rb().sp_direct_ctrl0().modify(|_, w| unsafe {
            match tx_chn {
                TxChn::Chn0 => w.tx_ch0_data_sel().bits(v),
                TxChn::Chn1 => w.tx_ch1_data_sel().bits(v),
                TxChn::Chn2 => w.tx_ch2_data_sel().bits(v),
                TxChn::Chn3 => w.tx_ch3_data_sel().bits(v),
                TxChn::Chn4 => w.tx_ch4_data_sel().bits(v),
                TxChn::Chn5 => w.tx_ch5_data_sel().bits(v),
                TxChn::Chn6 => w.tx_ch6_data_sel().bits(v),
                TxChn::Chn7 => w.tx_ch7_data_sel().bits(v),
            }
        });
    }

    /// Map a RX FIFO sub-register to an incoming RX channel.
    pub fn rx_fifo_src_sel(&self, fifo_chn: RxFifoChn, rx_chn: RxChn) {
        let v = rx_chn as u8;
        match fifo_chn {
            RxFifoChn::Fifo0Reg0L => self
                .rb()
                .sp_direct_ctrl3()
                .modify(|_, w| unsafe { w.rx_fifo_0_reg_0_l_sel().bits(v) }),
            RxFifoChn::Fifo0Reg0R => self
                .rb()
                .sp_direct_ctrl3()
                .modify(|_, w| unsafe { w.rx_fifo_0_reg_0_r_sel().bits(v) }),
            RxFifoChn::Fifo0Reg1L => self
                .rb()
                .sp_direct_ctrl3()
                .modify(|_, w| unsafe { w.rx_fifo_0_reg_1_l_sel().bits(v) }),
            RxFifoChn::Fifo0Reg1R => self
                .rb()
                .sp_direct_ctrl3()
                .modify(|_, w| unsafe { w.rx_fifo_0_reg_1_r_sel().bits(v) }),
            RxFifoChn::Fifo1Reg0L => self
                .rb()
                .sp_direct_ctrl4()
                .modify(|_, w| unsafe { w.rx_fifo_1_reg_0_l_sel().bits(v) }),
            RxFifoChn::Fifo1Reg0R => self
                .rb()
                .sp_direct_ctrl4()
                .modify(|_, w| unsafe { w.rx_fifo_1_reg_0_r_sel().bits(v) }),
            RxFifoChn::Fifo1Reg1L => self
                .rb()
                .sp_direct_ctrl4()
                .modify(|_, w| unsafe { w.rx_fifo_1_reg_1_l_sel().bits(v) }),
            RxFifoChn::Fifo1Reg1R => self
                .rb()
                .sp_direct_ctrl4()
                .modify(|_, w| unsafe { w.rx_fifo_1_reg_1_r_sel().bits(v) }),
        };
    }

    // -----------------------------------------------------------------------
    // Word-length accessors
    // -----------------------------------------------------------------------

    /// Set TX word (sample) length for both FIFO0 and FIFO1 paths.
    pub fn set_tx_word_len(&self, wl: WordLen) {
        let v = wl as u8;
        self.rb()
            .sp_ctrl0()
            .modify(|_, w| unsafe { w.sp_data_len_sel_tx_0().bits(v) });
        self.rb()
            .sp_direct_ctrl1()
            .modify(|_, w| unsafe { w.sp_data_len_sel_tx_1().bits(v) });
    }

    /// Set RX word (sample) length for both FIFO0 and FIFO1 paths.
    pub fn set_rx_word_len(&self, wl: WordLen) {
        let v = wl as u8;
        self.rb()
            .sp_format()
            .modify(|_, w| unsafe { w.sp_data_len_sel_rx_0().bits(v) });
        self.rb()
            .sp_direct_ctrl1()
            .modify(|_, w| unsafe { w.sp_data_len_sel_rx_1().bits(v) });
    }

    /// Read back current TX word length encoding (0–4).
    pub fn get_tx_word_len(&self) -> u8 {
        self.rb().sp_ctrl0().read().sp_data_len_sel_tx_0().bits()
    }

    /// Read back current RX word length encoding (0–4).
    pub fn get_rx_word_len(&self) -> u8 {
        self.rb().sp_format().read().sp_data_len_sel_rx_0().bits()
    }

    // -----------------------------------------------------------------------
    // Channel-length query
    // -----------------------------------------------------------------------

    /// Return TX channel length in bits (16/20/24/32).
    pub fn get_tx_chn_len(&self) -> u32 {
        match self.rb().sp_format().read().sp_ch_len_sel_tx().bits() {
            0 => 16,
            1 => 20,
            2 => 24,
            _ => 32,
        }
    }

    /// Return RX channel length in bits (16/20/24/32).
    pub fn get_rx_chn_len(&self) -> u32 {
        match self.rb().sp_format().read().sp_ch_len_sel_rx().bits() {
            0 => 16,
            1 => 20,
            2 => 24,
            _ => 32,
        }
    }

    // -----------------------------------------------------------------------
    // Mono/stereo and master/slave
    // -----------------------------------------------------------------------

    /// Select TX mono or stereo mode.
    pub fn set_mono_stereo(&self, ms: MonoStereo) {
        match ms {
            MonoStereo::Stereo => {
                self.rb()
                    .sp_ctrl0()
                    .modify(|_, w| w.sp_en_i2s_mono_tx_0().clear_bit());
            }
            MonoStereo::Mono => {
                self.rb()
                    .sp_ctrl0()
                    .modify(|_, w| w.sp_en_i2s_mono_tx_0().set_bit());
                // trx_same_ch lives in SP_FORMAT, not SP_CTRL0
                self.rb()
                    .sp_format()
                    .modify(|_, w| w.trx_same_ch().clear_bit());
            }
        }
    }

    /// Select master or slave clock mode.
    pub fn set_master_slave(&self, ms: MasterSlave) {
        match ms {
            MasterSlave::Master => self
                .rb()
                .sp_ctrl0()
                .modify(|_, w| w.slave_clk_sel().clear_bit().slave_data_sel().clear_bit()),
            MasterSlave::Slave => self
                .rb()
                .sp_ctrl0()
                .modify(|_, w| w.slave_clk_sel().set_bit().slave_data_sel().set_bit()),
        };
    }

    // -----------------------------------------------------------------------
    // DMA / DSP handshake mode
    // -----------------------------------------------------------------------

    /// Enable GDMA handshake (false = DSP mode, true = GDMA mode).
    pub fn dma_cmd(&self, enable: bool) {
        // dsp_ctl_mode=1 means DSP; dsp_ctl_mode=0 means GDMA
        self.rb()
            .sp_ctrl0()
            .modify(|_, w| w.dsp_ctl_mode().bit(!enable));
    }

    // -----------------------------------------------------------------------
    // Self-loopback
    // -----------------------------------------------------------------------

    /// Enable self-loopback mode (TX data loops back into RX path).
    pub fn set_self_lpbk(&self) {
        self.rb()
            .sp_ctrl0()
            .modify(|_, w| w.sp_loopback().set_bit());
    }

    // -----------------------------------------------------------------------
    // Data format
    // -----------------------------------------------------------------------

    /// Set TX data format.
    pub fn set_tx_data_format(&self, fmt: DataFormat) {
        self.rb()
            .sp_ctrl0()
            .modify(|_, w| unsafe { w.sp_data_format_sel_tx().bits(fmt as u8) });
    }

    /// Set RX data format (also mirrors to TX data format register per C driver).
    pub fn set_rx_data_format(&self, fmt: DataFormat) {
        self.rb()
            .sp_format()
            .modify(|_, w| unsafe { w.sp_data_format_sel_rx().bits(fmt as u8) });
        self.rb()
            .sp_ctrl0()
            .modify(|_, w| unsafe { w.sp_data_format_sel_tx().bits(fmt as u8) });
    }

    // -----------------------------------------------------------------------
    // Pinmux
    // -----------------------------------------------------------------------

    /// Configure DOUT/DIN pin-mux for an I2S interface.
    ///
    /// Uses a direct volatile read-modify-write on the PINMUX register block
    /// because PINMUX is not in the sport0_reg PAC module.
    pub fn set_pin_mux(&self, i2s: I2sSel, func: PinFunc) {
        // REG_I2S_CTRL (sysreg_pad.h : REG_I2S_CTRL) — accessed via PinmuxReg PAC.
        let reg = unsafe { &*PinmuxReg::PTR }.reg_i2s_ctrl();
        let mut tmp = reg.read().bits();
        match i2s {
            I2sSel::I2S0 => match func {
                PinFunc::Dout3 => tmp |= PAD_BIT_SP0_DIO0_MUXSEL,
                PinFunc::Dout2 => tmp |= PAD_BIT_SP0_DIO1_MUXSEL,
                PinFunc::Dout1 => tmp |= PAD_BIT_SP0_DIO2_MUXSEL,
                PinFunc::Dout0 => tmp |= PAD_BIT_SP0_DIO3_MUXSEL,
                PinFunc::Din0 => tmp &= !PAD_BIT_SP0_DIO0_MUXSEL,
                PinFunc::Din1 => tmp &= !PAD_BIT_SP0_DIO1_MUXSEL,
                PinFunc::Din2 => tmp &= !PAD_BIT_SP0_DIO2_MUXSEL,
                PinFunc::Din3 => tmp &= !PAD_BIT_SP0_DIO3_MUXSEL,
            },
            I2sSel::I2S1 => match func {
                PinFunc::Dout3 => tmp |= PAD_BIT_SP1_DIO0_MUXSEL,
                PinFunc::Dout2 => tmp |= PAD_BIT_SP1_DIO1_MUXSEL,
                PinFunc::Dout1 => tmp |= PAD_BIT_SP1_DIO2_MUXSEL,
                PinFunc::Dout0 => tmp |= PAD_BIT_SP1_DIO3_MUXSEL,
                PinFunc::Din0 => tmp &= !PAD_BIT_SP1_DIO0_MUXSEL,
                PinFunc::Din1 => tmp &= !PAD_BIT_SP1_DIO1_MUXSEL,
                PinFunc::Din2 => tmp &= !PAD_BIT_SP1_DIO2_MUXSEL,
                PinFunc::Din3 => tmp &= !PAD_BIT_SP1_DIO3_MUXSEL,
            },
        }
        reg.write(|w| unsafe { w.bits(tmp) });
    }

    // -----------------------------------------------------------------------
    // Phase counter and counter interrupts
    // -----------------------------------------------------------------------

    /// Enable or disable phase-counter divide-by-64 mode.
    pub fn set_phase_counter_div64(&self, enable: bool) {
        self.rb()
            .sp_rx_counter1()
            .modify(|_, w| w.fs_phase_en_d64().bit(enable));
    }

    /// Enable or disable the exact phase counter (counted by SPORT clock).
    pub fn set_exact_phase_counter(&self, enable: bool) {
        self.rb()
            .sp_ctrl2()
            .modify(|_, w| w.ex_phase_cnt_en().bit(enable));
    }

    /// Enable or disable WIFI TSFT trigger for audio stream start.
    pub fn set_wifi_tsft_trig(&self, enable: bool) {
        self.rb()
            .sp_tx_lrclk()
            .modify(|_, w| w.wifi_trig_audio_en().bit(enable));
    }

    /// Enable or disable TX counter interrupt.
    pub fn set_tx_counter(&self, enable: bool) {
        self.rb()
            .sp_rx_lrclk()
            .modify(|_, w| w.en_tx_sport_interrupt().bit(enable));
    }

    /// Set TX counter compare value (interrupt fires when counter == comp_val).
    pub fn set_tx_counter_comp_val(&self, comp_val: u32) {
        self.rb()
            .sp_rx_lrclk()
            .modify(|_, w| unsafe { w.tx_sport_compare_val().bits(comp_val) });
    }

    /// Clear the TX counter interrupt (W1C via volatile write).
    ///
    /// Avoids the PAC read-modify-write because `clr_tx_sport_rdy` is W1C and
    /// the PAC marks it write-only — a naïve RMW would inadvertently clear
    /// `en_fs_phase_latch` (bit 29) which is also write-only.
    pub fn clear_tx_counter_irq(&self) {
        // Read current readable bits, set clr_tx_sport_rdy (bit31), keep en_tsft_trig_latch
        // and en_tx_sport_interrupt as-is; leave en_fs_phase_latch=0 to avoid side-effects.
        let cur = self.rb().sp_rx_lrclk().read().bits();
        // Preserve readable/writable fields; assert bit31 (clr_tx_sport_rdy), clear bit29
        let val = (cur & 0x5FFF_FFFF) | (1 << 31);
        unsafe {
            self.rb().sp_rx_lrclk().as_ptr().write_volatile(val);
        }
    }

    /// Latch current phase values (W1C `en_fs_phase_latch`, bit 29).
    /// Preserves `clr_tx_sport_rdy` at 0 to avoid clearing TX IRQ.
    pub fn set_phase_latch(&self) {
        let cur = self.rb().sp_rx_lrclk().read().bits();
        let val = (cur & 0xBFFF_FFFF) | (1 << 29);
        unsafe {
            self.rb().sp_rx_lrclk().as_ptr().write_volatile(val);
        }
    }

    /// Enable or disable WIFI TSFT trigger for phase latch.
    pub fn set_wifi_tsft_trig_latch(&self, enable: bool) {
        self.rb()
            .sp_rx_lrclk()
            .modify(|_, w| w.en_tsft_trig_latch().bit(enable));
    }

    /// Read current TX counter value.
    pub fn get_tx_counter_val(&self) -> u32 {
        self.rb().sp_dsp_counter().read().tx_sport_counter().bits()
    }

    /// Read TX phase value (combines 5-bit field + overflow bit from SP_RX_LRCLK).
    pub fn get_tx_phase_val(&self) -> u32 {
        let phase1 = self.rb().sp_dsp_counter().read().tx_fs_phase_rpt().bits() as u32;
        let phase2 = self.rb().sp_rx_lrclk().read().tx_fs_phase_rpt_b5().bit() as u32;
        phase1 + phase2 * 32
    }

    /// Read TX exact phase value (counted by SPORT clock).
    pub fn get_tx_exact_phase_val(&self) -> u32 {
        self.rb()
            .sp_tx_ex_phase_rpt()
            .read()
            .tx_ex_phase_rpt()
            .bits() as u32
    }

    /// Enable or disable RX counter interrupt.
    pub fn set_rx_counter(&self, enable: bool) {
        self.rb()
            .sp_rx_counter1()
            .modify(|_, w| w.en_rx_sport_interrupt().bit(enable));
    }

    /// Set RX counter compare value.
    pub fn set_rx_counter_comp_val(&self, comp_val: u32) {
        self.rb()
            .sp_rx_counter1()
            .modify(|_, w| unsafe { w.rx_sport_compare_val().bits(comp_val) });
    }

    /// Clear the RX counter interrupt (W1C bit 31 of SP_RX_COUNTER1).
    pub fn clear_rx_counter_irq(&self) {
        let cur = self.rb().sp_rx_counter1().read().bits();
        let val = cur | (1u32 << 31);
        unsafe {
            self.rb().sp_rx_counter1().as_ptr().write_volatile(val);
        }
    }

    /// Read current RX counter value. Also latches phase values first.
    pub fn get_rx_counter_val(&self) -> u32 {
        // Latch phase before reading counter (mirrors C driver behaviour)
        let cur = self.rb().sp_rx_lrclk().read().bits();
        let val = (cur & 0xBFFF_FFFF) | (1 << 29);
        unsafe { self.rb().sp_rx_lrclk().as_ptr().write_volatile(val) };
        self.rb().sp_rx_counter2().read().rx_sport_counter().bits()
    }

    /// Read RX phase value (combines 5-bit field + overflow bit).
    pub fn get_rx_phase_val(&self) -> u32 {
        let phase1 = self.rb().sp_rx_counter2().read().rx_fs_phase_rpt().bits() as u32;
        let phase2 = self.rb().sp_rx_counter1().read().rx_fs_phase_rpt_b5().bit() as u32;
        phase1 + phase2 * 32
    }

    /// Read RX exact phase value (counted by SPORT clock).
    pub fn get_rx_exact_phase_val(&self) -> u32 {
        self.rb()
            .sp_rx_ex_phase_rpt()
            .read()
            .rx_ex_phase_rpt()
            .bits() as u32
    }

    // -----------------------------------------------------------------------
    // LRCLK start / stop interrupts
    // -----------------------------------------------------------------------

    /// Set LRCLK start interrupt trigger edge.
    pub fn set_lrclk_start_edge(&self, edge: LrclkEdge) {
        let pol = matches!(edge, LrclkEdge::Rising);
        self.rb()
            .sp_ctrl2()
            .modify(|_, w| w.lrc_start_pol().bit(pol));
    }

    /// Enable or disable LRCLK start interrupt.
    pub fn set_lrclk_start_irq(&self, enable: bool) {
        self.rb()
            .sp_ctrl2()
            .modify(|_, w| w.lrc_start_intr_mask().bit(enable));
    }

    /// Clear LRCLK start interrupt (W1C in SP_INT_CTRL bit 14).
    pub fn clear_lrclk_start_irq(&self) {
        let cur = self.rb().sp_int_ctrl().read().bits();
        let val = cur | (1u32 << 14);
        unsafe {
            self.rb().sp_int_ctrl().as_ptr().write_volatile(val);
        }
    }

    /// Enable or disable LRCLK stop interrupt.
    pub fn set_lrclk_stop_irq(&self, enable: bool) {
        self.rb()
            .sp_ctrl2()
            .modify(|_, w| w.lrc_stop_intr_mask().bit(enable));
    }

    /// Clear LRCLK stop interrupt (W1C in SP_INT_CTRL bit 15).
    pub fn clear_lrclk_stop_irq(&self) {
        let cur = self.rb().sp_int_ctrl().read().bits();
        let val = cur | (1u32 << 15);
        unsafe {
            self.rb().sp_int_ctrl().as_ptr().write_volatile(val);
        }
    }

    /// Set LRCLK stop threshold.
    pub fn set_lrclk_stop_threshold(&self, threshold: u16) {
        self.rb()
            .sp_ctrl2()
            .modify(|_, w| unsafe { w.lrc_stop_thresh().bits(threshold) });
    }

    /// Read LRCLK stop threshold.
    pub fn get_lrclk_stop_threshold(&self) -> u16 {
        self.rb().sp_ctrl2().read().lrc_stop_thresh().bits()
    }

    // -----------------------------------------------------------------------
    // Full initialisation
    // -----------------------------------------------------------------------

    /// Initialise SPORT for TX according to `cfg`.
    ///
    /// Mirrors `AUDIO_SP_Init(index, SP_DIR_TX, ...)` from the C driver.
    /// Returns `None` if registration fails (BCLK conflict).
    pub fn init_tx(self, cfg: &SpInitConfig) -> Option<Sport<Unconfigured>> {
        if !self.register(Direction::Tx, cfg) {
            return None;
        }

        let rb = self.rb();

        // Set SP_REG_MUX = 0xFFFFFFFF (only needed for non-SPORT0 devices;
        // SPORT0 base is the canonical address so the condition is false, but
        // we follow the C driver logic faithfully).

        // Enable MCLK
        self.set_mclk(true);

        // Disable TX first
        self.tx_start(false);

        let wl = cfg.word_len as u8;
        let fmt = cfg.data_format as u8;
        let ch = cfg.chan_sel as u8;
        let cl = cfg.chan_len as u8;
        let tdm = cfg.tdm as u8;

        // Clear trx_same_length
        rb.sp_format()
            .modify(|_, w| w.trx_same_length().clear_bit());

        // CTRL0: data format, word len, chan sel
        rb.sp_ctrl0().modify(|_, w| unsafe {
            w.sp_data_len_sel_tx_0()
                .bits(wl)
                .sp_data_format_sel_tx()
                .bits(fmt)
                .sp_sel_i2s_tx_ch()
                .bits(ch)
        });

        // Mono/stereo
        let mono = matches!(cfg.mono_stereo, MonoStereo::Mono);
        rb.sp_ctrl0()
            .modify(|_, w| w.sp_en_i2s_mono_tx_0().bit(mono));
        rb.sp_direct_ctrl1()
            .modify(|_, w| w.sp_en_i2s_mono_tx_1().bit(mono));

        // Channel length, clear trx_same_ch_len and trx_same_ch
        rb.sp_format().modify(|_, w| unsafe {
            w.sp_ch_len_sel_tx()
                .bits(cl)
                .trx_same_ch_len()
                .clear_bit()
                .trx_same_ch()
                .clear_bit()
        });

        // FIFO1 word len
        rb.sp_direct_ctrl1()
            .modify(|_, w| unsafe { w.sp_data_len_sel_tx_1().bits(wl) });

        // TDM mode
        rb.sp_ctrl0()
            .modify(|_, w| unsafe { w.sp_tdm_mode_sel_tx().bits(tdm) });
        rb.sp_ctrl1().modify(|_, w| w.multi_io_en_tx().clear_bit());

        // Clock dividers
        self.set_tx_clk_div(cfg.clk_src, cfg.sample_rate, cfg.tdm, cfg.chan_len);

        // TX FIFO enable
        self.tx_set_fifo(cfg.fifo, true);

        // Multi-IO
        if cfg.multi_io {
            rb.sp_ctrl1().modify(|_, w| w.multi_io_en_tx().set_bit());
            self.set_tx_clk_div(cfg.clk_src, cfg.sample_rate, TdmMode::NoTdm, cfg.chan_len);
        } else {
            rb.sp_ctrl1().modify(|_, w| w.multi_io_en_tx().clear_bit());
        }

        // DMA burst size = 8
        rb.sp_tx_lrclk()
            .modify(|_, w| unsafe { w.txdma_burstsize().bits(8) });

        if cfg.fix_bclk {
            rb.sp_format().modify(|_, w| {
                w.fixed_bclk()
                    .set_bit()
                    .fixed_bclk_sel()
                    .set_bit()
                    .trx_same_fs()
                    .clear_bit()
            });
        }

        Some(self)
    }

    /// Initialise SPORT for RX according to `cfg`.
    ///
    /// Mirrors `AUDIO_SP_Init(index, SP_DIR_RX, ...)` from the C driver.
    /// Returns `None` if registration fails (BCLK conflict).
    pub fn init_rx(self, cfg: &SpInitConfig) -> Option<Sport<Unconfigured>> {
        if !self.register(Direction::Rx, cfg) {
            return None;
        }

        let rb = self.rb();

        self.set_mclk(true);
        self.rx_start(false);

        let wl = cfg.word_len as u8;
        let fmt = cfg.data_format as u8;
        let ch = cfg.chan_sel as u8;
        let cl = cfg.chan_len as u8;
        let tdm = cfg.tdm as u8;

        // RX channel sel and TDM (clear both, then set)
        rb.sp_ctrl0()
            .modify(|_, w| unsafe { w.sp_sel_i2s_rx_ch().bits(ch).sp_tdm_mode_sel_rx().bits(0) });

        // Mono/stereo
        let mono = matches!(cfg.mono_stereo, MonoStereo::Mono);
        rb.sp_format()
            .modify(|_, w| w.sp_en_i2s_mono_rx_0().bit(mono));
        rb.sp_direct_ctrl1()
            .modify(|_, w| w.sp_en_i2s_mono_rx_1().bit(mono));

        // Channel len, data format, word len for RX; clear trx flags
        rb.sp_format().modify(|_, w| unsafe {
            w.sp_ch_len_sel_rx()
                .bits(cl)
                .sp_data_format_sel_rx()
                .bits(fmt)
                .sp_data_len_sel_rx_0()
                .bits(wl)
                .trx_same_ch_len()
                .clear_bit()
                .trx_same_ch()
                .clear_bit()
                .trx_same_length()
                .clear_bit()
        });

        // Also mirror data format to TX path (C driver does this)
        rb.sp_ctrl0()
            .modify(|_, w| unsafe { w.sp_data_format_sel_tx().bits(fmt) });

        // FIFO1 word len
        rb.sp_direct_ctrl1()
            .modify(|_, w| unsafe { w.sp_data_len_sel_rx_1().bits(wl) });

        // TDM mode for RX
        rb.sp_ctrl0()
            .modify(|_, w| unsafe { w.sp_tdm_mode_sel_rx().bits(tdm) });
        rb.sp_ctrl1().modify(|_, w| w.multi_io_en_rx().clear_bit());

        // Clock dividers
        if cfg.fix_bclk {
            rb.sp_format().modify(|_, w| {
                w.fixed_bclk()
                    .set_bit()
                    .fixed_bclk_sel()
                    .set_bit()
                    .trx_same_fs()
                    .clear_bit()
            });
            self.set_rx_clk_div(cfg.clk_src, cfg.sample_rate, cfg.tdm, cfg.chan_len);
        } else {
            self.set_tx_clk_div(cfg.clk_src, cfg.sample_rate, cfg.tdm, cfg.chan_len);
            self.set_rx_clk_div(cfg.clk_src, cfg.sample_rate, cfg.tdm, cfg.chan_len);
        }

        // RX FIFO enable
        self.rx_set_fifo(cfg.fifo, true);

        // Multi-IO
        if cfg.multi_io {
            if cfg.fix_bclk {
                rb.sp_format().modify(|_, w| {
                    w.fixed_bclk()
                        .set_bit()
                        .fixed_bclk_sel()
                        .set_bit()
                        .trx_same_fs()
                        .clear_bit()
                });
                self.set_rx_clk_div(cfg.clk_src, cfg.sample_rate, TdmMode::NoTdm, cfg.chan_len);
            } else {
                self.set_tx_clk_div(cfg.clk_src, cfg.sample_rate, TdmMode::NoTdm, cfg.chan_len);
                self.set_rx_clk_div(cfg.clk_src, cfg.sample_rate, TdmMode::NoTdm, cfg.chan_len);
            }
            rb.sp_ctrl1().modify(|_, w| w.multi_io_en_rx().set_bit());
        } else {
            rb.sp_ctrl1().modify(|_, w| w.multi_io_en_rx().clear_bit());
        }

        // DMA burst size = 8
        rb.sp_tx_lrclk()
            .modify(|_, w| unsafe { w.rxdma_busrtsize().bits(8) });

        Some(self)
    }

    // -----------------------------------------------------------------------
    // De-init
    // -----------------------------------------------------------------------

    /// Stop and unregister TX.
    pub fn deinit_tx(&self) {
        self.unregister(Direction::Tx);
        self.tx_start(false);
    }

    /// Stop and unregister RX.
    pub fn deinit_rx(&self) {
        self.unregister(Direction::Rx);
        self.rx_start(false);
    }
}

// ---------------------------------------------------------------------------
// Helper: compute MI / NI for a given clock, sample-rate, TDM, channel-length
// ---------------------------------------------------------------------------

fn compute_mi_ni(clock: u32, sr: u32, tdm: TdmMode, chan_len: ChanLen) -> (u32, u32) {
    let cl = chan_len.bits_value();
    let tdm_u = (tdm as u32) + 1;

    if clock.is_multiple_of(40_000_000) {
        if sr.is_multiple_of(4_000) {
            let mi = 1250u32;
            let ni = (cl / 4) * tdm_u * (sr / 4_000);
            (mi, ni)
        } else {
            // 44.1k family
            if cl == 16 || cl == 32 {
                let mi = 50_000u32;
                let ni = (441 * (cl / 4) * tdm_u * (sr / 11_025)) / 4;
                (mi, ni)
            } else if cl == 20 {
                let mi = 40_000u32;
                let ni = (441 * (cl / 4) * tdm_u * (sr / 11_025)) / 5;
                (mi, ni)
            } else {
                let mi = 60_000u32;
                let ni = (441 * 3 * (cl / 4) * tdm_u * (sr / 11_025)) / 10;
                (mi, ni)
            }
        }
    } else {
        let channel_count = tdm_u * 2;
        let bclk = channel_count * cl * sr;
        let mut ni = 1u32;
        let mut mi = 0u32;
        if clock.wrapping_mul(ni).is_multiple_of(bclk) {
            mi = clock * ni / bclk;
        } else {
            for i in 1u32..0x7FFF {
                if clock.wrapping_mul(i) % bclk == 0 {
                    ni = i;
                    mi = clock * ni / bclk;
                    break;
                }
            }
        }
        (mi, ni)
    }
}
