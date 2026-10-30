#[doc = "Register `RXTRIG_TEST_USER_1` reader"]
pub type R = crate::R<RxtrigTestUser1Spec>;
#[doc = "Register `RXTRIG_TEST_USER_1` writer"]
pub type W = crate::W<RxtrigTestUser1Spec>;
#[doc = "Field `BIT_RXTRIG_USERINFO_1` reader - high 32bit of user field in trigger frame of test mode"]
pub type BitRxtrigUserinfo1R = crate::FieldReader<u32>;
#[doc = "Field `BIT_RXTRIG_USERINFO_1` writer - high 32bit of user field in trigger frame of test mode"]
pub type BitRxtrigUserinfo1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - high 32bit of user field in trigger frame of test mode"]
    #[inline(always)]
    pub fn bit_rxtrig_userinfo_1(&self) -> BitRxtrigUserinfo1R {
        BitRxtrigUserinfo1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - high 32bit of user field in trigger frame of test mode"]
    #[inline(always)]
    pub fn bit_rxtrig_userinfo_1(&mut self) -> BitRxtrigUserinfo1W<'_, RxtrigTestUser1Spec> {
        BitRxtrigUserinfo1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rxtrig_test_user_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtrig_test_user_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxtrigTestUser1Spec;
impl crate::RegisterSpec for RxtrigTestUser1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxtrig_test_user_1::R`](R) reader structure"]
impl crate::Readable for RxtrigTestUser1Spec {}
#[doc = "`write(|w| ..)` method takes [`rxtrig_test_user_1::W`](W) writer structure"]
impl crate::Writable for RxtrigTestUser1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXTRIG_TEST_USER_1 to value 0"]
impl crate::Resettable for RxtrigTestUser1Spec {}
