#[doc = "Register `asrc_control_3` reader"]
pub type R = crate::R<AsrcControl3Spec>;
#[doc = "Register `asrc_control_3` writer"]
pub type W = crate::W<AsrcControl3Spec>;
#[doc = "Field `asrc_sdm_inti_rx_0` reader - Set initial value of tracked frequency"]
pub type AsrcSdmIntiRx0R = crate::FieldReader<u32>;
#[doc = "Field `asrc_sdm_inti_rx_0` writer - Set initial value of tracked frequency"]
pub type AsrcSdmIntiRx0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Set initial value of tracked frequency"]
    #[inline(always)]
    pub fn asrc_sdm_inti_rx_0(&self) -> AsrcSdmIntiRx0R {
        AsrcSdmIntiRx0R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Set initial value of tracked frequency"]
    #[inline(always)]
    pub fn asrc_sdm_inti_rx_0(&mut self) -> AsrcSdmIntiRx0W<'_, AsrcControl3Spec> {
        AsrcSdmIntiRx0W::new(self, 0)
    }
}
#[doc = "ASRC Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`asrc_control_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asrc_control_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AsrcControl3Spec;
impl crate::RegisterSpec for AsrcControl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`asrc_control_3::R`](R) reader structure"]
impl crate::Readable for AsrcControl3Spec {}
#[doc = "`write(|w| ..)` method takes [`asrc_control_3::W`](W) writer structure"]
impl crate::Writable for AsrcControl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets asrc_control_3 to value 0"]
impl crate::Resettable for AsrcControl3Spec {}
