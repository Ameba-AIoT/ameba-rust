#[doc = "Register `MASKBLOCK_L` reader"]
pub type R = crate::R<MaskblockLSpec>;
#[doc = "Register `MASKBLOCK_L` writer"]
pub type W = crate::W<MaskblockLSpec>;
#[doc = "Field `MASKBLOCK_L_INT_MASK` reader - Interrupt Mask(chn0-chn7) 0 = masked 1 = unmasked"]
pub type MaskblockLIntMaskR = crate::FieldReader;
#[doc = "Field `MASKBLOCK_L_INT_MASK` writer - Interrupt Mask(chn0-chn7) 0 = masked 1 = unmasked"]
pub type MaskblockLIntMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MASKBLOCK_L_INT_MASK_WE` writer - Interrupt Mask Write Enable(chn0-chn7) 0 = write disabled 1 = write enabled"]
pub type MaskblockLIntMaskWeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt Mask(chn0-chn7) 0 = masked 1 = unmasked"]
    #[inline(always)]
    pub fn maskblock_l_int_mask(&self) -> MaskblockLIntMaskR {
        MaskblockLIntMaskR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt Mask(chn0-chn7) 0 = masked 1 = unmasked"]
    #[inline(always)]
    pub fn maskblock_l_int_mask(&mut self) -> MaskblockLIntMaskW<'_, MaskblockLSpec> {
        MaskblockLIntMaskW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt Mask Write Enable(chn0-chn7) 0 = write disabled 1 = write enabled"]
    #[inline(always)]
    pub fn maskblock_l_int_mask_we(&mut self) -> MaskblockLIntMaskWeW<'_, MaskblockLSpec> {
        MaskblockLIntMaskWeW::new(self, 8)
    }
}
#[doc = "Mask for IntBlock Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`maskblock_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maskblock_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaskblockLSpec;
impl crate::RegisterSpec for MaskblockLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maskblock_l::R`](R) reader structure"]
impl crate::Readable for MaskblockLSpec {}
#[doc = "`write(|w| ..)` method takes [`maskblock_l::W`](W) writer structure"]
impl crate::Writable for MaskblockLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MASKBLOCK_L to value 0"]
impl crate::Resettable for MaskblockLSpec {}
