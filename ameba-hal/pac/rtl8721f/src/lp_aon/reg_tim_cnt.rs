#[doc = "Register `REG_TIM_CNT` reader"]
pub type R = crate::R<RegTimCntSpec>;
#[doc = "Field `aon_tim_cnt_cur` reader - Aon timer current count value."]
pub type AonTimCntCurR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Aon timer current count value."]
    #[inline(always)]
    pub fn aon_tim_cnt_cur(&self) -> AonTimCntCurR {
        AonTimCntCurR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTimCntSpec;
impl crate::RegisterSpec for RegTimCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tim_cnt::R`](R) reader structure"]
impl crate::Readable for RegTimCntSpec {}
#[doc = "`reset()` method sets REG_TIM_CNT to value 0"]
impl crate::Resettable for RegTimCntSpec {}
