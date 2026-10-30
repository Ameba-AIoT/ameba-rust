#[doc = "Register `REG_BT_ENHANCED_CTRL1` reader"]
pub type R = crate::R<RegBtEnhancedCtrl1Spec>;
#[doc = "Field `BIT_STATIS_BT_63_32` reader - "]
pub type BitStatisBt63_32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bit_statis_bt_63_32(&self) -> BitStatisBt63_32R {
        BitStatisBt63_32R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bt_enhanced_ctrl1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBtEnhancedCtrl1Spec;
impl crate::RegisterSpec for RegBtEnhancedCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bt_enhanced_ctrl1::R`](R) reader structure"]
impl crate::Readable for RegBtEnhancedCtrl1Spec {}
#[doc = "`reset()` method sets REG_BT_ENHANCED_CTRL1 to value 0"]
impl crate::Resettable for RegBtEnhancedCtrl1Spec {}
