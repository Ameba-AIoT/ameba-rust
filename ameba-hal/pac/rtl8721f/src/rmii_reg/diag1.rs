#[doc = "Register `DIAG1` reader"]
pub type R = crate::R<Diag1Spec>;
#[doc = "Field `lso_stats` reader - Reserved(3bits)"]
pub type LsoStatsR = crate::FieldReader;
#[doc = "Field `Rxmring_state` reader - "]
pub type RxmringStateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Reserved(3bits)"]
    #[inline(always)]
    pub fn lso_stats(&self) -> LsoStatsR {
        LsoStatsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn rxmring_state(&self) -> RxmringStateR {
        RxmringStateR::new(((self.bits >> 3) & 7) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`diag1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diag1Spec;
impl crate::RegisterSpec for Diag1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diag1::R`](R) reader structure"]
impl crate::Readable for Diag1Spec {}
#[doc = "`reset()` method sets DIAG1 to value 0"]
impl crate::Resettable for Diag1Spec {}
