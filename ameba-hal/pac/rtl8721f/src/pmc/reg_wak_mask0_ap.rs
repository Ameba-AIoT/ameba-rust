#[doc = "Register `REG_WAK_MASK0_AP` reader"]
pub type R = crate::R<RegWakMask0ApSpec>;
#[doc = "Register `REG_WAK_MASK0_AP` writer"]
pub type W = crate::W<RegWakMask0ApSpec>;
#[doc = "Field `wak_ap_imr_29_0` reader - 1:mask enable for wake AP"]
pub type WakApImr29_0R = crate::FieldReader<u32>;
#[doc = "Field `wak_ap_imr_29_0` writer - 1:mask enable for wake AP"]
pub type WakApImr29_0W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - 1:mask enable for wake AP"]
    #[inline(always)]
    pub fn wak_ap_imr_29_0(&self) -> WakApImr29_0R {
        WakApImr29_0R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - 1:mask enable for wake AP"]
    #[inline(always)]
    pub fn wak_ap_imr_29_0(&mut self) -> WakApImr29_0W<'_, RegWakMask0ApSpec> {
        WakApImr29_0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wak_mask0_ap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wak_mask0_ap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWakMask0ApSpec;
impl crate::RegisterSpec for RegWakMask0ApSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wak_mask0_ap::R`](R) reader structure"]
impl crate::Readable for RegWakMask0ApSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wak_mask0_ap::W`](W) writer structure"]
impl crate::Writable for RegWakMask0ApSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WAK_MASK0_AP to value 0"]
impl crate::Resettable for RegWakMask0ApSpec {}
