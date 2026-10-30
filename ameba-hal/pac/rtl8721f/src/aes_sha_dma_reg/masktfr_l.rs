#[doc = "Register `MASKTFR_L` reader"]
pub type R = crate::R<MasktfrLSpec>;
#[doc = "Register `MASKTFR_L` writer"]
pub type W = crate::W<MasktfrLSpec>;
#[doc = "Field `MASKTFR_L_INT_MASK` reader - Interrupt Mask(chn0-chn7) 0 = masked 1 = unmasked"]
pub type MasktfrLIntMaskR = crate::FieldReader;
#[doc = "Field `MASKTFR_L_INT_MASK` writer - Interrupt Mask(chn0-chn7) 0 = masked 1 = unmasked"]
pub type MasktfrLIntMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MASKTFR_L_INT_MASK_WE` writer - Interrupt Mask Write Enable(chn0-chn7) 0 = write disabled 1 = write enabled"]
pub type MasktfrLIntMaskWeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt Mask(chn0-chn7) 0 = masked 1 = unmasked"]
    #[inline(always)]
    pub fn masktfr_l_int_mask(&self) -> MasktfrLIntMaskR {
        MasktfrLIntMaskR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt Mask(chn0-chn7) 0 = masked 1 = unmasked"]
    #[inline(always)]
    pub fn masktfr_l_int_mask(&mut self) -> MasktfrLIntMaskW<'_, MasktfrLSpec> {
        MasktfrLIntMaskW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt Mask Write Enable(chn0-chn7) 0 = write disabled 1 = write enabled"]
    #[inline(always)]
    pub fn masktfr_l_int_mask_we(&mut self) -> MasktfrLIntMaskWeW<'_, MasktfrLSpec> {
        MasktfrLIntMaskWeW::new(self, 8)
    }
}
#[doc = "Mask for IntTfr Interrupt (chn0-7)\n\nYou can [`read`](crate::Reg::read) this register and get [`masktfr_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`masktfr_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MasktfrLSpec;
impl crate::RegisterSpec for MasktfrLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`masktfr_l::R`](R) reader structure"]
impl crate::Readable for MasktfrLSpec {}
#[doc = "`write(|w| ..)` method takes [`masktfr_l::W`](W) writer structure"]
impl crate::Writable for MasktfrLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MASKTFR_L to value 0"]
impl crate::Resettable for MasktfrLSpec {}
