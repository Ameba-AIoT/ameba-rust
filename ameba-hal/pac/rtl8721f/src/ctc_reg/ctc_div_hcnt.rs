#[doc = "Register `CTC_DIV_HCNT` reader"]
pub type R = crate::R<CtcDivHcntSpec>;
#[doc = "Register `CTC_DIV_HCNT` writer"]
pub type W = crate::W<CtcDivHcntSpec>;
#[doc = "Field `ctc_clk_hcnt` reader - This field defines sample clock driver high level of chn. high level = (ctc_clk_hcnt +1)* ip_clk period"]
pub type CtcClkHcntR = crate::FieldReader<u16>;
#[doc = "Field `ctc_clk_hcnt` writer - This field defines sample clock driver high level of chn. high level = (ctc_clk_hcnt +1)* ip_clk period"]
pub type CtcClkHcntW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - This field defines sample clock driver high level of chn. high level = (ctc_clk_hcnt +1)* ip_clk period"]
    #[inline(always)]
    pub fn ctc_clk_hcnt(&self) -> CtcClkHcntR {
        CtcClkHcntR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - This field defines sample clock driver high level of chn. high level = (ctc_clk_hcnt +1)* ip_clk period"]
    #[inline(always)]
    pub fn ctc_clk_hcnt(&mut self) -> CtcClkHcntW<'_, CtcDivHcntSpec> {
        CtcClkHcntW::new(self, 0)
    }
}
#[doc = "Captouch Div Hcnt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctc_div_hcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctc_div_hcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtcDivHcntSpec;
impl crate::RegisterSpec for CtcDivHcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctc_div_hcnt::R`](R) reader structure"]
impl crate::Readable for CtcDivHcntSpec {}
#[doc = "`write(|w| ..)` method takes [`ctc_div_hcnt::W`](W) writer structure"]
impl crate::Writable for CtcDivHcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTC_DIV_HCNT to value 0x0f"]
impl crate::Resettable for CtcDivHcntSpec {
    const RESET_VALUE: u32 = 0x0f;
}
