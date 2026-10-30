#[doc = "Register `REG_CH0_BUF0_SOF_CNT` reader"]
pub type R = crate::R<RegCh0Buf0SofCntSpec>;
#[doc = "Field `ch0_buf0_sof_cnt` reader - "]
pub type Ch0Buf0SofCntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ch0_buf0_sof_cnt(&self) -> Ch0Buf0SofCntR {
        Ch0Buf0SofCntR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ch0_buf0_sof_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCh0Buf0SofCntSpec;
impl crate::RegisterSpec for RegCh0Buf0SofCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ch0_buf0_sof_cnt::R`](R) reader structure"]
impl crate::Readable for RegCh0Buf0SofCntSpec {}
#[doc = "`reset()` method sets REG_CH0_BUF0_SOF_CNT to value 0"]
impl crate::Resettable for RegCh0Buf0SofCntSpec {}
