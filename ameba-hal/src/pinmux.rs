/// Typed Pinmux and PAD wrapper for RTL8721F.
///
/// Usage:
/// ```ignore
/// ameba_hal::pinmux::set(Pin::PA23, PinFunction::UART0_TXD);
/// ameba_hal::pinmux::set_pull(Pin::PA24, Pull::Up);
/// ameba_hal::pinmux::set_drive_strength(Pin::PA23, DriveStrength::High);
/// ```
use core::cell::UnsafeCell;
use rtl8721f_pac::{PinmuxReg, pinmux_reg};

// SDK: hal_platform.h : PINMUX = (PINMUX_TypeDef*) 0x4080A800 (NS alias)
const PINMUX_PTR: *const pinmux_reg::RegisterBlock = PinmuxReg::PTR;

// Per-pin register bit fields — SDK: sysreg_pad.h : PAD_BIT_Px_* / PAD_MASK_Px_SEL
const SEL_MASK: u32 = 0xFF; // PAD_MASK_Px_SEL  bits [7:0]: function select
const IE_BIT: u32 = 1 << 8; // PAD_BIT_Px_IE    bit  8: input enable (reset=1)
const PUPDC_BIT: u32 = 1 << 10; // PAD_BIT_Px_PUPDC bit 10: 1=small resistor 0=large
const E2_BIT: u32 = 1 << 11; // PAD_BIT_Px_E2    bit 11: drive e2 (reset=1)
const SMT_BIT: u32 = 1 << 12; // PAD_BIT_Px_SMT   bit 12: Schmitt trigger (reset=1)
const SR_BIT: u32 = 1 << 13; // PAD_BIT_Px_SR    bit 13: slew rate 1=slow 0=fast
const E3_BIT: u32 = 1 << 14; // PAD_BIT_Px_E3    bit 14: drive e3
const DIS_BIT: u32 = 1 << 15; // PAD_BIT_Px_DIS   bit 15: 1=shutdown 0=enabled

// Access the bitmap + control registers via PAC typed interface.
#[inline]
fn pad_regs() -> &'static pinmux_reg::RegisterBlock {
    unsafe { &*PINMUX_PTR }
}

// SDK: ameba_pinmux_rom.c — per-pin functions use PINMUX->REG_GPIOx[PinName],
// a u32 array at block base (offset 0). svd2rust generates distinct named types
// per pin so no indexed PAC accessor exists; use raw pointer stride from PinmuxReg::PTR.
// SDK: ameba_pinmux.h : PORT_NUM(pin)=(pin>>5)&0x03; PIN_NUM(pin)=pin&0x1f
#[inline]
fn pin_reg_read(pin: Pin) -> u32 {
    unsafe { core::ptr::read_volatile(PINMUX_PTR.cast::<u32>().wrapping_add(pin as usize)) }
}

