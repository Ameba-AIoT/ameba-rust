#[doc = "Register `REG_BT_POLLUTE_PKT_CNT` reader"]
pub type R = crate::R<RegBtPollutePktCntSpec>;
#[doc = "Field `BIT_bt_txpollute_pkt_cnt` reader - count the tx fail pkt caused by bt TX pollute, write 0x4E8\\[15\\] will clear the counter"]
pub type BitBtTxpollutePktCntR = crate::FieldReader<u16>;
#[doc = "Field `BIT_bt_rxpollute_pkt_cnt` reader - count the tx fail pkt caused by bt RX pollute, write 0x4E8\\[31\\] will clear the counter"]
pub type BitBtRxpollutePktCntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - count the tx fail pkt caused by bt TX pollute, write 0x4E8\\[15\\] will clear the counter"]
    #[inline(always)]
    pub fn bit_bt_txpollute_pkt_cnt(&self) -> BitBtTxpollutePktCntR {
        BitBtTxpollutePktCntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - count the tx fail pkt caused by bt RX pollute, write 0x4E8\\[31\\] will clear the counter"]
    #[inline(always)]
    pub fn bit_bt_rxpollute_pkt_cnt(&self) -> BitBtRxpollutePktCntR {
        BitBtRxpollutePktCntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bt_pollute_pkt_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBtPollutePktCntSpec;
impl crate::RegisterSpec for RegBtPollutePktCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bt_pollute_pkt_cnt::R`](R) reader structure"]
impl crate::Readable for RegBtPollutePktCntSpec {}
#[doc = "`reset()` method sets REG_BT_POLLUTE_PKT_CNT to value 0"]
impl crate::Resettable for RegBtPollutePktCntSpec {}
