#[doc = "Register `REG_MACID_RELEASE2` reader"]
pub type R = crate::R<RegMacidRelease2Spec>;
#[doc = "Register `REG_MACID_RELEASE2` writer"]
pub type W = crate::W<RegMacidRelease2Spec>;
#[doc = "Field `BIT_MACID95_64_RELEASE` reader - Macid95~Macid64 Release (per bit controlled)"]
pub type BitMacid95_64ReleaseR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MACID95_64_RELEASE` writer - Macid95~Macid64 Release (per bit controlled)"]
pub type BitMacid95_64ReleaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Macid95~Macid64 Release (per bit controlled)"]
    #[inline(always)]
    pub fn bit_macid95_64_release(&self) -> BitMacid95_64ReleaseR {
        BitMacid95_64ReleaseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Macid95~Macid64 Release (per bit controlled)"]
    #[inline(always)]
    pub fn bit_macid95_64_release(&mut self) -> BitMacid95_64ReleaseW<'_, RegMacidRelease2Spec> {
        BitMacid95_64ReleaseW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_macid_release2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_macid_release2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMacidRelease2Spec;
impl crate::RegisterSpec for RegMacidRelease2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_macid_release2::R`](R) reader structure"]
impl crate::Readable for RegMacidRelease2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_macid_release2::W`](W) writer structure"]
impl crate::Writable for RegMacidRelease2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MACID_RELEASE2 to value 0"]
impl crate::Resettable for RegMacidRelease2Spec {}
