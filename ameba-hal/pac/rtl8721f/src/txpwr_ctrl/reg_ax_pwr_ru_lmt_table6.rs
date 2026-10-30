#[doc = "Register `REG_AX_PWR_RU_LMT_TABLE6` reader"]
pub type R = crate::R<RegAxPwrRuLmtTable6Spec>;
#[doc = "Register `REG_AX_PWR_RU_LMT_TABLE6` writer"]
pub type W = crate::W<RegAxPwrRuLmtTable6Spec>;
#[doc = "Field `r_ru_lim_0tx_RU26_BW160_0_lv1` reader - s(7.1)"]
pub type RRuLim0txRu26Bw160_0Lv1R = crate::FieldReader;
#[doc = "Field `r_ru_lim_0tx_RU26_BW160_0_lv1` writer - s(7.1)"]
pub type RRuLim0txRu26Bw160_0Lv1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_ru_lim_0tx_RU26_BW160_0_lv2` reader - s(7.1)"]
pub type RRuLim0txRu26Bw160_0Lv2R = crate::FieldReader;
#[doc = "Field `r_ru_lim_0tx_RU26_BW160_0_lv2` writer - s(7.1)"]
pub type RRuLim0txRu26Bw160_0Lv2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_ru_lim_0tx_RU52_BW160_0_lv1` reader - s(7.1)"]
pub type RRuLim0txRu52Bw160_0Lv1R = crate::FieldReader;
#[doc = "Field `r_ru_lim_0tx_RU52_BW160_0_lv1` writer - s(7.1)"]
pub type RRuLim0txRu52Bw160_0Lv1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_ru_lim_0tx_RU52_BW160_0_lv2` reader - s(7.1)"]
pub type RRuLim0txRu52Bw160_0Lv2R = crate::FieldReader;
#[doc = "Field `r_ru_lim_0tx_RU52_BW160_0_lv2` writer - s(7.1)"]
pub type RRuLim0txRu52Bw160_0Lv2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru26_bw160_0_lv1(&self) -> RRuLim0txRu26Bw160_0Lv1R {
        RRuLim0txRu26Bw160_0Lv1R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru26_bw160_0_lv2(&self) -> RRuLim0txRu26Bw160_0Lv2R {
        RRuLim0txRu26Bw160_0Lv2R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru52_bw160_0_lv1(&self) -> RRuLim0txRu52Bw160_0Lv1R {
        RRuLim0txRu52Bw160_0Lv1R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru52_bw160_0_lv2(&self) -> RRuLim0txRu52Bw160_0Lv2R {
        RRuLim0txRu52Bw160_0Lv2R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru26_bw160_0_lv1(
        &mut self,
    ) -> RRuLim0txRu26Bw160_0Lv1W<'_, RegAxPwrRuLmtTable6Spec> {
        RRuLim0txRu26Bw160_0Lv1W::new(self, 0)
    }
    #[doc = "Bits 8:14 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru26_bw160_0_lv2(
        &mut self,
    ) -> RRuLim0txRu26Bw160_0Lv2W<'_, RegAxPwrRuLmtTable6Spec> {
        RRuLim0txRu26Bw160_0Lv2W::new(self, 8)
    }
    #[doc = "Bits 16:22 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru52_bw160_0_lv1(
        &mut self,
    ) -> RRuLim0txRu52Bw160_0Lv1W<'_, RegAxPwrRuLmtTable6Spec> {
        RRuLim0txRu52Bw160_0Lv1W::new(self, 16)
    }
    #[doc = "Bits 24:30 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru52_bw160_0_lv2(
        &mut self,
    ) -> RRuLim0txRu52Bw160_0Lv2W<'_, RegAxPwrRuLmtTable6Spec> {
        RRuLim0txRu52Bw160_0Lv2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_ru_lmt_table6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_ru_lmt_table6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrRuLmtTable6Spec;
impl crate::RegisterSpec for RegAxPwrRuLmtTable6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_ru_lmt_table6::R`](R) reader structure"]
impl crate::Readable for RegAxPwrRuLmtTable6Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_ru_lmt_table6::W`](W) writer structure"]
impl crate::Writable for RegAxPwrRuLmtTable6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_RU_LMT_TABLE6 to value 0"]
impl crate::Resettable for RegAxPwrRuLmtTable6Spec {}
