#[doc = "Register `RXTRIG_TEST_COMM_0` reader"]
pub type R = crate::R<RxtrigTestComm0Spec>;
#[doc = "Register `RXTRIG_TEST_COMM_0` writer"]
pub type W = crate::W<RxtrigTestComm0Spec>;
#[doc = "Field `BIT_RXTRIG_COMMON_0` reader - low 32bit of comm field in trigger frame of test mode"]
pub type BitRxtrigCommon0R = crate::FieldReader<u32>;
#[doc = "Field `BIT_RXTRIG_COMMON_0` writer - low 32bit of comm field in trigger frame of test mode"]
pub type BitRxtrigCommon0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - low 32bit of comm field in trigger frame of test mode"]
    #[inline(always)]
    pub fn bit_rxtrig_common_0(&self) -> BitRxtrigCommon0R {
        BitRxtrigCommon0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - low 32bit of comm field in trigger frame of test mode"]
    #[inline(always)]
    pub fn bit_rxtrig_common_0(&mut self) -> BitRxtrigCommon0W<'_, RxtrigTestComm0Spec> {
        BitRxtrigCommon0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rxtrig_test_comm_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtrig_test_comm_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxtrigTestComm0Spec;
impl crate::RegisterSpec for RxtrigTestComm0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxtrig_test_comm_0::R`](R) reader structure"]
impl crate::Readable for RxtrigTestComm0Spec {}
#[doc = "`write(|w| ..)` method takes [`rxtrig_test_comm_0::W`](W) writer structure"]
impl crate::Writable for RxtrigTestComm0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXTRIG_TEST_COMM_0 to value 0"]
impl crate::Resettable for RxtrigTestComm0Spec {}
