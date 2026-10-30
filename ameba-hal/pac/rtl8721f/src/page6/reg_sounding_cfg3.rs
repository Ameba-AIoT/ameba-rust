#[doc = "Register `REG_SOUNDING_CFG3` reader"]
pub type R = crate::R<RegSoundingCfg3Spec>;
#[doc = "Register `REG_SOUNDING_CFG3` writer"]
pub type W = crate::W<RegSoundingCfg3Spec>;
#[doc = "Field `BIT_r_WMAC_SOUNDING_RXADD_H` reader - MACID of the associated beamformer1 bit\\[15:0\\]"]
pub type BitRWmacSoundingRxaddHR = crate::FieldReader<u16>;
#[doc = "Field `BIT_r_WMAC_SOUNDING_RXADD_H` writer - MACID of the associated beamformer1 bit\\[15:0\\]"]
pub type BitRWmacSoundingRxaddHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_r_WMAC_TXCSI_AID1` reader - P_AID of the associated bemaformer1"]
pub type BitRWmacTxcsiAid1R = crate::FieldReader<u16>;
#[doc = "Field `BIT_r_WMAC_TXCSI_AID1` writer - P_AID of the associated bemaformer1"]
pub type BitRWmacTxcsiAid1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:15 - MACID of the associated beamformer1 bit\\[15:0\\]"]
    #[inline(always)]
    pub fn bit_r_wmac_sounding_rxadd_h(&self) -> BitRWmacSoundingRxaddHR {
        BitRWmacSoundingRxaddHR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:24 - P_AID of the associated bemaformer1"]
    #[inline(always)]
    pub fn bit_r_wmac_txcsi_aid1(&self) -> BitRWmacTxcsiAid1R {
        BitRWmacTxcsiAid1R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MACID of the associated beamformer1 bit\\[15:0\\]"]
    #[inline(always)]
    pub fn bit_r_wmac_sounding_rxadd_h(
        &mut self,
    ) -> BitRWmacSoundingRxaddHW<'_, RegSoundingCfg3Spec> {
        BitRWmacSoundingRxaddHW::new(self, 0)
    }
    #[doc = "Bits 16:24 - P_AID of the associated bemaformer1"]
    #[inline(always)]
    pub fn bit_r_wmac_txcsi_aid1(&mut self) -> BitRWmacTxcsiAid1W<'_, RegSoundingCfg3Spec> {
        BitRWmacTxcsiAid1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sounding_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sounding_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSoundingCfg3Spec;
impl crate::RegisterSpec for RegSoundingCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sounding_cfg3::R`](R) reader structure"]
impl crate::Readable for RegSoundingCfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_sounding_cfg3::W`](W) writer structure"]
impl crate::Writable for RegSoundingCfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SOUNDING_CFG3 to value 0"]
impl crate::Resettable for RegSoundingCfg3Spec {}
