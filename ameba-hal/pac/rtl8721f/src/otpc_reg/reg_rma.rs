#[doc = "Register `REG_RMA` reader"]
pub type R = crate::R<RegRmaSpec>;
#[doc = "Field `RMA_DATA` reader - If the number of 1 is odd, it will goto RMA state."]
pub type RmaDataR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - If the number of 1 is odd, it will goto RMA state."]
    #[inline(always)]
    pub fn rma_data(&self) -> RmaDataR {
        RmaDataR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rma::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRmaSpec;
impl crate::RegisterSpec for RegRmaSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`reg_rma::R`](R) reader structure"]
impl crate::Readable for RegRmaSpec {}
#[doc = "`reset()` method sets REG_RMA to value 0xff"]
impl crate::Resettable for RegRmaSpec {
    const RESET_VALUE: u8 = 0xff;
}
