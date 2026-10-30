#[doc = "Register `REG_CH1_BUF0_SCR_FIRST` reader"]
pub type R = crate::R<RegCh1Buf0ScrFirstSpec>;
#[doc = "Field `ch1_buf0_scr_first` reader - "]
pub type Ch1Buf0ScrFirstR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ch1_buf0_scr_first(&self) -> Ch1Buf0ScrFirstR {
        Ch1Buf0ScrFirstR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ch1_buf0_scr_first::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCh1Buf0ScrFirstSpec;
impl crate::RegisterSpec for RegCh1Buf0ScrFirstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ch1_buf0_scr_first::R`](R) reader structure"]
impl crate::Readable for RegCh1Buf0ScrFirstSpec {}
#[doc = "`reset()` method sets REG_CH1_BUF0_SCR_FIRST to value 0"]
impl crate::Resettable for RegCh1Buf0ScrFirstSpec {}
