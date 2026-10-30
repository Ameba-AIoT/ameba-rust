#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR12` reader"]
pub type R = crate::R<RegAxPwrTcmWrapperCr12Spec>;
#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR12` writer"]
pub type W = crate::W<RegAxPwrTcmWrapperCr12Spec>;
#[doc = "Field `r_force_pwrul_rpl_up_val` reader - "]
pub type RForcePwrulRplUpValR = crate::FieldReader<u16>;
#[doc = "Field `r_force_pwrul_rpl_up_val` writer - "]
pub type RForcePwrulRplUpValW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `r_force_pwrul_rpl_val` reader - "]
pub type RForcePwrulRplValR = crate::FieldReader<u16>;
#[doc = "Field `r_force_pwrul_rpl_val` writer - "]
pub type RForcePwrulRplValW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `r_force_pwrcom_ru_alloc_val` reader - "]
pub type RForcePwrcomRuAllocValR = crate::FieldReader;
#[doc = "Field `r_force_pwrcom_ru_alloc_val` writer - "]
pub type RForcePwrcomRuAllocValW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_force_pwrsr_txpwr_pd_val` reader - "]
pub type RForcePwrsrTxpwrPdValR = crate::FieldReader;
#[doc = "Field `r_force_pwrsr_txpwr_pd_val` writer - "]
pub type RForcePwrsrTxpwrPdValW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn r_force_pwrul_rpl_up_val(&self) -> RForcePwrulRplUpValR {
        RForcePwrulRplUpValR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    pub fn r_force_pwrul_rpl_val(&self) -> RForcePwrulRplValR {
        RForcePwrulRplValR::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bits 18:25"]
    #[inline(always)]
    pub fn r_force_pwrcom_ru_alloc_val(&self) -> RForcePwrcomRuAllocValR {
        RForcePwrcomRuAllocValR::new(((self.bits >> 18) & 0xff) as u8)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn r_force_pwrsr_txpwr_pd_val(&self) -> RForcePwrsrTxpwrPdValR {
        RForcePwrsrTxpwrPdValR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn r_force_pwrul_rpl_up_val(
        &mut self,
    ) -> RForcePwrulRplUpValW<'_, RegAxPwrTcmWrapperCr12Spec> {
        RForcePwrulRplUpValW::new(self, 0)
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    pub fn r_force_pwrul_rpl_val(&mut self) -> RForcePwrulRplValW<'_, RegAxPwrTcmWrapperCr12Spec> {
        RForcePwrulRplValW::new(self, 9)
    }
    #[doc = "Bits 18:25"]
    #[inline(always)]
    pub fn r_force_pwrcom_ru_alloc_val(
        &mut self,
    ) -> RForcePwrcomRuAllocValW<'_, RegAxPwrTcmWrapperCr12Spec> {
        RForcePwrcomRuAllocValW::new(self, 18)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn r_force_pwrsr_txpwr_pd_val(
        &mut self,
    ) -> RForcePwrsrTxpwrPdValW<'_, RegAxPwrTcmWrapperCr12Spec> {
        RForcePwrsrTxpwrPdValW::new(self, 26)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tcm_wrapper_cr12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tcm_wrapper_cr12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTcmWrapperCr12Spec;
impl crate::RegisterSpec for RegAxPwrTcmWrapperCr12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tcm_wrapper_cr12::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTcmWrapperCr12Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tcm_wrapper_cr12::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTcmWrapperCr12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TCM_WRAPPER_CR12 to value 0"]
impl crate::Resettable for RegAxPwrTcmWrapperCr12Spec {}
