#[doc = "Register `REG_FE1IMR` reader"]
pub type R = crate::R<RegFe1imrSpec>;
#[doc = "Register `REG_FE1IMR` writer"]
pub type W = crate::W<RegFe1imrSpec>;
#[doc = "Field `BIT_FS_RPC_O_T_INT_EN` reader - RPC over threshold Interrupt"]
pub type BitFsRpcOTIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RPC_O_T_INT_EN` writer - RPC over threshold Interrupt"]
pub type BitFsRpcOTIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TRPC_TO_INT_EN` reader - TRPC time out Interrupt"]
pub type BitFsTrpcToIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TRPC_TO_INT_EN` writer - TRPC time out Interrupt"]
pub type BitFsTrpcToIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_REG_Mailbox_TO_I2C_INT_EN` reader - Mailbox_TO_I2C_INT (BT bridge)"]
pub type BitFsRegMailboxToI2cIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_REG_Mailbox_TO_I2C_INT_EN` writer - Mailbox_TO_I2C_INT (BT bridge)"]
pub type BitFsRegMailboxToI2cIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BTCMD_INT_EN` reader - the interrupt is issued when btcmd occurs."]
pub type BitFsBtcmdIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BTCMD_INT_EN` writer - the interrupt is issued when btcmd occurs."]
pub type BitFsBtcmdIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_WLACTON_INT_EN` reader - WL_ACT_MASK_ON assertion for bt coexistence function interrupt"]
pub type BitFsWlactonIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_WLACTON_INT_EN` writer - WL_ACT_MASK_ON assertion for bt coexistence function interrupt"]
pub type BitFsWlactonIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_WLACTOFF_INT_EN` reader - WL_ACT_MASK_ON de-assertion for bt coexistence function interrupt"]
pub type BitFsWlactoffIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_WLACTOFF_INT_EN` writer - WL_ACT_MASK_ON de-assertion for bt coexistence function interrupt"]
pub type BitFsWlactoffIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_WLBB_HIGH_PRI_INT_EN` reader - fatal error happened in bb"]
pub type BitFsWlbbHighPriIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_WLBB_HIGH_PRI_INT_EN` writer - fatal error happened in bb"]
pub type BitFsWlbbHighPriIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_WLBB_LOW_PRI_INT_EN` reader - ERROR happened in bb,not fatal"]
pub type BitFsWlbbLowPriIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_WLBB_LOW_PRI_INT_EN` writer - ERROR happened in bb,not fatal"]
pub type BitFsWlbbLowPriIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PRETXERR_HANDLE_FSISR_INT_EN` reader - PRETXERR_HANDLE FW Flag Interrupt register."]
pub type BitFsPretxerrHandleFsisrIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PRETXERR_HANDLE_FSISR_INT_EN` writer - PRETXERR_HANDLE FW Flag Interrupt register."]
pub type BitFsPretxerrHandleFsisrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PTCL_RELEASE_MACID_INT_EN` reader - PTCL release macid interrupt"]
pub type BitFsPtclReleaseMacidIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PTCL_RELEASE_MACID_INT_EN` writer - PTCL release macid interrupt"]
pub type BitFsPtclReleaseMacidIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BF0_PRETO_INT_EN` reader - BF0 pretimeout interrupt"]
pub type BitFsBf0PretoIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BF0_PRETO_INT_EN` writer - BF0 pretimeout interrupt"]
pub type BitFsBf0PretoIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BF1_PRETO_INT_EN` reader - BF1 pretimeout interrupt"]
pub type BitFsBf1PretoIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BF1_PRETO_INT_EN` writer - BF1 pretimeout interrupt"]
pub type BitFsBf1PretoIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_RXUC_BBYE_OK_EN` reader - The interrupt is issued while receiving my unicast qos null data frame with indication of 'bye-bye' in A3 field(A3\\[4:1\\]=3)"]
pub type BitFsRxucBbyeOkEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RXUC_BBYE_OK_EN` writer - The interrupt is issued while receiving my unicast qos null data frame with indication of 'bye-bye' in A3 field(A3\\[4:1\\]=3)"]
pub type BitFsRxucBbyeOkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_LP_STBY_INT_EN` reader - Lower Power Standby Interrupt"]
pub type BitFsLpStbyIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_LP_STBY_INT_EN` writer - Lower Power Standby Interrupt"]
pub type BitFsLpStbyIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_SOUND_DONE_INT_EN` reader - Be a beamformer, this interrupt is issued at the time after sounding finish"]
pub type BitFsSoundDoneIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_SOUND_DONE_INT_EN` writer - Be a beamformer, this interrupt is issued at the time after sounding finish"]
pub type BitFsSoundDoneIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_WWLAN_INT_EN` reader - HW detect rxpkt Wol WLAN event interrupt"]
pub type BitFsWwlanIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_WWLAN_INT_EN` writer - HW detect rxpkt Wol WLAN event interrupt"]
pub type BitFsWwlanIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_RXDONE0_INT_EN` reader - Rx Packet 0 done interrupt (for WOL release packet)"]
pub type BitFsRxdone0IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RXDONE0_INT_EN` writer - Rx Packet 0 done interrupt (for WOL release packet)"]
pub type BitFsRxdone0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_RX_BMD1_INT_EN` reader - The interrupt is issued while receiving broadcast data frame with More Data=1"]
pub type BitFsRxBmd1IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RX_BMD1_INT_EN` writer - The interrupt is issued while receiving broadcast data frame with More Data=1"]
pub type BitFsRxBmd1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_RX_BMD0_INT_EN` reader - The interrupt is issued while receiving broadcast data frame with More Data=0"]
pub type BitFsRxBmd0IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RX_BMD0_INT_EN` writer - The interrupt is issued while receiving broadcast data frame with More Data=0"]
pub type BitFsRxBmd0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_RX_UMD1_INT_EN` reader - The interrupt is issued while receiving unicast data frame with More Data=1"]
pub type BitFsRxUmd1IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RX_UMD1_INT_EN` writer - The interrupt is issued while receiving unicast data frame with More Data=1"]
pub type BitFsRxUmd1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_RX_UMD0_INT_EN` reader - The interrupt is issued while receiving unicast data frame with More Data=0"]
pub type BitFsRxUmd0IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RX_UMD0_INT_EN` writer - The interrupt is issued while receiving unicast data frame with More Data=0"]
pub type BitFsRxUmd0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_RX_BCN_P0_INT_EN` reader - When this bit is enable, the interrupt is issued when RX beacon frame occurs in port0."]
pub type BitFsRxBcnP0IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RX_BCN_P0_INT_EN` writer - When this bit is enable, the interrupt is issued when RX beacon frame occurs in port0."]
pub type BitFsRxBcnP0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_RX_BCN_P1_INT_EN` reader - When this bit is enable, the interrupt is issued when RX beacon frame occurs in port1."]
pub type BitFsRxBcnP1IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RX_BCN_P1_INT_EN` writer - When this bit is enable, the interrupt is issued when RX beacon frame occurs in port1."]
pub type BitFsRxBcnP1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_RX_BCN_ERR_P0_INT_EN` reader - When this bit is enable, the interrupt is issued when RX beacon frame with fcs error occurs in port0."]
pub type BitFsRxBcnErrP0IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RX_BCN_ERR_P0_INT_EN` writer - When this bit is enable, the interrupt is issued when RX beacon frame with fcs error occurs in port0."]
pub type BitFsRxBcnErrP0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_RX_BCN_ERR_P1_INT_EN` reader - When this bit is enable, the interrupt is issued when RX beacon frame with fcs error occurs in port1."]
pub type BitFsRxBcnErrP1IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RX_BCN_ERR_P1_INT_EN` writer - When this bit is enable, the interrupt is issued when RX beacon frame with fcs error occurs in port1."]
pub type BitFsRxBcnErrP1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_RXBCN_TIMEUP_INT_EN` reader - When this bit is enable, the interrupt is issued when RX beacon frame timeout occurs in port0."]
pub type BitFsRxbcnTimeupIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RXBCN_TIMEUP_INT_EN` writer - When this bit is enable, the interrupt is issued when RX beacon frame timeout occurs in port0."]
pub type BitFsRxbcnTimeupIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_RXBCN_TIMEUP_CNT_INT_EN` reader - When this bit is enable, the interrupt is issued when RX beacon frame timeout occurs specified times in port0.specified by 0x17C\\[23:20\\]"]
pub type BitFsRxbcnTimeupCntIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RXBCN_TIMEUP_CNT_INT_EN` writer - When this bit is enable, the interrupt is issued when RX beacon frame timeout occurs specified times in port0.specified by 0x17C\\[23:20\\]"]
pub type BitFsRxbcnTimeupCntIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_RXPLCP_CHKOK_INT_EN` reader - RXPCLP check ok interrupt"]
pub type BitFsRxplcpChkokIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RXPLCP_CHKOK_INT_EN` writer - RXPCLP check ok interrupt"]
pub type BitFsRxplcpChkokIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TSF_GT_THRES_INT_EN` reader - The interrupt is issued when difference of tsft in beacon frame of port0 is greater than threshold"]
pub type BitFsTsfGtThresIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TSF_GT_THRES_INT_EN` writer - The interrupt is issued when difference of tsft in beacon frame of port0 is greater than threshold"]
pub type BitFsTsfGtThresIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BB_STOP_RX_INT_EN` reader - BB STOP RX interrupt"]
pub type BitFsBbStopRxIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BB_STOP_RX_INT_EN` writer - BB STOP RX interrupt"]
pub type BitFsBbStopRxIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_NOT_BEACON_INT_EN` reader - not beacon packet interrupt"]
pub type BitFsNotBeaconIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_NOT_BEACON_INT_EN` writer - not beacon packet interrupt"]
pub type BitFsNotBeaconIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RPC over threshold Interrupt"]
    #[inline(always)]
    pub fn bit_fs_rpc_o_t_int_en(&self) -> BitFsRpcOTIntEnR {
        BitFsRpcOTIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TRPC time out Interrupt"]
    #[inline(always)]
    pub fn bit_fs_trpc_to_int_en(&self) -> BitFsTrpcToIntEnR {
        BitFsTrpcToIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mailbox_TO_I2C_INT (BT bridge)"]
    #[inline(always)]
    pub fn bit_fs_reg_mailbox_to_i2c_int_en(&self) -> BitFsRegMailboxToI2cIntEnR {
        BitFsRegMailboxToI2cIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the interrupt is issued when btcmd occurs."]
    #[inline(always)]
    pub fn bit_fs_btcmd_int_en(&self) -> BitFsBtcmdIntEnR {
        BitFsBtcmdIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WL_ACT_MASK_ON assertion for bt coexistence function interrupt"]
    #[inline(always)]
    pub fn bit_fs_wlacton_int_en(&self) -> BitFsWlactonIntEnR {
        BitFsWlactonIntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WL_ACT_MASK_ON de-assertion for bt coexistence function interrupt"]
    #[inline(always)]
    pub fn bit_fs_wlactoff_int_en(&self) -> BitFsWlactoffIntEnR {
        BitFsWlactoffIntEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - fatal error happened in bb"]
    #[inline(always)]
    pub fn bit_fs_wlbb_high_pri_int_en(&self) -> BitFsWlbbHighPriIntEnR {
        BitFsWlbbHighPriIntEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ERROR happened in bb,not fatal"]
    #[inline(always)]
    pub fn bit_fs_wlbb_low_pri_int_en(&self) -> BitFsWlbbLowPriIntEnR {
        BitFsWlbbLowPriIntEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PRETXERR_HANDLE FW Flag Interrupt register."]
    #[inline(always)]
    pub fn bit_fs_pretxerr_handle_fsisr_int_en(&self) -> BitFsPretxerrHandleFsisrIntEnR {
        BitFsPretxerrHandleFsisrIntEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PTCL release macid interrupt"]
    #[inline(always)]
    pub fn bit_fs_ptcl_release_macid_int_en(&self) -> BitFsPtclReleaseMacidIntEnR {
        BitFsPtclReleaseMacidIntEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BF0 pretimeout interrupt"]
    #[inline(always)]
    pub fn bit_fs_bf0_preto_int_en(&self) -> BitFsBf0PretoIntEnR {
        BitFsBf0PretoIntEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BF1 pretimeout interrupt"]
    #[inline(always)]
    pub fn bit_fs_bf1_preto_int_en(&self) -> BitFsBf1PretoIntEnR {
        BitFsBf1PretoIntEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt is issued while receiving my unicast qos null data frame with indication of 'bye-bye' in A3 field(A3\\[4:1\\]=3)"]
    #[inline(always)]
    pub fn bit_fs_rxuc_bbye_ok_en(&self) -> BitFsRxucBbyeOkEnR {
        BitFsRxucBbyeOkEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Lower Power Standby Interrupt"]
    #[inline(always)]
    pub fn bit_fs_lp_stby_int_en(&self) -> BitFsLpStbyIntEnR {
        BitFsLpStbyIntEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Be a beamformer, this interrupt is issued at the time after sounding finish"]
    #[inline(always)]
    pub fn bit_fs_sound_done_int_en(&self) -> BitFsSoundDoneIntEnR {
        BitFsSoundDoneIntEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - HW detect rxpkt Wol WLAN event interrupt"]
    #[inline(always)]
    pub fn bit_fs_wwlan_int_en(&self) -> BitFsWwlanIntEnR {
        BitFsWwlanIntEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rx Packet 0 done interrupt (for WOL release packet)"]
    #[inline(always)]
    pub fn bit_fs_rxdone0_int_en(&self) -> BitFsRxdone0IntEnR {
        BitFsRxdone0IntEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt is issued while receiving broadcast data frame with More Data=1"]
    #[inline(always)]
    pub fn bit_fs_rx_bmd1_int_en(&self) -> BitFsRxBmd1IntEnR {
        BitFsRxBmd1IntEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The interrupt is issued while receiving broadcast data frame with More Data=0"]
    #[inline(always)]
    pub fn bit_fs_rx_bmd0_int_en(&self) -> BitFsRxBmd0IntEnR {
        BitFsRxBmd0IntEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt is issued while receiving unicast data frame with More Data=1"]
    #[inline(always)]
    pub fn bit_fs_rx_umd1_int_en(&self) -> BitFsRxUmd1IntEnR {
        BitFsRxUmd1IntEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The interrupt is issued while receiving unicast data frame with More Data=0"]
    #[inline(always)]
    pub fn bit_fs_rx_umd0_int_en(&self) -> BitFsRxUmd0IntEnR {
        BitFsRxUmd0IntEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - When this bit is enable, the interrupt is issued when RX beacon frame occurs in port0."]
    #[inline(always)]
    pub fn bit_fs_rx_bcn_p0_int_en(&self) -> BitFsRxBcnP0IntEnR {
        BitFsRxBcnP0IntEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - When this bit is enable, the interrupt is issued when RX beacon frame occurs in port1."]
    #[inline(always)]
    pub fn bit_fs_rx_bcn_p1_int_en(&self) -> BitFsRxBcnP1IntEnR {
        BitFsRxBcnP1IntEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - When this bit is enable, the interrupt is issued when RX beacon frame with fcs error occurs in port0."]
    #[inline(always)]
    pub fn bit_fs_rx_bcn_err_p0_int_en(&self) -> BitFsRxBcnErrP0IntEnR {
        BitFsRxBcnErrP0IntEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - When this bit is enable, the interrupt is issued when RX beacon frame with fcs error occurs in port1."]
    #[inline(always)]
    pub fn bit_fs_rx_bcn_err_p1_int_en(&self) -> BitFsRxBcnErrP1IntEnR {
        BitFsRxBcnErrP1IntEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - When this bit is enable, the interrupt is issued when RX beacon frame timeout occurs in port0."]
    #[inline(always)]
    pub fn bit_fs_rxbcn_timeup_int_en(&self) -> BitFsRxbcnTimeupIntEnR {
        BitFsRxbcnTimeupIntEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When this bit is enable, the interrupt is issued when RX beacon frame timeout occurs specified times in port0.specified by 0x17C\\[23:20\\]"]
    #[inline(always)]
    pub fn bit_fs_rxbcn_timeup_cnt_int_en(&self) -> BitFsRxbcnTimeupCntIntEnR {
        BitFsRxbcnTimeupCntIntEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - RXPCLP check ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_rxplcp_chkok_int_en(&self) -> BitFsRxplcpChkokIntEnR {
        BitFsRxplcpChkokIntEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The interrupt is issued when difference of tsft in beacon frame of port0 is greater than threshold"]
    #[inline(always)]
    pub fn bit_fs_tsf_gt_thres_int_en(&self) -> BitFsTsfGtThresIntEnR {
        BitFsTsfGtThresIntEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - BB STOP RX interrupt"]
    #[inline(always)]
    pub fn bit_fs_bb_stop_rx_int_en(&self) -> BitFsBbStopRxIntEnR {
        BitFsBbStopRxIntEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - not beacon packet interrupt"]
    #[inline(always)]
    pub fn bit_fs_not_beacon_int_en(&self) -> BitFsNotBeaconIntEnR {
        BitFsNotBeaconIntEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RPC over threshold Interrupt"]
    #[inline(always)]
    pub fn bit_fs_rpc_o_t_int_en(&mut self) -> BitFsRpcOTIntEnW<'_, RegFe1imrSpec> {
        BitFsRpcOTIntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - TRPC time out Interrupt"]
    #[inline(always)]
    pub fn bit_fs_trpc_to_int_en(&mut self) -> BitFsTrpcToIntEnW<'_, RegFe1imrSpec> {
        BitFsTrpcToIntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Mailbox_TO_I2C_INT (BT bridge)"]
    #[inline(always)]
    pub fn bit_fs_reg_mailbox_to_i2c_int_en(
        &mut self,
    ) -> BitFsRegMailboxToI2cIntEnW<'_, RegFe1imrSpec> {
        BitFsRegMailboxToI2cIntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - the interrupt is issued when btcmd occurs."]
    #[inline(always)]
    pub fn bit_fs_btcmd_int_en(&mut self) -> BitFsBtcmdIntEnW<'_, RegFe1imrSpec> {
        BitFsBtcmdIntEnW::new(self, 3)
    }
    #[doc = "Bit 4 - WL_ACT_MASK_ON assertion for bt coexistence function interrupt"]
    #[inline(always)]
    pub fn bit_fs_wlacton_int_en(&mut self) -> BitFsWlactonIntEnW<'_, RegFe1imrSpec> {
        BitFsWlactonIntEnW::new(self, 4)
    }
    #[doc = "Bit 5 - WL_ACT_MASK_ON de-assertion for bt coexistence function interrupt"]
    #[inline(always)]
    pub fn bit_fs_wlactoff_int_en(&mut self) -> BitFsWlactoffIntEnW<'_, RegFe1imrSpec> {
        BitFsWlactoffIntEnW::new(self, 5)
    }
    #[doc = "Bit 6 - fatal error happened in bb"]
    #[inline(always)]
    pub fn bit_fs_wlbb_high_pri_int_en(&mut self) -> BitFsWlbbHighPriIntEnW<'_, RegFe1imrSpec> {
        BitFsWlbbHighPriIntEnW::new(self, 6)
    }
    #[doc = "Bit 7 - ERROR happened in bb,not fatal"]
    #[inline(always)]
    pub fn bit_fs_wlbb_low_pri_int_en(&mut self) -> BitFsWlbbLowPriIntEnW<'_, RegFe1imrSpec> {
        BitFsWlbbLowPriIntEnW::new(self, 7)
    }
    #[doc = "Bit 8 - PRETXERR_HANDLE FW Flag Interrupt register."]
    #[inline(always)]
    pub fn bit_fs_pretxerr_handle_fsisr_int_en(
        &mut self,
    ) -> BitFsPretxerrHandleFsisrIntEnW<'_, RegFe1imrSpec> {
        BitFsPretxerrHandleFsisrIntEnW::new(self, 8)
    }
    #[doc = "Bit 9 - PTCL release macid interrupt"]
    #[inline(always)]
    pub fn bit_fs_ptcl_release_macid_int_en(
        &mut self,
    ) -> BitFsPtclReleaseMacidIntEnW<'_, RegFe1imrSpec> {
        BitFsPtclReleaseMacidIntEnW::new(self, 9)
    }
    #[doc = "Bit 10 - BF0 pretimeout interrupt"]
    #[inline(always)]
    pub fn bit_fs_bf0_preto_int_en(&mut self) -> BitFsBf0PretoIntEnW<'_, RegFe1imrSpec> {
        BitFsBf0PretoIntEnW::new(self, 10)
    }
    #[doc = "Bit 11 - BF1 pretimeout interrupt"]
    #[inline(always)]
    pub fn bit_fs_bf1_preto_int_en(&mut self) -> BitFsBf1PretoIntEnW<'_, RegFe1imrSpec> {
        BitFsBf1PretoIntEnW::new(self, 11)
    }
    #[doc = "Bit 12 - The interrupt is issued while receiving my unicast qos null data frame with indication of 'bye-bye' in A3 field(A3\\[4:1\\]=3)"]
    #[inline(always)]
    pub fn bit_fs_rxuc_bbye_ok_en(&mut self) -> BitFsRxucBbyeOkEnW<'_, RegFe1imrSpec> {
        BitFsRxucBbyeOkEnW::new(self, 12)
    }
    #[doc = "Bit 13 - Lower Power Standby Interrupt"]
    #[inline(always)]
    pub fn bit_fs_lp_stby_int_en(&mut self) -> BitFsLpStbyIntEnW<'_, RegFe1imrSpec> {
        BitFsLpStbyIntEnW::new(self, 13)
    }
    #[doc = "Bit 14 - Be a beamformer, this interrupt is issued at the time after sounding finish"]
    #[inline(always)]
    pub fn bit_fs_sound_done_int_en(&mut self) -> BitFsSoundDoneIntEnW<'_, RegFe1imrSpec> {
        BitFsSoundDoneIntEnW::new(self, 14)
    }
    #[doc = "Bit 15 - HW detect rxpkt Wol WLAN event interrupt"]
    #[inline(always)]
    pub fn bit_fs_wwlan_int_en(&mut self) -> BitFsWwlanIntEnW<'_, RegFe1imrSpec> {
        BitFsWwlanIntEnW::new(self, 15)
    }
    #[doc = "Bit 16 - Rx Packet 0 done interrupt (for WOL release packet)"]
    #[inline(always)]
    pub fn bit_fs_rxdone0_int_en(&mut self) -> BitFsRxdone0IntEnW<'_, RegFe1imrSpec> {
        BitFsRxdone0IntEnW::new(self, 16)
    }
    #[doc = "Bit 17 - The interrupt is issued while receiving broadcast data frame with More Data=1"]
    #[inline(always)]
    pub fn bit_fs_rx_bmd1_int_en(&mut self) -> BitFsRxBmd1IntEnW<'_, RegFe1imrSpec> {
        BitFsRxBmd1IntEnW::new(self, 17)
    }
    #[doc = "Bit 18 - The interrupt is issued while receiving broadcast data frame with More Data=0"]
    #[inline(always)]
    pub fn bit_fs_rx_bmd0_int_en(&mut self) -> BitFsRxBmd0IntEnW<'_, RegFe1imrSpec> {
        BitFsRxBmd0IntEnW::new(self, 18)
    }
    #[doc = "Bit 19 - The interrupt is issued while receiving unicast data frame with More Data=1"]
    #[inline(always)]
    pub fn bit_fs_rx_umd1_int_en(&mut self) -> BitFsRxUmd1IntEnW<'_, RegFe1imrSpec> {
        BitFsRxUmd1IntEnW::new(self, 19)
    }
    #[doc = "Bit 20 - The interrupt is issued while receiving unicast data frame with More Data=0"]
    #[inline(always)]
    pub fn bit_fs_rx_umd0_int_en(&mut self) -> BitFsRxUmd0IntEnW<'_, RegFe1imrSpec> {
        BitFsRxUmd0IntEnW::new(self, 20)
    }
    #[doc = "Bit 21 - When this bit is enable, the interrupt is issued when RX beacon frame occurs in port0."]
    #[inline(always)]
    pub fn bit_fs_rx_bcn_p0_int_en(&mut self) -> BitFsRxBcnP0IntEnW<'_, RegFe1imrSpec> {
        BitFsRxBcnP0IntEnW::new(self, 21)
    }
    #[doc = "Bit 22 - When this bit is enable, the interrupt is issued when RX beacon frame occurs in port1."]
    #[inline(always)]
    pub fn bit_fs_rx_bcn_p1_int_en(&mut self) -> BitFsRxBcnP1IntEnW<'_, RegFe1imrSpec> {
        BitFsRxBcnP1IntEnW::new(self, 22)
    }
    #[doc = "Bit 23 - When this bit is enable, the interrupt is issued when RX beacon frame with fcs error occurs in port0."]
    #[inline(always)]
    pub fn bit_fs_rx_bcn_err_p0_int_en(&mut self) -> BitFsRxBcnErrP0IntEnW<'_, RegFe1imrSpec> {
        BitFsRxBcnErrP0IntEnW::new(self, 23)
    }
    #[doc = "Bit 24 - When this bit is enable, the interrupt is issued when RX beacon frame with fcs error occurs in port1."]
    #[inline(always)]
    pub fn bit_fs_rx_bcn_err_p1_int_en(&mut self) -> BitFsRxBcnErrP1IntEnW<'_, RegFe1imrSpec> {
        BitFsRxBcnErrP1IntEnW::new(self, 24)
    }
    #[doc = "Bit 25 - When this bit is enable, the interrupt is issued when RX beacon frame timeout occurs in port0."]
    #[inline(always)]
    pub fn bit_fs_rxbcn_timeup_int_en(&mut self) -> BitFsRxbcnTimeupIntEnW<'_, RegFe1imrSpec> {
        BitFsRxbcnTimeupIntEnW::new(self, 25)
    }
    #[doc = "Bit 26 - When this bit is enable, the interrupt is issued when RX beacon frame timeout occurs specified times in port0.specified by 0x17C\\[23:20\\]"]
    #[inline(always)]
    pub fn bit_fs_rxbcn_timeup_cnt_int_en(
        &mut self,
    ) -> BitFsRxbcnTimeupCntIntEnW<'_, RegFe1imrSpec> {
        BitFsRxbcnTimeupCntIntEnW::new(self, 26)
    }
    #[doc = "Bit 27 - RXPCLP check ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_rxplcp_chkok_int_en(&mut self) -> BitFsRxplcpChkokIntEnW<'_, RegFe1imrSpec> {
        BitFsRxplcpChkokIntEnW::new(self, 27)
    }
    #[doc = "Bit 28 - The interrupt is issued when difference of tsft in beacon frame of port0 is greater than threshold"]
    #[inline(always)]
    pub fn bit_fs_tsf_gt_thres_int_en(&mut self) -> BitFsTsfGtThresIntEnW<'_, RegFe1imrSpec> {
        BitFsTsfGtThresIntEnW::new(self, 28)
    }
    #[doc = "Bit 29 - BB STOP RX interrupt"]
    #[inline(always)]
    pub fn bit_fs_bb_stop_rx_int_en(&mut self) -> BitFsBbStopRxIntEnW<'_, RegFe1imrSpec> {
        BitFsBbStopRxIntEnW::new(self, 29)
    }
    #[doc = "Bit 31 - not beacon packet interrupt"]
    #[inline(always)]
    pub fn bit_fs_not_beacon_int_en(&mut self) -> BitFsNotBeaconIntEnW<'_, RegFe1imrSpec> {
        BitFsNotBeaconIntEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fe1imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fe1imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFe1imrSpec;
impl crate::RegisterSpec for RegFe1imrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fe1imr::R`](R) reader structure"]
impl crate::Readable for RegFe1imrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_fe1imr::W`](W) writer structure"]
impl crate::Writable for RegFe1imrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FE1IMR to value 0"]
impl crate::Resettable for RegFe1imrSpec {}
