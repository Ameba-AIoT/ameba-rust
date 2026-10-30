#[doc = "Register `REG_MACID_RELEASE1` reader"]
pub type R = crate::R<RegMacidRelease1Spec>;
#[doc = "Register `REG_MACID_RELEASE1` writer"]
pub type W = crate::W<RegMacidRelease1Spec>;
#[doc = "Field `BIT_MACID63_32_RELEASE` reader - Macid63~Macid32 Release(per bit controlled)"]
pub type BitMacid63_32ReleaseR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MACID63_32_RELEASE` writer - Macid63~Macid32 Release(per bit controlled)"]
pub type BitMacid63_32ReleaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Macid63~Macid32 Release(per bit controlled)"]
    #[inline(always)]
    pub fn bit_macid63_32_release(&self) -> BitMacid63_32ReleaseR {
        BitMacid63_32ReleaseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Macid63~Macid32 Release(per bit controlled)"]
    #[inline(always)]
    pub fn bit_macid63_32_release(&mut self) -> BitMacid63_32ReleaseW<'_, RegMacidRelease1Spec> {
        BitMacid63_32ReleaseW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_macid_release1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_macid_release1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMacidRelease1Spec;
impl crate::RegisterSpec for RegMacidRelease1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_macid_release1::R`](R) reader structure"]
impl crate::Readable for RegMacidRelease1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_macid_release1::W`](W) writer structure"]
impl crate::Writable for RegMacidRelease1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MACID_RELEASE1 to value 0"]
impl crate::Resettable for RegMacidRelease1Spec {}
