#[doc = "Register `REG_r_macid_release_success_0` reader"]
pub type R = crate::R<RegRMacidReleaseSuccess0Spec>;
#[doc = "Register `REG_r_macid_release_success_0` writer"]
pub type W = crate::W<RegRMacidReleaseSuccess0Spec>;
#[doc = "Field `BIT_r_macid_release_success_0` reader - macid0~31"]
pub type BitRMacidReleaseSuccess0R = crate::FieldReader<u32>;
#[doc = "Field `BIT_r_macid_release_success_0` writer - macid0~31"]
pub type BitRMacidReleaseSuccess0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - macid0~31"]
    #[inline(always)]
    pub fn bit_r_macid_release_success_0(&self) -> BitRMacidReleaseSuccess0R {
        BitRMacidReleaseSuccess0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - macid0~31"]
    #[inline(always)]
    pub fn bit_r_macid_release_success_0(
        &mut self,
    ) -> BitRMacidReleaseSuccess0W<'_, RegRMacidReleaseSuccess0Spec> {
        BitRMacidReleaseSuccess0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_r_macid_release_success_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_r_macid_release_success_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRMacidReleaseSuccess0Spec;
impl crate::RegisterSpec for RegRMacidReleaseSuccess0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_r_macid_release_success_0::R`](R) reader structure"]
impl crate::Readable for RegRMacidReleaseSuccess0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_r_macid_release_success_0::W`](W) writer structure"]
impl crate::Writable for RegRMacidReleaseSuccess0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_r_macid_release_success_0 to value 0"]
impl crate::Resettable for RegRMacidReleaseSuccess0Spec {}
