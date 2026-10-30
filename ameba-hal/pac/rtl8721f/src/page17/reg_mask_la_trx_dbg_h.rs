#[doc = "Register `REG_MASK_LA_TRX_DBG_H` reader"]
pub type R = crate::R<RegMaskLaTrxDbgHSpec>;
#[doc = "Register `REG_MASK_LA_TRX_DBG_H` writer"]
pub type W = crate::W<RegMaskLaTrxDbgHSpec>;
#[doc = "Field `BIT_LA_TRX_PATTERN_MASK_H` reader - used for mask one or more bits of debug port"]
pub type BitLaTrxPatternMaskHR = crate::FieldReader<u32>;
#[doc = "Field `BIT_LA_TRX_PATTERN_MASK_H` writer - used for mask one or more bits of debug port"]
pub type BitLaTrxPatternMaskHW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - used for mask one or more bits of debug port"]
    #[inline(always)]
    pub fn bit_la_trx_pattern_mask_h(&self) -> BitLaTrxPatternMaskHR {
        BitLaTrxPatternMaskHR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - used for mask one or more bits of debug port"]
    #[inline(always)]
    pub fn bit_la_trx_pattern_mask_h(&mut self) -> BitLaTrxPatternMaskHW<'_, RegMaskLaTrxDbgHSpec> {
        BitLaTrxPatternMaskHW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mask_la_trx_dbg_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mask_la_trx_dbg_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMaskLaTrxDbgHSpec;
impl crate::RegisterSpec for RegMaskLaTrxDbgHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mask_la_trx_dbg_h::R`](R) reader structure"]
impl crate::Readable for RegMaskLaTrxDbgHSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mask_la_trx_dbg_h::W`](W) writer structure"]
impl crate::Writable for RegMaskLaTrxDbgHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MASK_LA_TRX_DBG_H to value 0"]
impl crate::Resettable for RegMaskLaTrxDbgHSpec {}
