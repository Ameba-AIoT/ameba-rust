#[doc = "Register `REG_AX_PWR_RU_LMT_TABLE5` reader"]
pub type R = crate::R<RegAxPwrRuLmtTable5Spec>;
#[doc = "Register `REG_AX_PWR_RU_LMT_TABLE5` writer"]
pub type W = crate::W<RegAxPwrRuLmtTable5Spec>;
#[doc = "Field `r_ru_lim_0tx_RU106_BW160_4_lv0` reader - s(7.1)"]
pub type RRuLim0txRu106Bw160_4Lv0R = crate::FieldReader;
#[doc = "Field `r_ru_lim_0tx_RU106_BW160_4_lv0` writer - s(7.1)"]
pub type RRuLim0txRu106Bw160_4Lv0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_ru_lim_0tx_RU106_BW160_5` reader - s(7.1)"]
pub type RRuLim0txRu106Bw160_5R = crate::FieldReader;
#[doc = "Field `r_ru_lim_0tx_RU106_BW160_5` writer - s(7.1)"]
pub type RRuLim0txRu106Bw160_5W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_ru_lim_0tx_RU106_BW160_6` reader - s(7.1)"]
pub type RRuLim0txRu106Bw160_6R = crate::FieldReader;
#[doc = "Field `r_ru_lim_0tx_RU106_BW160_6` writer - s(7.1)"]
pub type RRuLim0txRu106Bw160_6W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_ru_lim_0tx_RU106_BW160_7` reader - s(7.1)"]
pub type RRuLim0txRu106Bw160_7R = crate::FieldReader;
#[doc = "Field `r_ru_lim_0tx_RU106_BW160_7` writer - s(7.1)"]
pub type RRuLim0txRu106Bw160_7W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru106_bw160_4_lv0(&self) -> RRuLim0txRu106Bw160_4Lv0R {
        RRuLim0txRu106Bw160_4Lv0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru106_bw160_5(&self) -> RRuLim0txRu106Bw160_5R {
        RRuLim0txRu106Bw160_5R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru106_bw160_6(&self) -> RRuLim0txRu106Bw160_6R {
        RRuLim0txRu106Bw160_6R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru106_bw160_7(&self) -> RRuLim0txRu106Bw160_7R {
        RRuLim0txRu106Bw160_7R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru106_bw160_4_lv0(
        &mut self,
    ) -> RRuLim0txRu106Bw160_4Lv0W<'_, RegAxPwrRuLmtTable5Spec> {
        RRuLim0txRu106Bw160_4Lv0W::new(self, 0)
    }
    #[doc = "Bits 8:14 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru106_bw160_5(
        &mut self,
    ) -> RRuLim0txRu106Bw160_5W<'_, RegAxPwrRuLmtTable5Spec> {
        RRuLim0txRu106Bw160_5W::new(self, 8)
    }
    #[doc = "Bits 16:22 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru106_bw160_6(
        &mut self,
    ) -> RRuLim0txRu106Bw160_6W<'_, RegAxPwrRuLmtTable5Spec> {
        RRuLim0txRu106Bw160_6W::new(self, 16)
    }
    #[doc = "Bits 24:30 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru106_bw160_7(
        &mut self,
    ) -> RRuLim0txRu106Bw160_7W<'_, RegAxPwrRuLmtTable5Spec> {
        RRuLim0txRu106Bw160_7W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_ru_lmt_table5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_ru_lmt_table5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrRuLmtTable5Spec;
impl crate::RegisterSpec for RegAxPwrRuLmtTable5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_ru_lmt_table5::R`](R) reader structure"]
impl crate::Readable for RegAxPwrRuLmtTable5Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_ru_lmt_table5::W`](W) writer structure"]
impl crate::Writable for RegAxPwrRuLmtTable5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_RU_LMT_TABLE5 to value 0"]
impl crate::Resettable for RegAxPwrRuLmtTable5Spec {}
