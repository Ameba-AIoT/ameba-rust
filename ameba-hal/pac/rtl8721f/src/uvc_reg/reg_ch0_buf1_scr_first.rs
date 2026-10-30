#[doc = "Register `REG_CH0_BUF1_SCR_FIRST` reader"]
pub type R = crate::R<RegCh0Buf1ScrFirstSpec>;
#[doc = "Field `ch0_buf1_scr_first` reader - "]
pub type Ch0Buf1ScrFirstR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ch0_buf1_scr_first(&self) -> Ch0Buf1ScrFirstR {
        Ch0Buf1ScrFirstR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ch0_buf1_scr_first::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCh0Buf1ScrFirstSpec;
impl crate::RegisterSpec for RegCh0Buf1ScrFirstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ch0_buf1_scr_first::R`](R) reader structure"]
impl crate::Readable for RegCh0Buf1ScrFirstSpec {}
#[doc = "`reset()` method sets REG_CH0_BUF1_SCR_FIRST to value 0"]
impl crate::Resettable for RegCh0Buf1ScrFirstSpec {}
