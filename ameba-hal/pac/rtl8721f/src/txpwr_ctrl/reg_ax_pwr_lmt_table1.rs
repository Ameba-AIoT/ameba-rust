#[doc = "Register `REG_AX_PWR_LMT_TABLE1` reader"]
pub type R = crate::R<RegAxPwrLmtTable1Spec>;
#[doc = "Register `REG_AX_PWR_LMT_TABLE1` writer"]
pub type W = crate::W<RegAxPwrLmtTable1Spec>;
#[doc = "Field `r_txagc_max_legacy_non_dup_1tx` reader - s(7.1)"]
pub type RTxagcMaxLegacyNonDup1txR = crate::FieldReader;
#[doc = "Field `r_txagc_max_legacy_non_dup_1tx` writer - s(7.1)"]
pub type RTxagcMaxLegacyNonDup1txW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_txagc_max_1tx_BW20_0` reader - s(7.1)"]
pub type RTxagcMax1txBw20_0R = crate::FieldReader;
#[doc = "Field `r_txagc_max_1tx_BW20_0` writer - s(7.1)"]
pub type RTxagcMax1txBw20_0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_max_legacy_non_dup_1tx(&self) -> RTxagcMaxLegacyNonDup1txR {
        RTxagcMaxLegacyNonDup1txR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_max_1tx_bw20_0(&self) -> RTxagcMax1txBw20_0R {
        RTxagcMax1txBw20_0R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_max_legacy_non_dup_1tx(
        &mut self,
    ) -> RTxagcMaxLegacyNonDup1txW<'_, RegAxPwrLmtTable1Spec> {
        RTxagcMaxLegacyNonDup1txW::new(self, 0)
    }
    #[doc = "Bits 16:22 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_max_1tx_bw20_0(&mut self) -> RTxagcMax1txBw20_0W<'_, RegAxPwrLmtTable1Spec> {
        RTxagcMax1txBw20_0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_lmt_table1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_lmt_table1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrLmtTable1Spec;
impl crate::RegisterSpec for RegAxPwrLmtTable1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_lmt_table1::R`](R) reader structure"]
impl crate::Readable for RegAxPwrLmtTable1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_lmt_table1::W`](W) writer structure"]
impl crate::Writable for RegAxPwrLmtTable1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_LMT_TABLE1 to value 0"]
impl crate::Resettable for RegAxPwrLmtTable1Spec {}
