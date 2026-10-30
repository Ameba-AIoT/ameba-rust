#[doc = "Register `REG_WAK_MASK1_NP` reader"]
pub type R = crate::R<RegWakMask1NpSpec>;
#[doc = "Register `REG_WAK_MASK1_NP` writer"]
pub type W = crate::W<RegWakMask1NpSpec>;
#[doc = "Field `wak_np_imr_59_30` reader - 1:mask enable for wake NP, this used to wak pmc"]
pub type WakNpImr59_30R = crate::FieldReader<u32>;
#[doc = "Field `wak_np_imr_59_30` writer - 1:mask enable for wake NP, this used to wak pmc"]
pub type WakNpImr59_30W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - 1:mask enable for wake NP, this used to wak pmc"]
    #[inline(always)]
    pub fn wak_np_imr_59_30(&self) -> WakNpImr59_30R {
        WakNpImr59_30R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - 1:mask enable for wake NP, this used to wak pmc"]
    #[inline(always)]
    pub fn wak_np_imr_59_30(&mut self) -> WakNpImr59_30W<'_, RegWakMask1NpSpec> {
        WakNpImr59_30W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wak_mask1_np::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wak_mask1_np::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWakMask1NpSpec;
impl crate::RegisterSpec for RegWakMask1NpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wak_mask1_np::R`](R) reader structure"]
impl crate::Readable for RegWakMask1NpSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wak_mask1_np::W`](W) writer structure"]
impl crate::Writable for RegWakMask1NpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WAK_MASK1_NP to value 0"]
impl crate::Resettable for RegWakMask1NpSpec {}
