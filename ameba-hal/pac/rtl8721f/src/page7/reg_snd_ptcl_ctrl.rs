#[doc = "Register `REG_SND_PTCL_CTRL` reader"]
pub type R = crate::R<RegSndPtclCtrlSpec>;
#[doc = "Register `REG_SND_PTCL_CTRL` writer"]
pub type W = crate::W<RegSndPtclCtrlSpec>;
#[doc = "Field `BIT_r_WMAC_VHT_NDPA_EN` reader - Set to 1, MAC would inform BB of preparing to receive NDP if the received NDPA is valid."]
pub type BitRWmacVhtNdpaEnR = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_VHT_NDPA_EN` writer - Set to 1, MAC would inform BB of preparing to receive NDP if the received NDPA is valid."]
pub type BitRWmacVhtNdpaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_HT_NDPA_EN` reader - Set to 1, MAC would inform BB of preparing to receive NDP if the received NDP Announcement MPDU is valid."]
pub type BitRWmacHtNdpaEnR = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_HT_NDPA_EN` writer - Set to 1, MAC would inform BB of preparing to receive NDP if the received NDP Announcement MPDU is valid."]
pub type BitRWmacHtNdpaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_CSI_WITHHTC_EN` reader - Set to 1, CSI frame would carry HTC in MAC header."]
pub type BitRWmacCsiWithhtcEnR = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_CSI_WITHHTC_EN` writer - Set to 1, CSI frame would carry HTC in MAC header."]
pub type BitRWmacCsiWithhtcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_CSISEQ_SEL` reader - By which the sequence number of CSI frame is controlled 1: non-QoS SEQ number 0: counter"]
pub type BitRWmacCsiseqSelR = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_CSISEQ_SEL` writer - By which the sequence number of CSI frame is controlled 1: non-QoS SEQ number 0: counter"]
pub type BitRWmacCsiseqSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_BFPARAM_SEL` reader - 1: use beamforming nc/nr/ng parameter from register 0x6F4 0: use beamforming nc/nr/ng parameter from tx report buffer"]
pub type BitRWmacBfparamSelR = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_BFPARAM_SEL` writer - 1: use beamforming nc/nr/ng parameter from register 0x6F4 0: use beamforming nc/nr/ng parameter from tx report buffer"]
pub type BitRWmacBfparamSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_disable_check_vhtsiga_crc` reader - 1: disable, 0: enable"]
pub type BitRDisableCheckVhtsigaCrcR = crate::BitReader;
#[doc = "Field `BIT_r_disable_check_vhtsiga_crc` writer - 1: disable, 0: enable"]
pub type BitRDisableCheckVhtsigaCrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_Disable_check_vhtsigb_crc` reader - 1: disable, 0: enable"]
pub type BitRDisableCheckVhtsigbCrcR = crate::BitReader;
#[doc = "Field `BIT_r_Disable_check_vhtsigb_crc` writer - 1: disable, 0: enable"]
pub type BitRDisableCheckVhtsigbCrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_USE_NSTS` reader - 1: use Nsts as Nr, 0: use register value as Nr"]
pub type BitRWmacUseNstsR = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_USE_NSTS` writer - 1: use Nsts as Nr, 0: use register value as Nr"]
pub type BitRWmacUseNstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_VHT_CATEGORY` reader - used as csi report offset"]
pub type BitRWmacVhtCategoryR = crate::FieldReader;
#[doc = "Field `BIT_r_WMAC_VHT_CATEGORY` writer - used as csi report offset"]
pub type BitRWmacVhtCategoryW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BIT_r_WMAC_CSI_CHKSUM_DIS` reader - 1:Disable check 16bit checksum value of csi report from BB 0:Enable check 16bit checksum value of csi report from BB"]
pub type BitRWmacCsiChksumDisR = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_CSI_CHKSUM_DIS` writer - 1:Disable check 16bit checksum value of csi report from BB 0:Enable check 16bit checksum value of csi report from BB"]
pub type BitRWmacCsiChksumDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_OFFSET_RPTPOLL_EN` reader - 1: If previous Tx pkt is BF Rpt Poll or VHT NDP, use vht csi start offset; otherwise ht csi start offset 0: If previous Tx pkt is vht rate, use vht csi start offset; otherwise ht csi start offset"]
pub type BitRWmacOffsetRptpollEnR = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_OFFSET_RPTPOLL_EN` writer - 1: If previous Tx pkt is BF Rpt Poll or VHT NDP, use vht csi start offset; otherwise ht csi start offset 0: If previous Tx pkt is vht rate, use vht csi start offset; otherwise ht csi start offset"]
pub type BitRWmacOffsetRptpollEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_HT_CATEGORY` reader - csi report offset for ht mode"]
pub type BitRWmacHtCategoryR = crate::FieldReader;
#[doc = "Field `BIT_r_WMAC_HT_CATEGORY` writer - csi report offset for ht mode"]
pub type BitRWmacHtCategoryW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BIT_r_WMAC_CHK_UCNDPA_A2_DIS` reader - 0: If A1-matched NDPA's A2 fails to match 0x6E4 or 0x6EC, use default csi parameter 1: For any receiving A1-matched NDPA, use 0x6F4\\[11:0\\] as csi parameter"]
pub type BitRWmacChkUcndpaA2DisR = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_CHK_UCNDPA_A2_DIS` writer - 0: If A1-matched NDPA's A2 fails to match 0x6E4 or 0x6EC, use default csi parameter 1: For any receiving A1-matched NDPA, use 0x6F4\\[11:0\\] as csi parameter"]
pub type BitRWmacChkUcndpaA2DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_CHK_RPTPOLL_A2_DIS` reader - 1: For any A1-matched BF Rpt Poll, send back VHT-CB 0: If A1-matched BF Rpt Poll's A2 fails to match the previous receiving NDPA's A2, do not send VHT-CB"]
pub type BitRWmacChkRptpollA2DisR = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_CHK_RPTPOLL_A2_DIS` writer - 1: For any A1-matched BF Rpt Poll, send back VHT-CB 0: If A1-matched BF Rpt Poll's A2 fails to match the previous receiving NDPA's A2, do not send VHT-CB"]
pub type BitRWmacChkRptpollA2DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ndp_rx_standby_timer` reader - A timeout value for MAC to leave NDP_Rx_standby_state and go back to normal Rx mode. (unit: 2us)"]
pub type BitNdpRxStandbyTimerR = crate::FieldReader;
#[doc = "Field `BIT_ndp_rx_standby_timer` writer - A timeout value for MAC to leave NDP_Rx_standby_state and go back to normal Rx mode. (unit: 2us)"]
pub type BitNdpRxStandbyTimerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Set to 1, MAC would inform BB of preparing to receive NDP if the received NDPA is valid."]
    #[inline(always)]
    pub fn bit_r_wmac_vht_ndpa_en(&self) -> BitRWmacVhtNdpaEnR {
        BitRWmacVhtNdpaEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set to 1, MAC would inform BB of preparing to receive NDP if the received NDP Announcement MPDU is valid."]
    #[inline(always)]
    pub fn bit_r_wmac_ht_ndpa_en(&self) -> BitRWmacHtNdpaEnR {
        BitRWmacHtNdpaEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set to 1, CSI frame would carry HTC in MAC header."]
    #[inline(always)]
    pub fn bit_r_wmac_csi_withhtc_en(&self) -> BitRWmacCsiWithhtcEnR {
        BitRWmacCsiWithhtcEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - By which the sequence number of CSI frame is controlled 1: non-QoS SEQ number 0: counter"]
    #[inline(always)]
    pub fn bit_r_wmac_csiseq_sel(&self) -> BitRWmacCsiseqSelR {
        BitRWmacCsiseqSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: use beamforming nc/nr/ng parameter from register 0x6F4 0: use beamforming nc/nr/ng parameter from tx report buffer"]
    #[inline(always)]
    pub fn bit_r_wmac_bfparam_sel(&self) -> BitRWmacBfparamSelR {
        BitRWmacBfparamSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: disable, 0: enable"]
    #[inline(always)]
    pub fn bit_r_disable_check_vhtsiga_crc(&self) -> BitRDisableCheckVhtsigaCrcR {
        BitRDisableCheckVhtsigaCrcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: disable, 0: enable"]
    #[inline(always)]
    pub fn bit_r_disable_check_vhtsigb_crc(&self) -> BitRDisableCheckVhtsigbCrcR {
        BitRDisableCheckVhtsigbCrcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: use Nsts as Nr, 0: use register value as Nr"]
    #[inline(always)]
    pub fn bit_r_wmac_use_nsts(&self) -> BitRWmacUseNstsR {
        BitRWmacUseNstsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - used as csi report offset"]
    #[inline(always)]
    pub fn bit_r_wmac_vht_category(&self) -> BitRWmacVhtCategoryR {
        BitRWmacVhtCategoryR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - 1:Disable check 16bit checksum value of csi report from BB 0:Enable check 16bit checksum value of csi report from BB"]
    #[inline(always)]
    pub fn bit_r_wmac_csi_chksum_dis(&self) -> BitRWmacCsiChksumDisR {
        BitRWmacCsiChksumDisR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: If previous Tx pkt is BF Rpt Poll or VHT NDP, use vht csi start offset; otherwise ht csi start offset 0: If previous Tx pkt is vht rate, use vht csi start offset; otherwise ht csi start offset"]
    #[inline(always)]
    pub fn bit_r_wmac_offset_rptpoll_en(&self) -> BitRWmacOffsetRptpollEnR {
        BitRWmacOffsetRptpollEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - csi report offset for ht mode"]
    #[inline(always)]
    pub fn bit_r_wmac_ht_category(&self) -> BitRWmacHtCategoryR {
        BitRWmacHtCategoryR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - 0: If A1-matched NDPA's A2 fails to match 0x6E4 or 0x6EC, use default csi parameter 1: For any receiving A1-matched NDPA, use 0x6F4\\[11:0\\] as csi parameter"]
    #[inline(always)]
    pub fn bit_r_wmac_chk_ucndpa_a2_dis(&self) -> BitRWmacChkUcndpaA2DisR {
        BitRWmacChkUcndpaA2DisR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: For any A1-matched BF Rpt Poll, send back VHT-CB 0: If A1-matched BF Rpt Poll's A2 fails to match the previous receiving NDPA's A2, do not send VHT-CB"]
    #[inline(always)]
    pub fn bit_r_wmac_chk_rptpoll_a2_dis(&self) -> BitRWmacChkRptpollA2DisR {
        BitRWmacChkRptpollA2DisR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - A timeout value for MAC to leave NDP_Rx_standby_state and go back to normal Rx mode. (unit: 2us)"]
    #[inline(always)]
    pub fn bit_ndp_rx_standby_timer(&self) -> BitNdpRxStandbyTimerR {
        BitNdpRxStandbyTimerR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set to 1, MAC would inform BB of preparing to receive NDP if the received NDPA is valid."]
    #[inline(always)]
    pub fn bit_r_wmac_vht_ndpa_en(&mut self) -> BitRWmacVhtNdpaEnW<'_, RegSndPtclCtrlSpec> {
        BitRWmacVhtNdpaEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Set to 1, MAC would inform BB of preparing to receive NDP if the received NDP Announcement MPDU is valid."]
    #[inline(always)]
    pub fn bit_r_wmac_ht_ndpa_en(&mut self) -> BitRWmacHtNdpaEnW<'_, RegSndPtclCtrlSpec> {
        BitRWmacHtNdpaEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Set to 1, CSI frame would carry HTC in MAC header."]
    #[inline(always)]
    pub fn bit_r_wmac_csi_withhtc_en(&mut self) -> BitRWmacCsiWithhtcEnW<'_, RegSndPtclCtrlSpec> {
        BitRWmacCsiWithhtcEnW::new(self, 2)
    }
    #[doc = "Bit 3 - By which the sequence number of CSI frame is controlled 1: non-QoS SEQ number 0: counter"]
    #[inline(always)]
    pub fn bit_r_wmac_csiseq_sel(&mut self) -> BitRWmacCsiseqSelW<'_, RegSndPtclCtrlSpec> {
        BitRWmacCsiseqSelW::new(self, 3)
    }
    #[doc = "Bit 4 - 1: use beamforming nc/nr/ng parameter from register 0x6F4 0: use beamforming nc/nr/ng parameter from tx report buffer"]
    #[inline(always)]
    pub fn bit_r_wmac_bfparam_sel(&mut self) -> BitRWmacBfparamSelW<'_, RegSndPtclCtrlSpec> {
        BitRWmacBfparamSelW::new(self, 4)
    }
    #[doc = "Bit 5 - 1: disable, 0: enable"]
    #[inline(always)]
    pub fn bit_r_disable_check_vhtsiga_crc(
        &mut self,
    ) -> BitRDisableCheckVhtsigaCrcW<'_, RegSndPtclCtrlSpec> {
        BitRDisableCheckVhtsigaCrcW::new(self, 5)
    }
    #[doc = "Bit 6 - 1: disable, 0: enable"]
    #[inline(always)]
    pub fn bit_r_disable_check_vhtsigb_crc(
        &mut self,
    ) -> BitRDisableCheckVhtsigbCrcW<'_, RegSndPtclCtrlSpec> {
        BitRDisableCheckVhtsigbCrcW::new(self, 6)
    }
    #[doc = "Bit 7 - 1: use Nsts as Nr, 0: use register value as Nr"]
    #[inline(always)]
    pub fn bit_r_wmac_use_nsts(&mut self) -> BitRWmacUseNstsW<'_, RegSndPtclCtrlSpec> {
        BitRWmacUseNstsW::new(self, 7)
    }
    #[doc = "Bits 8:13 - used as csi report offset"]
    #[inline(always)]
    pub fn bit_r_wmac_vht_category(&mut self) -> BitRWmacVhtCategoryW<'_, RegSndPtclCtrlSpec> {
        BitRWmacVhtCategoryW::new(self, 8)
    }
    #[doc = "Bit 14 - 1:Disable check 16bit checksum value of csi report from BB 0:Enable check 16bit checksum value of csi report from BB"]
    #[inline(always)]
    pub fn bit_r_wmac_csi_chksum_dis(&mut self) -> BitRWmacCsiChksumDisW<'_, RegSndPtclCtrlSpec> {
        BitRWmacCsiChksumDisW::new(self, 14)
    }
    #[doc = "Bit 15 - 1: If previous Tx pkt is BF Rpt Poll or VHT NDP, use vht csi start offset; otherwise ht csi start offset 0: If previous Tx pkt is vht rate, use vht csi start offset; otherwise ht csi start offset"]
    #[inline(always)]
    pub fn bit_r_wmac_offset_rptpoll_en(
        &mut self,
    ) -> BitRWmacOffsetRptpollEnW<'_, RegSndPtclCtrlSpec> {
        BitRWmacOffsetRptpollEnW::new(self, 15)
    }
    #[doc = "Bits 16:21 - csi report offset for ht mode"]
    #[inline(always)]
    pub fn bit_r_wmac_ht_category(&mut self) -> BitRWmacHtCategoryW<'_, RegSndPtclCtrlSpec> {
        BitRWmacHtCategoryW::new(self, 16)
    }
    #[doc = "Bit 22 - 0: If A1-matched NDPA's A2 fails to match 0x6E4 or 0x6EC, use default csi parameter 1: For any receiving A1-matched NDPA, use 0x6F4\\[11:0\\] as csi parameter"]
    #[inline(always)]
    pub fn bit_r_wmac_chk_ucndpa_a2_dis(
        &mut self,
    ) -> BitRWmacChkUcndpaA2DisW<'_, RegSndPtclCtrlSpec> {
        BitRWmacChkUcndpaA2DisW::new(self, 22)
    }
    #[doc = "Bit 23 - 1: For any A1-matched BF Rpt Poll, send back VHT-CB 0: If A1-matched BF Rpt Poll's A2 fails to match the previous receiving NDPA's A2, do not send VHT-CB"]
    #[inline(always)]
    pub fn bit_r_wmac_chk_rptpoll_a2_dis(
        &mut self,
    ) -> BitRWmacChkRptpollA2DisW<'_, RegSndPtclCtrlSpec> {
        BitRWmacChkRptpollA2DisW::new(self, 23)
    }
    #[doc = "Bits 24:31 - A timeout value for MAC to leave NDP_Rx_standby_state and go back to normal Rx mode. (unit: 2us)"]
    #[inline(always)]
    pub fn bit_ndp_rx_standby_timer(&mut self) -> BitNdpRxStandbyTimerW<'_, RegSndPtclCtrlSpec> {
        BitNdpRxStandbyTimerW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_snd_ptcl_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_snd_ptcl_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSndPtclCtrlSpec;
impl crate::RegisterSpec for RegSndPtclCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_snd_ptcl_ctrl::R`](R) reader structure"]
impl crate::Readable for RegSndPtclCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_snd_ptcl_ctrl::W`](W) writer structure"]
impl crate::Writable for RegSndPtclCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SND_PTCL_CTRL to value 0x4200_0280"]
impl crate::Resettable for RegSndPtclCtrlSpec {
    const RESET_VALUE: u32 = 0x4200_0280;
}
