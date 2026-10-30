#[doc = "Register `MASKERR_H` reader"]
pub type R = crate::R<MaskerrHSpec>;
#[doc = "Register `MASKERR_H` writer"]
pub type W = crate::W<MaskerrHSpec>;
#[doc = "Field `MASKERR_H_INT_MASK` reader - Interrupt Mask(chn0-chn7) 0 = masked 1 = unmasked These bits only exist when DMAC_ SECURE_MODE has beed defined"]
pub type MaskerrHIntMaskR = crate::FieldReader;
#[doc = "Field `MASKERR_H_INT_MASK` writer - Interrupt Mask(chn0-chn7) 0 = masked 1 = unmasked These bits only exist when DMAC_ SECURE_MODE has beed defined"]
pub type MaskerrHIntMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MASKERR_H_INT_MASK_WE` writer - Interrupt Mask Write Enable(chn0-chn7) 0 = write disabled 1 = write enabled These bits only exist when DMAC_ SECURE_MODE has beed defined"]
pub type MaskerrHIntMaskWeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt Mask(chn0-chn7) 0 = masked 1 = unmasked These bits only exist when DMAC_ SECURE_MODE has beed defined"]
    #[inline(always)]
    pub fn maskerr_h_int_mask(&self) -> MaskerrHIntMaskR {
        MaskerrHIntMaskR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt Mask(chn0-chn7) 0 = masked 1 = unmasked These bits only exist when DMAC_ SECURE_MODE has beed defined"]
    #[inline(always)]
    pub fn maskerr_h_int_mask(&mut self) -> MaskerrHIntMaskW<'_, MaskerrHSpec> {
        MaskerrHIntMaskW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt Mask Write Enable(chn0-chn7) 0 = write disabled 1 = write enabled These bits only exist when DMAC_ SECURE_MODE has beed defined"]
    #[inline(always)]
    pub fn maskerr_h_int_mask_we(&mut self) -> MaskerrHIntMaskWeW<'_, MaskerrHSpec> {
        MaskerrHIntMaskWeW::new(self, 8)
    }
}
#[doc = "Mask for non-secure access secure register error if defined\n\nYou can [`read`](crate::Reg::read) this register and get [`maskerr_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maskerr_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaskerrHSpec;
impl crate::RegisterSpec for MaskerrHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maskerr_h::R`](R) reader structure"]
impl crate::Readable for MaskerrHSpec {}
#[doc = "`write(|w| ..)` method takes [`maskerr_h::W`](W) writer structure"]
impl crate::Writable for MaskerrHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MASKERR_H to value 0"]
impl crate::Resettable for MaskerrHSpec {}
