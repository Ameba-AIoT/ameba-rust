#[doc = "Register `REG_BT_ENHANCED_CTRL0` reader"]
pub type R = crate::R<RegBtEnhancedCtrl0Spec>;
#[doc = "Field `BIT_STATIS_BT_31_0` reader - "]
pub type BitStatisBt31_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bit_statis_bt_31_0(&self) -> BitStatisBt31_0R {
        BitStatisBt31_0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bt_enhanced_ctrl0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBtEnhancedCtrl0Spec;
impl crate::RegisterSpec for RegBtEnhancedCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bt_enhanced_ctrl0::R`](R) reader structure"]
impl crate::Readable for RegBtEnhancedCtrl0Spec {}
#[doc = "`reset()` method sets REG_BT_ENHANCED_CTRL0 to value 0"]
impl crate::Resettable for RegBtEnhancedCtrl0Spec {}
