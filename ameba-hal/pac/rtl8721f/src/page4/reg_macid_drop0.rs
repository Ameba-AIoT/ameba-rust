#[doc = "Register `REG_MACID_DROP0` reader"]
pub type R = crate::R<RegMacidDrop0Spec>;
#[doc = "Register `REG_MACID_DROP0` writer"]
pub type W = crate::W<RegMacidDrop0Spec>;
#[doc = "Field `BIT_MACID31_0_DROP` reader - Macid31~Macid0 Drop (per bit controlled)"]
pub type BitMacid31_0DropR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MACID31_0_DROP` writer - Macid31~Macid0 Drop (per bit controlled)"]
pub type BitMacid31_0DropW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Macid31~Macid0 Drop (per bit controlled)"]
    #[inline(always)]
    pub fn bit_macid31_0_drop(&self) -> BitMacid31_0DropR {
        BitMacid31_0DropR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Macid31~Macid0 Drop (per bit controlled)"]
    #[inline(always)]
    pub fn bit_macid31_0_drop(&mut self) -> BitMacid31_0DropW<'_, RegMacidDrop0Spec> {
        BitMacid31_0DropW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_macid_drop0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_macid_drop0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMacidDrop0Spec;
impl crate::RegisterSpec for RegMacidDrop0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_macid_drop0::R`](R) reader structure"]
impl crate::Readable for RegMacidDrop0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_macid_drop0::W`](W) writer structure"]
impl crate::Writable for RegMacidDrop0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MACID_DROP0 to value 0"]
impl crate::Resettable for RegMacidDrop0Spec {}
