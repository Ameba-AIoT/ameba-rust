#[doc = "Register `REG_TXBF_CTRL` reader"]
pub type R = crate::R<RegTxbfCtrlSpec>;
#[doc = "Register `REG_TXBF_CTRL` writer"]
pub type W = crate::W<RegTxbfCtrlSpec>;
#[doc = "Field `BIT_r_txbf0_aid` reader - P_AID\\[8:0\\] of the associated beamformee0"]
pub type BitRTxbf0AidR = crate::FieldReader<u16>;
#[doc = "Field `BIT_r_txbf0_aid` writer - P_AID\\[8:0\\] of the associated beamformee0"]
pub type BitRTxbf0AidW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `BIT_r_txbf0_20M` reader - TxBF status of beamformee0 @BW=20M"]
pub type BitRTxbf0_20mR = crate::BitReader;
#[doc = "Field `BIT_r_txbf0_20M` writer - TxBF status of beamformee0 @BW=20M"]
pub type BitRTxbf0_20mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_txbf0_40M` reader - TxBF status of beamformee0 @BW=40M"]
pub type BitRTxbf0_40mR = crate::BitReader;
#[doc = "Field `BIT_r_txbf0_40M` writer - TxBF status of beamformee0 @BW=40M"]
pub type BitRTxbf0_40mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_txbf0_80M` reader - TxBF status of beamformee0 @BW=80M"]
pub type BitRTxbf0_80mR = crate::BitReader;
#[doc = "Field `BIT_r_txbf0_80M` writer - TxBF status of beamformee0 @BW=80M"]
pub type BitRTxbf0_80mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BCN_NOBLOCK_NDP` reader - beacon packet will not block NDP packet"]
pub type BitBcnNoblockNdpR = crate::BitReader;
#[doc = "Field `BIT_BCN_NOBLOCK_NDP` writer - beacon packet will not block NDP packet"]
pub type BitBcnNoblockNdpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_NDP_BFEN_DIS` reader - 1: disable beamforming with NDP"]
pub type BitNdpBfenDisR = crate::BitReader;
#[doc = "Field `BIT_NDP_BFEN_DIS` writer - 1: disable beamforming with NDP"]
pub type BitNdpBfenDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_txbf1_aid` reader - P_AID\\[8:0\\] of the associated beamformee1"]
pub type BitRTxbf1AidR = crate::FieldReader<u16>;
#[doc = "Field `BIT_r_txbf1_aid` writer - P_AID\\[8:0\\] of the associated beamformee1"]
pub type BitRTxbf1AidW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `BIT_r_txbf1_20M` reader - TxBF status of beamformee1 @BW=20M"]
pub type BitRTxbf1_20mR = crate::BitReader;
#[doc = "Field `BIT_r_txbf1_20M` writer - TxBF status of beamformee1 @BW=20M"]
pub type BitRTxbf1_20mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_txbf1_40M` reader - TxBF status of beamformee1 @BW=40M"]
pub type BitRTxbf1_40mR = crate::BitReader;
#[doc = "Field `BIT_r_txbf1_40M` writer - TxBF status of beamformee1 @BW=40M"]
pub type BitRTxbf1_40mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_txbf1_80M` reader - TxBF status of beamformee1 @BW=80M."]
pub type BitRTxbf1_80mR = crate::BitReader;
#[doc = "Field `BIT_r_txbf1_80M` writer - TxBF status of beamformee1 @BW=80M."]
pub type BitRTxbf1_80mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_en_ndpa_int` reader - 1: enable ndpa interrupt; 0: disable ndpa interrupt"]
pub type BitREnNdpaIntR = crate::BitReader;
#[doc = "Field `BIT_r_en_ndpa_int` writer - 1: enable ndpa interrupt; 0: disable ndpa interrupt"]
pub type BitREnNdpaIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_use_ndpa_parameter` reader - 1: NDPA rate and BW use reg0x45f settings 0: NDPA rate and BW use NDP settings in Tx descriptor"]
pub type BitUseNdpaParameterR = crate::BitReader;
#[doc = "Field `BIT_use_ndpa_parameter` writer - 1: NDPA rate and BW use reg0x45f settings 0: NDPA rate and BW use NDP settings in Tx descriptor"]
pub type BitUseNdpaParameterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_enable_ndpa` reader - Set '1' for MAC to enable sending NDPA or NDP Announcement MPDU"]
pub type BitREnableNdpaR = crate::BitReader;
#[doc = "Field `BIT_r_enable_ndpa` writer - Set '1' for MAC to enable sending NDPA or NDP Announcement MPDU"]
pub type BitREnableNdpaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - P_AID\\[8:0\\] of the associated beamformee0"]
    #[inline(always)]
    pub fn bit_r_txbf0_aid(&self) -> BitRTxbf0AidR {
        BitRTxbf0AidR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - TxBF status of beamformee0 @BW=20M"]
    #[inline(always)]
    pub fn bit_r_txbf0_20m(&self) -> BitRTxbf0_20mR {
        BitRTxbf0_20mR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TxBF status of beamformee0 @BW=40M"]
    #[inline(always)]
    pub fn bit_r_txbf0_40m(&self) -> BitRTxbf0_40mR {
        BitRTxbf0_40mR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TxBF status of beamformee0 @BW=80M"]
    #[inline(always)]
    pub fn bit_r_txbf0_80m(&self) -> BitRTxbf0_80mR {
        BitRTxbf0_80mR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - beacon packet will not block NDP packet"]
    #[inline(always)]
    pub fn bit_bcn_noblock_ndp(&self) -> BitBcnNoblockNdpR {
        BitBcnNoblockNdpR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: disable beamforming with NDP"]
    #[inline(always)]
    pub fn bit_ndp_bfen_dis(&self) -> BitNdpBfenDisR {
        BitNdpBfenDisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:24 - P_AID\\[8:0\\] of the associated beamformee1"]
    #[inline(always)]
    pub fn bit_r_txbf1_aid(&self) -> BitRTxbf1AidR {
        BitRTxbf1AidR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 25 - TxBF status of beamformee1 @BW=20M"]
    #[inline(always)]
    pub fn bit_r_txbf1_20m(&self) -> BitRTxbf1_20mR {
        BitRTxbf1_20mR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TxBF status of beamformee1 @BW=40M"]
    #[inline(always)]
    pub fn bit_r_txbf1_40m(&self) -> BitRTxbf1_40mR {
        BitRTxbf1_40mR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TxBF status of beamformee1 @BW=80M."]
    #[inline(always)]
    pub fn bit_r_txbf1_80m(&self) -> BitRTxbf1_80mR {
        BitRTxbf1_80mR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: enable ndpa interrupt; 0: disable ndpa interrupt"]
    #[inline(always)]
    pub fn bit_r_en_ndpa_int(&self) -> BitREnNdpaIntR {
        BitREnNdpaIntR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: NDPA rate and BW use reg0x45f settings 0: NDPA rate and BW use NDP settings in Tx descriptor"]
    #[inline(always)]
    pub fn bit_use_ndpa_parameter(&self) -> BitUseNdpaParameterR {
        BitUseNdpaParameterR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set '1' for MAC to enable sending NDPA or NDP Announcement MPDU"]
    #[inline(always)]
    pub fn bit_r_enable_ndpa(&self) -> BitREnableNdpaR {
        BitREnableNdpaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - P_AID\\[8:0\\] of the associated beamformee0"]
    #[inline(always)]
    pub fn bit_r_txbf0_aid(&mut self) -> BitRTxbf0AidW<'_, RegTxbfCtrlSpec> {
        BitRTxbf0AidW::new(self, 0)
    }
    #[doc = "Bit 9 - TxBF status of beamformee0 @BW=20M"]
    #[inline(always)]
    pub fn bit_r_txbf0_20m(&mut self) -> BitRTxbf0_20mW<'_, RegTxbfCtrlSpec> {
        BitRTxbf0_20mW::new(self, 9)
    }
    #[doc = "Bit 10 - TxBF status of beamformee0 @BW=40M"]
    #[inline(always)]
    pub fn bit_r_txbf0_40m(&mut self) -> BitRTxbf0_40mW<'_, RegTxbfCtrlSpec> {
        BitRTxbf0_40mW::new(self, 10)
    }
    #[doc = "Bit 11 - TxBF status of beamformee0 @BW=80M"]
    #[inline(always)]
    pub fn bit_r_txbf0_80m(&mut self) -> BitRTxbf0_80mW<'_, RegTxbfCtrlSpec> {
        BitRTxbf0_80mW::new(self, 11)
    }
    #[doc = "Bit 14 - beacon packet will not block NDP packet"]
    #[inline(always)]
    pub fn bit_bcn_noblock_ndp(&mut self) -> BitBcnNoblockNdpW<'_, RegTxbfCtrlSpec> {
        BitBcnNoblockNdpW::new(self, 14)
    }
    #[doc = "Bit 15 - 1: disable beamforming with NDP"]
    #[inline(always)]
    pub fn bit_ndp_bfen_dis(&mut self) -> BitNdpBfenDisW<'_, RegTxbfCtrlSpec> {
        BitNdpBfenDisW::new(self, 15)
    }
    #[doc = "Bits 16:24 - P_AID\\[8:0\\] of the associated beamformee1"]
    #[inline(always)]
    pub fn bit_r_txbf1_aid(&mut self) -> BitRTxbf1AidW<'_, RegTxbfCtrlSpec> {
        BitRTxbf1AidW::new(self, 16)
    }
    #[doc = "Bit 25 - TxBF status of beamformee1 @BW=20M"]
    #[inline(always)]
    pub fn bit_r_txbf1_20m(&mut self) -> BitRTxbf1_20mW<'_, RegTxbfCtrlSpec> {
        BitRTxbf1_20mW::new(self, 25)
    }
    #[doc = "Bit 26 - TxBF status of beamformee1 @BW=40M"]
    #[inline(always)]
    pub fn bit_r_txbf1_40m(&mut self) -> BitRTxbf1_40mW<'_, RegTxbfCtrlSpec> {
        BitRTxbf1_40mW::new(self, 26)
    }
    #[doc = "Bit 27 - TxBF status of beamformee1 @BW=80M."]
    #[inline(always)]
    pub fn bit_r_txbf1_80m(&mut self) -> BitRTxbf1_80mW<'_, RegTxbfCtrlSpec> {
        BitRTxbf1_80mW::new(self, 27)
    }
    #[doc = "Bit 28 - 1: enable ndpa interrupt; 0: disable ndpa interrupt"]
    #[inline(always)]
    pub fn bit_r_en_ndpa_int(&mut self) -> BitREnNdpaIntW<'_, RegTxbfCtrlSpec> {
        BitREnNdpaIntW::new(self, 28)
    }
    #[doc = "Bit 30 - 1: NDPA rate and BW use reg0x45f settings 0: NDPA rate and BW use NDP settings in Tx descriptor"]
    #[inline(always)]
    pub fn bit_use_ndpa_parameter(&mut self) -> BitUseNdpaParameterW<'_, RegTxbfCtrlSpec> {
        BitUseNdpaParameterW::new(self, 30)
    }
    #[doc = "Bit 31 - Set '1' for MAC to enable sending NDPA or NDP Announcement MPDU"]
    #[inline(always)]
    pub fn bit_r_enable_ndpa(&mut self) -> BitREnableNdpaW<'_, RegTxbfCtrlSpec> {
        BitREnableNdpaW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_txbf_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_txbf_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTxbfCtrlSpec;
impl crate::RegisterSpec for RegTxbfCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_txbf_ctrl::R`](R) reader structure"]
impl crate::Readable for RegTxbfCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_txbf_ctrl::W`](W) writer structure"]
impl crate::Writable for RegTxbfCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TXBF_CTRL to value 0xc000"]
impl crate::Resettable for RegTxbfCtrlSpec {
    const RESET_VALUE: u32 = 0xc000;
}
