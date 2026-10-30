#[doc = "Register `REG_CH0_BUF0_SCR_LAST` reader"]
pub type R = crate::R<RegCh0Buf0ScrLastSpec>;
#[doc = "Field `ch0_buf0_scr_last` reader - "]
pub type Ch0Buf0ScrLastR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ch0_buf0_scr_last(&self) -> Ch0Buf0ScrLastR {
        Ch0Buf0ScrLastR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ch0_buf0_scr_last::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCh0Buf0ScrLastSpec;
impl crate::RegisterSpec for RegCh0Buf0ScrLastSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ch0_buf0_scr_last::R`](R) reader structure"]
impl crate::Readable for RegCh0Buf0ScrLastSpec {}
#[doc = "`reset()` method sets REG_CH0_BUF0_SCR_LAST to value 0"]
impl crate::Resettable for RegCh0Buf0ScrLastSpec {}
