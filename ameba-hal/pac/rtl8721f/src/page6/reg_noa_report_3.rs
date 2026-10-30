#[doc = "Register `REG_NOA_REPORT_3` reader"]
pub type R = crate::R<RegNoaReport3Spec>;
#[doc = "Register `REG_NOA_REPORT_3` writer"]
pub type W = crate::W<RegNoaReport3Spec>;
#[doc = "Field `NOA_RPT_3` reader - "]
pub type NoaRpt3R = crate::FieldReader;
#[doc = "Field `NOA_RPT_3` writer - "]
pub type NoaRpt3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn noa_rpt_3(&self) -> NoaRpt3R {
        NoaRpt3R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn noa_rpt_3(&mut self) -> NoaRpt3W<'_, RegNoaReport3Spec> {
        NoaRpt3W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_noa_report_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_noa_report_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNoaReport3Spec;
impl crate::RegisterSpec for RegNoaReport3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_noa_report_3::R`](R) reader structure"]
impl crate::Readable for RegNoaReport3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_noa_report_3::W`](W) writer structure"]
impl crate::Writable for RegNoaReport3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NOA_REPORT_3 to value 0"]
impl crate::Resettable for RegNoaReport3Spec {}
