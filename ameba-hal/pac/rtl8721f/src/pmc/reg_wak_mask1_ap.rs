#[doc = "Register `REG_WAK_MASK1_AP` reader"]
pub type R = crate::R<RegWakMask1ApSpec>;
#[doc = "Register `REG_WAK_MASK1_AP` writer"]
pub type W = crate::W<RegWakMask1ApSpec>;
#[doc = "Field `wak_ap_imr_59_30` reader - 1:mask enable for wake AP"]
pub type WakApImr59_30R = crate::FieldReader<u32>;
#[doc = "Field `wak_ap_imr_59_30` writer - 1:mask enable for wake AP"]
pub type WakApImr59_30W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - 1:mask enable for wake AP"]
    #[inline(always)]
    pub fn wak_ap_imr_59_30(&self) -> WakApImr59_30R {
        WakApImr59_30R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - 1:mask enable for wake AP"]
    #[inline(always)]
    pub fn wak_ap_imr_59_30(&mut self) -> WakApImr59_30W<'_, RegWakMask1ApSpec> {
        WakApImr59_30W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wak_mask1_ap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wak_mask1_ap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWakMask1ApSpec;
impl crate::RegisterSpec for RegWakMask1ApSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wak_mask1_ap::R`](R) reader structure"]
impl crate::Readable for RegWakMask1ApSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wak_mask1_ap::W`](W) writer structure"]
impl crate::Writable for RegWakMask1ApSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WAK_MASK1_AP to value 0"]
impl crate::Resettable for RegWakMask1ApSpec {}
