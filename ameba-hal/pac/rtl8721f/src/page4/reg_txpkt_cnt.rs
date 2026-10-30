#[doc = "Register `REG_TXPKT_CNT` reader"]
pub type R = crate::R<RegTxpktCntSpec>;
#[doc = "Field `BIT_TXPKT_CNT_RPT` reader - The results of TXPKT counter. Depends on BIT_TXPKT_CNT_SEL."]
pub type BitTxpktCntRptR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The results of TXPKT counter. Depends on BIT_TXPKT_CNT_SEL."]
    #[inline(always)]
    pub fn bit_txpkt_cnt_rpt(&self) -> BitTxpktCntRptR {
        BitTxpktCntRptR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_txpkt_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTxpktCntSpec;
impl crate::RegisterSpec for RegTxpktCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_txpkt_cnt::R`](R) reader structure"]
impl crate::Readable for RegTxpktCntSpec {}
#[doc = "`reset()` method sets REG_TXPKT_CNT to value 0"]
impl crate::Resettable for RegTxpktCntSpec {}