#[inline]
fn pin_reg_rmw(pin: Pin, mask: u32, bits: u32) {
    let ptr = PINMUX_PTR
        .cast::<u32>()
        .cast_mut()
        .wrapping_add(pin as usize);
    unsafe {
        let val = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, (val & !mask) | bits);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Pin enum — PA0-PA31, PB0-PB31, PC0-PC29
// SDK: ameba_pinmux.h : _PA_0.._PA_31, _PB_0.._PB_31, _PC_0.._PC_29
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin {
    // Port A: 0x00–0x1F  SDK: ameba_pinmux.h : _PA_0.._PA_31
    PA0 = 0x00,
    PA1 = 0x01,
    PA2 = 0x02,
    PA3 = 0x03,
    PA4 = 0x04,
    PA5 = 0x05,
    PA6 = 0x06,
    PA7 = 0x07,
    PA8 = 0x08,
    PA9 = 0x09,
    PA10 = 0x0A,
    PA11 = 0x0B,
    PA12 = 0x0C,
    PA13 = 0x0D,
    PA14 = 0x0E,
    PA15 = 0x0F,
    PA16 = 0x10,
    PA17 = 0x11,
    PA18 = 0x12,
    PA19 = 0x13,
    PA20 = 0x14,
    PA21 = 0x15,
    PA22 = 0x16,
    PA23 = 0x17,
    PA24 = 0x18,
    PA25 = 0x19,
    PA26 = 0x1A,
    PA27 = 0x1B,
    PA28 = 0x1C,
    PA29 = 0x1D,
    PA30 = 0x1E,
    PA31 = 0x1F,

    // Port B: 0x20–0x3F  SDK: ameba_pinmux.h : _PB_0.._PB_31
    PB0 = 0x20,
    PB1 = 0x21,
    PB2 = 0x22,
    PB3 = 0x23,
    PB4 = 0x24,
    PB5 = 0x25,
    PB6 = 0x26,
    PB7 = 0x27,
    PB8 = 0x28,
    PB9 = 0x29,
    PB10 = 0x2A,
    PB11 = 0x2B,
    PB12 = 0x2C,
    PB13 = 0x2D,
    PB14 = 0x2E,
    PB15 = 0x2F,
    PB16 = 0x30,
    PB17 = 0x31,
    PB18 = 0x32,
    PB19 = 0x33,
    PB20 = 0x34,
    PB21 = 0x35,
    PB22 = 0x36,
    PB23 = 0x37,
    PB24 = 0x38,
    PB25 = 0x39,
    PB26 = 0x3A,
    PB27 = 0x3B,
    PB28 = 0x3C,
    PB29 = 0x3D,
    PB30 = 0x3E,
    PB31 = 0x3F,

    // Port C: 0x40–0x5D  SDK: ameba_pinmux.h : _PC_0.._PC_29
    PC0 = 0x40,
    PC1 = 0x41,
    PC2 = 0x42,
    PC3 = 0x43,
    PC4 = 0x44,
    PC5 = 0x45,
    PC6 = 0x46,
    PC7 = 0x47,
    PC8 = 0x48,
    PC9 = 0x49,
    PC10 = 0x4A,
    PC11 = 0x4B,
    PC12 = 0x4C,
    PC13 = 0x4D,
    PC14 = 0x4E,
    PC15 = 0x4F,
    PC16 = 0x50,
    PC17 = 0x51,
    PC18 = 0x52,
    PC19 = 0x53,
    PC20 = 0x54,
    PC21 = 0x55,
    PC22 = 0x56,
    PC23 = 0x57,
    PC24 = 0x58,
    PC25 = 0x59,
    PC26 = 0x5A,
    PC27 = 0x5B,
    PC28 = 0x5C,
    PC29 = 0x5D,
}

impl Pin {
    // Named default-pin aliases. SDK: ameba_pinmux.h : UART_LOG_RXD, UART_LOG_TXD, SWD_CLK, SWD_DAT
    pub const UART_LOG_RXD: Self = Self::PA2; // SDK: ameba_pinmux.h : UART_LOG_RXD = _PA_2
    pub const UART_LOG_TXD: Self = Self::PB20; // SDK: ameba_pinmux.h : UART_LOG_TXD = _PB_20
    pub const SWD_CLK: Self = Self::PA18; // SDK: ameba_pinmux.h : SWD_CLK = _PA_18
    pub const SWD_DAT: Self = Self::PA19; // SDK: ameba_pinmux.h : SWD_DAT = _PA_19
}

// ─────────────────────────────────────────────────────────────────────────────
// PinFunction — newtype (not enum) to handle duplicate discriminant values
// All 155 constants from SDK: ameba_pinmux.h : PINMUX_FUNCTION_*
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PinFunction(pub u32);

#[allow(non_upper_case_globals)]
impl PinFunction {
    pub const GPIO: Self = Self(0); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_GPIO
    pub const LOG_UART: Self = Self(1); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LOG_UART
    pub const SPIC0_FLASH: Self = Self(2); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SPIC0_FLASH
    pub const SPIC1_FLASH: Self = Self(3); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SPIC1_FLASH
    pub const SPIC1_PSRAM: Self = Self(4); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SPIC1_PSRAM
    pub const ADC: Self = Self(5); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_ADC
    pub const CAP_TOUCH: Self = Self(5); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_CAP_TOUCH (same value as ADC)
    pub const TSSI: Self = Self(6); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_TSSI
    pub const USB: Self = Self(7); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_USB
    pub const RMII: Self = Self(8); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_RMII
    pub const SPI0: Self = Self(9); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SPI0
    pub const SPI1: Self = Self(10); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SPI1
    pub const SWD: Self = Self(11); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SWD
    pub const SDIO_MST: Self = Self(12); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SDIO_MST
    pub const SDIO_SLV: Self = Self(13); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SDIO_SLV
    pub const SIC: Self = Self(14); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SIC
    pub const BT_IO: Self = Self(15); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_BT_IO
    pub const BT: Self = Self(16); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_BT
    pub const DEBUG: Self = Self(17); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_DEBUG
    pub const I2S0_BCLK: Self = Self(32); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_I2S0_BCLK
    pub const I2S0_MCLK: Self = Self(33); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_I2S0_MCLK
    pub const I2S0_WS: Self = Self(34); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_I2S0_WS
    pub const I2S0_DIO0: Self = Self(35); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_I2S0_DIO0
    pub const I2S0_DIO1: Self = Self(36); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_I2S0_DIO1
    pub const I2S0_DIO2: Self = Self(37); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_I2S0_DIO2
    pub const I2S0_DIO3: Self = Self(38); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_I2S0_DIO3
    pub const LCD_RGB_DCLK: Self = Self(39); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_RGB_DCLK
    pub const LCD_MCU_WR: Self = Self(39); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_MCU_WR (same value as LCD_RGB_DCLK)
    pub const LCD_RGB_DE: Self = Self(40); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_RGB_DE
    pub const LCD_MCU_CSX: Self = Self(40); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_MCU_CSX (same value as LCD_RGB_DE)
    pub const LCD_RGB_HSYNC: Self = Self(41); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_RGB_HSYNC
    pub const LCD_MCU_RD: Self = Self(41); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_MCU_RD (same value as LCD_RGB_HSYNC)
    pub const LCD_RGB_TE: Self = Self(42); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_RGB_TE
    pub const LCD_MCU_TE: Self = Self(42); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_MCU_TE (same value as LCD_RGB_TE)
    pub const LCD_RGB_VSYNC: Self = Self(43); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_RGB_VSYNC
    pub const LCD_MCU_VSYNC: Self = Self(43); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_MCU_VSYNC (same value as LCD_RGB_VSYNC)
    pub const LCD_MCU_DCX: Self = Self(44); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_MCU_DCX
    pub const LCD_D0: Self = Self(45); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_D0
    pub const LCD_D1: Self = Self(46); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_D1
    pub const LCD_D2: Self = Self(47); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_D2
    pub const LCD_D3: Self = Self(48); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_D3
    pub const LCD_D4: Self = Self(49); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_D4
    pub const LCD_D5: Self = Self(50); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_D5
    pub const LCD_D6: Self = Self(51); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_D6
    pub const LCD_D7: Self = Self(52); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_D7
    pub const LCD_D8: Self = Self(53); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_D8
    pub const LCD_D9: Self = Self(54); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_D9
    pub const LCD_D10: Self = Self(55); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_D10
    pub const LCD_D11: Self = Self(56); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_D11
    pub const LCD_D12: Self = Self(57); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_D12
    pub const LCD_D13: Self = Self(58); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_D13
    pub const LCD_D14: Self = Self(59); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_D14
    pub const LCD_D15: Self = Self(60); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_D15
    pub const LCD_D16: Self = Self(61); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_D16
    pub const LCD_D17: Self = Self(62); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_D17
    pub const LCD_D18: Self = Self(63); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_D18
    pub const LCD_D19: Self = Self(64); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_D19
    pub const LCD_D20: Self = Self(65); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_D20
    pub const LCD_D21: Self = Self(66); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_D21
    pub const LCD_D22: Self = Self(67); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_D22
    pub const LCD_D23: Self = Self(68); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LCD_D23
    pub const SD_M_CLK: Self = Self(69); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SD_M_CLK
    pub const SD_M_CMD: Self = Self(70); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SD_M_CMD
    pub const SD_M_D0: Self = Self(71); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SD_M_D0
    pub const SD_M_D1: Self = Self(72); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SD_M_D1
    pub const SD_M_D2: Self = Self(73); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SD_M_D2
    pub const SD_M_D3: Self = Self(74); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SD_M_D3
    pub const SPI0_CLK: Self = Self(75); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SPI0_CLK
    pub const SPI0_MISO: Self = Self(76); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SPI0_MISO
    pub const SPI0_MOSI: Self = Self(77); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SPI0_MOSI
    pub const SPI0_CS: Self = Self(78); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SPI0_CS
    pub const SPI1_CLK: Self = Self(79); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SPI1_CLK
    pub const SPI1_MISO: Self = Self(80); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SPI1_MISO
    pub const SPI1_MOSI: Self = Self(81); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SPI1_MOSI
    pub const SPI1_CS: Self = Self(82); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SPI1_CS
    pub const SWD_CLK: Self = Self(83); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SWD_CLK
    pub const SWD_DAT: Self = Self(84); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SWD_DAT
    pub const DMIC_CLK: Self = Self(85); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_DMIC_CLK
    pub const DMIC_DATA: Self = Self(86); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_DMIC_DATA
    pub const LED_SCL: Self = Self(87); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LED_SCL
    pub const LED_SDA: Self = Self(88); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LED_SDA
    pub const I2C0_SCL: Self = Self(89); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_I2C0_SCL
    pub const I2C0_SDA: Self = Self(90); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_I2C0_SDA
    pub const I2C1_SCL: Self = Self(91); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_I2C1_SCL
    pub const I2C1_SDA: Self = Self(92); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_I2C1_SDA
    pub const RMII_MDIO: Self = Self(93); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_RMII_MDIO
    pub const RMII_MDC: Self = Self(94); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_RMII_MDC
    pub const UART0_TXD: Self = Self(95); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_UART0_TXD
    pub const UART0_RXD: Self = Self(96); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_UART0_RXD
    pub const UART0_CTS: Self = Self(97); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_UART0_CTS
    pub const UART0_RTS: Self = Self(98); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_UART0_RTS
    pub const UART1_TXD: Self = Self(99); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_UART1_TXD
    pub const UART1_RXD: Self = Self(100); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_UART1_RXD
    pub const UART2_TXD: Self = Self(101); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_UART2_TXD
    pub const UART2_RXD: Self = Self(102); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_UART2_RXD
    pub const UART3_TXD: Self = Self(103); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_UART3_TXD
    pub const UART3_RXD: Self = Self(104); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_UART3_RXD
    pub const UART3_CTS: Self = Self(105); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_UART3_CTS
    pub const UART3_RTS: Self = Self(106); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_UART3_RTS
    pub const CAN0_TX: Self = Self(107); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_CAN0_TX
    pub const CAN0_RX: Self = Self(108); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_CAN0_RX
    pub const CAN1_TX: Self = Self(109); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_CAN1_TX
    pub const CAN1_RX: Self = Self(110); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_CAN1_RX
    pub const TIM4_PWM0: Self = Self(111); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_TIM4_PWM0
    pub const TIM4_PWM1: Self = Self(112); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_TIM4_PWM1
    pub const TIM4_PWM2: Self = Self(113); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_TIM4_PWM2
    pub const TIM4_PWM3: Self = Self(114); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_TIM4_PWM3
    pub const TIM5_PWM0: Self = Self(115); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_TIM5_PWM0
    pub const TIM5_PWM1: Self = Self(116); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_TIM5_PWM1
    pub const TIM5_PWM2: Self = Self(117); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_TIM5_PWM2
    pub const TIM5_PWM3: Self = Self(118); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_TIM5_PWM3
    pub const TIM6_PWM0: Self = Self(119); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_TIM6_PWM0
    pub const TIM6_PWM1: Self = Self(120); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_TIM6_PWM1
    pub const TIM6_PWM2: Self = Self(121); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_TIM6_PWM2
    pub const TIM6_PWM3: Self = Self(122); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_TIM6_PWM3
    pub const TIM7_PWM0: Self = Self(123); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_TIM7_PWM0
    pub const TIM7_PWM1: Self = Self(124); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_TIM7_PWM1
    pub const TIM7_PWM2: Self = Self(125); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_TIM7_PWM2
    pub const TIM7_PWM3: Self = Self(126); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_TIM7_PWM3
    pub const PWM_TIM4_TRIG: Self = Self(127); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_PWM_TIM4_TRIG
    pub const PWM_TIM5_TRIG: Self = Self(128); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_PWM_TIM5_TRIG
    pub const PWM_TIM6_TRIG: Self = Self(129); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_PWM_TIM6_TRIG
    pub const PWM_TIM7_TRIG: Self = Self(130); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_PWM_TIM7_TRIG
    pub const CAPT_TIM8_TRIG: Self = Self(131); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_CAPT_TIM8_TRIG
    pub const IR_TX: Self = Self(132); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_IR_TX
    pub const IR_RX: Self = Self(133); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_IR_RX
    pub const ANT_SEL_P: Self = Self(134); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_ANT_SEL_P
    pub const ANT_SEL_N: Self = Self(135); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_ANT_SEL_N
    pub const TRSW_P: Self = Self(136); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_TRSW_P
    pub const TRSW_N: Self = Self(137); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_TRSW_N
    pub const PA_EN0: Self = Self(138); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_PA_EN0
    pub const LNA_EN0: Self = Self(139); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LNA_EN0
    pub const PA_EN1: Self = Self(140); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_PA_EN1
    pub const LNA_EN1: Self = Self(141); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_LNA_EN1
    pub const BT_CLK_REQ: Self = Self(142); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_BT_CLK_REQ
    pub const WLAN_ACT: Self = Self(143); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_WLAN_ACT
    pub const BT_ACT: Self = Self(144); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_BT_ACT
    pub const BT_STE: Self = Self(145); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_BT_STE
    pub const BT_CK: Self = Self(146); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_BT_CK
    pub const ZB_REQ: Self = Self(147); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_ZB_REQ
    pub const ZB_GRANT: Self = Self(148); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_ZB_GRANT
    pub const ZB_PRI: Self = Self(149); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_ZB_PRI
    pub const BT_ANT_SW0: Self = Self(150); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_BT_ANT_SW0
    pub const BT_ANT_SW1: Self = Self(151); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_BT_ANT_SW1
    pub const BT_ANT_SW2: Self = Self(152); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_BT_ANT_SW2
    pub const BT_ANT_SW3: Self = Self(153); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_BT_ANT_SW3
    pub const EXT_CLK50M_IN: Self = Self(154); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_EXT_CLK50M_IN
    pub const EXT_CLK_OUT: Self = Self(155); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_EXT_CLK_OUT
    pub const UART_LOG_RXD: Self = Self(156); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_UART_LOG_RXD
    pub const UART_LOG_TXD: Self = Self(157); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_UART_LOG_TXD
    pub const SIC_CLK: Self = Self(158); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SIC_CLK
    pub const SIC_DAT: Self = Self(159); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_SIC_DAT
    pub const BT_FWLOG: Self = Self(160); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_BT_FWLOG
    pub const BT_I2C_SDA: Self = Self(161); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_BT_I2C_SDA
    pub const BT_I2C_SCL: Self = Self(162); // SDK: ameba_pinmux.h : PINMUX_FUNCTION_BT_I2C_SCL
}

// ─────────────────────────────────────────────────────────────────────────────
// Pull enum
// SDK: ameba_gpio.h : GPIO_PuPd_NOPULL / GPIO_PuPd_DOWN / GPIO_PuPd_UP / GPIO_PuPd_SHUTDOWN
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pull {
    NoPull = 0,   // SDK: ameba_gpio.h : GPIO_PuPd_NOPULL
    Down = 1,     // SDK: ameba_gpio.h : GPIO_PuPd_DOWN
    Up = 2,       // SDK: ameba_gpio.h : GPIO_PuPd_UP
    Shutdown = 3, // SDK: ameba_gpio.h : GPIO_PuPd_SHUTDOWN
}

// ─────────────────────────────────────────────────────────────────────────────
// DriveStrength enum
// SDK: ameba_pinmux.h : PAD_DRV_ABILITITY_LOW / MID_LOW / MID_HIGH / HIGH
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DriveStrength {
    Low = 0,     // SDK: ameba_pinmux.h : PAD_DRV_ABILITITY_LOW
    MidLow = 1,  // SDK: ameba_pinmux.h : PAD_DRV_ABILITITY_MID_LOW
    MidHigh = 2, // SDK: ameba_pinmux.h : PAD_DRV_ABILITITY_MID_HIGH
    High = 3,    // SDK: ameba_pinmux.h : PAD_DRV_ABILITITY_HIGH
}

// ─────────────────────────────────────────────────────────────────────────────
// SlewRate enum
// SDK: ameba_pinmux.h : PAD_SlewRate_Fast / PAD_SlewRate_Slow
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SlewRate {
    Fast = 0, // SDK: ameba_pinmux.h : PAD_SlewRate_Fast
    Slow = 1, // SDK: ameba_pinmux.h : PAD_SlewRate_Slow
}

// ─────────────────────────────────────────────────────────────────────────────
// Resistor enum
// SDK: ameba_pinmux.h : PAD_Resistor_LARGE / PAD_Resistor_SMALL
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Resistor {
    Large = 0, // SDK: ameba_pinmux.h : PAD_Resistor_LARGE
    Small = 1, // SDK: ameba_pinmux.h : PAD_Resistor_SMALL
}

// ─────────────────────────────────────────────────────────────────────────────
// Pinmux API
// ─────────────────────────────────────────────────────────────────────────────

/// Raw-pin core of [`set`], shared with the `gpio` module (which addresses pins
/// by their u32 PinName encoding rather than the [`Pin`] enum). The encodings
/// are identical — bits[7:5]=port, bits[4:0]=pin_number — so `Pin as u32` and the
/// gpio pin index are the same value.
pub(crate) fn set_raw(pin: u32, func: u32) {
    // Lazily seed SWD tracking from bootloader state.
    init_swd_tracking();

    let p = Pin::from_u8(pin as u8);

    // If the pin is currently SWD and being reconfigured to something else,
    // clear the global swd_pmux_en switch (releases PA18/PA19 hardware lock).
    let pin_val = pin_reg_read(p);
    if is_swd_func(pin_val & SEL_MASK) && !is_swd_func(func) {
        swdoff();
    }

    // When configuring an SWD function, release previously tracked SWD pins
    // so that only one SWD pair is active.
    //
    // NOTE: release by role, not by pin — when configuring CLK and DAT
    // separately, the second call must not release the pin set by the first.
    // Therefore we only release the tracked pin whose *role* matches the new
    // function being set.
    if is_swd_func(func) {
        let old_clk = get_swd_clk();
        let old_dat = get_swd_dat();

        if (func == PinFunction::SWD_CLK.0 || func == PinFunction::SWD.0)
            && old_clk != u8::MAX
            && old_clk != pin as u8
        {
            release_swd_pin(Pin::from_u8(old_clk));
        }
        if (func == PinFunction::SWD_DAT.0 || func == PinFunction::SWD.0)
            && old_dat != u8::MAX
            && old_dat != pin as u8
        {
            release_swd_pin(Pin::from_u8(old_dat));
        }

        // Track the new SWD pins. Function 11 (SWD) uses the same value for
        // both CLK and DAT — each pin configures itself as its role.
        if func == PinFunction::SWD_CLK.0 {
            set_swd_clk(pin as u8);
        } else if func == PinFunction::SWD_DAT.0 {
            set_swd_dat(pin as u8);
        } else {
            // Function 11 (SWD): track as whichever slot is empty.
            if old_clk == u8::MAX {
                set_swd_clk(pin as u8);
            } else {
                set_swd_dat(pin as u8);
            }
        }

        // Clear swd_pmux_en — only PA18/PA19 need it, and we're moving SWD
        // elsewhere. Programmable-zone SWD and other dedicated pairs work
        // with swd_pmux_en = 0.
        swdoff();
    }

    // Write the pinmux register.
    let ptr = PINMUX_PTR
        .cast::<u32>()
        .cast_mut()
        .wrapping_add(pin as usize);
    unsafe {
        let v = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, (v & !SEL_MASK) | (func & SEL_MASK));
    }
}

