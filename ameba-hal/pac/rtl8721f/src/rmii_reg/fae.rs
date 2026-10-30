#[doc = "Register `FAE` reader"]
pub type R = crate::R<FaeSpec>;
#[doc = "Field `FAE` reader - 16-bit counter of FAE (Frame Alignment Error) packets. Rolls over automatically. Write to clear. RCR.AER should be high. When FAE occurs in 10/100mode, the following thins happens: *Rx status des.CRC error does not assert if the ingress(exclude alignment error nibble) if correct. *RxErr_reg increased by 1. *FAE_reg increased by 1 *Rx status des.RCDF does not asserts. data length does not include alignment error nibble."]
pub type FaeR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit counter of FAE (Frame Alignment Error) packets. Rolls over automatically. Write to clear. RCR.AER should be high. When FAE occurs in 10/100mode, the following thins happens: *Rx status des.CRC error does not assert if the ingress(exclude alignment error nibble) if correct. *RxErr_reg increased by 1. *FAE_reg increased by 1 *Rx status des.RCDF does not asserts. data length does not include alignment error nibble."]
    #[inline(always)]
    pub fn fae(&self) -> FaeR {
        FaeR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`fae::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FaeSpec;
impl crate::RegisterSpec for FaeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fae::R`](R) reader structure"]
impl crate::Readable for FaeSpec {}
#[doc = "`reset()` method sets FAE to value 0"]
impl crate::Resettable for FaeSpec {}
