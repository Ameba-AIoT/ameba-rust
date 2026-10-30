#[doc = "Register `MASKERR_L` reader"]
pub type R = crate::R<MaskerrLSpec>;
#[doc = "Register `MASKERR_L` writer"]
pub type W = crate::W<MaskerrLSpec>;
#[doc = "Field `MASKERR_L_INT_MASK` reader - Interrupt Mask(chn0-chn7) 0 = masked 1 = unmasked"]
pub type MaskerrLIntMaskR = crate::FieldReader;
#[doc = "Field `MASKERR_L_INT_MASK` writer - Interrupt Mask(chn0-chn7) 0 = masked 1 = unmasked"]
pub type MaskerrLIntMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MASKERR_L_INT_MASK_WE` writer - Interrupt Mask Write Enable(chn0-chn7) 0 = write disabled 1 = write enabled"]
pub type MaskerrLIntMaskWeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt Mask(chn0-chn7) 0 = masked 1 = unmasked"]
    #[inline(always)]
    pub fn maskerr_l_int_mask(&self) -> MaskerrLIntMaskR {
        MaskerrLIntMaskR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt Mask(chn0-chn7) 0 = masked 1 = unmasked"]
    #[inline(always)]
    pub fn maskerr_l_int_mask(&mut self) -> MaskerrLIntMaskW<'_, MaskerrLSpec> {
        MaskerrLIntMaskW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt Mask Write Enable(chn0-chn7) 0 = write disabled 1 = write enabled"]
    #[inline(always)]
    pub fn maskerr_l_int_mask_we(&mut self) -> MaskerrLIntMaskWeW<'_, MaskerrLSpec> {
        MaskerrLIntMaskWeW::new(self, 8)
    }
}
#[doc = "Mask for IntErr Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`maskerr_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maskerr_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaskerrLSpec;
impl crate::RegisterSpec for MaskerrLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maskerr_l::R`](R) reader structure"]
impl crate::Readable for MaskerrLSpec {}
#[doc = "`write(|w| ..)` method takes [`maskerr_l::W`](W) writer structure"]
impl crate::Writable for MaskerrLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MASKERR_L to value 0"]
impl crate::Resettable for MaskerrLSpec {}
