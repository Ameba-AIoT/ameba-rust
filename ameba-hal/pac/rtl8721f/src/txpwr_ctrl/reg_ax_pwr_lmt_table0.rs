#[doc = "Register `REG_AX_PWR_LMT_TABLE0` reader"]
pub type R = crate::R<RegAxPwrLmtTable0Spec>;
#[doc = "Register `REG_AX_PWR_LMT_TABLE0` writer"]
pub type W = crate::W<RegAxPwrLmtTable0Spec>;
#[doc = "Field `r_txagc_max_cck_1tx_BW20M` reader - s(7.1)"]
pub type RTxagcMaxCck1txBw20mR = crate::FieldReader;
#[doc = "Field `r_txagc_max_cck_1tx_BW20M` writer - s(7.1)"]
pub type RTxagcMaxCck1txBw20mW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_max_cck_1tx_bw20m(&self) -> RTxagcMaxCck1txBw20mR {
        RTxagcMaxCck1txBw20mR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_max_cck_1tx_bw20m(
        &mut self,
    ) -> RTxagcMaxCck1txBw20mW<'_, RegAxPwrLmtTable0Spec> {
        RTxagcMaxCck1txBw20mW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_lmt_table0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_lmt_table0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrLmtTable0Spec;
impl crate::RegisterSpec for RegAxPwrLmtTable0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_lmt_table0::R`](R) reader structure"]
impl crate::Readable for RegAxPwrLmtTable0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_lmt_table0::W`](W) writer structure"]
impl crate::Writable for RegAxPwrLmtTable0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_LMT_TABLE0 to value 0"]
impl crate::Resettable for RegAxPwrLmtTable0Spec {}
