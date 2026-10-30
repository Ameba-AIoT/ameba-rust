#[doc = "Register `REG_SOUNDING_CFG2` reader"]
pub type R = crate::R<RegSoundingCfg2Spec>;
#[doc = "Register `REG_SOUNDING_CFG2` writer"]
pub type W = crate::W<RegSoundingCfg2Spec>;
#[doc = "Field `BIT_r_WMAC_SOUNDING_RXADD_L` reader - MACID of the associated beamformer1 bit\\[31:0\\]"]
pub type BitRWmacSoundingRxaddLR = crate::FieldReader<u32>;
#[doc = "Field `BIT_r_WMAC_SOUNDING_RXADD_L` writer - MACID of the associated beamformer1 bit\\[31:0\\]"]
pub type BitRWmacSoundingRxaddLW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MACID of the associated beamformer1 bit\\[31:0\\]"]
    #[inline(always)]
    pub fn bit_r_wmac_sounding_rxadd_l(&self) -> BitRWmacSoundingRxaddLR {
        BitRWmacSoundingRxaddLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MACID of the associated beamformer1 bit\\[31:0\\]"]
    #[inline(always)]
    pub fn bit_r_wmac_sounding_rxadd_l(
        &mut self,
    ) -> BitRWmacSoundingRxaddLW<'_, RegSoundingCfg2Spec> {
        BitRWmacSoundingRxaddLW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sounding_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sounding_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSoundingCfg2Spec;
impl crate::RegisterSpec for RegSoundingCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sounding_cfg2::R`](R) reader structure"]
impl crate::Readable for RegSoundingCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_sounding_cfg2::W`](W) writer structure"]
impl crate::Writable for RegSoundingCfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SOUNDING_CFG2 to value 0"]
impl crate::Resettable for RegSoundingCfg2Spec {}
