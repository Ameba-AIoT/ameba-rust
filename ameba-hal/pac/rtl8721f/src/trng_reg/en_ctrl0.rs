#[doc = "Register `EN_CTRL0` reader"]
pub type R = crate::R<EnCtrl0Spec>;
#[doc = "Register `EN_CTRL0` writer"]
pub type W = crate::W<EnCtrl0Spec>;
#[doc = "Field `RING50_ENABLE_0` reader - Enable Ring #29~0"]
pub type Ring50Enable0R = crate::FieldReader<u32>;
#[doc = "Field `RING50_ENABLE_0` writer - Enable Ring #29~0"]
pub type Ring50Enable0W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Enable Ring #29~0"]
    #[inline(always)]
    pub fn ring50_enable_0(&self) -> Ring50Enable0R {
        Ring50Enable0R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Enable Ring #29~0"]
    #[inline(always)]
    pub fn ring50_enable_0(&mut self) -> Ring50Enable0W<'_, EnCtrl0Spec> {
        Ring50Enable0W::new(self, 0)
    }
}
#[doc = "Ring Enable Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`en_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnCtrl0Spec;
impl crate::RegisterSpec for EnCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`en_ctrl0::R`](R) reader structure"]
impl crate::Readable for EnCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`en_ctrl0::W`](W) writer structure"]
impl crate::Writable for EnCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EN_CTRL0 to value 0x3fff_ffff"]
impl crate::Resettable for EnCtrl0Spec {
    const RESET_VALUE: u32 = 0x3fff_ffff;
}
