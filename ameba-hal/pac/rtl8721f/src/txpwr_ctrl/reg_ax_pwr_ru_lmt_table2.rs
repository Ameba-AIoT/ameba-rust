#[doc = "Register `REG_AX_PWR_RU_LMT_TABLE2` reader"]
pub type R = crate::R<RegAxPwrRuLmtTable2Spec>;
#[doc = "Register `REG_AX_PWR_RU_LMT_TABLE2` writer"]
pub type W = crate::W<RegAxPwrRuLmtTable2Spec>;
#[doc = "Field `r_ru_lim_0tx_RU52_BW160_0_lv0` reader - s(7.1)"]
pub type RRuLim0txRu52Bw160_0Lv0R = crate::FieldReader;
#[doc = "Field `r_ru_lim_0tx_RU52_BW160_0_lv0` writer - s(7.1)"]
pub type RRuLim0txRu52Bw160_0Lv0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_ru_lim_0tx_RU52_BW160_1` reader - s(7.1)"]
pub type RRuLim0txRu52Bw160_1R = crate::FieldReader;
#[doc = "Field `r_ru_lim_0tx_RU52_BW160_1` writer - s(7.1)"]
pub type RRuLim0txRu52Bw160_1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_ru_lim_0tx_RU52_BW160_2` reader - s(7.1)"]
pub type RRuLim0txRu52Bw160_2R = crate::FieldReader;
#[doc = "Field `r_ru_lim_0tx_RU52_BW160_2` writer - s(7.1)"]
pub type RRuLim0txRu52Bw160_2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_ru_lim_0tx_RU52_BW160_3` reader - s(7.1)"]
pub type RRuLim0txRu52Bw160_3R = crate::FieldReader;
#[doc = "Field `r_ru_lim_0tx_RU52_BW160_3` writer - s(7.1)"]
pub type RRuLim0txRu52Bw160_3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru52_bw160_0_lv0(&self) -> RRuLim0txRu52Bw160_0Lv0R {
        RRuLim0txRu52Bw160_0Lv0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru52_bw160_1(&self) -> RRuLim0txRu52Bw160_1R {
        RRuLim0txRu52Bw160_1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru52_bw160_2(&self) -> RRuLim0txRu52Bw160_2R {
        RRuLim0txRu52Bw160_2R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru52_bw160_3(&self) -> RRuLim0txRu52Bw160_3R {
        RRuLim0txRu52Bw160_3R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru52_bw160_0_lv0(
        &mut self,
    ) -> RRuLim0txRu52Bw160_0Lv0W<'_, RegAxPwrRuLmtTable2Spec> {
        RRuLim0txRu52Bw160_0Lv0W::new(self, 0)
    }
    #[doc = "Bits 8:14 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru52_bw160_1(
        &mut self,
    ) -> RRuLim0txRu52Bw160_1W<'_, RegAxPwrRuLmtTable2Spec> {
        RRuLim0txRu52Bw160_1W::new(self, 8)
    }
    #[doc = "Bits 16:22 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru52_bw160_2(
        &mut self,
    ) -> RRuLim0txRu52Bw160_2W<'_, RegAxPwrRuLmtTable2Spec> {
        RRuLim0txRu52Bw160_2W::new(self, 16)
    }
    #[doc = "Bits 24:30 - s(7.1)"]
    #[inline(always)]
    pub fn r_ru_lim_0tx_ru52_bw160_3(
        &mut self,
    ) -> RRuLim0txRu52Bw160_3W<'_, RegAxPwrRuLmtTable2Spec> {
        RRuLim0txRu52Bw160_3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_ru_lmt_table2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_ru_lmt_table2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrRuLmtTable2Spec;
impl crate::RegisterSpec for RegAxPwrRuLmtTable2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_ru_lmt_table2::R`](R) reader structure"]
impl crate::Readable for RegAxPwrRuLmtTable2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_ru_lmt_table2::W`](W) writer structure"]
impl crate::Writable for RegAxPwrRuLmtTable2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_RU_LMT_TABLE2 to value 0"]
impl crate::Resettable for RegAxPwrRuLmtTable2Spec {}
