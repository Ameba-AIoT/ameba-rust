#[doc = "Register `DEV` reader"]
pub type R = crate::R<DevSpec>;
#[doc = "Field `DEV` reader - The developer of RXI-300."]
pub type DevR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The developer of RXI-300."]
    #[inline(always)]
    pub fn dev(&self) -> DevR {
        DevR::new(self.bits)
    }
}
#[doc = "RXI300 developer register\n\nYou can [`read`](crate::Reg::read) this register and get [`dev::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevSpec;
impl crate::RegisterSpec for DevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev::R`](R) reader structure"]
impl crate::Readable for DevSpec {}
#[doc = "`reset()` method sets DEV to value 0x0520_7433"]
impl crate::Resettable for DevSpec {
    const RESET_VALUE: u32 = 0x0520_7433;
}
