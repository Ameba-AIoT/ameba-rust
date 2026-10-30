#[doc = "Register `REG_MACID_DROP1` reader"]
pub type R = crate::R<RegMacidDrop1Spec>;
#[doc = "Register `REG_MACID_DROP1` writer"]
pub type W = crate::W<RegMacidDrop1Spec>;
#[doc = "Field `BIT_MACID63_32_DROP` reader - Macid63~Macid32 Drop (per bit controlled)"]
pub type BitMacid63_32DropR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MACID63_32_DROP` writer - Macid63~Macid32 Drop (per bit controlled)"]
pub type BitMacid63_32DropW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Macid63~Macid32 Drop (per bit controlled)"]
    #[inline(always)]
    pub fn bit_macid63_32_drop(&self) -> BitMacid63_32DropR {
        BitMacid63_32DropR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Macid63~Macid32 Drop (per bit controlled)"]
    #[inline(always)]
    pub fn bit_macid63_32_drop(&mut self) -> BitMacid63_32DropW<'_, RegMacidDrop1Spec> {
        BitMacid63_32DropW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_macid_drop1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_macid_drop1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMacidDrop1Spec;
impl crate::RegisterSpec for RegMacidDrop1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_macid_drop1::R`](R) reader structure"]
impl crate::Readable for RegMacidDrop1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_macid_drop1::W`](W) writer structure"]
impl crate::Writable for RegMacidDrop1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MACID_DROP1 to value 0"]
impl crate::Resettable for RegMacidDrop1Spec {}
