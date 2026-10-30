#[doc = "Register `REG_AX_PWR_RU_LMT_TABLE7` reader"]
pub type R = crate::R<RegAxPwrRuLmtTable7Spec>;
#[doc = "Register `REG_AX_PWR_RU_LMT_TABLE7` writer"]
pub type W = crate::W<RegAxPwrRuLmtTable7Spec>;
#[doc = "Field `r_ru_lim_0tx_RU106_BW160_0_lv1` reader - s(7.1)"]
pub type RRuLim0txRu106Bw160_0Lv1R = crate::FieldReader;
#[doc = "Field `r_ru_lim_0tx_RU106_BW160_0_lv1` writer - s(7.1)"]
pub type RRuLim0txRu106Bw160_0Lv1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru106_bw160_0_lv1(&self) -> RRuLim0txRu106Bw160_0Lv1R {
        RRuLim0txRu106Bw160_0Lv1R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru106_bw160_0_lv1(
        &mut self,
    ) -> RRuLim0txRu106Bw160_0Lv1W<'_, RegAxPwrRuLmtTable7Spec> {
        RRuLim0txRu106Bw160_0Lv1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_ru_lmt_table7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_ru_lmt_table7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrRuLmtTable7Spec;
impl crate::RegisterSpec for RegAxPwrRuLmtTable7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_ru_lmt_table7::R`](R) reader structure"]
impl crate::Readable for RegAxPwrRuLmtTable7Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_ru_lmt_table7::W`](W) writer structure"]
impl crate::Writable for RegAxPwrRuLmtTable7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_RU_LMT_TABLE7 to value 0"]
impl crate::Resettable for RegAxPwrRuLmtTable7Spec {}
