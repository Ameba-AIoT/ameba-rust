#[doc = "Register `REG_MACID_RELEASE3` reader"]
pub type R = crate::R<RegMacidRelease3Spec>;
#[doc = "Register `REG_MACID_RELEASE3` writer"]
pub type W = crate::W<RegMacidRelease3Spec>;
#[doc = "Field `BIT_MACID127_96_RELEASE` reader - Macid127~Macid96 Release (per bit controlled)"]
pub type BitMacid127_96ReleaseR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MACID127_96_RELEASE` writer - Macid127~Macid96 Release (per bit controlled)"]
pub type BitMacid127_96ReleaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Macid127~Macid96 Release (per bit controlled)"]
    #[inline(always)]
    pub fn bit_macid127_96_release(&self) -> BitMacid127_96ReleaseR {
        BitMacid127_96ReleaseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Macid127~Macid96 Release (per bit controlled)"]
    #[inline(always)]
    pub fn bit_macid127_96_release(&mut self) -> BitMacid127_96ReleaseW<'_, RegMacidRelease3Spec> {
        BitMacid127_96ReleaseW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_macid_release3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_macid_release3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMacidRelease3Spec;
impl crate::RegisterSpec for RegMacidRelease3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_macid_release3::R`](R) reader structure"]
impl crate::Readable for RegMacidRelease3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_macid_release3::W`](W) writer structure"]
impl crate::Writable for RegMacidRelease3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MACID_RELEASE3 to value 0"]
impl crate::Resettable for RegMacidRelease3Spec {}
