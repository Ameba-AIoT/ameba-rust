#[doc = "Register `RXTRIG_TEST_CTRL2` reader"]
pub type R = crate::R<RxtrigTestCtrl2Spec>;
#[doc = "Register `RXTRIG_TEST_CTRL2` writer"]
pub type W = crate::W<RxtrigTestCtrl2Spec>;
#[doc = "Field `BIT_RXTRIG_DURATION` reader - duration of trigger frame"]
pub type BitRxtrigDurationR = crate::FieldReader<u16>;
#[doc = "Field `BIT_RXTRIG_DURATION` writer - duration of trigger frame"]
pub type BitRxtrigDurationW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - duration of trigger frame"]
    #[inline(always)]
    pub fn bit_rxtrig_duration(&self) -> BitRxtrigDurationR {
        BitRxtrigDurationR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - duration of trigger frame"]
    #[inline(always)]
    pub fn bit_rxtrig_duration(&mut self) -> BitRxtrigDurationW<'_, RxtrigTestCtrl2Spec> {
        BitRxtrigDurationW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rxtrig_test_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtrig_test_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxtrigTestCtrl2Spec;
impl crate::RegisterSpec for RxtrigTestCtrl2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rxtrig_test_ctrl2::R`](R) reader structure"]
impl crate::Readable for RxtrigTestCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`rxtrig_test_ctrl2::W`](W) writer structure"]
impl crate::Writable for RxtrigTestCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXTRIG_TEST_CTRL2 to value 0"]
impl crate::Resettable for RxtrigTestCtrl2Spec {}
