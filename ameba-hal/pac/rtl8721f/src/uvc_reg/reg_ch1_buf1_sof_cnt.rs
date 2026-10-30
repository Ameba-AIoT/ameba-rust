#[doc = "Register `REG_CH1_BUF1_SOF_CNT` reader"]
pub type R = crate::R<RegCh1Buf1SofCntSpec>;
#[doc = "Field `ch1_buf1_sof_cnt` reader - "]
pub type Ch1Buf1SofCntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ch1_buf1_sof_cnt(&self) -> Ch1Buf1SofCntR {
        Ch1Buf1SofCntR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ch1_buf1_sof_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCh1Buf1SofCntSpec;
impl crate::RegisterSpec for RegCh1Buf1SofCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ch1_buf1_sof_cnt::R`](R) reader structure"]
impl crate::Readable for RegCh1Buf1SofCntSpec {}
#[doc = "`reset()` method sets REG_CH1_BUF1_SOF_CNT to value 0"]
impl crate::Resettable for RegCh1Buf1SofCntSpec {}
