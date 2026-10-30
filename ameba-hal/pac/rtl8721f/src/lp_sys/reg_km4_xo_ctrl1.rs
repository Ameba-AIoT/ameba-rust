#[doc = "Register `REG_KM4_XO_CTRL1` reader"]
pub type R = crate::R<RegKm4XoCtrl1Spec>;
#[doc = "Register `REG_KM4_XO_CTRL1` writer"]
pub type W = crate::W<RegKm4XoCtrl1Spec>;
#[doc = "Field `init_xo_top` reader - "]
pub type InitXoTopR = crate::FieldReader<u32>;
#[doc = "Field `init_xo_top` writer - "]
pub type InitXoTopW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn init_xo_top(&self) -> InitXoTopR {
        InitXoTopR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn init_xo_top(&mut self) -> InitXoTopW<'_, RegKm4XoCtrl1Spec> {
        InitXoTopW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_km4_xo_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_km4_xo_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKm4XoCtrl1Spec;
impl crate::RegisterSpec for RegKm4XoCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_km4_xo_ctrl1::R`](R) reader structure"]
impl crate::Readable for RegKm4XoCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_km4_xo_ctrl1::W`](W) writer structure"]
impl crate::Writable for RegKm4XoCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KM4_XO_CTRL1 to value 0"]
impl crate::Resettable for RegKm4XoCtrl1Spec {}
