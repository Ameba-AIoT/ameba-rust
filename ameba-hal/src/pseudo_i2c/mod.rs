use rtl8721f_pac::{LedcReg, ledc_reg};

// Byte offsets into the LED RAM area for the pseudo-I2C command byte and data.
// The PAC exposes reg_led_ram as a single 32-bit register, but the hardware
// allows byte-granular access within this RAM region.
const RAM_CMD_OFFSET: usize = 0x80; // SDK: ameba_ledc_pro.h : RAM_CMD_OFFSET
const RAM_DATA_OFFSET: usize = 0x81; // SDK: ameba_ledc_pro.h : RAM_DATA_OFFSET

const TRIGGER_SEQUENCE1: u32 = 0x6969_6969; // SDK: ameba_ledc_pro.h : TRIGGER_SEQUENCE1
const TRIGGER_SEQUENCE2: u32 = 0x9696_9696; // SDK: ameba_ledc_pro.h : TRIGGER_SEQUENCE2
const REG_PROTECT_DISABLE: u32 = 0xa5a5_a5a5; // SDK: ameba_ledc_pro.h : REG_PROTECT_DISABLE
const REG_PROTECT_ENABLE: u32 = 0x0000_0000; // SDK: ameba_ledc_pro.h : REG_PROTECT_ENABLE

fn regs() -> &'static ledc_reg::RegisterBlock {
    unsafe { &*LedcReg::PTR }
}

/// Pseudo-I2C driver using the LEDC controller.
///
/// Drives the hardware pseudo-I2C transmit path (no DMA; byte writes to LED RAM).
pub struct PseudoI2c(());

impl PseudoI2c {
    /// # Safety
    /// Only one `PseudoI2c` instance may exist at a time.
    pub unsafe fn steal() -> Self {
        PseudoI2c(())
    }

    fn set_protect_enabled(&self, enabled: bool) {
        let seq = if enabled {
            REG_PROTECT_ENABLE
        } else {
            REG_PROTECT_DISABLE
        };
        regs()
            .reg_sequence_check1()
            .write(|w| unsafe { w.bits(seq) });
    }

    /// Reset pseudo-I2C by toggling the LEDC clock.
    /// Call `ameba_hal::rcc::enable(rcc::Peripheral::LEDC)` before using this driver.
    /// Mirrors `pseudo_i2c_reset`.
    ///
    /// This function only clears the LED buffer internal to the controller.
    pub fn clear_buf(&self) {
        regs()
            .reg_led_paly_ctl0()
            .modify(|_, w| w.r_led_buf_clear().set_bit());
    }

    /// Set clock divider. Actual clock factor = `div * 2`, minimum 2.
    /// Mirrors `pseudo_i2c_set_clkdiv`.
    pub fn set_clk_div(&self, div: u8) {
        self.set_protect_enabled(false);
        regs()
            .reg_psdo_i2c_ctl()
            .modify(|_, w| unsafe { w.r_i2c_clk_div().bits(div) });
        self.set_protect_enabled(true);
    }

    /// Set the number of data bytes that will be sent (max 64). Mirrors
    /// `pseudo_i2c_set_len`.
    pub fn set_data_len(&self, len: u8) {
        self.set_protect_enabled(false);
        regs()
            .reg_psdo_i2c_ctl()
            .modify(|_, w| unsafe { w.r_i2c_data_num().bits(len) });
        self.set_protect_enabled(true);
    }

    /// Enable pseudo-I2C mode and set clock divider. Mirrors `pseudo_i2c_init`.
    pub fn init(&self, div: u8) {
        regs()
            .reg_psdo_i2c_ctl()
            .modify(|_, w| w.r_psedo_i2c_en().set_bit());
        self.set_protect_enabled(false);
        regs()
            .reg_psdo_i2c_ctl()
            .modify(|_, w| unsafe { w.r_i2c_clk_div().bits(div) });
        self.set_protect_enabled(true);
    }

    /// Transmit a command byte and optional data over pseudo-I2C.
    ///
    /// Blocks until the TX path is idle, writes to LED RAM, then triggers
    /// transmission. Returns when the sequence-pass flag is set.
    ///
    /// `data` may be `None` to send only the command byte.
    ///
    /// Mirrors `pseudo_i2c_write_ram`.
    pub fn write(&self, cmd: u8, data: Option<&[u8]>) {
        while !regs().reg_psdo_i2c_ctl().read().r_i2c_tx_status().bit() {}

        self.clear_buf();

        let base = LedcReg::PTR as *mut u8;
        unsafe {
            base.add(RAM_CMD_OFFSET).write_volatile(cmd);
        }

        match data {
            Some(buf) => {
                unsafe {
                    for (i, &byte) in buf.iter().enumerate() {
                        base.add(RAM_DATA_OFFSET + i).write_volatile(byte);
                    }
                }
                self.set_data_len((buf.len() + 1) as u8);
            }
            None => {
                self.set_data_len(1);
            }
        }

        regs()
            .reg_sequence_check0()
            .write(|w| unsafe { w.bits(TRIGGER_SEQUENCE1) });
        regs()
            .reg_sequence_check0()
            .write(|w| unsafe { w.bits(TRIGGER_SEQUENCE2) });

        while !regs().reg_led_paly_ctl0().read().r_seq_pass_flag().bit() {}
    }
}
