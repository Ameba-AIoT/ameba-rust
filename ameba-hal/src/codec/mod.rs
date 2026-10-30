//! RTL8721F Audio Codec driver — idiomatic `#![no_std]` Rust, PAC-based.
//!
//! # Hardware summary
//! - Base address  : 0x4100_a000 (AUDIO_REG, non-secure)
//! - Channels      : 2 ADC channels (ADCHN1 / ADCHN2)
//! - DMIC          : single clock generator (dmic1_clk), 2 mic sources
//! - EQ            : 5 biquad bands per ADC channel
//! - Supported path: ADC / capture only (no DAC in this file)
//!
//! # Usage sketch
//! ```ignore
//! use ameba_codec::{Codec, I2sInitParams, AdcIdx, AdcChn, SampleRateSrc};
//!
//! let codec = unsafe { Codec::steal() };
//! let mut params = I2sInitParams::default();
//! codec.record(0 /* I2S0 */, RecordType::Dmic, &params);
//! ```

use rtl8721f_pac::{AudioReg, audio_reg};

// ────────────────────────────────────────────────────────────────────────────
// ROM / FFI bindings — none needed for this file.
// Present as an empty private module to satisfy the project convention.
// ────────────────────────────────────────────────────────────────────────────
mod sys {}

// ────────────────────────────────────────────────────────────────────────────
// Constants
// ────────────────────────────────────────────────────────────────────────────

// ────────────────────────────────────────────────────────────────────────────
// Public enumerations
// ────────────────────────────────────────────────────────────────────────────

/// I2S instance selector (only I2S0 exists on RTL8721F audio codec).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2sSel {
    I2s0 = 0,
}

/// ADC channel index (0-based internally; matches PAC `adc_0_*` / `adc_1_*`).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AdcIdx {
    /// ADC channel 0  (C: `ADC1`)
    Adc0 = 0,
    /// ADC channel 1  (C: `ADC2`)
    Adc1 = 1,
}

/// ADC FIFO / clock channel index.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AdcChn {
    /// Channel 0  (C: `ADCHN1`)
    Chn0 = 0,
    /// Channel 1  (C: `ADCHN2`)
    Chn1 = 1,
}

/// Sample-rate source selector (SOURCE0 / SOURCE1).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SampleRateSrc {
    Source0 = 0,
    Source1 = 1,
    Source2 = 2,
}

/// DMIC clock frequency selection.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DmicClk {
    /// 5 MHz
    Dmic5M = 0,
    /// 2.5 MHz
    Dmic2p5M = 1,
    /// 1.25 MHz
    Dmic1p25M = 2,
    /// 625 kHz
    Dmic625K = 3,
    /// 312.5 kHz
    Dmic312p5K = 4,
    /// 769.2 kHz
    Dmic769p2K = 7,
}

/// DMIC microphone source number.
///
/// Register field encoding per PAC:
///   0 = MIC 1 rising edge
///   1 = MIC 1 falling edge
///   2 = MIC 2 rising edge
///   3 = MIC 2 falling edge
///
/// The C driver passes `dmic_num - 1` where `DMIC1 = 1` and `DMIC2 = 2`,
/// so DMIC1 → value 0 and DMIC2 → value 1 (MIC 1 falling). The C code
/// therefore routes ADCHN1→DMIC1 (MIC 1 rising) and ADCHN2→DMIC2
/// (MIC 1 falling). The Rust enum mirrors that exact mapping.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DmicSrc {
    /// MIC 1 rising edge  (C: `DMIC1`, hw value = 0)
    Dmic1 = 0,
    /// MIC 1 falling edge (C: `DMIC2`, hw value = 1)
    Dmic2 = 1,
}

/// ADC input type for mix-mute.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AdcInputType {
    Dmic,
}

/// I2S RX TDM mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2sTdmMode {
    /// No TDM — 2 channels  (C: `I2S_NOTDM`)
    NoTdm = 0,
    /// 4 channels TDM
    Tdm4Ch = 1,
}

/// I2S word-clock (LRCK) channel length.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ChannelLen {
    /// 16-bit channel
    Cl16 = 0,
    /// 32-bit channel
    Cl32 = 1,
    /// 24-bit channel
    Cl24 = 2,
    /// 8-bit channel
    Cl8 = 3,
}

/// I2S sample word length.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WordLen {
    /// 16-bit samples
    Wl16 = 0,
    /// 24-bit samples
    Wl24 = 2,
    /// 8-bit samples
    Wl8 = 3,
}

/// I2S data format.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DataFormat {
    /// Standard I2S
    I2s = 0,
    /// Left-justified
    LeftJ = 1,
    /// PCM mode A
    PcmA = 2,
    /// PCM mode B
    PcmB = 3,
}

/// I2S master clock source.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum I2sMasterSrc {
    InternalSport,
    ExternalI2s,
}

