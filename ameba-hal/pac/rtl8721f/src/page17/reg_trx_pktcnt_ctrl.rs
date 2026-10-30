#[doc = "Register `REG_TRX_PKTCNT_CTRL` reader"]
pub type R = crate::R<RegTrxPktcntCtrlSpec>;
#[doc = "Register `REG_TRX_PKTCNT_CTRL` writer"]
pub type W = crate::W<RegTrxPktcntCtrlSpec>;
#[doc = "Field `r_wmac_txpktcnt_cfg` reader - Tx packet conuter user define configuration \\[5:2\\]:subtype \\[1:0\\]:type"]
pub type RWmacTxpktcntCfgR = crate::FieldReader;
#[doc = "Field `r_wmac_txpktcnt_cfg` writer - Tx packet conuter user define configuration \\[5:2\\]:subtype \\[1:0\\]:type"]
pub type RWmacTxpktcntCfgW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `r_wmac_txpktcnt_sel` reader - Tx packet counter report selection 4'd0: user define 0 counter 4'd1: user define 1 counter 4'd2: user define 2 counter 4'd3: user define 3 counter 4'd4: HT mpdu tx ok counter 4'd5: VHT SU mpdu tx ok counter 4'd6: HE SU mpdu tx ok counter 4'd7: HE TB mpdu tx ok counter 4'd8: CCK ppdu tx ok counter 4'd9: OFDM ppdu tx ok counter 4'd10: HT ppdu tx ok counter 4'd11: VHT SU ppdu tx ok counter 4'd12: HE SU ppdu tx ok counter 4'd13: HE TB ppdu tx ok counter 4'd14~4'd15: RSVD"]
pub type RWmacTxpktcntSelR = crate::FieldReader;
#[doc = "Field `r_wmac_txpktcnt_sel` writer - Tx packet counter report selection 4'd0: user define 0 counter 4'd1: user define 1 counter 4'd2: user define 2 counter 4'd3: user define 3 counter 4'd4: HT mpdu tx ok counter 4'd5: VHT SU mpdu tx ok counter 4'd6: HE SU mpdu tx ok counter 4'd7: HE TB mpdu tx ok counter 4'd8: CCK ppdu tx ok counter 4'd9: OFDM ppdu tx ok counter 4'd10: HT ppdu tx ok counter 4'd11: VHT SU ppdu tx ok counter 4'd12: HE SU ppdu tx ok counter 4'd13: HE TB ppdu tx ok counter 4'd14~4'd15: RSVD"]
pub type RWmacTxpktcntSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_tx_udpktcnt_set` reader - Set user define packet configure, write one setting, it is only valid for r_wmac_txpktcnt_sel (4'd0 ~ 4'd3)"]
pub type RTxUdpktcntSetR = crate::BitReader;
#[doc = "Field `r_tx_udpktcnt_set` writer - Set user define packet configure, write one setting, it is only valid for r_wmac_txpktcnt_sel (4'd0 ~ 4'd3)"]
pub type RTxUdpktcntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_txpktcnt_rst` reader - Reset tx packet counter, write \"one\" to set the counter to zero"]
pub type RTxpktcntRstR = crate::BitReader;
#[doc = "Field `r_txpktcnt_rst` writer - Reset tx packet counter, write \"one\" to set the counter to zero"]
pub type RTxpktcntRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Tx packet conuter user define configuration \\[5:2\\]:subtype \\[1:0\\]:type"]
    #[inline(always)]
    pub fn r_wmac_txpktcnt_cfg(&self) -> RWmacTxpktcntCfgR {
        RWmacTxpktcntCfgR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - Tx packet counter report selection 4'd0: user define 0 counter 4'd1: user define 1 counter 4'd2: user define 2 counter 4'd3: user define 3 counter 4'd4: HT mpdu tx ok counter 4'd5: VHT SU mpdu tx ok counter 4'd6: HE SU mpdu tx ok counter 4'd7: HE TB mpdu tx ok counter 4'd8: CCK ppdu tx ok counter 4'd9: OFDM ppdu tx ok counter 4'd10: HT ppdu tx ok counter 4'd11: VHT SU ppdu tx ok counter 4'd12: HE SU ppdu tx ok counter 4'd13: HE TB ppdu tx ok counter 4'd14~4'd15: RSVD"]
    #[inline(always)]
    pub fn r_wmac_txpktcnt_sel(&self) -> RWmacTxpktcntSelR {
        RWmacTxpktcntSelR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - Set user define packet configure, write one setting, it is only valid for r_wmac_txpktcnt_sel (4'd0 ~ 4'd3)"]
    #[inline(always)]
    pub fn r_tx_udpktcnt_set(&self) -> RTxUdpktcntSetR {
        RTxUdpktcntSetR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reset tx packet counter, write \"one\" to set the counter to zero"]
    #[inline(always)]
    pub fn r_txpktcnt_rst(&self) -> RTxpktcntRstR {
        RTxpktcntRstR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Tx packet conuter user define configuration \\[5:2\\]:subtype \\[1:0\\]:type"]
    #[inline(always)]
    pub fn r_wmac_txpktcnt_cfg(&mut self) -> RWmacTxpktcntCfgW<'_, RegTrxPktcntCtrlSpec> {
        RWmacTxpktcntCfgW::new(self, 0)
    }
    #[doc = "Bits 6:9 - Tx packet counter report selection 4'd0: user define 0 counter 4'd1: user define 1 counter 4'd2: user define 2 counter 4'd3: user define 3 counter 4'd4: HT mpdu tx ok counter 4'd5: VHT SU mpdu tx ok counter 4'd6: HE SU mpdu tx ok counter 4'd7: HE TB mpdu tx ok counter 4'd8: CCK ppdu tx ok counter 4'd9: OFDM ppdu tx ok counter 4'd10: HT ppdu tx ok counter 4'd11: VHT SU ppdu tx ok counter 4'd12: HE SU ppdu tx ok counter 4'd13: HE TB ppdu tx ok counter 4'd14~4'd15: RSVD"]
    #[inline(always)]
    pub fn r_wmac_txpktcnt_sel(&mut self) -> RWmacTxpktcntSelW<'_, RegTrxPktcntCtrlSpec> {
        RWmacTxpktcntSelW::new(self, 6)
    }
    #[doc = "Bit 10 - Set user define packet configure, write one setting, it is only valid for r_wmac_txpktcnt_sel (4'd0 ~ 4'd3)"]
    #[inline(always)]
    pub fn r_tx_udpktcnt_set(&mut self) -> RTxUdpktcntSetW<'_, RegTrxPktcntCtrlSpec> {
        RTxUdpktcntSetW::new(self, 10)
    }
    #[doc = "Bit 11 - Reset tx packet counter, write \"one\" to set the counter to zero"]
    #[inline(always)]
    pub fn r_txpktcnt_rst(&mut self) -> RTxpktcntRstW<'_, RegTrxPktcntCtrlSpec> {
        RTxpktcntRstW::new(self, 11)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_trx_pktcnt_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_trx_pktcnt_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTrxPktcntCtrlSpec;
impl crate::RegisterSpec for RegTrxPktcntCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_trx_pktcnt_ctrl::R`](R) reader structure"]
impl crate::Readable for RegTrxPktcntCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_trx_pktcnt_ctrl::W`](W) writer structure"]
impl crate::Writable for RegTrxPktcntCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TRX_PKTCNT_CTRL to value 0"]
impl crate::Resettable for RegTrxPktcntCtrlSpec {}
