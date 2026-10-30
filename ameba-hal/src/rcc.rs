use embedded_hal::delay::DelayNs;
/// Typed RCC wrapper for RTL8721F peripheral clock/function enable.
///
/// Usage: `ameba_hal::rcc::enable(Peripheral::Uart0)`
///
/// Each `Peripheral` constant encodes `(apb, clk)` where:
/// - `apb` = APBPeriph_XXX (function-enable mask; 0 = none, SDK: ameba_rcc.h : APBPeriph_NULL)
/// - `clk` = APBPeriph_XXX_CLOCK (clock-enable mask; 0 = none, SDK: ameba_rcc.h : APBPeriph_CLOCK_NULL)
///
/// Top 2 bits of each value encode the register bank:
/// 00 = LSYS FEN/CKE GRP0, 01 = LSYS FEN/CKE GRP1, 11 = AON FEN/CKE
///
/// SDK: sysreg_lsys.h, sysreg_aon.h, ameba_rcc.h
use rtl8721f_pac::{LpAon, LpSys};

/// Peripheral clock/function descriptor.
#[derive(Clone, Copy)]
pub struct Peripheral {
    pub(crate) apb: u32, // function-enable mask (APBPeriph_XXX, 0=none)
    pub(crate) clk: u32, // clock-enable mask (APBPeriph_XXX_CLOCK, 0=none)
}

// Column-aligned mirror of the SDK's APBPeriph_XXX #defines; rustfmt would
// explode each `Self { apb, clk }` onto four lines and lose the alignment.
// Consts only, no methods, so skipping the whole impl is safe.
#[rustfmt::skip]
impl Peripheral {
    // ── LSYS GRP0 (FEN bit30=0 / CKE bit30=0) ───────────────────────────────
    // Note: for GRP0, (0<<30)=0 so values have no bit30 set.

