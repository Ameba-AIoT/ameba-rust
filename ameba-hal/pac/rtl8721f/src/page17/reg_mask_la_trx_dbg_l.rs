#[doc = "Register `REG_MASK_LA_TRX_DBG_L` reader"]
pub type R = crate::R<RegMaskLaTrxDbgLSpec>;
#[doc = "Register `REG_MASK_LA_TRX_DBG_L` writer"]
pub type W = crate::W<RegMaskLaTrxDbgLSpec>;
#[doc = "Field `BIT_LA_TRX_PATTERN_MASK_L` reader - used for mask one or more bits of debug port"]
pub type BitLaTrxPatternMaskLR = crate::FieldReader<u32>;
#[doc = "Field `BIT_LA_TRX_PATTERN_MASK_L` writer - used for mask one or more bits of debug port"]
pub type BitLaTrxPatternMaskLW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - used for mask one or more bits of debug port"]
    #[inline(always)]
    pub fn bit_la_trx_pattern_mask_l(&self) -> BitLaTrxPatternMaskLR {
        BitLaTrxPatternMaskLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - used for mask one or more bits of debug port"]
    #[inline(always)]
    pub fn bit_la_trx_pattern_mask_l(&mut self) -> BitLaTrxPatternMaskLW<'_, RegMaskLaTrxDbgLSpec> {
        BitLaTrxPatternMaskLW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mask_la_trx_dbg_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mask_la_trx_dbg_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMaskLaTrxDbgLSpec;
impl crate::RegisterSpec for RegMaskLaTrxDbgLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mask_la_trx_dbg_l::R`](R) reader structure"]
impl crate::Readable for RegMaskLaTrxDbgLSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mask_la_trx_dbg_l::W`](W) writer structure"]
impl crate::Writable for RegMaskLaTrxDbgLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MASK_LA_TRX_DBG_L to value 0"]
impl crate::Resettable for RegMaskLaTrxDbgLSpec {}
