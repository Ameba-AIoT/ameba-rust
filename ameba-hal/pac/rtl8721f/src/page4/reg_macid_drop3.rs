#[doc = "Register `REG_MACID_DROP3` reader"]
pub type R = crate::R<RegMacidDrop3Spec>;
#[doc = "Register `REG_MACID_DROP3` writer"]
pub type W = crate::W<RegMacidDrop3Spec>;
#[doc = "Field `BIT_MACID127_96_DROP` reader - Macid127~Macid96 Drop (per bit controlled)"]
pub type BitMacid127_96DropR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MACID127_96_DROP` writer - Macid127~Macid96 Drop (per bit controlled)"]
pub type BitMacid127_96DropW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Macid127~Macid96 Drop (per bit controlled)"]
    #[inline(always)]
    pub fn bit_macid127_96_drop(&self) -> BitMacid127_96DropR {
        BitMacid127_96DropR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Macid127~Macid96 Drop (per bit controlled)"]
    #[inline(always)]
    pub fn bit_macid127_96_drop(&mut self) -> BitMacid127_96DropW<'_, RegMacidDrop3Spec> {
        BitMacid127_96DropW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_macid_drop3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_macid_drop3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMacidDrop3Spec;
impl crate::RegisterSpec for RegMacidDrop3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_macid_drop3::R`](R) reader structure"]
impl crate::Readable for RegMacidDrop3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_macid_drop3::W`](W) writer structure"]
impl crate::Writable for RegMacidDrop3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MACID_DROP3 to value 0"]
impl crate::Resettable for RegMacidDrop3Spec {}
