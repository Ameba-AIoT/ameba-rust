#[doc = "Register `RXTRIG_TEST_USER_0` reader"]
pub type R = crate::R<RxtrigTestUser0Spec>;
#[doc = "Register `RXTRIG_TEST_USER_0` writer"]
pub type W = crate::W<RxtrigTestUser0Spec>;
#[doc = "Field `BIT_RXTRIG_USERINFO_0` reader - low 32bit of user field in trigger frame of test mode"]
pub type BitRxtrigUserinfo0R = crate::FieldReader<u32>;
#[doc = "Field `BIT_RXTRIG_USERINFO_0` writer - low 32bit of user field in trigger frame of test mode"]
pub type BitRxtrigUserinfo0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - low 32bit of user field in trigger frame of test mode"]
    #[inline(always)]
    pub fn bit_rxtrig_userinfo_0(&self) -> BitRxtrigUserinfo0R {
        BitRxtrigUserinfo0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - low 32bit of user field in trigger frame of test mode"]
    #[inline(always)]
    pub fn bit_rxtrig_userinfo_0(&mut self) -> BitRxtrigUserinfo0W<'_, RxtrigTestUser0Spec> {
        BitRxtrigUserinfo0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rxtrig_test_user_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtrig_test_user_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxtrigTestUser0Spec;
impl crate::RegisterSpec for RxtrigTestUser0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxtrig_test_user_0::R`](R) reader structure"]
impl crate::Readable for RxtrigTestUser0Spec {}
#[doc = "`write(|w| ..)` method takes [`rxtrig_test_user_0::W`](W) writer structure"]
impl crate::Writable for RxtrigTestUser0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXTRIG_TEST_USER_0 to value 0"]
impl crate::Resettable for RxtrigTestUser0Spec {}
