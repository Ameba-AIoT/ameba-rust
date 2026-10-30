#[doc = "Register `SHA_Status` reader"]
pub type R = crate::R<ShaStatusSpec>;
#[doc = "Field `FIFO_space` reader - Number of available FIFO space in byte"]
pub type FifoSpaceR = crate::FieldReader;
#[doc = "Field `RSVD_2` reader - Reserved"]
pub type Rsvd2R = crate::FieldReader<u32>;
#[doc = "Field `Engine_busy` reader - 1: engine busy"]
pub type EngineBusyR = crate::BitReader;
#[doc = "Field `RSVD` reader - Reserved"]
pub type RsvdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Number of available FIFO space in byte"]
    #[inline(always)]
    pub fn fifo_space(&self) -> FifoSpaceR {
        FifoSpaceR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:27 - Reserved"]
    #[inline(always)]
    pub fn rsvd_2(&self) -> Rsvd2R {
        Rsvd2R::new((self.bits >> 5) & 0x007f_ffff)
    }
    #[doc = "Bit 28 - 1: engine busy"]
    #[inline(always)]
    pub fn engine_busy(&self) -> EngineBusyR {
        EngineBusyR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 29) & 7) as u8)
    }
}
#[doc = "HMAC engine status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShaStatusSpec;
impl crate::RegisterSpec for ShaStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha_status::R`](R) reader structure"]
impl crate::Readable for ShaStatusSpec {}
#[doc = "`reset()` method sets SHA_Status to value 0x10"]
impl crate::Resettable for ShaStatusSpec {
    const RESET_VALUE: u32 = 0x10;
}
