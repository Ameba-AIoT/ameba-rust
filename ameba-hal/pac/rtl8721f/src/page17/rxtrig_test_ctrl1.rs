#[doc = "Register `RXTRIG_TEST_CTRL1` reader"]
pub type R = crate::R<RxtrigTestCtrl1Spec>;
#[doc = "Field `BIT_RXTRIG_STATUS` reader - 7:4\\]:trig_cntr \\[3:0\\]:ctl_state"]
pub type BitRxtrigStatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 7:4\\]:trig_cntr \\[3:0\\]:ctl_state"]
    #[inline(always)]
    pub fn bit_rxtrig_status(&self) -> BitRxtrigStatusR {
        BitRxtrigStatusR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rxtrig_test_ctrl1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxtrigTestCtrl1Spec;
impl crate::RegisterSpec for RxtrigTestCtrl1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxtrig_test_ctrl1::R`](R) reader structure"]
impl crate::Readable for RxtrigTestCtrl1Spec {}
#[doc = "`reset()` method sets RXTRIG_TEST_CTRL1 to value 0"]
impl crate::Resettable for RxtrigTestCtrl1Spec {}
