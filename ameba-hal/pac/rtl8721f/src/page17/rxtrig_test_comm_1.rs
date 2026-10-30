#[doc = "Register `RXTRIG_TEST_COMM_1` reader"]
pub type R = crate::R<RxtrigTestComm1Spec>;
#[doc = "Register `RXTRIG_TEST_COMM_1` writer"]
pub type W = crate::W<RxtrigTestComm1Spec>;
#[doc = "Field `BIT_RXTRIG_COMMON_1` reader - high32bit of comm field in trigger frame of test mode"]
pub type BitRxtrigCommon1R = crate::FieldReader<u32>;
#[doc = "Field `BIT_RXTRIG_COMMON_1` writer - high32bit of comm field in trigger frame of test mode"]
pub type BitRxtrigCommon1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - high32bit of comm field in trigger frame of test mode"]
    #[inline(always)]
    pub fn bit_rxtrig_common_1(&self) -> BitRxtrigCommon1R {
        BitRxtrigCommon1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - high32bit of comm field in trigger frame of test mode"]
    #[inline(always)]
    pub fn bit_rxtrig_common_1(&mut self) -> BitRxtrigCommon1W<'_, RxtrigTestComm1Spec> {
        BitRxtrigCommon1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rxtrig_test_comm_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtrig_test_comm_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxtrigTestComm1Spec;
impl crate::RegisterSpec for RxtrigTestComm1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxtrig_test_comm_1::R`](R) reader structure"]
impl crate::Readable for RxtrigTestComm1Spec {}
#[doc = "`write(|w| ..)` method takes [`rxtrig_test_comm_1::W`](W) writer structure"]
impl crate::Writable for RxtrigTestComm1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXTRIG_TEST_COMM_1 to value 0"]
impl crate::Resettable for RxtrigTestComm1Spec {}
