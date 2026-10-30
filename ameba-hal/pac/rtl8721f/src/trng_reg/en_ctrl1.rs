#[doc = "Register `EN_CTRL1` reader"]
pub type R = crate::R<EnCtrl1Spec>;
#[doc = "Register `EN_CTRL1` writer"]
pub type W = crate::W<EnCtrl1Spec>;
#[doc = "Field `RING50_ENABLE_1` reader - Enable Ring #49~30"]
pub type Ring50Enable1R = crate::FieldReader<u32>;
#[doc = "Field `RING50_ENABLE_1` writer - Enable Ring #49~30"]
pub type Ring50Enable1W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `RING50_RNG_ENABLE` reader - Enable Ring50 noise source"]
pub type Ring50RngEnableR = crate::BitReader;
#[doc = "Field `RING50_RNG_ENABLE` writer - Enable Ring50 noise source"]
pub type Ring50RngEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:19 - Enable Ring #49~30"]
    #[inline(always)]
    pub fn ring50_enable_1(&self) -> Ring50Enable1R {
        Ring50Enable1R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 20 - Enable Ring50 noise source"]
    #[inline(always)]
    pub fn ring50_rng_enable(&self) -> Ring50RngEnableR {
        Ring50RngEnableR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - Enable Ring #49~30"]
    #[inline(always)]
    pub fn ring50_enable_1(&mut self) -> Ring50Enable1W<'_, EnCtrl1Spec> {
        Ring50Enable1W::new(self, 0)
    }
    #[doc = "Bit 20 - Enable Ring50 noise source"]
    #[inline(always)]
    pub fn ring50_rng_enable(&mut self) -> Ring50RngEnableW<'_, EnCtrl1Spec> {
        Ring50RngEnableW::new(self, 20)
    }
}
#[doc = "Ring Enable Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`en_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnCtrl1Spec;
impl crate::RegisterSpec for EnCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`en_ctrl1::R`](R) reader structure"]
impl crate::Readable for EnCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`en_ctrl1::W`](W) writer structure"]
impl crate::Writable for EnCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EN_CTRL1 to value 0x001f_ffff"]
impl crate::Resettable for EnCtrl1Spec {
    const RESET_VALUE: u32 = 0x001f_ffff;
}
