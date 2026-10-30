#[doc = "Register `REG_NOA_REPORT_2` reader"]
pub type R = crate::R<RegNoaReport2Spec>;
#[doc = "Register `REG_NOA_REPORT_2` writer"]
pub type W = crate::W<RegNoaReport2Spec>;
#[doc = "Field `NOA_RPT_2` reader - "]
pub type NoaRpt2R = crate::FieldReader<u32>;
#[doc = "Field `NOA_RPT_2` writer - "]
pub type NoaRpt2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn noa_rpt_2(&self) -> NoaRpt2R {
        NoaRpt2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn noa_rpt_2(&mut self) -> NoaRpt2W<'_, RegNoaReport2Spec> {
        NoaRpt2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_noa_report_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_noa_report_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNoaReport2Spec;
impl crate::RegisterSpec for RegNoaReport2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_noa_report_2::R`](R) reader structure"]
impl crate::Readable for RegNoaReport2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_noa_report_2::W`](W) writer structure"]
impl crate::Writable for RegNoaReport2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NOA_REPORT_2 to value 0"]
impl crate::Resettable for RegNoaReport2Spec {}
