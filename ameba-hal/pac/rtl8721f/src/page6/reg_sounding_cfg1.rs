#[doc = "Register `REG_SOUNDING_CFG1` reader"]
pub type R = crate::R<RegSoundingCfg1Spec>;
#[doc = "Register `REG_SOUNDING_CFG1` writer"]
pub type W = crate::W<RegSoundingCfg1Spec>;
#[doc = "Field `BIT_r_WMAC_SOUNDING_RXADD_R0_H` reader - MACID of the associated beamformer0 bit\\[47:32\\]"]
pub type BitRWmacSoundingRxaddR0HR = crate::FieldReader<u16>;
#[doc = "Field `BIT_r_WMAC_SOUNDING_RXADD_R0_H` writer - MACID of the associated beamformer0 bit\\[47:32\\]"]
pub type BitRWmacSoundingRxaddR0HW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_r_WMAC_TXCSI_AID0` reader - P_AID of the associated bemaformer0"]
pub type BitRWmacTxcsiAid0R = crate::FieldReader<u16>;
#[doc = "Field `BIT_r_WMAC_TXCSI_AID0` writer - P_AID of the associated bemaformer0"]
pub type BitRWmacTxcsiAid0W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:15 - MACID of the associated beamformer0 bit\\[47:32\\]"]
    #[inline(always)]
    pub fn bit_r_wmac_sounding_rxadd_r0_h(&self) -> BitRWmacSoundingRxaddR0HR {
        BitRWmacSoundingRxaddR0HR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:24 - P_AID of the associated bemaformer0"]
    #[inline(always)]
    pub fn bit_r_wmac_txcsi_aid0(&self) -> BitRWmacTxcsiAid0R {
        BitRWmacTxcsiAid0R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MACID of the associated beamformer0 bit\\[47:32\\]"]
    #[inline(always)]
    pub fn bit_r_wmac_sounding_rxadd_r0_h(
        &mut self,
    ) -> BitRWmacSoundingRxaddR0HW<'_, RegSoundingCfg1Spec> {
        BitRWmacSoundingRxaddR0HW::new(self, 0)
    }
    #[doc = "Bits 16:24 - P_AID of the associated bemaformer0"]
    #[inline(always)]
    pub fn bit_r_wmac_txcsi_aid0(&mut self) -> BitRWmacTxcsiAid0W<'_, RegSoundingCfg1Spec> {
        BitRWmacTxcsiAid0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sounding_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sounding_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSoundingCfg1Spec;
impl crate::RegisterSpec for RegSoundingCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sounding_cfg1::R`](R) reader structure"]
impl crate::Readable for RegSoundingCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_sounding_cfg1::W`](W) writer structure"]
impl crate::Writable for RegSoundingCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SOUNDING_CFG1 to value 0"]
impl crate::Resettable for RegSoundingCfg1Spec {}
