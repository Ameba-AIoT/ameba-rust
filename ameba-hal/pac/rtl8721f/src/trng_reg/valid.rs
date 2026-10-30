#[doc = "Register `Valid` reader"]
pub type R = crate::R<ValidSpec>;
#[doc = "Field `VALID` reader - Encoded status register. 0x0: buffer empty or APT is not done. 0x1~0x8: number of word is valid. 0xF: APT test fails."]
pub type ValidR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Encoded status register. 0x0: buffer empty or APT is not done. 0x1~0x8: number of word is valid. 0xF: APT test fails."]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "RNG Valid Register\n\nYou can [`read`](crate::Reg::read) this register and get [`valid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ValidSpec;
impl crate::RegisterSpec for ValidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`valid::R`](R) reader structure"]
impl crate::Readable for ValidSpec {}
#[doc = "`reset()` method sets Valid to value 0"]
impl crate::Resettable for ValidSpec {}
