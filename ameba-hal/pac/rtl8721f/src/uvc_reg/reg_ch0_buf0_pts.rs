#[doc = "Register `REG_CH0_BUF0_PTS` reader"]
pub type R = crate::R<RegCh0Buf0PtsSpec>;
#[doc = "Field `ch0_buf0_pts` reader - "]
pub type Ch0Buf0PtsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ch0_buf0_pts(&self) -> Ch0Buf0PtsR {
        Ch0Buf0PtsR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ch0_buf0_pts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCh0Buf0PtsSpec;
impl crate::RegisterSpec for RegCh0Buf0PtsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ch0_buf0_pts::R`](R) reader structure"]
impl crate::Readable for RegCh0Buf0PtsSpec {}
#[doc = "`reset()` method sets REG_CH0_BUF0_PTS to value 0"]
impl crate::Resettable for RegCh0Buf0PtsSpec {}