    /// HS SoC system enable. SDK: sysreg_lsys.h : APBPeriph_SOC (FEN only)
    pub const SOC:    Self = Self { apb: (1<<0),  clk: 0 };
    /// Platform enable. SDK: sysreg_lsys.h : APBPeriph_PLFM (FEN only)
    pub const PLFM:   Self = Self { apb: (1<<1),  clk: 0 };
    /// KM4NS (CPU1) enable. SDK: sysreg_lsys.h : APBPeriph_CPU1 / APBPeriph_CPU1_CLOCK
    pub const CPU1:   Self = Self { apb: (1<<2),  clk: (1<<3)  };
    /// KM4TZ (CPU0) enable. SDK: sysreg_lsys.h : APBPeriph_CPU0 / APBPeriph_CPU0_CLOCK
    pub const CPU0:   Self = Self { apb: (1<<3),  clk: (1<<4)  };
    /// MJPEG. SDK: sysreg_lsys.h : APBPeriph_MJPEG / APBPeriph_MJPEG_CLOCK
    pub const MJPEG:  Self = Self { apb: (1<<4),  clk: (1<<19) };
    /// IPC. SDK: sysreg_lsys.h : APBPeriph_IPC / APBPeriph_IPC_CLOCK
    pub const IPC:    Self = Self { apb: (1<<5),  clk: (1<<5)  };
    /// GDMA0. SDK: sysreg_lsys.h : APBPeriph_DMAC / APBPeriph_DMAC_CLOCK
    pub const DMAC:   Self = Self { apb: (1<<6),  clk: (1<<6)  };
    /// PPE (FEN GRP0 bit7, CKE GRP1 bit29). SDK: sysreg_lsys.h : APBPeriph_PPE / APBPeriph_PPE_CLOCK
    pub const PPE:    Self = Self { apb: (1<<7),  clk: (1<<30)|(1<<29) };
    /// IPSEC (AES+HMAC). SDK: sysreg_lsys.h : APBPeriph_CRYPTO / APBPeriph_CRYPTO_CLOCK
    pub const CRYPTO: Self = Self { apb: (1<<8),  clk: (1<<8)  };
    /// PDM/Audio codec (FEN GRP0 bit9, CKE GRP1 bit18). SDK: sysreg_lsys.h : APBPeriph_AC / APBPeriph_AC_CLOCK
    pub const AC:     Self = Self { apb: (1<<9),  clk: (1<<30)|(1<<18) };
    /// Flash SPIC. SDK: sysreg_lsys.h : APBPeriph_FLASH / APBPeriph_FLASH_CLOCK
    pub const FLASH:  Self = Self { apb: (1<<10), clk: (1<<10) };
    /// LCDC (FEN bit11, CKE bit9). SDK: sysreg_lsys.h : APBPeriph_LCDC / APBPeriph_LCDC_CLOCK
    pub const LCDC:   Self = Self { apb: (1<<11), clk: (1<<9)  };
    /// PSRAM. SDK: sysreg_lsys.h : APBPeriph_PSRAM / APBPeriph_PSRAM_CLOCK
    pub const PSRAM:  Self = Self { apb: (1<<13), clk: (1<<13) };
    /// SPI0. SDK: sysreg_lsys.h : APBPeriph_SPI0 / APBPeriph_SPI0_CLOCK
    pub const SPI0:   Self = Self { apb: (1<<14), clk: (1<<14) };
    /// SPI1. SDK: sysreg_lsys.h : APBPeriph_SPI1 / APBPeriph_SPI1_CLOCK
    pub const SPI1:   Self = Self { apb: (1<<15), clk: (1<<15) };
    /// CAN0. SDK: sysreg_lsys.h : APBPeriph_CAN0 / APBPeriph_CAN0_CLOCK
    pub const CAN0:   Self = Self { apb: (1<<16), clk: (1<<16) };
    /// CAN1. SDK: sysreg_lsys.h : APBPeriph_CAN1 / APBPeriph_CAN1_CLOCK
    pub const CAN1:   Self = Self { apb: (1<<17), clk: (1<<17) };
    /// TRNG. SDK: sysreg_lsys.h : APBPeriph_TRNG / APBPeriph_TRNG_CLOCK
    pub const TRNG:   Self = Self { apb: (1<<18), clk: (1<<18) };
    /// GMAC (FEN GRP0 bit19, CKE GRP1 bit21). SDK: sysreg_lsys.h : APBPeriph_GMAC / APBPeriph_GMAC_CLOCK
    pub const GMAC:   Self = Self { apb: (1<<19), clk: (1<<30)|(1<<21) };
    /// SPORT. SDK: sysreg_lsys.h : APBPeriph_SPORT / APBPeriph_SPORT_CLOCK
    pub const SPORT:  Self = Self { apb: (1<<20), clk: (1<<20) };
    /// SDIO host (FEN bit21, CKE bit23). SDK: sysreg_lsys.h : APBPeriph_SDH / APBPeriph_SDH_CLOCK
    pub const SDH:    Self = Self { apb: (1<<21), clk: (1<<23) };
    /// SDIO device. SDK: sysreg_lsys.h : APBPeriph_SDD / APBPeriph_SDD_CLOCK
    pub const SDD:    Self = Self { apb: (1<<22), clk: (1<<22) };
    /// PWM timer 0 (FEN bit23, CKE bit24). SDK: sysreg_lsys.h : APBPeriph_PWM0 / APBPeriph_PWM0_CLOCK
    pub const PWM0:   Self = Self { apb: (1<<23), clk: (1<<24) };
    /// PWM timer 1 (FEN bit24, CKE bit25). SDK: sysreg_lsys.h : APBPeriph_PWM1 / APBPeriph_PWM1_CLOCK
    pub const PWM1:   Self = Self { apb: (1<<24), clk: (1<<25) };
    /// PWM timer 2 (FEN bit25, CKE bit26). SDK: sysreg_lsys.h : APBPeriph_PWM2 / APBPeriph_PWM2_CLOCK
    pub const PWM2:   Self = Self { apb: (1<<25), clk: (1<<26) };
    /// PWM timer 3 (FEN bit26, CKE bit27). SDK: sysreg_lsys.h : APBPeriph_PWM3 / APBPeriph_PWM3_CLOCK
    pub const PWM3:   Self = Self { apb: (1<<26), clk: (1<<27) };
    /// PWM timer 4 (FEN bit27, CKE bit28). SDK: sysreg_lsys.h : APBPeriph_PWM4 / APBPeriph_PWM4_CLOCK
    pub const PWM4:   Self = Self { apb: (1<<27), clk: (1<<28) };
    /// PKE (FEN bit28, CKE bit29). SDK: sysreg_lsys.h : APBPeriph_PKE / APBPeriph_PKE_CLOCK
    pub const PKE:    Self = Self { apb: (1<<28), clk: (1<<29) };