/// Configure the pinmux function for a pin.
pub fn set(pin: Pin, func: PinFunction) {
    set_raw(pin as u32, func.0);
}

/// Read the current pinmux function of a pin.
pub fn get(pin: Pin) -> PinFunction {
    // SDK: ameba_pinmux_rom.c : Pinmux_ConfigGet — PAD_GET_Px_SEL(val) = (val>>0) & 0xFF
    PinFunction(pin_reg_read(pin) & SEL_MASK)
}

/// Return true if swd_pmux_en is set (PA18/PA19 forced as SWD pins).
pub fn swd_is_enabled() -> bool {
    pad_regs().swd_sdd_ctrl().read().swd_pmux_en().bit_is_set()
}

/// Disable SWD debug port. Clears swd_pmux_en; must be called before repurposing
/// shared SWD pins to other functions.
pub fn swdoff() {
    // SDK: ameba_pinmux_rom.c : Pinmux_Swdoff —
    // sysreg_pad.h : REG_SWD_SDD_CTRL offset 0x1F4, PAD_BIT_SWD_PMUX_EN = bit 0
    pad_regs()
        .swd_sdd_ctrl()
        .modify(|_, w| w.swd_pmux_en().clear_bit());
}

// ─────────────────────────────────────────────────────────────────────────────
// SWD pin tracking — only one SWD pair can be active at a time
// ─────────────────────────────────────────────────────────────────────────────

