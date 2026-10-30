#[doc = "Register `REG_WMAC_CR` reader"]
pub type R = crate::R<RegWmacCrSpec>;
#[doc = "Register `REG_WMAC_CR` writer"]
pub type W = crate::W<RegWmacCrSpec>;
#[doc = "Field `BIT_IC_MACPHY_M` reader - To make ASIC become BB+RF only, and put out the MAC_PHY interface"]
pub type BitIcMacphyMR = crate::BitReader;
#[doc = "Field `BIT_IC_MACPHY_M` writer - To make ASIC become BB+RF only, and put out the MAC_PHY interface"]
pub type BitIcMacphyMW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_dis_abort_tb_ely` reader - 1:will abort ptcl generating tb data when macrx_active is low 0:will abort ptcl generating tb data when new rxtrig req_in is assert"]
pub type RDisAbortTbElyR = crate::BitReader;
#[doc = "Field `r_dis_abort_tb_ely` writer - 1:will abort ptcl generating tb data when macrx_active is low 0:will abort ptcl generating tb data when new rxtrig req_in is assert"]
pub type RDisAbortTbElyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_dis_chk_new_tbreq` reader - 1:rxtrig req_in will assert every time when aid fit if two or more trigger aggregate in a ppdu 0:rxtrig req_in will not assert again when there is already a correct trigger received"]
pub type RDisChkNewTbreqR = crate::BitReader;
#[doc = "Field `r_dis_chk_new_tbreq` writer - 1:rxtrig req_in will assert every time when aid fit if two or more trigger aggregate in a ppdu 0:rxtrig req_in will not assert again when there is already a correct trigger received"]
pub type RDisChkNewTbreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_rxtrig_dis_chk_a1` reader - 1:will not check if A1 of trigger frame is my unicast or broadcast address 0:will check if A1 of trigger frame is my unicast or broadcast address"]
pub type RRxtrigDisChkA1R = crate::BitReader;
#[doc = "Field `r_rxtrig_dis_chk_a1` writer - 1:will not check if A1 of trigger frame is my unicast or broadcast address 0:will check if A1 of trigger frame is my unicast or broadcast address"]
pub type RRxtrigDisChkA1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_rxtrig_dis_upd_aid` reader - 1:will only update aid_fit info when the first trigger in a ppdu is parsered 0:will update aid_fit info every time when two or more trigger aggregate in a ppdu"]
pub type RRxtrigDisUpdAidR = crate::BitReader;
#[doc = "Field `r_rxtrig_dis_upd_aid` writer - 1:will only update aid_fit info when the first trigger in a ppdu is parsered 0:will update aid_fit info every time when two or more trigger aggregate in a ppdu"]
pub type RRxtrigDisUpdAidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_APSDOFF` reader - 1:MAC issue sleep signal to disable BB/AFE/RF TRX function. After MAC has finished, APSDOFF_STATUS will report\"1\". 0:MAC issue wakeup signal to recall BB/AFE/RF back to RX idle mode. After MAC has finished,APSDOFF _STATUS will report \"0\"."]
pub type BitApsdoffR = crate::BitReader;
#[doc = "Field `BIT_APSDOFF` writer - 1:MAC issue sleep signal to disable BB/AFE/RF TRX function. After MAC has finished, APSDOFF_STATUS will report\"1\". 0:MAC issue wakeup signal to recall BB/AFE/RF back to RX idle mode. After MAC has finished,APSDOFF _STATUS will report \"0\"."]
pub type BitApsdoffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_APPEND_MHDR_LEN` reader - Appended MAC header length, unit in 32 bytes"]
pub type BitAppendMhdrLenR = crate::FieldReader;
#[doc = "Field `BIT_APPEND_MHDR_LEN` writer - Appended MAC header length, unit in 32 bytes"]
pub type BitAppendMhdrLenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_FWPARSING_EN` reader - If set to 1, mac header and rx desc of the pkt into rx buff are also received into FWFF."]
pub type BitFwparsingEnR = crate::BitReader;
#[doc = "Field `BIT_FWPARSING_EN` writer - If set to 1, mac header and rx desc of the pkt into rx buff are also received into FWFF."]
pub type BitFwparsingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_APPHDR_MIDSRCH_FAIL` reader - If set to 1, Hw will append 16 bytes MAC header into FWFF when MACID search fail"]
pub type BitApphdrMidsrchFailR = crate::BitReader;
#[doc = "Field `BIT_APPHDR_MIDSRCH_FAIL` writer - If set to 1, Hw will append 16 bytes MAC header into FWFF when MACID search fail"]
pub type BitApphdrMidsrchFailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PHYSTS_PKT_CTRL` reader - If set to 1,HW will send interrupt (0x124\\[26\\]) only when there is phy status valid fwsts pkt written into FWFF"]
pub type BitPhystsPktCtrlR = crate::BitReader;
#[doc = "Field `BIT_PHYSTS_PKT_CTRL` writer - If set to 1,HW will send interrupt (0x124\\[26\\]) only when there is phy status valid fwsts pkt written into FWFF"]
pub type BitPhystsPktCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FWRX_EN` reader - Enable to receive pkt into FWFF, default 1'b0. When Fw needs to receive pkt, this bit must be set to 1'b1"]
pub type BitFwrxEnR = crate::BitReader;
#[doc = "Field `BIT_FWRX_EN` writer - Enable to receive pkt into FWFF, default 1'b0. When Fw needs to receive pkt, this bit must be set to 1'b1"]
pub type BitFwrxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MGNT_FW_STS_FILTER` reader - When this bit is set,Accept management frame in partial length condition. It will take effect when RxFilter3-5 is disabled and RxFilter 0-2 is enabled."]
pub type BitMgntFwStsFilterR = crate::BitReader;
#[doc = "Field `BIT_MGNT_FW_STS_FILTER` writer - When this bit is set,Accept management frame in partial length condition. It will take effect when RxFilter3-5 is disabled and RxFilter 0-2 is enabled."]
pub type BitMgntFwStsFilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CTRL_FW_STS_FILTER` reader - When this bit is set,Accept control frame in partial length condition. It will take effect when RxFilter3-5 is disabled and RxFilter 0-2 is enabled."]
pub type BitCtrlFwStsFilterR = crate::BitReader;
#[doc = "Field `BIT_CTRL_FW_STS_FILTER` writer - When this bit is set,Accept control frame in partial length condition. It will take effect when RxFilter3-5 is disabled and RxFilter 0-2 is enabled."]
pub type BitCtrlFwStsFilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DATA_FW_STS_FILTER` reader - When this bit is set,Accept data frame in partial length condition. It will take effect when RxFilter3-5 is disabled and RxFilter 0-2 is enabled."]
pub type BitDataFwStsFilterR = crate::BitReader;
#[doc = "Field `BIT_DATA_FW_STS_FILTER` writer - When this bit is set,Accept data frame in partial length condition. It will take effect when RxFilter3-5 is disabled and RxFilter 0-2 is enabled."]
pub type BitDataFwStsFilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wmac_rxuc_bbye_macid` reader - hw latch macid when rx my unicast bye-bye frame in relay mode 7E:rxpktbuf full 7F:not valid macid"]
pub type WmacRxucBbyeMacidR = crate::FieldReader;
#[doc = "Field `BIT_WMAC_M11J` reader - "]
pub type BitWmacM11jR = crate::BitReader;
#[doc = "Field `BIT_WMAC_M11J` writer - "]
pub type BitWmacM11jW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_20MHZBW` reader - "]
pub type BitWmac20mhzbwR = crate::BitReader;
#[doc = "Field `BIT_WMAC_20MHZBW` writer - "]
pub type BitWmac20mhzbwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_WAIT_HE_SIGB` reader - When set, will not wait and check he mu sigb and ndp info"]
pub type BitDisWaitHeSigbR = crate::BitReader;
#[doc = "Field `BIT_DIS_WAIT_HE_SIGB` writer - When set, will not wait and check he mu sigb and ndp info"]
pub type BitDisWaitHeSigbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WLAN_RX_REQ_SIFS_CLR` reader - When set, enable sifs clear wlan_rx_req"]
pub type BitWlanRxReqSifsClrR = crate::BitReader;
#[doc = "Field `BIT_WLAN_RX_REQ_SIFS_CLR` writer - When set, enable sifs clear wlan_rx_req"]
pub type BitWlanRxReqSifsClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TXPWR_CTL_ENGCLK` reader - If set to 1, enable txpwr_ctrl clock gating function."]
pub type BitTxpwrCtlEngclkR = crate::BitReader;
#[doc = "Field `BIT_TXPWR_CTL_ENGCLK` writer - If set to 1, enable txpwr_ctrl clock gating function."]
pub type BitTxpwrCtlEngclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TXPWR_CTL_CLK_EN` reader - IF set to 1, txpwr_ctrl clock enable, only work when BIT_TXPWR_CTL_ENGCLK set to 1,"]
pub type BitTxpwrCtlClkEnR = crate::BitReader;
#[doc = "Field `BIT_TXPWR_CTL_CLK_EN` writer - IF set to 1, txpwr_ctrl clock enable, only work when BIT_TXPWR_CTL_ENGCLK set to 1,"]
pub type BitTxpwrCtlClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - To make ASIC become BB+RF only, and put out the MAC_PHY interface"]
    #[inline(always)]
    pub fn bit_ic_macphy_m(&self) -> BitIcMacphyMR {
        BitIcMacphyMR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:will abort ptcl generating tb data when macrx_active is low 0:will abort ptcl generating tb data when new rxtrig req_in is assert"]
    #[inline(always)]
    pub fn r_dis_abort_tb_ely(&self) -> RDisAbortTbElyR {
        RDisAbortTbElyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1:rxtrig req_in will assert every time when aid fit if two or more trigger aggregate in a ppdu 0:rxtrig req_in will not assert again when there is already a correct trigger received"]
    #[inline(always)]
    pub fn r_dis_chk_new_tbreq(&self) -> RDisChkNewTbreqR {
        RDisChkNewTbreqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1:will not check if A1 of trigger frame is my unicast or broadcast address 0:will check if A1 of trigger frame is my unicast or broadcast address"]
    #[inline(always)]
    pub fn r_rxtrig_dis_chk_a1(&self) -> RRxtrigDisChkA1R {
        RRxtrigDisChkA1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1:will only update aid_fit info when the first trigger in a ppdu is parsered 0:will update aid_fit info every time when two or more trigger aggregate in a ppdu"]
    #[inline(always)]
    pub fn r_rxtrig_dis_upd_aid(&self) -> RRxtrigDisUpdAidR {
        RRxtrigDisUpdAidR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - 1:MAC issue sleep signal to disable BB/AFE/RF TRX function. After MAC has finished, APSDOFF_STATUS will report\"1\". 0:MAC issue wakeup signal to recall BB/AFE/RF back to RX idle mode. After MAC has finished,APSDOFF _STATUS will report \"0\"."]
    #[inline(always)]
    pub fn bit_apsdoff(&self) -> BitApsdoffR {
        BitApsdoffR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Appended MAC header length, unit in 32 bytes"]
    #[inline(always)]
    pub fn bit_append_mhdr_len(&self) -> BitAppendMhdrLenR {
        BitAppendMhdrLenR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - If set to 1, mac header and rx desc of the pkt into rx buff are also received into FWFF."]
    #[inline(always)]
    pub fn bit_fwparsing_en(&self) -> BitFwparsingEnR {
        BitFwparsingEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - If set to 1, Hw will append 16 bytes MAC header into FWFF when MACID search fail"]
    #[inline(always)]
    pub fn bit_apphdr_midsrch_fail(&self) -> BitApphdrMidsrchFailR {
        BitApphdrMidsrchFailR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - If set to 1,HW will send interrupt (0x124\\[26\\]) only when there is phy status valid fwsts pkt written into FWFF"]
    #[inline(always)]
    pub fn bit_physts_pkt_ctrl(&self) -> BitPhystsPktCtrlR {
        BitPhystsPktCtrlR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable to receive pkt into FWFF, default 1'b0. When Fw needs to receive pkt, this bit must be set to 1'b1"]
    #[inline(always)]
    pub fn bit_fwrx_en(&self) -> BitFwrxEnR {
        BitFwrxEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - When this bit is set,Accept management frame in partial length condition. It will take effect when RxFilter3-5 is disabled and RxFilter 0-2 is enabled."]
    #[inline(always)]
    pub fn bit_mgnt_fw_sts_filter(&self) -> BitMgntFwStsFilterR {
        BitMgntFwStsFilterR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When this bit is set,Accept control frame in partial length condition. It will take effect when RxFilter3-5 is disabled and RxFilter 0-2 is enabled."]
    #[inline(always)]
    pub fn bit_ctrl_fw_sts_filter(&self) -> BitCtrlFwStsFilterR {
        BitCtrlFwStsFilterR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - When this bit is set,Accept data frame in partial length condition. It will take effect when RxFilter3-5 is disabled and RxFilter 0-2 is enabled."]
    #[inline(always)]
    pub fn bit_data_fw_sts_filter(&self) -> BitDataFwStsFilterR {
        BitDataFwStsFilterR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:25 - hw latch macid when rx my unicast bye-bye frame in relay mode 7E:rxpktbuf full 7F:not valid macid"]
    #[inline(always)]
    pub fn wmac_rxuc_bbye_macid(&self) -> WmacRxucBbyeMacidR {
        WmacRxucBbyeMacidR::new(((self.bits >> 19) & 0x7f) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn bit_wmac_m11j(&self) -> BitWmacM11jR {
        BitWmacM11jR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn bit_wmac_20mhzbw(&self) -> BitWmac20mhzbwR {
        BitWmac20mhzbwR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - When set, will not wait and check he mu sigb and ndp info"]
    #[inline(always)]
    pub fn bit_dis_wait_he_sigb(&self) -> BitDisWaitHeSigbR {
        BitDisWaitHeSigbR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - When set, enable sifs clear wlan_rx_req"]
    #[inline(always)]
    pub fn bit_wlan_rx_req_sifs_clr(&self) -> BitWlanRxReqSifsClrR {
        BitWlanRxReqSifsClrR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - If set to 1, enable txpwr_ctrl clock gating function."]
    #[inline(always)]
    pub fn bit_txpwr_ctl_engclk(&self) -> BitTxpwrCtlEngclkR {
        BitTxpwrCtlEngclkR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - IF set to 1, txpwr_ctrl clock enable, only work when BIT_TXPWR_CTL_ENGCLK set to 1,"]
    #[inline(always)]
    pub fn bit_txpwr_ctl_clk_en(&self) -> BitTxpwrCtlClkEnR {
        BitTxpwrCtlClkEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - To make ASIC become BB+RF only, and put out the MAC_PHY interface"]
    #[inline(always)]
    pub fn bit_ic_macphy_m(&mut self) -> BitIcMacphyMW<'_, RegWmacCrSpec> {
        BitIcMacphyMW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:will abort ptcl generating tb data when macrx_active is low 0:will abort ptcl generating tb data when new rxtrig req_in is assert"]
    #[inline(always)]
    pub fn r_dis_abort_tb_ely(&mut self) -> RDisAbortTbElyW<'_, RegWmacCrSpec> {
        RDisAbortTbElyW::new(self, 1)
    }
    #[doc = "Bit 2 - 1:rxtrig req_in will assert every time when aid fit if two or more trigger aggregate in a ppdu 0:rxtrig req_in will not assert again when there is already a correct trigger received"]
    #[inline(always)]
    pub fn r_dis_chk_new_tbreq(&mut self) -> RDisChkNewTbreqW<'_, RegWmacCrSpec> {
        RDisChkNewTbreqW::new(self, 2)
    }
    #[doc = "Bit 3 - 1:will not check if A1 of trigger frame is my unicast or broadcast address 0:will check if A1 of trigger frame is my unicast or broadcast address"]
    #[inline(always)]
    pub fn r_rxtrig_dis_chk_a1(&mut self) -> RRxtrigDisChkA1W<'_, RegWmacCrSpec> {
        RRxtrigDisChkA1W::new(self, 3)
    }
    #[doc = "Bit 4 - 1:will only update aid_fit info when the first trigger in a ppdu is parsered 0:will update aid_fit info every time when two or more trigger aggregate in a ppdu"]
    #[inline(always)]
    pub fn r_rxtrig_dis_upd_aid(&mut self) -> RRxtrigDisUpdAidW<'_, RegWmacCrSpec> {
        RRxtrigDisUpdAidW::new(self, 4)
    }
    #[doc = "Bit 6 - 1:MAC issue sleep signal to disable BB/AFE/RF TRX function. After MAC has finished, APSDOFF_STATUS will report\"1\". 0:MAC issue wakeup signal to recall BB/AFE/RF back to RX idle mode. After MAC has finished,APSDOFF _STATUS will report \"0\"."]
    #[inline(always)]
    pub fn bit_apsdoff(&mut self) -> BitApsdoffW<'_, RegWmacCrSpec> {
        BitApsdoffW::new(self, 6)
    }
    #[doc = "Bits 8:10 - Appended MAC header length, unit in 32 bytes"]
    #[inline(always)]
    pub fn bit_append_mhdr_len(&mut self) -> BitAppendMhdrLenW<'_, RegWmacCrSpec> {
        BitAppendMhdrLenW::new(self, 8)
    }
    #[doc = "Bit 11 - If set to 1, mac header and rx desc of the pkt into rx buff are also received into FWFF."]
    #[inline(always)]
    pub fn bit_fwparsing_en(&mut self) -> BitFwparsingEnW<'_, RegWmacCrSpec> {
        BitFwparsingEnW::new(self, 11)
    }
    #[doc = "Bit 12 - If set to 1, Hw will append 16 bytes MAC header into FWFF when MACID search fail"]
    #[inline(always)]
    pub fn bit_apphdr_midsrch_fail(&mut self) -> BitApphdrMidsrchFailW<'_, RegWmacCrSpec> {
        BitApphdrMidsrchFailW::new(self, 12)
    }
    #[doc = "Bit 14 - If set to 1,HW will send interrupt (0x124\\[26\\]) only when there is phy status valid fwsts pkt written into FWFF"]
    #[inline(always)]
    pub fn bit_physts_pkt_ctrl(&mut self) -> BitPhystsPktCtrlW<'_, RegWmacCrSpec> {
        BitPhystsPktCtrlW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable to receive pkt into FWFF, default 1'b0. When Fw needs to receive pkt, this bit must be set to 1'b1"]
    #[inline(always)]
    pub fn bit_fwrx_en(&mut self) -> BitFwrxEnW<'_, RegWmacCrSpec> {
        BitFwrxEnW::new(self, 15)
    }
    #[doc = "Bit 16 - When this bit is set,Accept management frame in partial length condition. It will take effect when RxFilter3-5 is disabled and RxFilter 0-2 is enabled."]
    #[inline(always)]
    pub fn bit_mgnt_fw_sts_filter(&mut self) -> BitMgntFwStsFilterW<'_, RegWmacCrSpec> {
        BitMgntFwStsFilterW::new(self, 16)
    }
    #[doc = "Bit 17 - When this bit is set,Accept control frame in partial length condition. It will take effect when RxFilter3-5 is disabled and RxFilter 0-2 is enabled."]
    #[inline(always)]
    pub fn bit_ctrl_fw_sts_filter(&mut self) -> BitCtrlFwStsFilterW<'_, RegWmacCrSpec> {
        BitCtrlFwStsFilterW::new(self, 17)
    }
    #[doc = "Bit 18 - When this bit is set,Accept data frame in partial length condition. It will take effect when RxFilter3-5 is disabled and RxFilter 0-2 is enabled."]
    #[inline(always)]
    pub fn bit_data_fw_sts_filter(&mut self) -> BitDataFwStsFilterW<'_, RegWmacCrSpec> {
        BitDataFwStsFilterW::new(self, 18)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn bit_wmac_m11j(&mut self) -> BitWmacM11jW<'_, RegWmacCrSpec> {
        BitWmacM11jW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn bit_wmac_20mhzbw(&mut self) -> BitWmac20mhzbwW<'_, RegWmacCrSpec> {
        BitWmac20mhzbwW::new(self, 27)
    }
    #[doc = "Bit 28 - When set, will not wait and check he mu sigb and ndp info"]
    #[inline(always)]
    pub fn bit_dis_wait_he_sigb(&mut self) -> BitDisWaitHeSigbW<'_, RegWmacCrSpec> {
        BitDisWaitHeSigbW::new(self, 28)
    }
    #[doc = "Bit 29 - When set, enable sifs clear wlan_rx_req"]
    #[inline(always)]
    pub fn bit_wlan_rx_req_sifs_clr(&mut self) -> BitWlanRxReqSifsClrW<'_, RegWmacCrSpec> {
        BitWlanRxReqSifsClrW::new(self, 29)
    }
    #[doc = "Bit 30 - If set to 1, enable txpwr_ctrl clock gating function."]
    #[inline(always)]
    pub fn bit_txpwr_ctl_engclk(&mut self) -> BitTxpwrCtlEngclkW<'_, RegWmacCrSpec> {
        BitTxpwrCtlEngclkW::new(self, 30)
    }
    #[doc = "Bit 31 - IF set to 1, txpwr_ctrl clock enable, only work when BIT_TXPWR_CTL_ENGCLK set to 1,"]
    #[inline(always)]
    pub fn bit_txpwr_ctl_clk_en(&mut self) -> BitTxpwrCtlClkEnW<'_, RegWmacCrSpec> {
        BitTxpwrCtlClkEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wmac_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wmac_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWmacCrSpec;
impl crate::RegisterSpec for RegWmacCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wmac_cr::R`](R) reader structure"]
impl crate::Readable for RegWmacCrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wmac_cr::W`](W) writer structure"]
impl crate::Writable for RegWmacCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WMAC_CR to value 0x0800_0000"]
impl crate::Resettable for RegWmacCrSpec {
    const RESET_VALUE: u32 = 0x0800_0000;
}