    // ── LSYS GRP0 clock-only (no FEN) ────────────────────────────────────────
    /// LPON domain clock (LSYS CKE GRP0 bit0). SDK: sysreg_lsys.h : APBPeriph_LPON_CLOCK (see also LPON for AON FEN+this clk)
    pub const LPON_CLK:   Self = Self { apb: 0, clk: (1<<0)  };
    /// LP peripheral bus clock. SDK: sysreg_lsys.h : APBPeriph_LPERI_CLOCK
    pub const LPERI:      Self = Self { apb: 0, clk: (1<<1)  };
    /// HP peripheral bus clock. SDK: sysreg_lsys.h : APBPeriph_HPERI_CLOCK
    pub const HPERI:      Self = Self { apb: 0, clk: (1<<2)  };
    /// OTP controller clock. SDK: sysreg_lsys.h : APBPeriph_OTPC_CLOCK
    pub const OTPC:       Self = Self { apb: 0, clk: (1<<7)  };
    /// ROM clock. SDK: sysreg_lsys.h : APBPeriph_ROM_CLOCK
    pub const ROM:        Self = Self { apb: 0, clk: (1<<11) };
    /// SRAM clock. SDK: sysreg_lsys.h : APBPeriph_SRAM_CLOCK
    pub const SRAM:       Self = Self { apb: 0, clk: (1<<12) };
    /// Shared HP peripheral bus clock. SDK: sysreg_lsys.h : APBPeriph_SHPERI_CLOCK
    pub const SHPERI:     Self = Self { apb: 0, clk: (1<<21) };

    // ── LSYS GRP1 (FEN bit30=1 / CKE bit30=1) ────────────────────────────────
    /// WLON (WiFi on). SDK: sysreg_lsys.h : APBPeriph_WLON / APBPeriph_WLON_CLOCK
    pub const WLON:    Self = Self { apb: (1<<30)|(1<<0),  clk: (1<<30)|(1<<0)  };
    /// BTON (BT on). SDK: sysreg_lsys.h : APBPeriph_BTON / APBPeriph_BTON_CLOCK
    pub const BTON:    Self = Self { apb: (1<<30)|(1<<1),  clk: (1<<30)|(1<<1)  };
    /// WL RF AFE (FEN only). SDK: sysreg_lsys.h : APBPeriph_WLAFE
    pub const WLAFE:   Self = Self { apb: (1<<30)|(1<<2),  clk: 0               };
    /// SIC. SDK: sysreg_lsys.h : APBPeriph_SIC / APBPeriph_SIC_CLOCK
    pub const SIC:     Self = Self { apb: (1<<30)|(1<<3),  clk: (1<<30)|(1<<3)  };
    /// GPIO. SDK: sysreg_lsys.h : APBPeriph_GPIO / APBPeriph_GPIO_CLOCK
    pub const GPIO:    Self = Self { apb: (1<<30)|(1<<4),  clk: (1<<30)|(1<<4)  };
    /// LogUART. SDK: sysreg_lsys.h : APBPeriph_LOGUART / APBPeriph_LOGUART_CLOCK
    pub const LOGUART: Self = Self { apb: (1<<30)|(1<<5),  clk: (1<<30)|(1<<5)  };
    /// UART0. SDK: sysreg_lsys.h : APBPeriph_UART0 / APBPeriph_UART0_CLOCK
    pub const UART0:   Self = Self { apb: (1<<30)|(1<<6),  clk: (1<<30)|(1<<6)  };
    /// UART1. SDK: sysreg_lsys.h : APBPeriph_UART1 / APBPeriph_UART1_CLOCK
    pub const UART1:   Self = Self { apb: (1<<30)|(1<<7),  clk: (1<<30)|(1<<7)  };
    /// UART2. SDK: sysreg_lsys.h : APBPeriph_UART2 / APBPeriph_UART2_CLOCK
    pub const UART2:   Self = Self { apb: (1<<30)|(1<<8),  clk: (1<<30)|(1<<8)  };
    /// UART3. SDK: sysreg_lsys.h : APBPeriph_UART3 / APBPeriph_UART3_CLOCK
    pub const UART3:   Self = Self { apb: (1<<30)|(1<<9),  clk: (1<<30)|(1<<9)  };
    /// I2C0. SDK: sysreg_lsys.h : APBPeriph_I2C0 / APBPeriph_I2C0_CLOCK
    pub const I2C0:    Self = Self { apb: (1<<30)|(1<<10), clk: (1<<30)|(1<<10) };
    /// I2C1. SDK: sysreg_lsys.h : APBPeriph_I2C1 / APBPeriph_I2C1_CLOCK
    pub const I2C1:    Self = Self { apb: (1<<30)|(1<<11), clk: (1<<30)|(1<<11) };
    /// Basic timer 0. SDK: sysreg_lsys.h : APBPeriph_LTIM0 / APBPeriph_LTIM0_CLOCK
    pub const LTIM0:   Self = Self { apb: (1<<30)|(1<<12), clk: (1<<30)|(1<<12) };
    /// Basic timer 1. SDK: sysreg_lsys.h : APBPeriph_LTIM1 / APBPeriph_LTIM1_CLOCK
    pub const LTIM1:   Self = Self { apb: (1<<30)|(1<<13), clk: (1<<30)|(1<<13) };
    /// Basic timer 2. SDK: sysreg_lsys.h : APBPeriph_LTIM2 / APBPeriph_LTIM2_CLOCK
    pub const LTIM2:   Self = Self { apb: (1<<30)|(1<<14), clk: (1<<30)|(1<<14) };
    /// Basic timer 3. SDK: sysreg_lsys.h : APBPeriph_LTIM3 / APBPeriph_LTIM3_CLOCK
    pub const LTIM3:   Self = Self { apb: (1<<30)|(1<<15), clk: (1<<30)|(1<<15) };
    /// Debug timer. SDK: sysreg_lsys.h : APBPeriph_DTIM / APBPeriph_DTIM_CLOCK
    pub const DTIM:    Self = Self { apb: (1<<30)|(1<<20), clk: (1<<30)|(1<<20) };
    /// BT share memory (FEN only). SDK: sysreg_lsys.h : APBPeriph_SHRM
    pub const SHRM:    Self = Self { apb: (1<<30)|(1<<21), clk: 0               };
    /// USB. SDK: sysreg_lsys.h : APBPeriph_USB / APBPeriph_USB_CLOCK
    pub const USB:     Self = Self { apb: (1<<30)|(1<<22), clk: (1<<30)|(1<<22) };
    /// LP ADC. SDK: sysreg_lsys.h : APBPeriph_ADC / APBPeriph_ADC_CLOCK
    pub const ADC:     Self = Self { apb: (1<<30)|(1<<23), clk: (1<<30)|(1<<23) };
    /// Thermal. SDK: sysreg_lsys.h : APBPeriph_THM / APBPeriph_THM_CLOCK
    pub const THM:     Self = Self { apb: (1<<30)|(1<<24), clk: (1<<30)|(1<<24) };
    /// PMC timer 0. SDK: sysreg_lsys.h : APBPeriph_PTIM0 / APBPeriph_PTIM0_CLOCK
    pub const PTIM0:   Self = Self { apb: (1<<30)|(1<<25), clk: (1<<30)|(1<<25) };
    /// PMC timer 1. SDK: sysreg_lsys.h : APBPeriph_PTIM1 / APBPeriph_PTIM1_CLOCK
    pub const PTIM1:   Self = Self { apb: (1<<30)|(1<<26), clk: (1<<30)|(1<<26) };
    /// IrDA. SDK: sysreg_lsys.h : APBPeriph_IRDA / APBPeriph_IRDA_CLOCK
    pub const IRDA:    Self = Self { apb: (1<<30)|(1<<28), clk: (1<<30)|(1<<28) };

