#[doc = "Register `REG_CH1_BUF0_SCR_LAST` reader"]
pub type R = crate::R<RegCh1Buf0ScrLastSpec>;
#[doc = "Field `ch1_buf0_scr_last` reader - "]
pub type Ch1Buf0ScrLastR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ch1_buf0_scr_last(&self) -> Ch1Buf0ScrLastR {
        Ch1Buf0ScrLastR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ch1_buf0_scr_last::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCh1Buf0ScrLastSpec;
impl crate::RegisterSpec for RegCh1Buf0ScrLastSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ch1_buf0_scr_last::R`](R) reader structure"]
impl crate::Readable for RegCh1Buf0ScrLastSpec {}
#[doc = "`reset()` method sets REG_CH1_BUF0_SCR_LAST to value 0"]
impl crate::Resettable for RegCh1Buf0ScrLastSpec {}