/// Track which pins are currently configured as SWD (CLK or DAT).
/// `u8::MAX` means not yet configured.
struct PinState(UnsafeCell<u8>);
unsafe impl Sync for PinState {}
static SWD_CLK_PIN: PinState = PinState(UnsafeCell::new(u8::MAX));
static SWD_DAT_PIN: PinState = PinState(UnsafeCell::new(u8::MAX));

#[inline]
fn get_swd_clk() -> u8 {
    unsafe { *SWD_CLK_PIN.0.get() }
}

#[inline]
fn get_swd_dat() -> u8 {
    unsafe { *SWD_DAT_PIN.0.get() }
}

#[inline]
fn set_swd_clk(p: u8) {
    unsafe {
        *SWD_CLK_PIN.0.get() = p;
    }
}

#[inline]
fn set_swd_dat(p: u8) {
    unsafe {
        *SWD_DAT_PIN.0.get() = p;
    }
}

/// Initialize SWD pin tracking from hardware state.
///
/// After boot, ROM sets PA18/PA19 pinmux sel to 0x0B (SWD). This function
/// reads the actual hardware registers to detect if PA18/PA19 are configured
/// as SWD and seeds the tracking variables accordingly.
///
/// Idempotent — called lazily before the first SWD-related configuration.
fn init_swd_tracking() {
    let cur_clk = get_swd_clk();
    if cur_clk != u8::MAX {
        return; // already initialized
    }

    // Check if default SWD pins are currently configured as SWD (ROM default).
    let clk_val = pin_reg_read(Pin::SWD_CLK);
    let dat_val = pin_reg_read(Pin::SWD_DAT);
    if is_swd_func(clk_val & SEL_MASK) && is_swd_func(dat_val & SEL_MASK) {
        set_swd_clk(Pin::SWD_CLK as u8);
        set_swd_dat(Pin::SWD_DAT as u8);
    }
}

