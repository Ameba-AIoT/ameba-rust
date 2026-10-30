#[doc = "Register `REG_FWIMR1` reader"]
pub type R = crate::R<RegFwimr1Spec>;
#[doc = "Register `REG_FWIMR1` writer"]
pub type W = crate::W<RegFwimr1Spec>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BIT_FS_HIOE_CHN_OFF_BCN_MODE_EN` reader - hioe_chn_bcn_mode done interrupt"]
pub type BitFsHioeChnOffBcnModeEnR = crate::BitReader;
#[doc = "Field `BIT_FS_HIOE_CHN_OFF_BCN_MODE_EN` writer - hioe_chn_bcn_mode done interrupt"]
pub type BitFsHioeChnOffBcnModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_EDCCA_ILDE_TOCNT_INT_EN` reader - when edcca is low after tbtt, start timer,when timeout for few times(0x180~0x188),set this interrupt"]
pub type BitFsEdccaIldeTocntIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_EDCCA_ILDE_TOCNT_INT_EN` writer - when edcca is low after tbtt, start timer,when timeout for few times(0x180~0x188),set this interrupt"]
pub type BitFsEdccaIldeTocntIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_RXBCN_TIMEUP_CNT1_INT_EN` reader - the interrupt is issued when RX beacon frame timeout occurs specified times in port0.specified by 0x10A\\[22:16\\](only enabled when 0x17C\\[26\\]=1)"]
pub type BitFsRxbcnTimeupCnt1IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RXBCN_TIMEUP_CNT1_INT_EN` writer - the interrupt is issued when RX beacon frame timeout occurs specified times in port0.specified by 0x10A\\[22:16\\](only enabled when 0x17C\\[26\\]=1)"]
pub type BitFsRxbcnTimeupCnt1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_RXBCN_CNT_END_INT_EN` reader - Generate the interrupt when RXBCN period cnt == 2^ BIT_RXBCN_CNT_PRD"]
pub type BitFsRxbcnCntEndIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RXBCN_CNT_END_INT_EN` writer - Generate the interrupt when RXBCN period cnt == 2^ BIT_RXBCN_CNT_PRD"]
pub type BitFsRxbcnCntEndIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_CCA2HIOE_TSFT_AVG_GT_THRES_INT_EN` reader - If the BIT_TSFT_AVG_VALUE exceeds the BIT_TSFT_AVG_THRES several times in a row, and the number exceeds the BIT_TSFT_AVG_GT_THRES_CNT_MAX, generate an interrupt."]
pub type BitFsCca2hioeTsftAvgGtThresIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_CCA2HIOE_TSFT_AVG_GT_THRES_INT_EN` writer - If the BIT_TSFT_AVG_VALUE exceeds the BIT_TSFT_AVG_THRES several times in a row, and the number exceeds the BIT_TSFT_AVG_GT_THRES_CNT_MAX, generate an interrupt."]
pub type BitFsCca2hioeTsftAvgGtThresIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_OPS_TIM_HIT_INT_EN` reader - when my aid or my bssid is in tim frame/ops frame,generate this interrupt"]
pub type BitFsOpsTimHitIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_OPS_TIM_HIT_INT_EN` writer - when my aid or my bssid is in tim frame/ops frame,generate this interrupt"]
pub type BitFsOpsTimHitIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_RX_PHYSTS_DONE_INT__EN` reader - indicate physts is successfully write into physts_mem"]
pub type BitFsRxPhystsDoneInt_EnR = crate::BitReader;
#[doc = "Field `BIT_FS_RX_PHYSTS_DONE_INT__EN` writer - indicate physts is successfully write into physts_mem"]
pub type BitFsRxPhystsDoneInt_EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_DFS_PKT_INT_EN` reader - enable interrupt when receiving dfs pkt and about to reach threshold of given pkt number or timeout"]
pub type BitFsDfsPktIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_DFS_PKT_INT_EN` writer - enable interrupt when receiving dfs pkt and about to reach threshold of given pkt number or timeout"]
pub type BitFsDfsPktIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_CHINFO_SEG_DONE_INT_NE` reader - indicate one channel info segment is received ok"]
pub type BitFsChinfoSegDoneIntNeR = crate::BitReader;
#[doc = "Field `BIT_FS_CHINFO_SEG_DONE_INT_NE` writer - indicate one channel info segment is received ok"]
pub type BitFsChinfoSegDoneIntNeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_HIOE_CHN_OFF_INT_EN` reader - hioe_chn_off done interrupt"]
pub type BitFsHioeChnOffIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_HIOE_CHN_OFF_INT_EN` writer - hioe_chn_off done interrupt"]
pub type BitFsHioeChnOffIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_HIOE_CHN_ON_INT_EN` reader - hioe_chn_on done interrupt"]
pub type BitFsHioeChnOnIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_HIOE_CHN_ON_INT_EN` writer - hioe_chn_on done interrupt"]
pub type BitFsHioeChnOnIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_HIOE_CHN1_INT_EN` reader - hioe_chn1 done interrupt"]
pub type BitFsHioeChn1IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_HIOE_CHN1_INT_EN` writer - hioe_chn1 done interrupt"]
pub type BitFsHioeChn1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_HIOE_CHN0_INT_EN` reader - hioe_chn0 done interrupt"]
pub type BitFsHioeChn0IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_HIOE_CHN0_INT_EN` writer - hioe_chn0 done interrupt"]
pub type BitFsHioeChn0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TIM_HIT_INT_EN` reader - when my aid or aid is 0 (0x16DC\\[8\\]) or my bssid is in tim element,generate this interrupt"]
pub type BitFsTimHitIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TIM_HIT_INT_EN` writer - when my aid or aid is 0 (0x16DC\\[8\\]) or my bssid is in tim element,generate this interrupt"]
pub type BitFsTimHitIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BC_TWT_PARA_CHG_INT_EN` reader - RX broadcast twt with parameter changing,notify fw to read twt report"]
pub type BitFsBcTwtParaChgIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BC_TWT_PARA_CHG_INT_EN` writer - RX broadcast twt with parameter changing,notify fw to read twt report"]
pub type BitFsBcTwtParaChgIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TIM_NO_HIT_INT_EN` reader - when no my aid or aid is 0(0x16DC\\[8\\]) or my bssid is not in tim element(0x16DB\\[31\\]),generate this interrupt"]
pub type BitFsTimNoHitIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TIM_NO_HIT_INT_EN` writer - when no my aid or aid is 0(0x16DC\\[8\\]) or my bssid is not in tim element(0x16DB\\[31\\]),generate this interrupt"]
pub type BitFsTimNoHitIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCN_IE_1_HIT_INT_EN` reader - rx bcn with ie1 hit"]
pub type BitFsBcnIe1HitIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCN_IE_1_HIT_INT_EN` writer - rx bcn with ie1 hit"]
pub type BitFsBcnIe1HitIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCN_IE_0_HIT_INT_EN` reader - rx bcn with ie0 hit"]
pub type BitFsBcnIe0HitIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCN_IE_0_HIT_INT_EN` writer - rx bcn with ie0 hit"]
pub type BitFsBcnIe0HitIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXBCNERR_PORT1_INT_EN` reader - ENABLE PORT1 TX beacon Error interrupt"]
pub type BitFsTxbcnerrPort1IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNERR_PORT1_INT_EN` writer - ENABLE PORT1 TX beacon Error interrupt"]
pub type BitFsTxbcnerrPort1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXBCNOK_PORT1_INT_EN` reader - ENABLE PORT1 TX beacon OK interrupt"]
pub type BitFsTxbcnokPort1IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNOK_PORT1_INT_EN` writer - ENABLE PORT1 TX beacon OK interrupt"]
pub type BitFsTxbcnokPort1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - hioe_chn_bcn_mode done interrupt"]
    #[inline(always)]
    pub fn bit_fs_hioe_chn_off_bcn_mode_en(&self) -> BitFsHioeChnOffBcnModeEnR {
        BitFsHioeChnOffBcnModeEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - when edcca is low after tbtt, start timer,when timeout for few times(0x180~0x188),set this interrupt"]
    #[inline(always)]
    pub fn bit_fs_edcca_ilde_tocnt_int_en(&self) -> BitFsEdccaIldeTocntIntEnR {
        BitFsEdccaIldeTocntIntEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - the interrupt is issued when RX beacon frame timeout occurs specified times in port0.specified by 0x10A\\[22:16\\](only enabled when 0x17C\\[26\\]=1)"]
    #[inline(always)]
    pub fn bit_fs_rxbcn_timeup_cnt1_int_en(&self) -> BitFsRxbcnTimeupCnt1IntEnR {
        BitFsRxbcnTimeupCnt1IntEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Generate the interrupt when RXBCN period cnt == 2^ BIT_RXBCN_CNT_PRD"]
    #[inline(always)]
    pub fn bit_fs_rxbcn_cnt_end_int_en(&self) -> BitFsRxbcnCntEndIntEnR {
        BitFsRxbcnCntEndIntEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - If the BIT_TSFT_AVG_VALUE exceeds the BIT_TSFT_AVG_THRES several times in a row, and the number exceeds the BIT_TSFT_AVG_GT_THRES_CNT_MAX, generate an interrupt."]
    #[inline(always)]
    pub fn bit_fs_cca2hioe_tsft_avg_gt_thres_int_en(&self) -> BitFsCca2hioeTsftAvgGtThresIntEnR {
        BitFsCca2hioeTsftAvgGtThresIntEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - when my aid or my bssid is in tim frame/ops frame,generate this interrupt"]
    #[inline(always)]
    pub fn bit_fs_ops_tim_hit_int_en(&self) -> BitFsOpsTimHitIntEnR {
        BitFsOpsTimHitIntEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - indicate physts is successfully write into physts_mem"]
    #[inline(always)]
    pub fn bit_fs_rx_physts_done_int__en(&self) -> BitFsRxPhystsDoneInt_EnR {
        BitFsRxPhystsDoneInt_EnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - enable interrupt when receiving dfs pkt and about to reach threshold of given pkt number or timeout"]
    #[inline(always)]
    pub fn bit_fs_dfs_pkt_int_en(&self) -> BitFsDfsPktIntEnR {
        BitFsDfsPktIntEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - indicate one channel info segment is received ok"]
    #[inline(always)]
    pub fn bit_fs_chinfo_seg_done_int_ne(&self) -> BitFsChinfoSegDoneIntNeR {
        BitFsChinfoSegDoneIntNeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - hioe_chn_off done interrupt"]
    #[inline(always)]
    pub fn bit_fs_hioe_chn_off_int_en(&self) -> BitFsHioeChnOffIntEnR {
        BitFsHioeChnOffIntEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - hioe_chn_on done interrupt"]
    #[inline(always)]
    pub fn bit_fs_hioe_chn_on_int_en(&self) -> BitFsHioeChnOnIntEnR {
        BitFsHioeChnOnIntEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - hioe_chn1 done interrupt"]
    #[inline(always)]
    pub fn bit_fs_hioe_chn1_int_en(&self) -> BitFsHioeChn1IntEnR {
        BitFsHioeChn1IntEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - hioe_chn0 done interrupt"]
    #[inline(always)]
    pub fn bit_fs_hioe_chn0_int_en(&self) -> BitFsHioeChn0IntEnR {
        BitFsHioeChn0IntEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - when my aid or aid is 0 (0x16DC\\[8\\]) or my bssid is in tim element,generate this interrupt"]
    #[inline(always)]
    pub fn bit_fs_tim_hit_int_en(&self) -> BitFsTimHitIntEnR {
        BitFsTimHitIntEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - RX broadcast twt with parameter changing,notify fw to read twt report"]
    #[inline(always)]
    pub fn bit_fs_bc_twt_para_chg_int_en(&self) -> BitFsBcTwtParaChgIntEnR {
        BitFsBcTwtParaChgIntEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - when no my aid or aid is 0(0x16DC\\[8\\]) or my bssid is not in tim element(0x16DB\\[31\\]),generate this interrupt"]
    #[inline(always)]
    pub fn bit_fs_tim_no_hit_int_en(&self) -> BitFsTimNoHitIntEnR {
        BitFsTimNoHitIntEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - rx bcn with ie1 hit"]
    #[inline(always)]
    pub fn bit_fs_bcn_ie_1_hit_int_en(&self) -> BitFsBcnIe1HitIntEnR {
        BitFsBcnIe1HitIntEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - rx bcn with ie0 hit"]
    #[inline(always)]
    pub fn bit_fs_bcn_ie_0_hit_int_en(&self) -> BitFsBcnIe0HitIntEnR {
        BitFsBcnIe0HitIntEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30 - ENABLE PORT1 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_port1_int_en(&self) -> BitFsTxbcnerrPort1IntEnR {
        BitFsTxbcnerrPort1IntEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ENABLE PORT1 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_port1_int_en(&self) -> BitFsTxbcnokPort1IntEnR {
        BitFsTxbcnokPort1IntEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegFwimr1Spec> {
        RsvdW::new(self, 0)
    }
    #[doc = "Bit 6 - hioe_chn_bcn_mode done interrupt"]
    #[inline(always)]
    pub fn bit_fs_hioe_chn_off_bcn_mode_en(
        &mut self,
    ) -> BitFsHioeChnOffBcnModeEnW<'_, RegFwimr1Spec> {
        BitFsHioeChnOffBcnModeEnW::new(self, 6)
    }
    #[doc = "Bit 7 - when edcca is low after tbtt, start timer,when timeout for few times(0x180~0x188),set this interrupt"]
    #[inline(always)]
    pub fn bit_fs_edcca_ilde_tocnt_int_en(
        &mut self,
    ) -> BitFsEdccaIldeTocntIntEnW<'_, RegFwimr1Spec> {
        BitFsEdccaIldeTocntIntEnW::new(self, 7)
    }
    #[doc = "Bit 8 - the interrupt is issued when RX beacon frame timeout occurs specified times in port0.specified by 0x10A\\[22:16\\](only enabled when 0x17C\\[26\\]=1)"]
    #[inline(always)]
    pub fn bit_fs_rxbcn_timeup_cnt1_int_en(
        &mut self,
    ) -> BitFsRxbcnTimeupCnt1IntEnW<'_, RegFwimr1Spec> {
        BitFsRxbcnTimeupCnt1IntEnW::new(self, 8)
    }
    #[doc = "Bit 9 - Generate the interrupt when RXBCN period cnt == 2^ BIT_RXBCN_CNT_PRD"]
    #[inline(always)]
    pub fn bit_fs_rxbcn_cnt_end_int_en(&mut self) -> BitFsRxbcnCntEndIntEnW<'_, RegFwimr1Spec> {
        BitFsRxbcnCntEndIntEnW::new(self, 9)
    }
    #[doc = "Bit 10 - If the BIT_TSFT_AVG_VALUE exceeds the BIT_TSFT_AVG_THRES several times in a row, and the number exceeds the BIT_TSFT_AVG_GT_THRES_CNT_MAX, generate an interrupt."]
    #[inline(always)]
    pub fn bit_fs_cca2hioe_tsft_avg_gt_thres_int_en(
        &mut self,
    ) -> BitFsCca2hioeTsftAvgGtThresIntEnW<'_, RegFwimr1Spec> {
        BitFsCca2hioeTsftAvgGtThresIntEnW::new(self, 10)
    }
    #[doc = "Bit 11 - when my aid or my bssid is in tim frame/ops frame,generate this interrupt"]
    #[inline(always)]
    pub fn bit_fs_ops_tim_hit_int_en(&mut self) -> BitFsOpsTimHitIntEnW<'_, RegFwimr1Spec> {
        BitFsOpsTimHitIntEnW::new(self, 11)
    }
    #[doc = "Bit 12 - indicate physts is successfully write into physts_mem"]
    #[inline(always)]
    pub fn bit_fs_rx_physts_done_int__en(&mut self) -> BitFsRxPhystsDoneInt_EnW<'_, RegFwimr1Spec> {
        BitFsRxPhystsDoneInt_EnW::new(self, 12)
    }
    #[doc = "Bit 13 - enable interrupt when receiving dfs pkt and about to reach threshold of given pkt number or timeout"]
    #[inline(always)]
    pub fn bit_fs_dfs_pkt_int_en(&mut self) -> BitFsDfsPktIntEnW<'_, RegFwimr1Spec> {
        BitFsDfsPktIntEnW::new(self, 13)
    }
    #[doc = "Bit 14 - indicate one channel info segment is received ok"]
    #[inline(always)]
    pub fn bit_fs_chinfo_seg_done_int_ne(&mut self) -> BitFsChinfoSegDoneIntNeW<'_, RegFwimr1Spec> {
        BitFsChinfoSegDoneIntNeW::new(self, 14)
    }
    #[doc = "Bit 15 - hioe_chn_off done interrupt"]
    #[inline(always)]
    pub fn bit_fs_hioe_chn_off_int_en(&mut self) -> BitFsHioeChnOffIntEnW<'_, RegFwimr1Spec> {
        BitFsHioeChnOffIntEnW::new(self, 15)
    }
    #[doc = "Bit 16 - hioe_chn_on done interrupt"]
    #[inline(always)]
    pub fn bit_fs_hioe_chn_on_int_en(&mut self) -> BitFsHioeChnOnIntEnW<'_, RegFwimr1Spec> {
        BitFsHioeChnOnIntEnW::new(self, 16)
    }
    #[doc = "Bit 17 - hioe_chn1 done interrupt"]
    #[inline(always)]
    pub fn bit_fs_hioe_chn1_int_en(&mut self) -> BitFsHioeChn1IntEnW<'_, RegFwimr1Spec> {
        BitFsHioeChn1IntEnW::new(self, 17)
    }
    #[doc = "Bit 18 - hioe_chn0 done interrupt"]
    #[inline(always)]
    pub fn bit_fs_hioe_chn0_int_en(&mut self) -> BitFsHioeChn0IntEnW<'_, RegFwimr1Spec> {
        BitFsHioeChn0IntEnW::new(self, 18)
    }
    #[doc = "Bit 19 - when my aid or aid is 0 (0x16DC\\[8\\]) or my bssid is in tim element,generate this interrupt"]
    #[inline(always)]
    pub fn bit_fs_tim_hit_int_en(&mut self) -> BitFsTimHitIntEnW<'_, RegFwimr1Spec> {
        BitFsTimHitIntEnW::new(self, 19)
    }
    #[doc = "Bit 20 - RX broadcast twt with parameter changing,notify fw to read twt report"]
    #[inline(always)]
    pub fn bit_fs_bc_twt_para_chg_int_en(&mut self) -> BitFsBcTwtParaChgIntEnW<'_, RegFwimr1Spec> {
        BitFsBcTwtParaChgIntEnW::new(self, 20)
    }
    #[doc = "Bit 21 - when no my aid or aid is 0(0x16DC\\[8\\]) or my bssid is not in tim element(0x16DB\\[31\\]),generate this interrupt"]
    #[inline(always)]
    pub fn bit_fs_tim_no_hit_int_en(&mut self) -> BitFsTimNoHitIntEnW<'_, RegFwimr1Spec> {
        BitFsTimNoHitIntEnW::new(self, 21)
    }
    #[doc = "Bit 23 - rx bcn with ie1 hit"]
    #[inline(always)]
    pub fn bit_fs_bcn_ie_1_hit_int_en(&mut self) -> BitFsBcnIe1HitIntEnW<'_, RegFwimr1Spec> {
        BitFsBcnIe1HitIntEnW::new(self, 23)
    }
    #[doc = "Bit 24 - rx bcn with ie0 hit"]
    #[inline(always)]
    pub fn bit_fs_bcn_ie_0_hit_int_en(&mut self) -> BitFsBcnIe0HitIntEnW<'_, RegFwimr1Spec> {
        BitFsBcnIe0HitIntEnW::new(self, 24)
    }
    #[doc = "Bit 30 - ENABLE PORT1 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_port1_int_en(&mut self) -> BitFsTxbcnerrPort1IntEnW<'_, RegFwimr1Spec> {
        BitFsTxbcnerrPort1IntEnW::new(self, 30)
    }
    #[doc = "Bit 31 - ENABLE PORT1 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_port1_int_en(&mut self) -> BitFsTxbcnokPort1IntEnW<'_, RegFwimr1Spec> {
        BitFsTxbcnokPort1IntEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fwimr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fwimr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFwimr1Spec;
impl crate::RegisterSpec for RegFwimr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fwimr1::R`](R) reader structure"]
impl crate::Readable for RegFwimr1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_fwimr1::W`](W) writer structure"]
impl crate::Writable for RegFwimr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FWIMR1 to value 0"]
impl crate::Resettable for RegFwimr1Spec {}