/// ADC zero-detection function.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ZdetFunc {
    /// Immediate change
    Immediate = 0,
    /// Zero detection + immediate change
    ZdetImmediate = 1,
    /// Zero detection + step
    ZdetStep = 2,
    /// Zero detection + timeout step
    ZdetTimeoutStep = 3,
}

/// ADC zero-detection timeout.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ZdetTimeout {
    /// 1024 * 16 samples
    S1024x16 = 0,
    /// 1024 * 32 samples
    S1024x32 = 1,
    /// 1024 * 64 samples
    S1024x64 = 2,
    /// 64 samples
    S64 = 3,
}

/// EQ band selector (5 bands: 0–4).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EqBand {
    Band0 = 0,
    Band1 = 1,
    Band2 = 2,
    Band3 = 3,
    Band4 = 4,
}

/// Record application type.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RecordType {
    /// Analog microphone
    Amic,
    /// Digital microphone
    Dmic,
}

// ────────────────────────────────────────────────────────────────────────────
// I2S initialisation parameter struct
// ────────────────────────────────────────────────────────────────────────────

/// I2S initialisation parameters (C: `I2S_InitTypeDef`).
///
/// All fields default to the values used by `AUDIO_CODEC_I2S_StructInit()`.
#[derive(Copy, Clone, Debug)]
pub struct I2sInitParams {
    /// TX sample rate (encoded as a 4-bit HW value)
    pub tx_sr: u8,
    /// TX word length
    pub tx_word_len: WordLen,
    /// TX channel length
    pub tx_ch_len: ChannelLen,
    /// TX data format
    pub tx_fmt: DataFormat,
    /// RX sample rate (encoded as a 4-bit HW value)
    pub rx_sr: u8,
    /// RX word length
    pub rx_word_len: WordLen,
    /// RX channel length
    pub rx_ch_len: ChannelLen,
    /// RX data format
    pub rx_fmt: DataFormat,
    /// RX TDM mode
    pub rx_tdm: I2sTdmMode,
}

/// Sample-rate hardware code constants (C: `SR_48K`, `SR_96K`, etc.).
pub mod sr {
    pub const SR_48K: u8 = 0;
    pub const SR_96K: u8 = 1;
    pub const SR_192K: u8 = 2;
    pub const SR_32K: u8 = 3;
    pub const SR_176P4K: u8 = 4;
    pub const SR_16K: u8 = 5;
    pub const SR_8K: u8 = 7;
    pub const SR_44P1K: u8 = 8;
    pub const SR_88P2K: u8 = 9;
    pub const SR_24K: u8 = 0xa;
    pub const SR_12K: u8 = 0xb;
    pub const SR_22P05K: u8 = 0xc;
    pub const SR_11P025K: u8 = 0xd;
}

