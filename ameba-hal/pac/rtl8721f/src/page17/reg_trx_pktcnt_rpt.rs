#[doc = "Register `REG_TRX_PKTCNT_RPT` reader"]
pub type R = crate::R<RegTrxPktcntRptSpec>;
#[doc = "Field `ro_wmac_txpktcnt_rpt` reader - 25:22\\]:Tx packet counter report selection,r_wmac_txpktcnt_sel \\[21:16\\]:user define->tx packet type,\\[21:18\\]:subtype,\\[17:16\\]:type; others->6'h0 \\[15:0\\]:The tx reported counter value corresponding to Report type selected"]
pub type RoWmacTxpktcntRptR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:25 - 25:22\\]:Tx packet counter report selection,r_wmac_txpktcnt_sel \\[21:16\\]:user define->tx packet type,\\[21:18\\]:subtype,\\[17:16\\]:type; others->6'h0 \\[15:0\\]:The tx reported counter value corresponding to Report type selected"]
    #[inline(always)]
    pub fn ro_wmac_txpktcnt_rpt(&self) -> RoWmacTxpktcntRptR {
        RoWmacTxpktcntRptR::new(self.bits & 0x03ff_ffff)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_trx_pktcnt_rpt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTrxPktcntRptSpec;
impl crate::RegisterSpec for RegTrxPktcntRptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_trx_pktcnt_rpt::R`](R) reader structure"]
impl crate::Readable for RegTrxPktcntRptSpec {}
#[doc = "`reset()` method sets REG_TRX_PKTCNT_RPT to value 0"]
impl crate::Resettable for RegTrxPktcntRptSpec {}
