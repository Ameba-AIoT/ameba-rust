#[doc = "Register `REG_CH0_BUF0_FRM_SIZE` reader"]
pub type R = crate::R<RegCh0Buf0FrmSizeSpec>;
#[doc = "Field `ch0_buf0_frm_size` reader - "]
pub type Ch0Buf0FrmSizeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn ch0_buf0_frm_size(&self) -> Ch0Buf0FrmSizeR {
        Ch0Buf0FrmSizeR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ch0_buf0_frm_size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCh0Buf0FrmSizeSpec;
impl crate::RegisterSpec for RegCh0Buf0FrmSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ch0_buf0_frm_size::R`](R) reader structure"]
impl crate::Readable for RegCh0Buf0FrmSizeSpec {}
#[doc = "`reset()` method sets REG_CH0_BUF0_FRM_SIZE to value 0"]
impl crate::Resettable for RegCh0Buf0FrmSizeSpec {}
