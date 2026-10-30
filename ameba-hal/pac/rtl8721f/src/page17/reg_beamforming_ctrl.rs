#[doc = "Register `REG_BEAMFORMING_CTRL` reader"]
pub type R = crate::R<RegBeamformingCtrlSpec>;
#[doc = "Register `REG_BEAMFORMING_CTRL` writer"]
pub type W = crate::W<RegBeamformingCtrlSpec>;
#[doc = "Field `BIT_WMAC_HE_NDPA_EN` reader - Set to 1, MAC would inform BB of preparing to receive NDP if the received NDPA is valid."]
pub type BitWmacHeNdpaEnR = crate::BitReader;
#[doc = "Field `BIT_WMAC_HE_NDPA_EN` writer - Set to 1, MAC would inform BB of preparing to receive NDP if the received NDPA is valid."]
pub type BitWmacHeNdpaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_CSI_BFRP_STOPTX` reader - stop tx when rx ndpa/ndp and wait for bfrp poll frame"]
pub type BitWmacCsiBfrpStoptxR = crate::BitReader;
#[doc = "Field `BIT_WMAC_CSI_BFRP_STOPTX` writer - stop tx when rx ndpa/ndp and wait for bfrp poll frame"]
pub type BitWmacCsiBfrpStoptxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_MIMO_PARA_CNT` reader - width of mimo_para_en,deafult:3(mac clk:40M,pluse width:100ns)"]
pub type BitWmacMimoParaCntR = crate::FieldReader;
#[doc = "Field `BIT_WMAC_MIMO_PARA_CNT` writer - width of mimo_para_en,deafult:3(mac clk:40M,pluse width:100ns)"]
pub type BitWmacMimoParaCntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_WMAC_HT_CSI_RATE` reader - HT CSI init rate(HT mode + NSS + MCS)"]
pub type BitWmacHtCsiRateR = crate::FieldReader<u16>;
#[doc = "Field `BIT_WMAC_HT_CSI_RATE` writer - HT CSI init rate(HT mode + NSS + MCS)"]
pub type BitWmacHtCsiRateW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `BIT_WMAC_VHT_CSI_RATE` reader - VHT CSI init rate(VHT mode+ NSS + MCS)"]
pub type BitWmacVhtCsiRateR = crate::FieldReader<u16>;
#[doc = "Field `BIT_WMAC_VHT_CSI_RATE` writer - VHT CSI init rate(VHT mode+ NSS + MCS)"]
pub type BitWmacVhtCsiRateW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `BIT_WMAC_HE_CSI_RATE` reader - HE CSI init rate(HE mode+NSS + MCS)"]
pub type BitWmacHeCsiRateR = crate::FieldReader<u16>;
#[doc = "Field `BIT_WMAC_HE_CSI_RATE` writer - HE CSI init rate(HE mode+NSS + MCS)"]
pub type BitWmacHeCsiRateW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bit 0 - Set to 1, MAC would inform BB of preparing to receive NDP if the received NDPA is valid."]
    #[inline(always)]
    pub fn bit_wmac_he_ndpa_en(&self) -> BitWmacHeNdpaEnR {
        BitWmacHeNdpaEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - stop tx when rx ndpa/ndp and wait for bfrp poll frame"]
    #[inline(always)]
    pub fn bit_wmac_csi_bfrp_stoptx(&self) -> BitWmacCsiBfrpStoptxR {
        BitWmacCsiBfrpStoptxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - width of mimo_para_en,deafult:3(mac clk:40M,pluse width:100ns)"]
    #[inline(always)]
    pub fn bit_wmac_mimo_para_cnt(&self) -> BitWmacMimoParaCntR {
        BitWmacMimoParaCntR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:13 - HT CSI init rate(HT mode + NSS + MCS)"]
    #[inline(always)]
    pub fn bit_wmac_ht_csi_rate(&self) -> BitWmacHtCsiRateR {
        BitWmacHtCsiRateR::new(((self.bits >> 5) & 0x01ff) as u16)
    }
    #[doc = "Bits 14:22 - VHT CSI init rate(VHT mode+ NSS + MCS)"]
    #[inline(always)]
    pub fn bit_wmac_vht_csi_rate(&self) -> BitWmacVhtCsiRateR {
        BitWmacVhtCsiRateR::new(((self.bits >> 14) & 0x01ff) as u16)
    }
    #[doc = "Bits 23:31 - HE CSI init rate(HE mode+NSS + MCS)"]
    #[inline(always)]
    pub fn bit_wmac_he_csi_rate(&self) -> BitWmacHeCsiRateR {
        BitWmacHeCsiRateR::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Set to 1, MAC would inform BB of preparing to receive NDP if the received NDPA is valid."]
    #[inline(always)]
    pub fn bit_wmac_he_ndpa_en(&mut self) -> BitWmacHeNdpaEnW<'_, RegBeamformingCtrlSpec> {
        BitWmacHeNdpaEnW::new(self, 0)
    }
    #[doc = "Bit 1 - stop tx when rx ndpa/ndp and wait for bfrp poll frame"]
    #[inline(always)]
    pub fn bit_wmac_csi_bfrp_stoptx(
        &mut self,
    ) -> BitWmacCsiBfrpStoptxW<'_, RegBeamformingCtrlSpec> {
        BitWmacCsiBfrpStoptxW::new(self, 1)
    }
    #[doc = "Bits 2:4 - width of mimo_para_en,deafult:3(mac clk:40M,pluse width:100ns)"]
    #[inline(always)]
    pub fn bit_wmac_mimo_para_cnt(&mut self) -> BitWmacMimoParaCntW<'_, RegBeamformingCtrlSpec> {
        BitWmacMimoParaCntW::new(self, 2)
    }
    #[doc = "Bits 5:13 - HT CSI init rate(HT mode + NSS + MCS)"]
    #[inline(always)]
    pub fn bit_wmac_ht_csi_rate(&mut self) -> BitWmacHtCsiRateW<'_, RegBeamformingCtrlSpec> {
        BitWmacHtCsiRateW::new(self, 5)
    }
    #[doc = "Bits 14:22 - VHT CSI init rate(VHT mode+ NSS + MCS)"]
    #[inline(always)]
    pub fn bit_wmac_vht_csi_rate(&mut self) -> BitWmacVhtCsiRateW<'_, RegBeamformingCtrlSpec> {
        BitWmacVhtCsiRateW::new(self, 14)
    }
    #[doc = "Bits 23:31 - HE CSI init rate(HE mode+NSS + MCS)"]
    #[inline(always)]
    pub fn bit_wmac_he_csi_rate(&mut self) -> BitWmacHeCsiRateW<'_, RegBeamformingCtrlSpec> {
        BitWmacHeCsiRateW::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_beamforming_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_beamforming_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBeamformingCtrlSpec;
impl crate::RegisterSpec for RegBeamformingCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_beamforming_ctrl::R`](R) reader structure"]
impl crate::Readable for RegBeamformingCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_beamforming_ctrl::W`](W) writer structure"]
impl crate::Writable for RegBeamformingCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BEAMFORMING_CTRL to value 0xc040_100f"]
impl crate::Resettable for RegBeamformingCtrlSpec {
    const RESET_VALUE: u32 = 0xc040_100f;
}
