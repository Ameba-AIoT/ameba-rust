#[doc = "Register `REG_r_macid_release_success_3` reader"]
pub type R = crate::R<RegRMacidReleaseSuccess3Spec>;
#[doc = "Register `REG_r_macid_release_success_3` writer"]
pub type W = crate::W<RegRMacidReleaseSuccess3Spec>;
#[doc = "Field `BIT_r_macid_release_success_3` reader - macid96~127"]
pub type BitRMacidReleaseSuccess3R = crate::FieldReader<u32>;
#[doc = "Field `BIT_r_macid_release_success_3` writer - macid96~127"]
pub type BitRMacidReleaseSuccess3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - macid96~127"]
    #[inline(always)]
    pub fn bit_r_macid_release_success_3(&self) -> BitRMacidReleaseSuccess3R {
        BitRMacidReleaseSuccess3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - macid96~127"]
    #[inline(always)]
    pub fn bit_r_macid_release_success_3(
        &mut self,
    ) -> BitRMacidReleaseSuccess3W<'_, RegRMacidReleaseSuccess3Spec> {
        BitRMacidReleaseSuccess3W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_r_macid_release_success_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_r_macid_release_success_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRMacidReleaseSuccess3Spec;
impl crate::RegisterSpec for RegRMacidReleaseSuccess3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_r_macid_release_success_3::R`](R) reader structure"]
impl crate::Readable for RegRMacidReleaseSuccess3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_r_macid_release_success_3::W`](W) writer structure"]
impl crate::Writable for RegRMacidReleaseSuccess3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_r_macid_release_success_3 to value 0"]
impl crate::Resettable for RegRMacidReleaseSuccess3Spec {}
