#[doc = "Register `REG_TIM_ISR` reader"]
pub type R = crate::R<RegTimIsrSpec>;
#[doc = "Field `AON_TIM_ISR` reader - Timer interrupt status 0:no interrupt 1:interrupt occurs"]
pub type AonTimIsrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timer interrupt status 0:no interrupt 1:interrupt occurs"]
    #[inline(always)]
    pub fn aon_tim_isr(&self) -> AonTimIsrR {
        AonTimIsrR::new((self.bits & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim_isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTimIsrSpec;
impl crate::RegisterSpec for RegTimIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tim_isr::R`](R) reader structure"]
impl crate::Readable for RegTimIsrSpec {}
#[doc = "`reset()` method sets REG_TIM_ISR to value 0"]
impl crate::Resettable for RegTimIsrSpec {}
