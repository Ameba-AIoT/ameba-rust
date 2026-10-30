#[doc = "Register `REG_AX_PWR_UL_CTRL0` reader"]
pub type R = crate::R<RegAxPwrUlCtrl0Spec>;
#[doc = "Register `REG_AX_PWR_UL_CTRL0` writer"]
pub type W = crate::W<RegAxPwrUlCtrl0Spec>;
#[doc = "Field `r_PWR_error_toler` reader - S(9.2) STA_UL"]
pub type RPwrErrorTolerR = crate::FieldReader;
#[doc = "Field `r_PWR_error_toler` writer - S(9.2) STA_UL"]
pub type RPwrErrorTolerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_PWR_bb_min_dbm` reader - S(9.2) STA_UL"]
pub type RPwrBbMinDbmR = crate::FieldReader<u16>;
#[doc = "Field `r_PWR_bb_min_dbm` writer - S(9.2) STA_UL"]
pub type RPwrBbMinDbmW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `r_PL_toler_range` reader - U(9.2) STA_UL"]
pub type RPlTolerRangeR = crate::FieldReader<u16>;
#[doc = "Field `r_PL_toler_range` writer - U(9.2) STA_UL"]
pub type RPlTolerRangeW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `r_bypass_abort_tx_flag` reader - 1: force off abort_tx flag STA_UL"]
pub type RBypassAbortTxFlagR = crate::BitReader;
#[doc = "Field `r_bypass_abort_tx_flag` writer - 1: force off abort_tx flag STA_UL"]
pub type RBypassAbortTxFlagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - S(9.2) STA_UL"]
    #[inline(always)]
    pub fn r_pwr_error_toler(&self) -> RPwrErrorTolerR {
        RPwrErrorTolerR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 9:17 - S(9.2) STA_UL"]
    #[inline(always)]
    pub fn r_pwr_bb_min_dbm(&self) -> RPwrBbMinDbmR {
        RPwrBbMinDbmR::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bits 20:28 - U(9.2) STA_UL"]
    #[inline(always)]
    pub fn r_pl_toler_range(&self) -> RPlTolerRangeR {
        RPlTolerRangeR::new(((self.bits >> 20) & 0x01ff) as u16)
    }
    #[doc = "Bit 29 - 1: force off abort_tx flag STA_UL"]
    #[inline(always)]
    pub fn r_bypass_abort_tx_flag(&self) -> RBypassAbortTxFlagR {
        RBypassAbortTxFlagR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - S(9.2) STA_UL"]
    #[inline(always)]
    pub fn r_pwr_error_toler(&mut self) -> RPwrErrorTolerW<'_, RegAxPwrUlCtrl0Spec> {
        RPwrErrorTolerW::new(self, 0)
    }
    #[doc = "Bits 9:17 - S(9.2) STA_UL"]
    #[inline(always)]
    pub fn r_pwr_bb_min_dbm(&mut self) -> RPwrBbMinDbmW<'_, RegAxPwrUlCtrl0Spec> {
        RPwrBbMinDbmW::new(self, 9)
    }
    #[doc = "Bits 20:28 - U(9.2) STA_UL"]
    #[inline(always)]
    pub fn r_pl_toler_range(&mut self) -> RPlTolerRangeW<'_, RegAxPwrUlCtrl0Spec> {
        RPlTolerRangeW::new(self, 20)
    }
    #[doc = "Bit 29 - 1: force off abort_tx flag STA_UL"]
    #[inline(always)]
    pub fn r_bypass_abort_tx_flag(&mut self) -> RBypassAbortTxFlagW<'_, RegAxPwrUlCtrl0Spec> {
        RBypassAbortTxFlagW::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_ul_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_ul_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrUlCtrl0Spec;
impl crate::RegisterSpec for RegAxPwrUlCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_ul_ctrl0::R`](R) reader structure"]
impl crate::Readable for RegAxPwrUlCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_ul_ctrl0::W`](W) writer structure"]
impl crate::Writable for RegAxPwrUlCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_UL_CTRL0 to value 0x0003_f808"]
impl crate::Resettable for RegAxPwrUlCtrl0Spec {
    const RESET_VALUE: u32 = 0x0003_f808;
}
