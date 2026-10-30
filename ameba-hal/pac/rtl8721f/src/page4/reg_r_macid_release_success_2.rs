#[doc = "Register `REG_r_macid_release_success_2` reader"]
pub type R = crate::R<RegRMacidReleaseSuccess2Spec>;
#[doc = "Register `REG_r_macid_release_success_2` writer"]
pub type W = crate::W<RegRMacidReleaseSuccess2Spec>;
#[doc = "Field `BIT_r_macid_release_success_2` reader - macid64~95"]
pub type BitRMacidReleaseSuccess2R = crate::FieldReader<u32>;
#[doc = "Field `BIT_r_macid_release_success_2` writer - macid64~95"]
pub type BitRMacidReleaseSuccess2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - macid64~95"]
    #[inline(always)]
    pub fn bit_r_macid_release_success_2(&self) -> BitRMacidReleaseSuccess2R {
        BitRMacidReleaseSuccess2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - macid64~95"]
    #[inline(always)]
    pub fn bit_r_macid_release_success_2(
        &mut self,
    ) -> BitRMacidReleaseSuccess2W<'_, RegRMacidReleaseSuccess2Spec> {
        BitRMacidReleaseSuccess2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_r_macid_release_success_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_r_macid_release_success_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRMacidReleaseSuccess2Spec;
impl crate::RegisterSpec for RegRMacidReleaseSuccess2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_r_macid_release_success_2::R`](R) reader structure"]
impl crate::Readable for RegRMacidReleaseSuccess2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_r_macid_release_success_2::W`](W) writer structure"]
impl crate::Writable for RegRMacidReleaseSuccess2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_r_macid_release_success_2 to value 0"]
impl crate::Resettable for RegRMacidReleaseSuccess2Spec {}