    // ── LSYS GRP1 clock-only (no FEN in GRP1) ────────────────────────────────
    /// Watchdog bus clock. SDK: sysreg_lsys.h : APBPeriph_WDG_CLOCK
    pub const WDG:     Self = Self { apb: 0, clk: (1<<30)|(1<<2)  };
    /// WMAC AXI/AHB clock. SDK: sysreg_lsys.h : APBPeriph_WMAC_CLOCK
    pub const WMAC:    Self = Self { apb: 0, clk: (1<<30)|(1<<27) };

    // ── AON domain (bit30..31 = 0b11 = 3) ────────────────────────────────────
    /// BOR (FEN only). SDK: sysreg_aon.h : APBPeriph_BOR
    pub const BOR:     Self = Self { apb: (3<<30)|(1<<0),  clk: 0              };
    /// IWDG (FEN bit1, CKE bit5). SDK: sysreg_aon.h : APBPeriph_IWDG / APBPeriph_IWDG_CLOCK
    pub const IWDG:    Self = Self { apb: (3<<30)|(1<<1),  clk: (3<<30)|(1<<5) };
    /// AON timer. SDK: sysreg_aon.h : APBPeriph_ATIM / APBPeriph_ATIM_CLOCK
    pub const ATIM:    Self = Self { apb: (3<<30)|(1<<2),  clk: (3<<30)|(1<<2) };
    /// 32.768 kHz SDM. SDK: sysreg_aon.h : APBPeriph_SDM / APBPeriph_SDM_CLOCK
    pub const SDM:     Self = Self { apb: (3<<30)|(1<<4),  clk: (3<<30)|(1<<4) };
    /// LED display. SDK: sysreg_aon.h : APBPeriph_LEDC / APBPeriph_LEDC_CLOCK
    pub const LEDC:    Self = Self { apb: (3<<30)|(1<<8),  clk: (3<<30)|(1<<8) };
    /// PAD interface (FEN only). SDK: sysreg_aon.h : APBPeriph_PX
    pub const PX:      Self = Self { apb: (3<<30)|(1<<9),  clk: 0              };
    /// LS global (AON FEN bit16, LSYS CKE GRP0 bit0). SDK: sysreg_aon.h : APBPeriph_LPON / sysreg_lsys.h : APBPeriph_LPON_CLOCK
    pub const LPON:    Self = Self { apb: (3<<30)|(1<<16), clk: (1<<0)         };

