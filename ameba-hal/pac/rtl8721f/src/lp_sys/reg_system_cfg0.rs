#[doc = "Register `REG_SYSTEM_CFG0` reader"]
pub type R = crate::R<RegSystemCfg0Spec>;
#[doc = "Field `hw_cpu_id` reader - Different CPU reading this register will return different value. If read by KR4: return 0 If read by KM4: return 1 Other will return 2"]
pub type HwCpuIdR = crate::FieldReader;
#[doc = "Field `hw_chip_type` reader - Chip type 00: ASIC 01: FPGA 10: PALADIUM 11: RTL_SIM"]
pub type HwChipTypeR = crate::FieldReader;
#[doc = "Field `ptrp_otpbyp` reader - 1: force efuse autoload fail"]
pub type PtrpOtpbypR = crate::BitReader;
#[doc = "Field `ptrp_tmode` reader - 0: normal mode 1: test mode"]
pub type PtrpTmodeR = crate::BitReader;
#[doc = "Field `ptrp_uartdl` reader - Power on trapped uart download value 0: Normal boot 1: uartdownload boot TODO: it may be overwrited by efuse autoload"]
pub type PtrpUartdlR = crate::BitReader;
#[doc = "Field `ptrp_icfg` reader - Trapped ICFG data value 0: normall mode XXXX: the other mode (Test Mode)"]
pub type PtrpIcfgR = crate::FieldReader;
#[doc = "Field `ptrp_boot_int` reader - "]
pub type PtrpBootIntR = crate::BitReader;
#[doc = "Field `ptrp_boot_nor` reader - "]
pub type PtrpBootNorR = crate::BitReader;
#[doc = "Field `sys_pll_valid` reader - Indicator sys pll is ready"]
pub type SysPllValidR = crate::BitReader;
#[doc = "Field `xtal_valid` reader - Indicator xtal 40M is ready"]
pub type XtalValidR = crate::BitReader;
#[doc = "Field `usb_pll_valid` reader - Indicator usb pll is ready"]
pub type UsbPllValidR = crate::BitReader;
#[doc = "Field `eng_mode` reader - Indicate PKG_ENG_SEL pad value 0: normal package; 1: engineering mode"]
pub type EngModeR = crate::BitReader;
impl R {
    #[doc = "Bits 6:7 - Different CPU reading this register will return different value. If read by KR4: return 0 If read by KM4: return 1 Other will return 2"]
    #[inline(always)]
    pub fn hw_cpu_id(&self) -> HwCpuIdR {
        HwCpuIdR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Chip type 00: ASIC 01: FPGA 10: PALADIUM 11: RTL_SIM"]
    #[inline(always)]
    pub fn hw_chip_type(&self) -> HwChipTypeR {
        HwChipTypeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - 1: force efuse autoload fail"]
    #[inline(always)]
    pub fn ptrp_otpbyp(&self) -> PtrpOtpbypR {
        PtrpOtpbypR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 0: normal mode 1: test mode"]
    #[inline(always)]
    pub fn ptrp_tmode(&self) -> PtrpTmodeR {
        PtrpTmodeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Power on trapped uart download value 0: Normal boot 1: uartdownload boot TODO: it may be overwrited by efuse autoload"]
    #[inline(always)]
    pub fn ptrp_uartdl(&self) -> PtrpUartdlR {
        PtrpUartdlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Trapped ICFG data value 0: normall mode XXXX: the other mode (Test Mode)"]
    #[inline(always)]
    pub fn ptrp_icfg(&self) -> PtrpIcfgR {
        PtrpIcfgR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ptrp_boot_int(&self) -> PtrpBootIntR {
        PtrpBootIntR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ptrp_boot_nor(&self) -> PtrpBootNorR {
        PtrpBootNorR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Indicator sys pll is ready"]
    #[inline(always)]
    pub fn sys_pll_valid(&self) -> SysPllValidR {
        SysPllValidR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Indicator xtal 40M is ready"]
    #[inline(always)]
    pub fn xtal_valid(&self) -> XtalValidR {
        XtalValidR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Indicator usb pll is ready"]
    #[inline(always)]
    pub fn usb_pll_valid(&self) -> UsbPllValidR {
        UsbPllValidR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Indicate PKG_ENG_SEL pad value 0: normal package; 1: engineering mode"]
    #[inline(always)]
    pub fn eng_mode(&self) -> EngModeR {
        EngModeR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_system_cfg0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSystemCfg0Spec;
impl crate::RegisterSpec for RegSystemCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_system_cfg0::R`](R) reader structure"]
impl crate::Readable for RegSystemCfg0Spec {}
#[doc = "`reset()` method sets REG_SYSTEM_CFG0 to value 0"]
impl crate::Resettable for RegSystemCfg0Spec {}
