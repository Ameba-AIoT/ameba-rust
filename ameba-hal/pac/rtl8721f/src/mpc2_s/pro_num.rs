#[doc = "Register `PRO_NUM` reader"]
pub type R = crate::R<ProNumSpec>;
#[doc = "Field `PRO_NUM` reader - The project number of RXI-300."]
pub type ProNumR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The project number of RXI-300."]
    #[inline(always)]
    pub fn pro_num(&self) -> ProNumR {
        ProNumR::new(self.bits)
    }
}
#[doc = "RXI300 project number register\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_num::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProNumSpec;
impl crate::RegisterSpec for ProNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_num::R`](R) reader structure"]
impl crate::Readable for ProNumSpec {}
#[doc = "`reset()` method sets PRO_NUM to value 0x7005"]
impl crate::Resettable for ProNumSpec {
    const RESET_VALUE: u32 = 0x7005;
}
