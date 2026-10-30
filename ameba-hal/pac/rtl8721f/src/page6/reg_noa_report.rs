#[doc = "Register `REG_NOA_REPORT` reader"]
pub type R = crate::R<RegNoaReportSpec>;
#[doc = "Register `REG_NOA_REPORT` writer"]
pub type W = crate::W<RegNoaReportSpec>;
#[doc = "Field `NOA_RPT` reader - "]
pub type NoaRptR = crate::FieldReader<u32>;
#[doc = "Field `NOA_RPT` writer - "]
pub type NoaRptW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn noa_rpt(&self) -> NoaRptR {
        NoaRptR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn noa_rpt(&mut self) -> NoaRptW<'_, RegNoaReportSpec> {
        NoaRptW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_noa_report::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_noa_report::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNoaReportSpec;
impl crate::RegisterSpec for RegNoaReportSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_noa_report::R`](R) reader structure"]
impl crate::Readable for RegNoaReportSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_noa_report::W`](W) writer structure"]
impl crate::Writable for RegNoaReportSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NOA_REPORT to value 0"]
impl crate::Resettable for RegNoaReportSpec {}
