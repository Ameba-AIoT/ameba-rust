#[doc = "Register `REG_CH1_BUF0_FRM_SIZE` reader"]
pub type R = crate::R<RegCh1Buf0FrmSizeSpec>;
#[doc = "Field `ch1_buf0_frm_size` reader - "]
pub type Ch1Buf0FrmSizeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn ch1_buf0_frm_size(&self) -> Ch1Buf0FrmSizeR {
        Ch1Buf0FrmSizeR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ch1_buf0_frm_size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCh1Buf0FrmSizeSpec;
impl crate::RegisterSpec for RegCh1Buf0FrmSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ch1_buf0_frm_size::R`](R) reader structure"]
impl crate::Readable for RegCh1Buf0FrmSizeSpec {}
#[doc = "`reset()` method sets REG_CH1_BUF0_FRM_SIZE to value 0"]
impl crate::Resettable for RegCh1Buf0FrmSizeSpec {}
