#[doc = "Register `REG_FWISR1` reader"]
pub type R = crate::R<RegFwisr1Spec>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `BIT_FS_HIOE_CHN_OFF_BCN_MODE` reader - hioe_chn_bcn_mode done interrupt"]
pub type BitFsHioeChnOffBcnModeR = crate::BitReader;
#[doc = "Field `BIT_FS_EDCCA_ILDE_TOCNT_INT` reader - when edcca is low after tbtt, start timer,when timeout for few times(0x180~0x188),set this interrupt"]
pub type BitFsEdccaIldeTocntIntR = crate::BitReader;
#[doc = "Field `BIT_FS_RXBCN_TIMEUP_CNT1_INT` reader - the interrupt is issued when RX beacon frame timeout occurs specified times in port0.specified by 0x10A\\[22:16\\](only enabled when 0x17C\\[26\\]=1)"]
pub type BitFsRxbcnTimeupCnt1IntR = crate::BitReader;
#[doc = "Field `BIT_FS_RXBCN_CNT_END_INT` reader - Generate the interrupt when RXBCN period cnt == 2^ BIT_RXBCN_CNT_PRD"]
pub type BitFsRxbcnCntEndIntR = crate::BitReader;
#[doc = "Field `BIT_FS_CCA2HIOE_TSFT_AVG_GT_THRES_INT` reader - If the BIT_TSFT_AVG_VALUE exceeds the BIT_TSFT_AVG_THRES several times in a row, and the number exceeds the BIT_TSFT_AVG_GT_THRES_CNT_MAX, generate an interrupt."]
pub type BitFsCca2hioeTsftAvgGtThresIntR = crate::BitReader;
#[doc = "Field `BIT_FS_OPS_TIM_HIT_INT` reader - when my aid or my bssid is in tim frame/ops frame,generate this interrupt"]
pub type BitFsOpsTimHitIntR = crate::BitReader;
#[doc = "Field `BIT_FS_RX_PHYSTS_DONE_INT` reader - indicate physts is successfully write into physts_mem"]
pub type BitFsRxPhystsDoneIntR = crate::BitReader;
#[doc = "Field `BIT_FS_DFS_PKT_INT` reader - enable interrupt when receiving dfs pkt and about to reach threshold of given pkt number or timeout"]
pub type BitFsDfsPktIntR = crate::BitReader;
#[doc = "Field `BIT_FS_CHINFO_SEG_DONE_INT` reader - indicate one channel info segment is received ok"]
pub type BitFsChinfoSegDoneIntR = crate::BitReader;
#[doc = "Field `BIT_FS_HIOE_CHN_OFF_INT` reader - hioe_chn_off done interrupt"]
pub type BitFsHioeChnOffIntR = crate::BitReader;
#[doc = "Field `BIT_FS_HIOE_CHN_ON_INT` reader - hioe_chn_on done interrupt"]
pub type BitFsHioeChnOnIntR = crate::BitReader;
#[doc = "Field `BIT_FS_HIOE_CHN1_INT` reader - hioe_chn1 done interrupt"]
pub type BitFsHioeChn1IntR = crate::BitReader;
#[doc = "Field `BIT_FS_HIOE_CHN0_INT` reader - hioe_chn0 done interrupt"]
pub type BitFsHioeChn0IntR = crate::BitReader;
#[doc = "Field `BIT_FS_TIM_HIT_INT` reader - when my aid or aid is 0 (0x16DC\\[8\\]) or my bssid is in tim element,generate this interrupt"]
pub type BitFsTimHitIntR = crate::BitReader;
#[doc = "Field `BIT_FS_BC_TWT_PARA_CHG_INT` reader - RX broadcast twt with parameter changing,notify fw to read twt report"]
pub type BitFsBcTwtParaChgIntR = crate::BitReader;
#[doc = "Field `BIT_FS_TIM_NO_HIT_INT` reader - when no my aid or aid is 0(0x16DC\\[8\\]) or my bssid is not in tim element(0x16DB\\[31\\]),generate this interrupt"]
pub type BitFsTimNoHitIntR = crate::BitReader;
#[doc = "Field `BIT_FS_BCN_IE_1_HIT_INT` reader - rx bcn with ie1 hit"]
pub type BitFsBcnIe1HitIntR = crate::BitReader;
#[doc = "Field `BIT_FS_BCN_IE_0_HIT_INT` reader - rx bcn with ie0 hit"]
pub type BitFsBcnIe0HitIntR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNERR_PORT1_INT` reader - PORT1 TX beacon Error interrupt"]
pub type BitFsTxbcnerrPort1IntR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNOK_PORT1_INT` reader - PORT1 TX beacon OK interrupt"]
pub type BitFsTxbcnokPort1IntR = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - hioe_chn_bcn_mode done interrupt"]
    #[inline(always)]
    pub fn bit_fs_hioe_chn_off_bcn_mode(&self) -> BitFsHioeChnOffBcnModeR {
        BitFsHioeChnOffBcnModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - when edcca is low after tbtt, start timer,when timeout for few times(0x180~0x188),set this interrupt"]
    #[inline(always)]
    pub fn bit_fs_edcca_ilde_tocnt_int(&self) -> BitFsEdccaIldeTocntIntR {
        BitFsEdccaIldeTocntIntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - the interrupt is issued when RX beacon frame timeout occurs specified times in port0.specified by 0x10A\\[22:16\\](only enabled when 0x17C\\[26\\]=1)"]
    #[inline(always)]
    pub fn bit_fs_rxbcn_timeup_cnt1_int(&self) -> BitFsRxbcnTimeupCnt1IntR {
        BitFsRxbcnTimeupCnt1IntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Generate the interrupt when RXBCN period cnt == 2^ BIT_RXBCN_CNT_PRD"]
    #[inline(always)]
    pub fn bit_fs_rxbcn_cnt_end_int(&self) -> BitFsRxbcnCntEndIntR {
        BitFsRxbcnCntEndIntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - If the BIT_TSFT_AVG_VALUE exceeds the BIT_TSFT_AVG_THRES several times in a row, and the number exceeds the BIT_TSFT_AVG_GT_THRES_CNT_MAX, generate an interrupt."]
    #[inline(always)]
    pub fn bit_fs_cca2hioe_tsft_avg_gt_thres_int(&self) -> BitFsCca2hioeTsftAvgGtThresIntR {
        BitFsCca2hioeTsftAvgGtThresIntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - when my aid or my bssid is in tim frame/ops frame,generate this interrupt"]
    #[inline(always)]
    pub fn bit_fs_ops_tim_hit_int(&self) -> BitFsOpsTimHitIntR {
        BitFsOpsTimHitIntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - indicate physts is successfully write into physts_mem"]
    #[inline(always)]
    pub fn bit_fs_rx_physts_done_int(&self) -> BitFsRxPhystsDoneIntR {
        BitFsRxPhystsDoneIntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - enable interrupt when receiving dfs pkt and about to reach threshold of given pkt number or timeout"]
    #[inline(always)]
    pub fn bit_fs_dfs_pkt_int(&self) -> BitFsDfsPktIntR {
        BitFsDfsPktIntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - indicate one channel info segment is received ok"]
    #[inline(always)]
    pub fn bit_fs_chinfo_seg_done_int(&self) -> BitFsChinfoSegDoneIntR {
        BitFsChinfoSegDoneIntR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - hioe_chn_off done interrupt"]
    #[inline(always)]
    pub fn bit_fs_hioe_chn_off_int(&self) -> BitFsHioeChnOffIntR {
        BitFsHioeChnOffIntR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - hioe_chn_on done interrupt"]
    #[inline(always)]
    pub fn bit_fs_hioe_chn_on_int(&self) -> BitFsHioeChnOnIntR {
        BitFsHioeChnOnIntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - hioe_chn1 done interrupt"]
    #[inline(always)]
    pub fn bit_fs_hioe_chn1_int(&self) -> BitFsHioeChn1IntR {
        BitFsHioeChn1IntR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - hioe_chn0 done interrupt"]
    #[inline(always)]
    pub fn bit_fs_hioe_chn0_int(&self) -> BitFsHioeChn0IntR {
        BitFsHioeChn0IntR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - when my aid or aid is 0 (0x16DC\\[8\\]) or my bssid is in tim element,generate this interrupt"]
    #[inline(always)]
    pub fn bit_fs_tim_hit_int(&self) -> BitFsTimHitIntR {
        BitFsTimHitIntR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - RX broadcast twt with parameter changing,notify fw to read twt report"]
    #[inline(always)]
    pub fn bit_fs_bc_twt_para_chg_int(&self) -> BitFsBcTwtParaChgIntR {
        BitFsBcTwtParaChgIntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - when no my aid or aid is 0(0x16DC\\[8\\]) or my bssid is not in tim element(0x16DB\\[31\\]),generate this interrupt"]
    #[inline(always)]
    pub fn bit_fs_tim_no_hit_int(&self) -> BitFsTimNoHitIntR {
        BitFsTimNoHitIntR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - rx bcn with ie1 hit"]
    #[inline(always)]
    pub fn bit_fs_bcn_ie_1_hit_int(&self) -> BitFsBcnIe1HitIntR {
        BitFsBcnIe1HitIntR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - rx bcn with ie0 hit"]
    #[inline(always)]
    pub fn bit_fs_bcn_ie_0_hit_int(&self) -> BitFsBcnIe0HitIntR {
        BitFsBcnIe0HitIntR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30 - PORT1 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_port1_int(&self) -> BitFsTxbcnerrPort1IntR {
        BitFsTxbcnerrPort1IntR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PORT1 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_port1_int(&self) -> BitFsTxbcnokPort1IntR {
        BitFsTxbcnokPort1IntR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fwisr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFwisr1Spec;
impl crate::RegisterSpec for RegFwisr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fwisr1::R`](R) reader structure"]
impl crate::Readable for RegFwisr1Spec {}
#[doc = "`reset()` method sets REG_FWISR1 to value 0"]
impl crate::Resettable for RegFwisr1Spec {}
