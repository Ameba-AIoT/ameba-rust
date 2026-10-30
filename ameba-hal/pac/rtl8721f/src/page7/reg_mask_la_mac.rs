#[doc = "Register `REG_MASK_LA_MAC` reader"]
pub type R = crate::R<RegMaskLaMacSpec>;
#[doc = "Register `REG_MASK_LA_MAC` writer"]
pub type W = crate::W<RegMaskLaMacSpec>;
#[doc = "Field `BIT_MASK_LA_MAC` reader - used for mask one or more bits of debug port"]
pub type BitMaskLaMacR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MASK_LA_MAC` writer - used for mask one or more bits of debug port"]
pub type BitMaskLaMacW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - used for mask one or more bits of debug port"]
    #[inline(always)]
    pub fn bit_mask_la_mac(&self) -> BitMaskLaMacR {
        BitMaskLaMacR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - used for mask one or more bits of debug port"]
    #[inline(always)]
    pub fn bit_mask_la_mac(&mut self) -> BitMaskLaMacW<'_, RegMaskLaMacSpec> {
        BitMaskLaMacW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mask_la_mac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mask_la_mac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMaskLaMacSpec;
impl crate::RegisterSpec for RegMaskLaMacSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mask_la_mac::R`](R) reader structure"]
impl crate::Readable for RegMaskLaMacSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mask_la_mac::W`](W) writer structure"]
impl crate::Writable for RegMaskLaMacSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MASK_LA_MAC to value 0"]
impl crate::Resettable for RegMaskLaMacSpec {}