impl Default for I2sInitParams {
    /// Matches `AUDIO_CODEC_I2S_StructInit()` defaults.
    fn default() -> Self {
        Self {
            tx_sr: sr::SR_48K,
            tx_word_len: WordLen::Wl16,
            tx_ch_len: ChannelLen::Cl32,
            tx_fmt: DataFormat::I2s,
            rx_sr: sr::SR_48K,
            rx_word_len: WordLen::Wl16,
            rx_ch_len: ChannelLen::Cl32,
            rx_fmt: DataFormat::I2s,
            rx_tdm: I2sTdmMode::NoTdm,
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// EQ filter coefficient struct
// ────────────────────────────────────────────────────────────────────────────

/// Biquad EQ filter coefficients in 4.25 fixed-point, 29-bit 2's complement
/// (C: `CODEC_EQFilterCoef`).
#[derive(Copy, Clone, Debug)]
pub struct EqFilterCoef {
    /// H0 coefficient (bits 0:28)
    pub h0: u32,
    /// B1 coefficient (bits 0:28)
    pub b1: u32,
    /// B2 coefficient (bits 0:28)
    pub b2: u32,
    /// A1 coefficient (bits 0:28)
    pub a1: u32,
    /// A2 coefficient (bits 0:28)
    pub a2: u32,
}

// ────────────────────────────────────────────────────────────────────────────
// Driver handle
// ────────────────────────────────────────────────────────────────────────────

/// Audio Codec driver handle.
///
/// Obtain via [`Codec::steal()`] — see safety note.
pub struct Codec {
    rb: &'static audio_reg::RegisterBlock,
}

impl Codec {
    /// Construct a `Codec` from the fixed NS base address without taking
    /// ownership through the PAC singleton mechanism.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - Only one `Codec` instance is live at any time (or all accesses are
    ///   serialised externally).
    /// - The peripheral clock is enabled before calling any method.
    #[inline]
    pub unsafe fn steal() -> Self {
        unsafe {
            Self {
                rb: &*AudioReg::PTR,
            }
        }
    }

    // ────────────────────────────────────────────────────────────────────────
    // Audio IP enable / disable
    // ────────────────────────────────────────────────────────────────────────

    /// Enable or disable the audio codec IP.
    ///
    /// C: `AUDIO_CODEC_SetAudioIP(newstate)`
    pub fn set_audio_ip(&self, enable: bool) {
        self.rb
            .audio_control_0()
            .modify(|_, w| w.audio_ip_en().bit(enable));
    }

    // ────────────────────────────────────────────────────────────────────────
    // I2S IP
    // ────────────────────────────────────────────────────────────────────────

    /// Enable or disable the I2S IP (de-assert / assert reset).
    ///
    /// C: `AUDIO_CODEC_SetI2SIP(i2s_sel, newstate)`
    pub fn set_i2s_ip(&self, _i2s_sel: I2sSel, enable: bool) {
        self.rb
            .i2s_0_control()
            .modify(|_, w| w.i2s_0_rst_n_reg().bit(enable));
    }

    /// Select I2S master source.
    ///
    /// C: `AUDIO_CODEC_SetI2SSRC(i2s_sel, src)`
    pub fn set_i2s_src(&self, _i2s_sel: I2sSel, src: I2sMasterSrc) {
        let external = matches!(src, I2sMasterSrc::ExternalI2s);
        self.rb
            .i2s_0_control()
            .modify(|_, w| w.i2s_0_master_sel().bit(external));
    }

    /// Select I2S RX TDM mode and configure channel routing for NoTdm.
    ///
    /// C: `AUDIO_CODEC_SetI2SRXTDM(i2s_sel, tdmmode)`
    pub fn set_i2s_rx_tdm(&self, _i2s_sel: I2sSel, tdm: I2sTdmMode) {
        // Always selects NoTdm (0) per the C source which only handles I2S_NOTDM.
        let mode_val = tdm as u8;
        self.rb
            .i2s_0_control()
            .modify(|_, w| unsafe { w.i2s_0_tdm_mode_rx().bits(mode_val) });
        // CH0 -> source CH0, CH1 -> source CH1, both enabled (disable=0)
        self.rb.i2s_0_control_1().modify(|_, w| unsafe {
            w.i2s_0_data_ch0_sel_rx()
                .bits(0)
                .i2s_0_data_ch1_sel_rx()
                .bits(1)
                .i2s_0_data_ch0_rx_disable()
                .clear_bit()
                .i2s_0_data_ch1_rx_disable()
                .clear_bit()
        });
    }

    // ────────────────────────────────────────────────────────────────────────
    // I2S parameters
    // ────────────────────────────────────────────────────────────────────────

    /// Set I2S ADC path parameters from an [`I2sInitParams`] struct.
    ///
    /// C: `AUDIO_CODEC_SetI2SParameters(i2s_sel, ADCPATH, I2S_Struct)`.
    /// Only the ADC (RX) path is supported; passing a TX-only path is a no-op.
    pub fn set_i2s_parameters(&self, _i2s_sel: I2sSel, params: &I2sInitParams) {
        self.rb.i2s_0_control().modify(|_, w| unsafe {
            w.i2s_0_ch_len_sel_rx()
                .bits(params.rx_ch_len as u8)
                .i2s_0_data_len_sel_rx()
                .bits(params.rx_word_len as u8)
                .i2s_0_data_format_sel_rx()
                .bits(params.rx_fmt as u8)
        });
        self.set_i2s_rx_tdm(_i2s_sel, params.rx_tdm);
    }

    // ────────────────────────────────────────────────────────────────────────
    // ADC sample-rate source
    // ────────────────────────────────────────────────────────────────────────

    /// Set ADC sample-rate source and rate for both channels.
    ///
    /// C: `AUDIO_CODEC_SetADCSRSrc(src, sr)`
    pub fn set_adc_sr_src(&self, src: SampleRateSrc, sr: u8) {
        let src_val = src as u8;
        self.rb.clock_control_5().modify(|_, w| unsafe {
            w.adc_0_fs_src_sel()
                .bits(src_val)
                .adc_1_fs_src_sel()
                .bits(src_val)
        });
        match src {
            SampleRateSrc::Source0 => {
                self.rb
                    .clock_control_4()
                    .modify(|_, w| unsafe { w.sample_rate_0().bits(sr) });
            }
            SampleRateSrc::Source1 | SampleRateSrc::Source2 => {
                self.rb
                    .clock_control_4()
                    .modify(|_, w| unsafe { w.sample_rate_1().bits(sr) });
            }
        }
    }

    // ────────────────────────────────────────────────────────────────────────
    // ADC channel clock enable
    // ────────────────────────────────────────────────────────────────────────

    /// Enable ADC FIFO clocks for channels indicated by a bitmask.
    ///
    /// Bit 0 = channel 0 (ADCHN1), bit 1 = channel 1 (ADCHN2).
    ///
    /// C: `AUDIO_CODEC_EnableADCFifoForMask(mask)`
    pub fn enable_adc_fifo_mask(&self, mask: u32) {
        self.rb.clock_control_1().modify(|_, w| {
            let mut w = w;
            if mask & 1 != 0 {
                w = w.ad_0_fifo_en().set_bit();
            }
            if mask & 2 != 0 {
                w = w.ad_1_fifo_en().set_bit();
            }
            w
        });
    }

    /// Enable or disable an individual ADC FIFO channel clock.
    ///
    /// C: `AUDIO_CODEC_EnableADCFifo(ad_chn, newstate)`
    pub fn enable_adc_fifo(&self, chn: AdcChn, enable: bool) {
        match chn {
            AdcChn::Chn0 => self
                .rb
                .clock_control_1()
                .modify(|_, w| w.ad_0_fifo_en().bit(enable)),
            AdcChn::Chn1 => self
                .rb
                .clock_control_1()
                .modify(|_, w| w.ad_1_fifo_en().bit(enable)),
        };
    }

    /// Enable ADC channel clocks for channels indicated by a bitmask.
    ///
    /// Bit 0 = channel 0 (ADCHN1), bit 1 = channel 1 (ADCHN2).
    ///
    /// C: `AUDIO_CODEC_EnableADCForMask(mask)`
    pub fn enable_adc_mask(&self, mask: u32) {
        self.rb.clock_control_1().modify(|_, w| {
            let mut w = w;
            if mask & 1 != 0 {
                w = w.ad_0_en().set_bit();
            }
            if mask & 2 != 0 {
                w = w.ad_1_en().set_bit();
            }
            w
        });
    }

    /// Enable or disable an individual ADC channel clock.
    ///
    /// C: `AUDIO_CODEC_EnableADC(ad_chn, newstate)`
    pub fn enable_adc(&self, chn: AdcChn, enable: bool) {
        match chn {
            AdcChn::Chn0 => self
                .rb
                .clock_control_1()
                .modify(|_, w| w.ad_0_en().bit(enable)),
            AdcChn::Chn1 => self
                .rb
                .clock_control_1()
                .modify(|_, w| w.ad_1_en().bit(enable)),
        };
    }

    // ────────────────────────────────────────────────────────────────────────
    // ADC digital volume
    // ────────────────────────────────────────────────────────────────────────

    /// Set ADC digital volume gain.
    ///
    /// `gain`: 0x00 = −17.625 dB, 0x2f = 0 dB, 0xaf = 48 dB (0.375 dB/step).
    ///
    /// C: `AUDIO_CODEC_SetADCVolume(adc_sel, gain)`
    pub fn set_adc_volume(&self, adc: AdcIdx, gain: u8) {
        match adc {
            AdcIdx::Adc0 => self
                .rb
                .adc_0_control_1()
                .modify(|_, w| unsafe { w.adc_0_ad_gain().bits(gain) }),
            AdcIdx::Adc1 => self
                .rb
                .adc_1_control_1()
                .modify(|_, w| unsafe { w.adc_1_ad_gain().bits(gain) }),
        };
    }

    // ────────────────────────────────────────────────────────────────────────
    // ADC high-pass filter
    // ────────────────────────────────────────────────────────────────────────

    /// Enable / disable the ADC digital high-pass filter (DC-blocking).
    ///
    /// `fc`: 0–7 selects the cutoff-frequency (see register description).
    ///
    /// C: `AUDIO_CODEC_SetADCHPF(adc_sel, fc, newstate)`
    pub fn set_adc_hpf(&self, adc: AdcIdx, fc: u8, enable: bool) {
        match adc {
            AdcIdx::Adc0 => self.rb.adc_0_control_0().modify(|_, w| {
                if enable {
                    unsafe { w.adc_0_dchpf_en().set_bit().adc_0_dchpf_fc_sel().bits(fc) }
                } else {
                    w.adc_0_dchpf_en().clear_bit()
                }
            }),
            AdcIdx::Adc1 => self.rb.adc_1_control_0().modify(|_, w| {
                if enable {
                    unsafe { w.adc_1_dchpf_en().set_bit().adc_1_dchpf_fc_sel().bits(fc) }
                } else {
                    w.adc_1_dchpf_en().clear_bit()
                }
            }),
        };
    }

    // ────────────────────────────────────────────────────────────────────────
    // ADC ASRC
    // ────────────────────────────────────────────────────────────────────────

    /// Configure and enable / disable ADC ASRC.
    ///
    /// C: `AUDIO_CODEC_SetADCASRC(i2s_sel, adc_sel, sr, newstate)`
    pub fn set_adc_asrc(&self, _i2s_sel: I2sSel, adc: AdcIdx, sr: u8, enable: bool) {
        // rate_sel: 1 for 96K/88.2K, 0 otherwise
        let rate = if sr == sr::SR_96K || sr == sr::SR_88P2K {
            1u8
        } else {
            0u8
        };
        self.rb
            .asrc_control_2()
            .modify(|_, w| unsafe { w.asrc_rate_sel_rx_0().bits(rate) });
        match adc {
            AdcIdx::Adc0 => self
                .rb
                .clock_control_6()
                .modify(|_, w| w.adc_0_asrc_en().bit(enable)),
            AdcIdx::Adc1 => self
                .rb
                .clock_control_6()
                .modify(|_, w| w.adc_1_asrc_en().bit(enable)),
        };
    }

    // ────────────────────────────────────────────────────────────────────────
    // ADC mute
    // ────────────────────────────────────────────────────────────────────────

    /// Mute or unmute an ADC channel.
    ///
    /// C: `AUDIO_CODEC_SetADCMute(adc_sel, newstate)` — `mute=true` silences the path.
    pub fn set_adc_mute(&self, adc: AdcIdx, mute: bool) {
        match adc {
            AdcIdx::Adc0 => self
                .rb
                .adc_0_control_0()
                .modify(|_, w| w.adc_0_ad_mute().bit(mute)),
            AdcIdx::Adc1 => self
                .rb
                .adc_1_control_0()
                .modify(|_, w| w.adc_1_ad_mute().bit(mute)),
        };
    }

    /// Mute or unmute the DMIC mixer input for an ADC channel.
    ///
    /// C: `AUDIO_CODEC_SetADCMixMute(adc_num, type, newstate)`
    pub fn set_adc_mix_mute(&self, adc: AdcIdx, _input: AdcInputType, mute: bool) {
        // Only DMIC type exists; _input is accepted for forward-compatibility.
        match adc {
            AdcIdx::Adc0 => self
                .rb
                .adc_0_control_0()
                .modify(|_, w| w.adc_0_dmic_mix_mute().bit(mute)),
            AdcIdx::Adc1 => self
                .rb
                .adc_1_control_0()
                .modify(|_, w| w.adc_1_dmic_mix_mute().bit(mute)),
        };
    }

    // ────────────────────────────────────────────────────────────────────────
    // DMIC
    // ────────────────────────────────────────────────────────────────────────

    /// Enable / disable the DMIC filter clock for an ADC channel.
    ///
    /// C: `AUDIO_CODEC_SetADCDmicFilter(adc_num, newstate)`
    pub fn set_adc_dmic_filter(&self, adc: AdcIdx, enable: bool) {
        match adc {
            AdcIdx::Adc0 => self
                .rb
                .clock_control_2()
                .modify(|_, w| w.dmic_0_en().bit(enable)),
            AdcIdx::Adc1 => self
                .rb
                .clock_control_2()
                .modify(|_, w| w.dmic_1_en().bit(enable)),
        };
    }

    /// Enable / disable the DMIC clock and select its frequency.
    ///
    /// C: `AUDIO_CODEC_SetDmicClk(clk, newstate)`
    pub fn set_dmic_clk(&self, clk: DmicClk, enable: bool) {
        if enable {
            self.rb.clock_control_3().modify(|_, w| unsafe {
                w.dmic1_clk_en().set_bit().dmic1_clk_sel().bits(clk as u8)
            });
        } else {
            self.rb
                .clock_control_3()
                .modify(|_, w| w.dmic1_clk_en().clear_bit());
        }
    }

    /// Select the DMIC source for an ADC channel.
    ///
    /// C: `AUDIO_CODEC_SetDmicSrc(ad_chn, dmic_num)`
    pub fn set_dmic_src(&self, chn: AdcChn, src: DmicSrc) {
        let val = src as u8;
        match chn {
            AdcChn::Chn0 => self
                .rb
                .adc_0_control_0()
                .modify(|_, w| unsafe { w.adc_0_dmic_src_sel().bits(val) }),
            AdcChn::Chn1 => self
                .rb
                .adc_1_control_0()
                .modify(|_, w| unsafe { w.adc_1_dmic_src_sel().bits(val) }),
        };
    }

    // ────────────────────────────────────────────────────────────────────────
    // EQ
    // ────────────────────────────────────────────────────────────────────────

    /// Enable / disable the EQ clock for an ADC channel.
    ///
    /// C: `AUDIO_CODEC_SetADCEQClk(ad_chn, newstate)`
    pub fn set_adc_eq_clk(&self, chn: AdcChn, enable: bool) {
        match chn {
            AdcChn::Chn0 => self
                .rb
                .clock_control_2()
                .modify(|_, w| w.ad_0_eq_en().bit(enable)),
            AdcChn::Chn1 => self
                .rb
                .clock_control_2()
                .modify(|_, w| w.ad_1_eq_en().bit(enable)),
        };
    }

    /// Write biquad EQ filter coefficients for a specific channel and band.
    ///
    /// C: `AUDIO_CODEC_SetADCEQFilter(ad_chn, band_sel, EQFilterCoefPoint)`
    pub fn set_adc_eq_filter(&self, chn: AdcChn, band: EqBand, coef: &EqFilterCoef) {
        // Mask to 29 bits (4.25 fixed-point).
        let mask29 = 0x1fff_ffffu32;
        let h0 = coef.h0 & mask29;
        let b1 = coef.b1 & mask29;
        let b2 = coef.b2 & mask29;
        let a1 = coef.a1 & mask29;
        let a2 = coef.a2 & mask29;

        match chn {
            AdcChn::Chn0 => match band {
                EqBand::Band0 => {
                    self.rb
                        .adc_0_biquad_h0_0()
                        .modify(|_, w| unsafe { w.adc_0_biquad_h0_0().bits(h0) });
                    self.rb
                        .adc_0_biquad_b1_0()
                        .modify(|_, w| unsafe { w.adc_0_biquad_b1_0().bits(b1) });
                    self.rb
                        .adc_0_biquad_b2_0()
                        .modify(|_, w| unsafe { w.adc_0_biquad_b2_0().bits(b2) });
                    self.rb
                        .adc_0_biquad_a1_0()
                        .modify(|_, w| unsafe { w.adc_0_biquad_a1_0().bits(a1) });
                    self.rb
                        .adc_0_biquad_a2_0()
                        .modify(|_, w| unsafe { w.adc_0_biquad_a2_0().bits(a2) });
                }
                EqBand::Band1 => {
                    self.rb
                        .adc_0_biquad_h0_1()
                        .modify(|_, w| unsafe { w.adc_0_biquad_h0_1().bits(h0) });
                    self.rb
                        .adc_0_biquad_b1_1()
                        .modify(|_, w| unsafe { w.adc_0_biquad_b1_1().bits(b1) });
                    self.rb
                        .adc_0_biquad_b2_1()
                        .modify(|_, w| unsafe { w.adc_0_biquad_b2_1().bits(b2) });
                    self.rb
                        .adc_0_biquad_a1_1()
                        .modify(|_, w| unsafe { w.adc_0_biquad_a1_1().bits(a1) });
                    self.rb
                        .adc_0_biquad_a2_1()
                        .modify(|_, w| unsafe { w.adc_0_biquad_a2_1().bits(a2) });
                }
                EqBand::Band2 => {
                    self.rb
                        .adc_0_biquad_h0_2()
                        .modify(|_, w| unsafe { w.adc_0_biquad_h0_2().bits(h0) });
                    self.rb
                        .adc_0_biquad_b1_2()
                        .modify(|_, w| unsafe { w.adc_0_biquad_b1_2().bits(b1) });
                    self.rb
                        .adc_0_biquad_b2_2()
                        .modify(|_, w| unsafe { w.adc_0_biquad_b2_2().bits(b2) });
                    self.rb
                        .adc_0_biquad_a1_2()
                        .modify(|_, w| unsafe { w.adc_0_biquad_a1_2().bits(a1) });
                    self.rb
                        .adc_0_biquad_a2_2()
                        .modify(|_, w| unsafe { w.adc_0_biquad_a2_2().bits(a2) });
                }
                EqBand::Band3 => {
                    self.rb
                        .adc_0_biquad_h0_3()
                        .modify(|_, w| unsafe { w.adc_0_biquad_h0_3().bits(h0) });
                    self.rb
                        .adc_0_biquad_b1_3()
                        .modify(|_, w| unsafe { w.adc_0_biquad_b1_3().bits(b1) });
                    self.rb
                        .adc_0_biquad_b2_3()
                        .modify(|_, w| unsafe { w.adc_0_biquad_b2_3().bits(b2) });
                    self.rb
                        .adc_0_biquad_a1_3()
                        .modify(|_, w| unsafe { w.adc_0_biquad_a1_3().bits(a1) });
                    self.rb
                        .adc_0_biquad_a2_3()
                        .modify(|_, w| unsafe { w.adc_0_biquad_a2_3().bits(a2) });
                }
                EqBand::Band4 => {
                    self.rb
                        .adc_0_biquad_h0_4()
                        .modify(|_, w| unsafe { w.adc_0_biquad_h0_4().bits(h0) });
                    self.rb
                        .adc_0_biquad_b1_4()
                        .modify(|_, w| unsafe { w.adc_0_biquad_b1_4().bits(b1) });
                    self.rb
                        .adc_0_biquad_b2_4()
                        .modify(|_, w| unsafe { w.adc_0_biquad_b2_4().bits(b2) });
                    self.rb
                        .adc_0_biquad_a1_4()
                        .modify(|_, w| unsafe { w.adc_0_biquad_a1_4().bits(a1) });
                    self.rb
                        .adc_0_biquad_a2_4()
                        .modify(|_, w| unsafe { w.adc_0_biquad_a2_4().bits(a2) });
                }
            },
            AdcChn::Chn1 => match band {
                EqBand::Band0 => {
                    self.rb
                        .adc_1_biquad_h0_0()
                        .modify(|_, w| unsafe { w.adc_1_biquad_h0_0().bits(h0) });
                    self.rb
                        .adc_1_biquad_b1_0()
                        .modify(|_, w| unsafe { w.adc_1_biquad_b1_0().bits(b1) });
                    self.rb
                        .adc_1_biquad_b2_0()
                        .modify(|_, w| unsafe { w.adc_1_biquad_b2_0().bits(b2) });
                    self.rb
                        .adc_1_biquad_a1_0()
                        .modify(|_, w| unsafe { w.adc_1_biquad_a1_0().bits(a1) });
                    self.rb
                        .adc_1_biquad_a2_0()
                        .modify(|_, w| unsafe { w.adc_1_biquad_a2_0().bits(a2) });
                }
                EqBand::Band1 => {
                    self.rb
                        .adc_1_biquad_h0_1()
                        .modify(|_, w| unsafe { w.adc_1_biquad_h0_1().bits(h0) });
                    self.rb
                        .adc_1_biquad_b1_1()
                        .modify(|_, w| unsafe { w.adc_1_biquad_b1_1().bits(b1) });
                    self.rb
                        .adc_1_biquad_b2_1()
                        .modify(|_, w| unsafe { w.adc_1_biquad_b2_1().bits(b2) });
                    self.rb
                        .adc_1_biquad_a1_1()
                        .modify(|_, w| unsafe { w.adc_1_biquad_a1_1().bits(a1) });
                    self.rb
                        .adc_1_biquad_a2_1()
                        .modify(|_, w| unsafe { w.adc_1_biquad_a2_1().bits(a2) });
                }
                EqBand::Band2 => {
                    self.rb
                        .adc_1_biquad_h0_2()
                        .modify(|_, w| unsafe { w.adc_1_biquad_h0_2().bits(h0) });
                    self.rb
                        .adc_1_biquad_b1_2()
                        .modify(|_, w| unsafe { w.adc_1_biquad_b1_2().bits(b1) });
                    self.rb
                        .adc_1_biquad_b2_2()
                        .modify(|_, w| unsafe { w.adc_1_biquad_b2_2().bits(b2) });
                    self.rb
                        .adc_1_biquad_a1_2()
                        .modify(|_, w| unsafe { w.adc_1_biquad_a1_2().bits(a1) });
                    self.rb
                        .adc_1_biquad_a2_2()
                        .modify(|_, w| unsafe { w.adc_1_biquad_a2_2().bits(a2) });
                }
                EqBand::Band3 => {
                    self.rb
                        .adc_1_biquad_h0_3()
                        .modify(|_, w| unsafe { w.adc_1_biquad_h0_3().bits(h0) });
                    self.rb
                        .adc_1_biquad_b1_3()
                        .modify(|_, w| unsafe { w.adc_1_biquad_b1_3().bits(b1) });
                    self.rb
                        .adc_1_biquad_b2_3()
                        .modify(|_, w| unsafe { w.adc_1_biquad_b2_3().bits(b2) });
                    self.rb
                        .adc_1_biquad_a1_3()
                        .modify(|_, w| unsafe { w.adc_1_biquad_a1_3().bits(a1) });
                    self.rb
                        .adc_1_biquad_a2_3()
                        .modify(|_, w| unsafe { w.adc_1_biquad_a2_3().bits(a2) });
                }
                EqBand::Band4 => {
                    self.rb
                        .adc_1_biquad_h0_4()
                        .modify(|_, w| unsafe { w.adc_1_biquad_h0_4().bits(h0) });
                    self.rb
                        .adc_1_biquad_b1_4()
                        .modify(|_, w| unsafe { w.adc_1_biquad_b1_4().bits(b1) });
                    self.rb
                        .adc_1_biquad_b2_4()
                        .modify(|_, w| unsafe { w.adc_1_biquad_b2_4().bits(b2) });
                    self.rb
                        .adc_1_biquad_a1_4()
                        .modify(|_, w| unsafe { w.adc_1_biquad_a1_4().bits(a1) });
                    self.rb
                        .adc_1_biquad_a2_4()
                        .modify(|_, w| unsafe { w.adc_1_biquad_a2_4().bits(a2) });
                }
            },
        }
    }

    /// Enable / disable an individual EQ biquad band for an ADC channel.
    ///
    /// C: `AUDIO_CODEC_SetADCEQBand(ad_chn, band_sel, newstate)`
    pub fn set_adc_eq_band(&self, chn: AdcChn, band: EqBand, enable: bool) {
        match chn {
            AdcChn::Chn0 => match band {
                EqBand::Band0 => self
                    .rb
                    .adc_0_eq_ctrl()
                    .modify(|_, w| w.adc_0_biquad_en_0().bit(enable)),
                EqBand::Band1 => self
                    .rb
                    .adc_0_eq_ctrl()
                    .modify(|_, w| w.adc_0_biquad_en_1().bit(enable)),
                EqBand::Band2 => self
                    .rb
                    .adc_0_eq_ctrl()
                    .modify(|_, w| w.adc_0_biquad_en_2().bit(enable)),
                EqBand::Band3 => self
                    .rb
                    .adc_0_eq_ctrl()
                    .modify(|_, w| w.adc_0_biquad_en_3().bit(enable)),
                EqBand::Band4 => self
                    .rb
                    .adc_0_eq_ctrl()
                    .modify(|_, w| w.adc_0_biquad_en_4().bit(enable)),
            },
            AdcChn::Chn1 => match band {
                EqBand::Band0 => self
                    .rb
                    .adc_1_eq_ctrl()
                    .modify(|_, w| w.adc_1_biquad_en_0().bit(enable)),
                EqBand::Band1 => self
                    .rb
                    .adc_1_eq_ctrl()
                    .modify(|_, w| w.adc_1_biquad_en_1().bit(enable)),
                EqBand::Band2 => self
                    .rb
                    .adc_1_eq_ctrl()
                    .modify(|_, w| w.adc_1_biquad_en_2().bit(enable)),
                EqBand::Band3 => self
                    .rb
                    .adc_1_eq_ctrl()
                    .modify(|_, w| w.adc_1_biquad_en_3().bit(enable)),
                EqBand::Band4 => self
                    .rb
                    .adc_1_eq_ctrl()
                    .modify(|_, w| w.adc_1_biquad_en_4().bit(enable)),
            },
        };
    }

    // ────────────────────────────────────────────────────────────────────────
    // ADC zero-detection
    // ────────────────────────────────────────────────────────────────────────

    /// Set ADC path zero-detection function.
    ///
    /// C: `AUDIO_CODEC_SetADCZDET(adc_sel, type)`
    pub fn set_adc_zdet(&self, adc: AdcIdx, func: ZdetFunc) {
        let val = func as u8;
        match adc {
            AdcIdx::Adc0 => self
                .rb
                .adc_0_control_0()
                .modify(|_, w| unsafe { w.adc_0_ad_zdet_func().bits(val) }),
            AdcIdx::Adc1 => self
                .rb
                .adc_1_control_0()
                .modify(|_, w| unsafe { w.adc_1_ad_zdet_func().bits(val) }),
        };
    }

    /// Set ADC path zero-detection timeout.
    ///
    /// C: `AUDIO_CODEC_SetADCZDETTimeOut(adc_sel, time_out)`
    pub fn set_adc_zdet_timeout(&self, adc: AdcIdx, timeout: ZdetTimeout) {
        let val = timeout as u8;
        match adc {
            AdcIdx::Adc0 => self
                .rb
                .adc_0_control_0()
                .modify(|_, w| unsafe { w.adc_0_ad_zdet_tout().bits(val) }),
            AdcIdx::Adc1 => self
                .rb
                .adc_1_control_0()
                .modify(|_, w| unsafe { w.adc_1_ad_zdet_tout().bits(val) }),
        };
    }

    // ────────────────────────────────────────────────────────────────────────
    // High-level record flow
    // ────────────────────────────────────────────────────────────────────────

    /// Run the full codec initialisation sequence for recording.
    ///
    /// Mirrors `AUDIO_CODEC_Record()` exactly, supporting both AMIC and DMIC
    /// in non-TDM mode.  Only `I2sTdmMode::NoTdm` is fully wired up; TDM
    /// would require channel-count expansion not present in the original C
    /// file.
    ///
    /// C: `AUDIO_CODEC_Record(i2s_sel, type, I2S_InitStruct)`
    pub fn record(&self, i2s_sel: I2sSel, rtype: RecordType, params: &I2sInitParams) {
        self.set_audio_ip(true);
        self.set_i2s_ip(i2s_sel, true);
        self.set_i2s_src(i2s_sel, I2sMasterSrc::InternalSport);
        self.set_adc_sr_src(SampleRateSrc::Source1, params.rx_sr);
        self.set_i2s_parameters(i2s_sel, params);

        if params.rx_tdm == I2sTdmMode::NoTdm {
            self.enable_adc(AdcChn::Chn0, true);
            self.enable_adc(AdcChn::Chn1, true);
            self.enable_adc_fifo(AdcChn::Chn0, true);
            self.enable_adc_fifo(AdcChn::Chn1, true);
            self.set_adc_volume(AdcIdx::Adc0, 0x2f);
            self.set_adc_volume(AdcIdx::Adc1, 0x2f);
            self.set_adc_hpf(AdcIdx::Adc0, 3, true);
            self.set_adc_hpf(AdcIdx::Adc1, 3, true);
            self.set_adc_mute(AdcIdx::Adc0, false);
            self.set_adc_mute(AdcIdx::Adc1, false);
        }

        if matches!(rtype, RecordType::Dmic) {
            self.set_dmic_clk(DmicClk::Dmic2p5M, true);

            if params.rx_tdm == I2sTdmMode::NoTdm {
                self.set_adc_mix_mute(AdcIdx::Adc0, AdcInputType::Dmic, false);
                self.set_adc_mix_mute(AdcIdx::Adc1, AdcInputType::Dmic, false);
                self.set_adc_dmic_filter(AdcIdx::Adc0, true);
                self.set_adc_dmic_filter(AdcIdx::Adc1, true);
                self.set_dmic_src(AdcChn::Chn0, DmicSrc::Dmic1);
                self.set_dmic_src(AdcChn::Chn1, DmicSrc::Dmic2);
            }
        }
    }
}
