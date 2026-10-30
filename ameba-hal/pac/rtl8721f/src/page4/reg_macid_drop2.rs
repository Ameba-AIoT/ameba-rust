#[doc = "Register `REG_MACID_DROP2` reader"]
pub type R = crate::R<RegMacidDrop2Spec>;
#[doc = "Register `REG_MACID_DROP2` writer"]
pub type W = crate::W<RegMacidDrop2Spec>;
#[doc = "Field `BIT_MACID95_64_DROP` reader - Macid95~Macid64 Drop (per bit controlled)"]
pub type BitMacid95_64DropR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MACID95_64_DROP` writer - Macid95~Macid64 Drop (per bit controlled)"]
pub type BitMacid95_64DropW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Macid95~Macid64 Drop (per bit controlled)"]
    #[inline(always)]
    pub fn bit_macid95_64_drop(&self) -> BitMacid95_64DropR {
        BitMacid95_64DropR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Macid95~Macid64 Drop (per bit controlled)"]
    #[inline(always)]
    pub fn bit_macid95_64_drop(&mut self) -> BitMacid95_64DropW<'_, RegMacidDrop2Spec> {
        BitMacid95_64DropW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_macid_drop2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_macid_drop2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMacidDrop2Spec;
impl crate::RegisterSpec for RegMacidDrop2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_macid_drop2::R`](R) reader structure"]
impl crate::Readable for RegMacidDrop2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_macid_drop2::W`](W) writer structure"]
impl crate::Writable for RegMacidDrop2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MACID_DROP2 to value 0"]
impl crate::Resettable for RegMacidDrop2Spec {}