/// Check whether a function value is SWD-related.
fn is_swd_func(func: u32) -> bool {
    func == PinFunction::SWD.0 || func == PinFunction::SWD_CLK.0 || func == PinFunction::SWD_DAT.0
}

/// Release a pin's SWD pinmux — change it to GPIO (0) only if it is currently
/// SWD-related. If the pin is PA18 or PA19, clear swd_pmux_en first so the
/// register write actually takes effect (hardware locks sel to SWD when
/// swd_pmux_en = 1).
fn release_swd_pin(pin: Pin) {
    // SWD_CLK (PA18) / SWD_DAT (PA19) require swd_pmux_en = 0 before their sel register can be written.
    if pin == Pin::SWD_CLK || pin == Pin::SWD_DAT {
        swdoff();
    }
    let ptr = PINMUX_PTR
        .cast::<u32>()
        .cast_mut()
        .wrapping_add(pin as usize);
    unsafe {
        let val = core::ptr::read_volatile(ptr);
        if is_swd_func(val & SEL_MASK) {
            core::ptr::write_volatile(ptr, val & !SEL_MASK); // → GPIO(0)
        }
    }
}

impl Pin {
    /// Create a Pin from a u8 (inverse of `Pin as u8`).
    const fn from_u8(v: u8) -> Self {
        // SAFETY: valid discriminants are 0x00..=0x5D
        unsafe { core::mem::transmute(v) }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PAD API
// ─────────────────────────────────────────────────────────────────────────────

/// Enable the PAD output driver.
pub fn pad_enable(pin: Pin) {
    // SDK: ameba_rom_patch.c : PAD_CMD(pin, ENABLE) — clears PAD_BIT_Px_DIS (bit 15)
    pin_reg_rmw(pin, DIS_BIT, 0);
}

/// Disable (high-impedance) the PAD output driver.
pub fn pad_disable(pin: Pin) {
    // SDK: ameba_rom_patch.c : PAD_CMD(pin, DISABLE) — sets PAD_BIT_Px_DIS (bit 15)
    pin_reg_rmw(pin, DIS_BIT, DIS_BIT);
}

/// Raw-pin core of [`set_pull`], shared with the `gpio` module (see [`set_raw`]).
pub(crate) fn set_pull_raw(pin: u32, pull: Pull) {
    // SDK: ameba_pinmux_rom.c : PAD_PullCtrl —
    // sysreg_pad.h : REG_PAD_PU_1/2/3 (0x180/0x184/0x188), REG_PAD_PD_1/2/3 (0x190/0x194/0x198)
    // PORT_NUM(pin) selects the register set; PIN_NUM(pin) selects the bit.
    let bit = 1u32 << (pin & 0x1F);
    let port = pin >> 5;
    let p = pad_regs();
    macro_rules! apply {
        ($pu:ident, $pd:ident) => {{
            let pu = p.$pu().read().bits();
            let pd = p.$pd().read().bits();
            let (npu, npd) = match pull {
                Pull::Down => (pu & !bit, pd | bit),
                Pull::Up => (pu | bit, pd & !bit),
                Pull::NoPull => (pu & !bit, pd & !bit),
                Pull::Shutdown => return, // SDK has no branch for SHUTDOWN in PAD_PullCtrl
            };
            p.$pu().write(|w| unsafe { w.bits(npu) });
            p.$pd().write(|w| unsafe { w.bits(npd) });
        }};
    }
    match port {
        0 => apply!(pad_pu_1, pad_pd_1),
        1 => apply!(pad_pu_2, pad_pd_2),
        _ => apply!(pad_pu_3, pad_pd_3),
    }
}

/// Configure the pull resistor for a pin.
pub fn set_pull(pin: Pin, pull: Pull) {
    set_pull_raw(pin as u32, pull);
}

/// Read the current pull setting of a pin.
pub fn get_pull(pin: Pin) -> u32 {
    // SDK: ameba_rom_patch.c : PAD_PullCtrlGet —
    // returns GPIO_PuPd_UP=2, GPIO_PuPd_DOWN=1, GPIO_PuPd_NOPULL=0
    let bit_idx = pin as u32 & 0x1F;
    let port = (pin as u32) >> 5;
    let p = pad_regs();
    let (pu, pd) = match port {
        0 => (p.pad_pu_1().read().bits(), p.pad_pd_1().read().bits()),
        1 => (p.pad_pu_2().read().bits(), p.pad_pd_2().read().bits()),
        _ => (p.pad_pu_3().read().bits(), p.pad_pd_3().read().bits()),
    };
    let pu_bit = (pu >> bit_idx) & 1;
    let pd_bit = (pd >> bit_idx) & 1;
    if pu_bit == 1 && pd_bit == 0 {
        2
    } else if pu_bit == 0 && pd_bit == 1 {
        1
    } else {
        0
    }
}

/// Configure the pull resistor for a pin in sleep mode.
pub fn set_sleep_pull(pin: Pin, pull: Pull) {
    // SDK: ameba_pinmux_rom.c : PAD_SleepPullCtrl — same logic as PAD_PullCtrl but
    // sysreg_pad.h : REG_PAD_PU_SLP_1/2/3 (0x1A0/0x1A4/0x1A8), REG_PAD_PD_SLP_1/2/3 (0x1B0/0x1B4/0x1B8)
    let bit = 1u32 << (pin as u32 & 0x1F);
    let port = (pin as u32) >> 5;
    let p = pad_regs();
    macro_rules! apply {
        ($pu:ident, $pd:ident) => {{
            let pu = p.$pu().read().bits();
            let pd = p.$pd().read().bits();
            let (npu, npd) = match pull {
                Pull::Down => (pu & !bit, pd | bit),
                Pull::Up => (pu | bit, pd & !bit),
                Pull::NoPull => (pu & !bit, pd & !bit),
                Pull::Shutdown => return,
            };
            p.$pu().write(|w| unsafe { w.bits(npu) });
            p.$pd().write(|w| unsafe { w.bits(npd) });
        }};
    }
    match port {
        0 => apply!(pad_pu_slp_1, pad_pd_slp_1),
        1 => apply!(pad_pu_slp_2, pad_pd_slp_2),
        _ => apply!(pad_pu_slp_3, pad_pd_slp_3),
    }
}

/// Enable or disable the input receiver of a pin.
pub fn set_input(pin: Pin, enable: bool) {
    // SDK: ameba_rom_patch.c : PAD_InputCtrl — PAD_BIT_Px_IE (bit 8)
    pin_reg_rmw(pin, IE_BIT, if enable { IE_BIT } else { 0 });
}

/// Set the drive strength of a pin.
pub fn set_drive_strength(pin: Pin, strength: DriveStrength) {
    // SDK: ameba_rom_patch.c : PAD_DrvStrength — 4-level using e2 (bit 11) + e3 (bit 14)
    // PAD_DRV_ABILITITY_LOW=00, MID_LOW=e2, MID_HIGH=e3, HIGH=e2+e3
    let bits = match strength {
        DriveStrength::Low => 0,
        DriveStrength::MidLow => E2_BIT,
        DriveStrength::MidHigh => E3_BIT,
        DriveStrength::High => E2_BIT | E3_BIT,
    };
    pin_reg_rmw(pin, E2_BIT | E3_BIT, bits);
}

/// Enable or disable Schmitt trigger hysteresis on a pin.
pub fn set_schmitt(pin: Pin, enable: bool) {
    // SDK: ameba_pinmux_rom.c : PAD_SchmitCtrl — PAD_BIT_Px_SMT (bit 12)
    pin_reg_rmw(pin, SMT_BIT, if enable { SMT_BIT } else { 0 });
}

/// Set the output slew rate of a pin.
pub fn set_slew_rate(pin: Pin, rate: SlewRate) {
    // SDK: ameba_pinmux_rom.c : PAD_SlewRateCtrl — PAD_BIT_Px_SR (bit 13): 1=slow 0=fast
    pin_reg_rmw(pin, SR_BIT, if rate == SlewRate::Slow { SR_BIT } else { 0 });
}

/// Select the pull resistor size (large or small) for a pin.
pub fn set_resistor(pin: Pin, rtype: Resistor) {
    // SDK: ameba_pinmux_rom.c : PAD_ResistorCtrl — PAD_BIT_Px_PUPDC (bit 10): 1=small 0=large
    pin_reg_rmw(
        pin,
        PUPDC_BIT,
        if rtype == Resistor::Small {
            PUPDC_BIT
        } else {
            0
        },
    );
}
