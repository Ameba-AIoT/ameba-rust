#[doc = "Register `REG_MACID_RELEASE0` reader"]
pub type R = crate::R<RegMacidRelease0Spec>;
#[doc = "Register `REG_MACID_RELEASE0` writer"]
pub type W = crate::W<RegMacidRelease0Spec>;
#[doc = "Field `BIT_MACID31_0_RELEASE` reader - Macid31~Macid0 Release(per bit controlled)"]
pub type BitMacid31_0ReleaseR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MACID31_0_RELEASE` writer - Macid31~Macid0 Release(per bit controlled)"]
pub type BitMacid31_0ReleaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Macid31~Macid0 Release(per bit controlled)"]
    #[inline(always)]
    pub fn bit_macid31_0_release(&self) -> BitMacid31_0ReleaseR {
        BitMacid31_0ReleaseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Macid31~Macid0 Release(per bit controlled)"]
    #[inline(always)]
    pub fn bit_macid31_0_release(&mut self) -> BitMacid31_0ReleaseW<'_, RegMacidRelease0Spec> {
        BitMacid31_0ReleaseW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_macid_release0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_macid_release0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMacidRelease0Spec;
impl crate::RegisterSpec for RegMacidRelease0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_macid_release0::R`](R) reader structure"]
impl crate::Readable for RegMacidRelease0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_macid_release0::W`](W) writer structure"]
impl crate::Writable for RegMacidRelease0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MACID_RELEASE0 to value 0"]
impl crate::Resettable for RegMacidRelease0Spec {}
