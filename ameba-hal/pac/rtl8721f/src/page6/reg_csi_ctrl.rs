#[doc = "Register `REG_CSI_CTRL` reader"]
pub type R = crate::R<RegCsiCtrlSpec>;
#[doc = "Register `REG_CSI_CTRL` writer"]
pub type W = crate::W<RegCsiCtrlSpec>;
#[doc = "Field `BIT_RXFIFO_WRPTR_WO_CHKSUM` reader - This bit is set to 1 when rx_fifo don't enable csi data checksum."]
pub type BitRxfifoWrptrWoChksumR = crate::BitReader;
#[doc = "Field `BIT_RXFIFO_WRPTR_WO_CHKSUM` writer - This bit is set to 1 when rx_fifo don't enable csi data checksum."]
pub type BitRxfifoWrptrWoChksumW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMC_CSI_RATE_FB_EN` reader - Enable CSI frame rate fallback (only one step in CSI RRSR) This bit is set to 1 to enable MAC to use previous latched csi resp rate for retry pkt."]
pub type BitWmcCsiRateFbEnR = crate::BitReader;
#[doc = "Field `BIT_WMC_CSI_RATE_FB_EN` writer - Enable CSI frame rate fallback (only one step in CSI RRSR) This bit is set to 1 to enable MAC to use previous latched csi resp rate for retry pkt."]
pub type BitWmcCsiRateFbEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CSIRD_EMPTY_APPZERO` reader - 0: Not append zero data when rx_fifo read csi data to empty. If csi data length of BB transmitted is smaller than MAC expected, Tx CSI report would be FCS error. 1: Append zero data when rx_fifo read csi data to empty. If csi data length of BB transmitted is smaller than MAC expected, there would be zero before FCS field of Tx CSI report pkt and the CSI report pkt would be FCS ok"]
pub type BitCsirdEmptyAppzeroR = crate::BitReader;
#[doc = "Field `BIT_CSIRD_EMPTY_APPZERO` writer - 0: Not append zero data when rx_fifo read csi data to empty. If csi data length of BB transmitted is smaller than MAC expected, Tx CSI report would be FCS error. 1: Append zero data when rx_fifo read csi data to empty. If csi data length of BB transmitted is smaller than MAC expected, there would be zero before FCS field of Tx CSI report pkt and the CSI report pkt would be FCS ok"]
pub type BitCsirdEmptyAppzeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_NDPVLD_PROTECT_RDRDY_DIS` reader - 0: Protect write enable of rx_fifo when ndp_valid =1 1: Not Protect write enable of rx_fifo when ndp_valid =1"]
pub type BitNdpvldProtectRdrdyDisR = crate::BitReader;
#[doc = "Field `BIT_NDPVLD_PROTECT_RDRDY_DIS` writer - 0: Protect write enable of rx_fifo when ndp_valid =1 1: Not Protect write enable of rx_fifo when ndp_valid =1"]
pub type BitNdpvldProtectRdrdyDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_NDPVLD_POS_RST_FFPTR_DIS_V1` reader - 0: Reset rx_fifo read/wrtie pointer when ndp_valid positive edge 1: Not reset rx_fifo read/wrtie pointer when ndp_valid positive edge"]
pub type BitNdpvldPosRstFfptrDisV1R = crate::BitReader;
#[doc = "Field `BIT_NDPVLD_POS_RST_FFPTR_DIS_V1` writer - 0: Reset rx_fifo read/wrtie pointer when ndp_valid positive edge 1: Not reset rx_fifo read/wrtie pointer when ndp_valid positive edge"]
pub type BitNdpvldPosRstFfptrDisV1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RDCSIMD_FLAG_TRIG_SEL` reader - 0: Trigger read csi mode flag raising when mac_csi_granted & ndp_valid positive edge 1: Trigger read csi mode flag raising when mac_csi_granted positive edge"]
pub type BitRdcsimdFlagTrigSelR = crate::BitReader;
#[doc = "Field `BIT_RDCSIMD_FLAG_TRIG_SEL` writer - 0: Trigger read csi mode flag raising when mac_csi_granted & ndp_valid positive edge 1: Trigger read csi mode flag raising when mac_csi_granted positive edge"]
pub type BitRdcsimdFlagTrigSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CSI_GID_SEL` reader - SIG-A content 1 : group id=0, 0 : group id=63"]
pub type BitCsiGidSelR = crate::BitReader;
#[doc = "Field `BIT_CSI_GID_SEL` writer - SIG-A content 1 : group id=0, 0 : group id=63"]
pub type BitCsiGidSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_CSI_RSC_FORCE` reader - If csi_force_rate_en = 1, CSI frame sub-channel configuration 00: Primary 20Mhz BW(0x483\\[3:0\\]) 01: Follow Rx packet BW. 10: Duplicate mode. 11: Reserved"]
pub type BitWmacCsiRscForceR = crate::FieldReader;
#[doc = "Field `BIT_WMAC_CSI_RSC_FORCE` writer - If csi_force_rate_en = 1, CSI frame sub-channel configuration 00: Primary 20Mhz BW(0x483\\[3:0\\]) 01: Follow Rx packet BW. 10: Duplicate mode. 11: Reserved"]
pub type BitWmacCsiRscForceW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_WMAC_CSI_RATE_FORCE_EN` reader - 0: disable CSI force rate. Refer to RRSR bitmask 1: CSI frame Tx rate refers to 0x6DC\\[29:24\\]"]
pub type BitWmacCsiRateForceEnR = crate::BitReader;
#[doc = "Field `BIT_WMAC_CSI_RATE_FORCE_EN` writer - 0: disable CSI force rate. Refer to RRSR bitmask 1: CSI frame Tx rate refers to 0x6DC\\[29:24\\]"]
pub type BitWmacCsiRateForceEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_USE_NDPARATE` reader - 1: use ndpa rx rate to decide csi report rate 0: use ndp rx rate to decide csi report rate"]
pub type BitUseNdparateR = crate::BitReader;
#[doc = "Field `BIT_USE_NDPARATE` writer - 1: use ndpa rx rate to decide csi report rate 0: use ndp rx rate to decide csi report rate"]
pub type BitUseNdparateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CTL_IDLE_CLR_CSI_RPT_V1` reader - ckr csi report request if response fsm idle"]
pub type BitCtlIdleClrCsiRptV1R = crate::BitReader;
#[doc = "Field `BIT_CTL_IDLE_CLR_CSI_RPT_V1` writer - ckr csi report request if response fsm idle"]
pub type BitCtlIdleClrCsiRptV1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - This bit is set to 1 when rx_fifo don't enable csi data checksum."]
    #[inline(always)]
    pub fn bit_rxfifo_wrptr_wo_chksum(&self) -> BitRxfifoWrptrWoChksumR {
        BitRxfifoWrptrWoChksumR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable CSI frame rate fallback (only one step in CSI RRSR) This bit is set to 1 to enable MAC to use previous latched csi resp rate for retry pkt."]
    #[inline(always)]
    pub fn bit_wmc_csi_rate_fb_en(&self) -> BitWmcCsiRateFbEnR {
        BitWmcCsiRateFbEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0: Not append zero data when rx_fifo read csi data to empty. If csi data length of BB transmitted is smaller than MAC expected, Tx CSI report would be FCS error. 1: Append zero data when rx_fifo read csi data to empty. If csi data length of BB transmitted is smaller than MAC expected, there would be zero before FCS field of Tx CSI report pkt and the CSI report pkt would be FCS ok"]
    #[inline(always)]
    pub fn bit_csird_empty_appzero(&self) -> BitCsirdEmptyAppzeroR {
        BitCsirdEmptyAppzeroR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0: Protect write enable of rx_fifo when ndp_valid =1 1: Not Protect write enable of rx_fifo when ndp_valid =1"]
    #[inline(always)]
    pub fn bit_ndpvld_protect_rdrdy_dis(&self) -> BitNdpvldProtectRdrdyDisR {
        BitNdpvldProtectRdrdyDisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 0: Reset rx_fifo read/wrtie pointer when ndp_valid positive edge 1: Not reset rx_fifo read/wrtie pointer when ndp_valid positive edge"]
    #[inline(always)]
    pub fn bit_ndpvld_pos_rst_ffptr_dis_v1(&self) -> BitNdpvldPosRstFfptrDisV1R {
        BitNdpvldPosRstFfptrDisV1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 0: Trigger read csi mode flag raising when mac_csi_granted & ndp_valid positive edge 1: Trigger read csi mode flag raising when mac_csi_granted positive edge"]
    #[inline(always)]
    pub fn bit_rdcsimd_flag_trig_sel(&self) -> BitRdcsimdFlagTrigSelR {
        BitRdcsimdFlagTrigSelR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SIG-A content 1 : group id=0, 0 : group id=63"]
    #[inline(always)]
    pub fn bit_csi_gid_sel(&self) -> BitCsiGidSelR {
        BitCsiGidSelR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - If csi_force_rate_en = 1, CSI frame sub-channel configuration 00: Primary 20Mhz BW(0x483\\[3:0\\]) 01: Follow Rx packet BW. 10: Duplicate mode. 11: Reserved"]
    #[inline(always)]
    pub fn bit_wmac_csi_rsc_force(&self) -> BitWmacCsiRscForceR {
        BitWmacCsiRscForceR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - 0: disable CSI force rate. Refer to RRSR bitmask 1: CSI frame Tx rate refers to 0x6DC\\[29:24\\]"]
    #[inline(always)]
    pub fn bit_wmac_csi_rate_force_en(&self) -> BitWmacCsiRateForceEnR {
        BitWmacCsiRateForceEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: use ndpa rx rate to decide csi report rate 0: use ndp rx rate to decide csi report rate"]
    #[inline(always)]
    pub fn bit_use_ndparate(&self) -> BitUseNdparateR {
        BitUseNdparateR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ckr csi report request if response fsm idle"]
    #[inline(always)]
    pub fn bit_ctl_idle_clr_csi_rpt_v1(&self) -> BitCtlIdleClrCsiRptV1R {
        BitCtlIdleClrCsiRptV1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - This bit is set to 1 when rx_fifo don't enable csi data checksum."]
    #[inline(always)]
    pub fn bit_rxfifo_wrptr_wo_chksum(&mut self) -> BitRxfifoWrptrWoChksumW<'_, RegCsiCtrlSpec> {
        BitRxfifoWrptrWoChksumW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable CSI frame rate fallback (only one step in CSI RRSR) This bit is set to 1 to enable MAC to use previous latched csi resp rate for retry pkt."]
    #[inline(always)]
    pub fn bit_wmc_csi_rate_fb_en(&mut self) -> BitWmcCsiRateFbEnW<'_, RegCsiCtrlSpec> {
        BitWmcCsiRateFbEnW::new(self, 7)
    }
    #[doc = "Bit 8 - 0: Not append zero data when rx_fifo read csi data to empty. If csi data length of BB transmitted is smaller than MAC expected, Tx CSI report would be FCS error. 1: Append zero data when rx_fifo read csi data to empty. If csi data length of BB transmitted is smaller than MAC expected, there would be zero before FCS field of Tx CSI report pkt and the CSI report pkt would be FCS ok"]
    #[inline(always)]
    pub fn bit_csird_empty_appzero(&mut self) -> BitCsirdEmptyAppzeroW<'_, RegCsiCtrlSpec> {
        BitCsirdEmptyAppzeroW::new(self, 8)
    }
    #[doc = "Bit 9 - 0: Protect write enable of rx_fifo when ndp_valid =1 1: Not Protect write enable of rx_fifo when ndp_valid =1"]
    #[inline(always)]
    pub fn bit_ndpvld_protect_rdrdy_dis(
        &mut self,
    ) -> BitNdpvldProtectRdrdyDisW<'_, RegCsiCtrlSpec> {
        BitNdpvldProtectRdrdyDisW::new(self, 9)
    }
    #[doc = "Bit 10 - 0: Reset rx_fifo read/wrtie pointer when ndp_valid positive edge 1: Not reset rx_fifo read/wrtie pointer when ndp_valid positive edge"]
    #[inline(always)]
    pub fn bit_ndpvld_pos_rst_ffptr_dis_v1(
        &mut self,
    ) -> BitNdpvldPosRstFfptrDisV1W<'_, RegCsiCtrlSpec> {
        BitNdpvldPosRstFfptrDisV1W::new(self, 10)
    }
    #[doc = "Bit 11 - 0: Trigger read csi mode flag raising when mac_csi_granted & ndp_valid positive edge 1: Trigger read csi mode flag raising when mac_csi_granted positive edge"]
    #[inline(always)]
    pub fn bit_rdcsimd_flag_trig_sel(&mut self) -> BitRdcsimdFlagTrigSelW<'_, RegCsiCtrlSpec> {
        BitRdcsimdFlagTrigSelW::new(self, 11)
    }
    #[doc = "Bit 12 - SIG-A content 1 : group id=0, 0 : group id=63"]
    #[inline(always)]
    pub fn bit_csi_gid_sel(&mut self) -> BitCsiGidSelW<'_, RegCsiCtrlSpec> {
        BitCsiGidSelW::new(self, 12)
    }
    #[doc = "Bits 13:14 - If csi_force_rate_en = 1, CSI frame sub-channel configuration 00: Primary 20Mhz BW(0x483\\[3:0\\]) 01: Follow Rx packet BW. 10: Duplicate mode. 11: Reserved"]
    #[inline(always)]
    pub fn bit_wmac_csi_rsc_force(&mut self) -> BitWmacCsiRscForceW<'_, RegCsiCtrlSpec> {
        BitWmacCsiRscForceW::new(self, 13)
    }
    #[doc = "Bit 15 - 0: disable CSI force rate. Refer to RRSR bitmask 1: CSI frame Tx rate refers to 0x6DC\\[29:24\\]"]
    #[inline(always)]
    pub fn bit_wmac_csi_rate_force_en(&mut self) -> BitWmacCsiRateForceEnW<'_, RegCsiCtrlSpec> {
        BitWmacCsiRateForceEnW::new(self, 15)
    }
    #[doc = "Bit 30 - 1: use ndpa rx rate to decide csi report rate 0: use ndp rx rate to decide csi report rate"]
    #[inline(always)]
    pub fn bit_use_ndparate(&mut self) -> BitUseNdparateW<'_, RegCsiCtrlSpec> {
        BitUseNdparateW::new(self, 30)
    }
    #[doc = "Bit 31 - ckr csi report request if response fsm idle"]
    #[inline(always)]
    pub fn bit_ctl_idle_clr_csi_rpt_v1(&mut self) -> BitCtlIdleClrCsiRptV1W<'_, RegCsiCtrlSpec> {
        BitCtlIdleClrCsiRptV1W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_csi_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_csi_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCsiCtrlSpec;
impl crate::RegisterSpec for RegCsiCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_csi_ctrl::R`](R) reader structure"]
impl crate::Readable for RegCsiCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_csi_ctrl::W`](W) writer structure"]
impl crate::Writable for RegCsiCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CSI_CTRL to value 0x4000_0000"]
impl crate::Resettable for RegCsiCtrlSpec {
    const RESET_VALUE: u32 = 0x4000_0000;
}
