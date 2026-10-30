#[doc = "Register `REG_FE1ISR` reader"]
pub type R = crate::R<RegFe1isrSpec>;
#[doc = "Field `BIT_FS_RPC_O_T_INT` reader - RXPKT counter over threshold Interrupt"]
pub type BitFsRpcOTIntR = crate::BitReader;
#[doc = "Field `BIT_FS_TRPC_TO_INT` reader - tx/rx pkt time out Interrupt"]
pub type BitFsTrpcToIntR = crate::BitReader;
#[doc = "Field `BIT_FS_REG_Mailbox_TO_I2C_INT` reader - Mailbox_TO_I2C_INT (BT bridge)"]
pub type BitFsRegMailboxToI2cIntR = crate::BitReader;
#[doc = "Field `BIT_FS_BTCMD_INT` reader - the interrupt is issued when btcmd occurs."]
pub type BitFsBtcmdIntR = crate::BitReader;
#[doc = "Field `BIT_FS_WLACTON_INT` reader - WL_ACT_MASK_ON assertion for bt coexistence function interrupt"]
pub type BitFsWlactonIntR = crate::BitReader;
#[doc = "Field `BIT_FS_WLACTOFF_INT` reader - WL_ACT_MASK_ON de-assertion for bt coexistence function interrupt"]
pub type BitFsWlactoffIntR = crate::BitReader;
#[doc = "Field `BIT_FS_WLBB_HIGH_PRI_INT` reader - fatal error happened in bb"]
pub type BitFsWlbbHighPriIntR = crate::BitReader;
#[doc = "Field `BIT_FS_WLBB_LOW_PRI_INT` reader - ERROR happened in bb,not fatal"]
pub type BitFsWlbbLowPriIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PRETXERR_HANDLE_FSISR_INT` reader - PRETXERR_HANDLE FW Flag Interrupt register."]
pub type BitFsPretxerrHandleFsisrIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PTCL_RELEASE_MACID_INT` reader - PTCL release macid interrupt"]
pub type BitFsPtclReleaseMacidIntR = crate::BitReader;
#[doc = "Field `BIT_FS_BF0_PRETO_INT` reader - BF0 pretimeout interrupt"]
pub type BitFsBf0PretoIntR = crate::BitReader;
#[doc = "Field `BIT_FS_BF1_PRETO_INT` reader - BF1 pretimeout interrupt"]
pub type BitFsBf1PretoIntR = crate::BitReader;
#[doc = "Field `BIT_FS_RXUC_BBYE_OK_INT` reader - The interrupt is issued while receiving my unicast qos null data frame with indication of 'bye-bye' in A3 field(A3\\[4:1\\]=3)"]
pub type BitFsRxucBbyeOkIntR = crate::BitReader;
#[doc = "Field `BIT_FS_LP_STBY_INT` reader - Lower Power Standby Interrupt"]
pub type BitFsLpStbyIntR = crate::BitReader;
#[doc = "Field `BIT_FS_SOUND_DONE_INT` reader - Be a beamformer, this interrupt is issued at the time after sounding finish"]
pub type BitFsSoundDoneIntR = crate::BitReader;
#[doc = "Field `BIT_FS_WWLAN_INT` reader - HW detect rxpkt Wol WLAN event interrupt"]
pub type BitFsWwlanIntR = crate::BitReader;
#[doc = "Field `BIT_FS_RXDONE0_INT` reader - Rx Packet 0 done interrupt (for WOL release packet)"]
pub type BitFsRxdone0IntR = crate::BitReader;
#[doc = "Field `BIT_FS_RX_BMD1_INT` reader - The interrupt is issued while receiving broadcast data frame with More Data=1"]
pub type BitFsRxBmd1IntR = crate::BitReader;
#[doc = "Field `BIT_FS_RX_BMD0_INT` reader - The interrupt is issued while receiving broadcast data frame with More Data=0"]
pub type BitFsRxBmd0IntR = crate::BitReader;
#[doc = "Field `BIT_FS_RX_UMD1_INT` reader - The interrupt is issued while receiving unicast data frame with More Data=1"]
pub type BitFsRxUmd1IntR = crate::BitReader;
#[doc = "Field `BIT_FS_RX_UMD0_INT` reader - The interrupt is issued while receiving unicast data frame with More Data=0"]
pub type BitFsRxUmd0IntR = crate::BitReader;
#[doc = "Field `BIT_FS_RX_BCN_P0_INT` reader - When this bit is enable, the interrupt is issued when RX beacon frame occurs in port0."]
pub type BitFsRxBcnP0IntR = crate::BitReader;
#[doc = "Field `BIT_FS_RX_BCN_P1_INT` reader - When this bit is enable, the interrupt is issued when RX beacon frame occurs in port1."]
pub type BitFsRxBcnP1IntR = crate::BitReader;
#[doc = "Field `BIT_FS_RX_BCN_ERR_P0_INT` reader - When this bit is enable, the interrupt is issued when RX beacon frame with fcs error occurs in port0."]
pub type BitFsRxBcnErrP0IntR = crate::BitReader;
#[doc = "Field `BIT_FS_RX_BCN_ERR_P1_INT` reader - When this bit is enable, the interrupt is issued when RX beacon frame with fcs error occurs in port1."]
pub type BitFsRxBcnErrP1IntR = crate::BitReader;
#[doc = "Field `BIT_FS_RXBCN_TIMEUP_INT` reader - When this bit is enable, the interrupt is issued when RX beacon frame timeout occurs in port0."]
pub type BitFsRxbcnTimeupIntR = crate::BitReader;
#[doc = "Field `BIT_FS_RXBCN_TIMEUP_CNT_INT` reader - When this bit is enable, the interrupt is issued when RX beacon frame timeout occurs specified times in port0.specified by 0x17C\\[23:20\\]"]
pub type BitFsRxbcnTimeupCntIntR = crate::BitReader;
#[doc = "Field `BIT_FS_RXPLCP_CHKOK_INT` reader - RXPCLP check ok interrupt"]
pub type BitFsRxplcpChkokIntR = crate::BitReader;
#[doc = "Field `BIT_FS_TSF_GT_THRES_INT` reader - The interrupt is issued when difference of tsft in beacon frame is greater than threshold"]
pub type BitFsTsfGtThresIntR = crate::BitReader;
#[doc = "Field `BIT_FS_BB_STOP_RX_INT` reader - BB STOP RX interrupt"]
pub type BitFsBbStopRxIntR = crate::BitReader;
#[doc = "Field `BIT_FS_NOT_BEACON_INT` reader - not beacon packet interrupt"]
pub type BitFsNotBeaconIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RXPKT counter over threshold Interrupt"]
    #[inline(always)]
    pub fn bit_fs_rpc_o_t_int(&self) -> BitFsRpcOTIntR {
        BitFsRpcOTIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tx/rx pkt time out Interrupt"]
    #[inline(always)]
    pub fn bit_fs_trpc_to_int(&self) -> BitFsTrpcToIntR {
        BitFsTrpcToIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mailbox_TO_I2C_INT (BT bridge)"]
    #[inline(always)]
    pub fn bit_fs_reg_mailbox_to_i2c_int(&self) -> BitFsRegMailboxToI2cIntR {
        BitFsRegMailboxToI2cIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the interrupt is issued when btcmd occurs."]
    #[inline(always)]
    pub fn bit_fs_btcmd_int(&self) -> BitFsBtcmdIntR {
        BitFsBtcmdIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WL_ACT_MASK_ON assertion for bt coexistence function interrupt"]
    #[inline(always)]
    pub fn bit_fs_wlacton_int(&self) -> BitFsWlactonIntR {
        BitFsWlactonIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WL_ACT_MASK_ON de-assertion for bt coexistence function interrupt"]
    #[inline(always)]
    pub fn bit_fs_wlactoff_int(&self) -> BitFsWlactoffIntR {
        BitFsWlactoffIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - fatal error happened in bb"]
    #[inline(always)]
    pub fn bit_fs_wlbb_high_pri_int(&self) -> BitFsWlbbHighPriIntR {
        BitFsWlbbHighPriIntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ERROR happened in bb,not fatal"]
    #[inline(always)]
    pub fn bit_fs_wlbb_low_pri_int(&self) -> BitFsWlbbLowPriIntR {
        BitFsWlbbLowPriIntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PRETXERR_HANDLE FW Flag Interrupt register."]
    #[inline(always)]
    pub fn bit_fs_pretxerr_handle_fsisr_int(&self) -> BitFsPretxerrHandleFsisrIntR {
        BitFsPretxerrHandleFsisrIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PTCL release macid interrupt"]
    #[inline(always)]
    pub fn bit_fs_ptcl_release_macid_int(&self) -> BitFsPtclReleaseMacidIntR {
        BitFsPtclReleaseMacidIntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BF0 pretimeout interrupt"]
    #[inline(always)]
    pub fn bit_fs_bf0_preto_int(&self) -> BitFsBf0PretoIntR {
        BitFsBf0PretoIntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BF1 pretimeout interrupt"]
    #[inline(always)]
    pub fn bit_fs_bf1_preto_int(&self) -> BitFsBf1PretoIntR {
        BitFsBf1PretoIntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt is issued while receiving my unicast qos null data frame with indication of 'bye-bye' in A3 field(A3\\[4:1\\]=3)"]
    #[inline(always)]
    pub fn bit_fs_rxuc_bbye_ok_int(&self) -> BitFsRxucBbyeOkIntR {
        BitFsRxucBbyeOkIntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Lower Power Standby Interrupt"]
    #[inline(always)]
    pub fn bit_fs_lp_stby_int(&self) -> BitFsLpStbyIntR {
        BitFsLpStbyIntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Be a beamformer, this interrupt is issued at the time after sounding finish"]
    #[inline(always)]
    pub fn bit_fs_sound_done_int(&self) -> BitFsSoundDoneIntR {
        BitFsSoundDoneIntR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - HW detect rxpkt Wol WLAN event interrupt"]
    #[inline(always)]
    pub fn bit_fs_wwlan_int(&self) -> BitFsWwlanIntR {
        BitFsWwlanIntR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rx Packet 0 done interrupt (for WOL release packet)"]
    #[inline(always)]
    pub fn bit_fs_rxdone0_int(&self) -> BitFsRxdone0IntR {
        BitFsRxdone0IntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt is issued while receiving broadcast data frame with More Data=1"]
    #[inline(always)]
    pub fn bit_fs_rx_bmd1_int(&self) -> BitFsRxBmd1IntR {
        BitFsRxBmd1IntR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The interrupt is issued while receiving broadcast data frame with More Data=0"]
    #[inline(always)]
    pub fn bit_fs_rx_bmd0_int(&self) -> BitFsRxBmd0IntR {
        BitFsRxBmd0IntR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt is issued while receiving unicast data frame with More Data=1"]
    #[inline(always)]
    pub fn bit_fs_rx_umd1_int(&self) -> BitFsRxUmd1IntR {
        BitFsRxUmd1IntR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The interrupt is issued while receiving unicast data frame with More Data=0"]
    #[inline(always)]
    pub fn bit_fs_rx_umd0_int(&self) -> BitFsRxUmd0IntR {
        BitFsRxUmd0IntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - When this bit is enable, the interrupt is issued when RX beacon frame occurs in port0."]
    #[inline(always)]
    pub fn bit_fs_rx_bcn_p0_int(&self) -> BitFsRxBcnP0IntR {
        BitFsRxBcnP0IntR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - When this bit is enable, the interrupt is issued when RX beacon frame occurs in port1."]
    #[inline(always)]
    pub fn bit_fs_rx_bcn_p1_int(&self) -> BitFsRxBcnP1IntR {
        BitFsRxBcnP1IntR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - When this bit is enable, the interrupt is issued when RX beacon frame with fcs error occurs in port0."]
    #[inline(always)]
    pub fn bit_fs_rx_bcn_err_p0_int(&self) -> BitFsRxBcnErrP0IntR {
        BitFsRxBcnErrP0IntR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - When this bit is enable, the interrupt is issued when RX beacon frame with fcs error occurs in port1."]
    #[inline(always)]
    pub fn bit_fs_rx_bcn_err_p1_int(&self) -> BitFsRxBcnErrP1IntR {
        BitFsRxBcnErrP1IntR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - When this bit is enable, the interrupt is issued when RX beacon frame timeout occurs in port0."]
    #[inline(always)]
    pub fn bit_fs_rxbcn_timeup_int(&self) -> BitFsRxbcnTimeupIntR {
        BitFsRxbcnTimeupIntR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When this bit is enable, the interrupt is issued when RX beacon frame timeout occurs specified times in port0.specified by 0x17C\\[23:20\\]"]
    #[inline(always)]
    pub fn bit_fs_rxbcn_timeup_cnt_int(&self) -> BitFsRxbcnTimeupCntIntR {
        BitFsRxbcnTimeupCntIntR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - RXPCLP check ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_rxplcp_chkok_int(&self) -> BitFsRxplcpChkokIntR {
        BitFsRxplcpChkokIntR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The interrupt is issued when difference of tsft in beacon frame is greater than threshold"]
    #[inline(always)]
    pub fn bit_fs_tsf_gt_thres_int(&self) -> BitFsTsfGtThresIntR {
        BitFsTsfGtThresIntR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - BB STOP RX interrupt"]
    #[inline(always)]
    pub fn bit_fs_bb_stop_rx_int(&self) -> BitFsBbStopRxIntR {
        BitFsBbStopRxIntR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - not beacon packet interrupt"]
    #[inline(always)]
    pub fn bit_fs_not_beacon_int(&self) -> BitFsNotBeaconIntR {
        BitFsNotBeaconIntR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Firmware Extend Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fe1isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFe1isrSpec;
impl crate::RegisterSpec for RegFe1isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fe1isr::R`](R) reader structure"]
impl crate::Readable for RegFe1isrSpec {}
#[doc = "`reset()` method sets REG_FE1ISR to value 0"]
impl crate::Resettable for RegFe1isrSpec {}
