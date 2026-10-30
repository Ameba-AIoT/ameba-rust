#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `BUFFER_ENTRY_LEVEL` reader - Number of 32-bit random number are valid. 0: empty 8: full"]
pub type BufferEntryLevelR = crate::FieldReader;
#[doc = "Field `BUFFER_OVERFLOW` reader - Buffer overflow status. It is clear by RNG read out."]
pub type BufferOverflowR = crate::BitReader;
#[doc = "Field `APT0_DONE` reader - Adaptive Proportion Test done status. (APT test pass count >= APT done threshold setting)"]
pub type Apt0DoneR = crate::BitReader;
#[doc = "Field `RCT0_FAIL` reader - RCT0 error detected"]
pub type Rct0FailR = crate::BitReader;
#[doc = "Field `RCT1_FAIL` reader - RCT1 error detected"]
pub type Rct1FailR = crate::BitReader;
#[doc = "Field `APT0_FAIL` reader - APT0 error detected"]
pub type Apt0FailR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Number of 32-bit random number are valid. 0: empty 8: full"]
    #[inline(always)]
    pub fn buffer_entry_level(&self) -> BufferEntryLevelR {
        BufferEntryLevelR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Buffer overflow status. It is clear by RNG read out."]
    #[inline(always)]
    pub fn buffer_overflow(&self) -> BufferOverflowR {
        BufferOverflowR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Adaptive Proportion Test done status. (APT test pass count >= APT done threshold setting)"]
    #[inline(always)]
    pub fn apt0_done(&self) -> Apt0DoneR {
        Apt0DoneR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - RCT0 error detected"]
    #[inline(always)]
    pub fn rct0_fail(&self) -> Rct0FailR {
        Rct0FailR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RCT1 error detected"]
    #[inline(always)]
    pub fn rct1_fail(&self) -> Rct1FailR {
        Rct1FailR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - APT0 error detected"]
    #[inline(always)]
    pub fn apt0_fail(&self) -> Apt0FailR {
        Apt0FailR::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "RNG Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
