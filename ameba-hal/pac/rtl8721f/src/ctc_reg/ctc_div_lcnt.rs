#[doc = "Register `CTC_DIV_LCNT` reader"]
pub type R = crate::R<CtcDivLcntSpec>;
#[doc = "Register `CTC_DIV_LCNT` writer"]
pub type W = crate::W<CtcDivLcntSpec>;
#[doc = "Field `ctc_clk_lcnt0` reader - This field defines sample clock driver lower level of chn lower level = ctc_clk_lcnt0 * ip_clk period"]
pub type CtcClkLcnt0R = crate::FieldReader<u16>;
#[doc = "Field `ctc_clk_lcnt0` writer - This field defines sample clock driver lower level of chn lower level = ctc_clk_lcnt0 * ip_clk period"]
pub type CtcClkLcnt0W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `ctc_clk_lcnt1` reader - This field defines sample clock driver lower level of chn. lower level = ctc_clk_lcnt1 * ip_clk period"]
pub type CtcClkLcnt1R = crate::FieldReader<u16>;
#[doc = "Field `ctc_clk_lcnt1` writer - This field defines sample clock driver lower level of chn. lower level = ctc_clk_lcnt1 * ip_clk period"]
pub type CtcClkLcnt1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `ctc_clk_lcnt2` reader - This field defines sample clock driver lower level of chn. lower level = ctc_clk_lcnt2 * ip_clk period"]
pub type CtcClkLcnt2R = crate::FieldReader<u16>;
#[doc = "Field `ctc_clk_lcnt2` writer - This field defines sample clock driver lower level of chn. lower level = ctc_clk_lcnt2 * ip_clk period"]
pub type CtcClkLcnt2W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - This field defines sample clock driver lower level of chn lower level = ctc_clk_lcnt0 * ip_clk period"]
    #[inline(always)]
    pub fn ctc_clk_lcnt0(&self) -> CtcClkLcnt0R {
        CtcClkLcnt0R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - This field defines sample clock driver lower level of chn. lower level = ctc_clk_lcnt1 * ip_clk period"]
    #[inline(always)]
    pub fn ctc_clk_lcnt1(&self) -> CtcClkLcnt1R {
        CtcClkLcnt1R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bits 18:26 - This field defines sample clock driver lower level of chn. lower level = ctc_clk_lcnt2 * ip_clk period"]
    #[inline(always)]
    pub fn ctc_clk_lcnt2(&self) -> CtcClkLcnt2R {
        CtcClkLcnt2R::new(((self.bits >> 18) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - This field defines sample clock driver lower level of chn lower level = ctc_clk_lcnt0 * ip_clk period"]
    #[inline(always)]
    pub fn ctc_clk_lcnt0(&mut self) -> CtcClkLcnt0W<'_, CtcDivLcntSpec> {
        CtcClkLcnt0W::new(self, 0)
    }
    #[doc = "Bits 9:17 - This field defines sample clock driver lower level of chn. lower level = ctc_clk_lcnt1 * ip_clk period"]
    #[inline(always)]
    pub fn ctc_clk_lcnt1(&mut self) -> CtcClkLcnt1W<'_, CtcDivLcntSpec> {
        CtcClkLcnt1W::new(self, 9)
    }
    #[doc = "Bits 18:26 - This field defines sample clock driver lower level of chn. lower level = ctc_clk_lcnt2 * ip_clk period"]
    #[inline(always)]
    pub fn ctc_clk_lcnt2(&mut self) -> CtcClkLcnt2W<'_, CtcDivLcntSpec> {
        CtcClkLcnt2W::new(self, 18)
    }
}
#[doc = "Captouch Div Lcnt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctc_div_lcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctc_div_lcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtcDivLcntSpec;
impl crate::RegisterSpec for CtcDivLcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctc_div_lcnt::R`](R) reader structure"]
impl crate::Readable for CtcDivLcntSpec {}
#[doc = "`write(|w| ..)` method takes [`ctc_div_lcnt::W`](W) writer structure"]
impl crate::Writable for CtcDivLcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTC_DIV_LCNT to value 0x0040_2010"]
impl crate::Resettable for CtcDivLcntSpec {
    const RESET_VALUE: u32 = 0x0040_2010;
}
