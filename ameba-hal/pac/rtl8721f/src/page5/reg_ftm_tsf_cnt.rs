#[doc = "Register `REG_FTM_TSF_CNT` reader"]
pub type R = crate::R<RegFtmTsfCntSpec>;
#[doc = "Field `BIT_FTM_TSF_T2R` reader - T2R latch count sync tsf timer"]
pub type BitFtmTsfT2rR = crate::FieldReader<u16>;
#[doc = "Field `BIT_FTM_TSF_R2T` reader - R2T latch count sync tsf timer"]
pub type BitFtmTsfR2tR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - T2R latch count sync tsf timer"]
    #[inline(always)]
    pub fn bit_ftm_tsf_t2r(&self) -> BitFtmTsfT2rR {
        BitFtmTsfT2rR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - R2T latch count sync tsf timer"]
    #[inline(always)]
    pub fn bit_ftm_tsf_r2t(&self) -> BitFtmTsfR2tR {
        BitFtmTsfR2tR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ftm_tsf_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFtmTsfCntSpec;
impl crate::RegisterSpec for RegFtmTsfCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ftm_tsf_cnt::R`](R) reader structure"]
impl crate::Readable for RegFtmTsfCntSpec {}
#[doc = "`reset()` method sets REG_FTM_TSF_CNT to value 0"]
impl crate::Resettable for RegFtmTsfCntSpec {}
