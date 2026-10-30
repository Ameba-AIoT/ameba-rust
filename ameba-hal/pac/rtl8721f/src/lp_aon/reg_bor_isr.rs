#[doc = "Register `REG_BOR_ISR` reader"]
pub type R = crate::R<RegBorIsrSpec>;
#[doc = "Field `BOR_ISR` reader - BOR interrupt status 0:no interrupt 1:interrupt occurs"]
pub type BorIsrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - BOR interrupt status 0:no interrupt 1:interrupt occurs"]
    #[inline(always)]
    pub fn bor_isr(&self) -> BorIsrR {
        BorIsrR::new((self.bits & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bor_isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBorIsrSpec;
impl crate::RegisterSpec for RegBorIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bor_isr::R`](R) reader structure"]
impl crate::Readable for RegBorIsrSpec {}
#[doc = "`reset()` method sets REG_BOR_ISR to value 0"]
impl crate::Resettable for RegBorIsrSpec {}
