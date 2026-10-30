#[doc = "Register `REG_NOA_REPORT_1` reader"]
pub type R = crate::R<RegNoaReport1Spec>;
#[doc = "Register `REG_NOA_REPORT_1` writer"]
pub type W = crate::W<RegNoaReport1Spec>;
#[doc = "Field `NOA_RPT_1` reader - "]
pub type NoaRpt1R = crate::FieldReader<u32>;
#[doc = "Field `NOA_RPT_1` writer - "]
pub type NoaRpt1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn noa_rpt_1(&self) -> NoaRpt1R {
        NoaRpt1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn noa_rpt_1(&mut self) -> NoaRpt1W<'_, RegNoaReport1Spec> {
        NoaRpt1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_noa_report_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_noa_report_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNoaReport1Spec;
impl crate::RegisterSpec for RegNoaReport1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_noa_report_1::R`](R) reader structure"]
impl crate::Readable for RegNoaReport1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_noa_report_1::W`](W) writer structure"]
impl crate::Writable for RegNoaReport1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NOA_REPORT_1 to value 0"]
impl crate::Resettable for RegNoaReport1Spec {}