    // ── AON clock-only (no FEN) ───────────────────────────────────────────────
    /// RTC clock-only (step 1 of enable sequence). SDK: sysreg_aon.h : APBPeriph_RTC_CLOCK
    pub const RTC:     Self = Self { apb: 0,             clk: (3<<30)|(1<<3) };
    /// RTC function+clock enable (step 3, after RTC_Enable). SDK: sysreg_aon.h : APBPeriph_RTC / APBPeriph_RTC_CLOCK
    pub const RTC_FEN: Self = Self { apb: (3<<30)|(1<<3), clk: (3<<30)|(1<<3) };
    /// AON domain clock. SDK: sysreg_aon.h : APBPeriph_AON_CLOCK
    pub const AON:     Self = Self { apb: 0, clk: (3<<30)|(1<<0) };
}

/// Enable the peripheral's APB function gate and clock.
pub fn enable(p: Peripheral) {
    rcc_cmd(p, true);
}

/// Disable the peripheral's APB function gate and clock.
pub fn disable(p: Peripheral) {
    rcc_cmd(p, false);
}

// SDK: ameba_rom_patch.c : RCC_PeriphClockCmd
// Top 2 bits of apb/clk select the register bank (same encoding used in Peripheral constants):
//   0b00 → LSYS FEN/CKE GRP0 (LpSys::reg_fen_grp0 / reg_cke_grp0)
//   0b01 → LSYS FEN/CKE GRP1 (LpSys::reg_fen_grp1 / reg_cke_grp1)
//   0b11 → AON FEN/CLK        (LpAon::reg_fen / reg_clk)
// SDK uses IPC_SEMTake(IPC_SEM_SYSON) for cross-core safety; omitted here since
// RCC changes in Rust firmware happen only on KM4TZ at init time.
fn rcc_cmd(p: Peripheral, on: bool) {
    let lp_aon = unsafe { &*LpAon::PTR };
    let lp_sys = unsafe { &*LpSys::PTR };

    // SDK: writes clock first, then function enable
    if p.clk != 0 {
        let mask = p.clk & !(3u32 << 30);
        match (p.clk >> 30) & 0x3 {
            0x0 => {
                lp_sys.reg_cke_grp0().modify(|r, w| unsafe {
                    w.bits(if on {
                        r.bits() | mask
                    } else {
                        r.bits() & !mask
                    })
                });
            }
            0x1 => {
                lp_sys.reg_cke_grp1().modify(|r, w| unsafe {
                    w.bits(if on {
                        r.bits() | mask
                    } else {
                        r.bits() & !mask
                    })
                });
            }
            0x3 => {
                lp_aon.reg_clk().modify(|r, w| unsafe {
                    w.bits(if on {
                        r.bits() | mask
                    } else {
                        r.bits() & !mask
                    })
                });
            }
            _ => {}
        }
    }

    if p.apb != 0 {
        let mask = p.apb & !(3u32 << 30);
        match (p.apb >> 30) & 0x3 {
            0x0 => {
                lp_sys.reg_fen_grp0().modify(|r, w| unsafe {
                    w.bits(if on {
                        r.bits() | mask
                    } else {
                        r.bits() & !mask
                    })
                });
            }
            0x1 => {
                lp_sys.reg_fen_grp1().modify(|r, w| unsafe {
                    w.bits(if on {
                        r.bits() | mask
                    } else {
                        r.bits() & !mask
                    })
                });
            }
            0x3 => {
                lp_aon.reg_fen().modify(|r, w| unsafe {
                    w.bits(if on {
                        r.bits() | mask
                    } else {
                        r.bits() & !mask
                    })
                });
            }
            _ => {}
        }
    }

    // SDK: DelayUs(10) after writes to let clock stabilize
    crate::delay::Delay.delay_us(10u32);
}
