#[doc = "Register `REG_CH0_BUF1_PTS` reader"]
pub type R = crate::R<RegCh0Buf1PtsSpec>;
#[doc = "Field `ch0_buf1_pts` reader - "]
pub type Ch0Buf1PtsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ch0_buf1_pts(&self) -> Ch0Buf1PtsR {
        Ch0Buf1PtsR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ch0_buf1_pts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCh0Buf1PtsSpec;
impl crate::RegisterSpec for RegCh0Buf1PtsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ch0_buf1_pts::R`](R) reader structure"]
impl crate::Readable for RegCh0Buf1PtsSpec {}
#[doc = "`reset()` method sets REG_CH0_BUF1_PTS to value 0"]
impl crate::Resettable for RegCh0Buf1PtsSpec {}
