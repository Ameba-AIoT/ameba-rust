#[doc = "Register `REG_r_macid_release_success_1` reader"]
pub type R = crate::R<RegRMacidReleaseSuccess1Spec>;
#[doc = "Register `REG_r_macid_release_success_1` writer"]
pub type W = crate::W<RegRMacidReleaseSuccess1Spec>;
#[doc = "Field `BIT_r_macid_release_success_1` reader - macid32~63"]
pub type BitRMacidReleaseSuccess1R = crate::FieldReader<u32>;
#[doc = "Field `BIT_r_macid_release_success_1` writer - macid32~63"]
pub type BitRMacidReleaseSuccess1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - macid32~63"]
    #[inline(always)]
    pub fn bit_r_macid_release_success_1(&self) -> BitRMacidReleaseSuccess1R {
        BitRMacidReleaseSuccess1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - macid32~63"]
    #[inline(always)]
    pub fn bit_r_macid_release_success_1(
        &mut self,
    ) -> BitRMacidReleaseSuccess1W<'_, RegRMacidReleaseSuccess1Spec> {
        BitRMacidReleaseSuccess1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_r_macid_release_success_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_r_macid_release_success_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRMacidReleaseSuccess1Spec;
impl crate::RegisterSpec for RegRMacidReleaseSuccess1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_r_macid_release_success_1::R`](R) reader structure"]
impl crate::Readable for RegRMacidReleaseSuccess1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_r_macid_release_success_1::W`](W) writer structure"]
impl crate::Writable for RegRMacidReleaseSuccess1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_r_macid_release_success_1 to value 0"]
impl crate::Resettable for RegRMacidReleaseSuccess1Spec {}
