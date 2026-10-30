#[doc = "Register `MISC` reader"]
pub type R = crate::R<MiscSpec>;
#[doc = "Field `SECURE` reader - Secure enable This field can only be modified by secure APB transaction. 0x0: PPE in non-secure mode 0x1: PPE in secure mode"]
pub type SecureR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Secure enable This field can only be modified by secure APB transaction. 0x0: PPE in non-secure mode 0x1: PPE in secure mode"]
    #[inline(always)]
    pub fn secure(&self) -> SecureR {
        SecureR::new((self.bits & 1) != 0)
    }
}
#[doc = "Secure enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiscSpec;
impl crate::RegisterSpec for MiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc::R`](R) reader structure"]
impl crate::Readable for MiscSpec {}
#[doc = "`reset()` method sets MISC to value 0"]
impl crate::Resettable for MiscSpec {}
