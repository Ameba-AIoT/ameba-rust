#[doc = "Register `SP_TRX_COUNTER_STATUS` reader"]
pub type R = crate::R<SpTrxCounterStatusSpec>;
#[doc = "Field `tx_depth_cnt_0` reader - TX FIFO_0 depth counter status (SPK path)"]
pub type TxDepthCnt0R = crate::FieldReader;
#[doc = "Field `rx_depth_cnt_0` reader - RX FIFO_0 depth counter status (MIC path)"]
pub type RxDepthCnt0R = crate::FieldReader;
#[doc = "Field `tx_depth_cnt_1` reader - TX FIFO_1 depth counter status (SPK path)"]
pub type TxDepthCnt1R = crate::FieldReader;
#[doc = "Field `rx_depth_cnt_1` reader - RX FIFO_1 depth counter status (MIC path)"]
pub type RxDepthCnt1R = crate::FieldReader;
#[doc = "Field `sp_reset_state` reader - 1'b1: sp_reset is enable 1'b0: sp_reset is disable"]
pub type SpResetStateR = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - TX FIFO_0 depth counter status (SPK path)"]
    #[inline(always)]
    pub fn tx_depth_cnt_0(&self) -> TxDepthCnt0R {
        TxDepthCnt0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - RX FIFO_0 depth counter status (MIC path)"]
    #[inline(always)]
    pub fn rx_depth_cnt_0(&self) -> RxDepthCnt0R {
        RxDepthCnt0R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - TX FIFO_1 depth counter status (SPK path)"]
    #[inline(always)]
    pub fn tx_depth_cnt_1(&self) -> TxDepthCnt1R {
        TxDepthCnt1R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - RX FIFO_1 depth counter status (MIC path)"]
    #[inline(always)]
    pub fn rx_depth_cnt_1(&self) -> RxDepthCnt1R {
        RxDepthCnt1R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - 1'b1: sp_reset is enable 1'b0: sp_reset is disable"]
    #[inline(always)]
    pub fn sp_reset_state(&self) -> SpResetStateR {
        SpResetStateR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_trx_counter_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpTrxCounterStatusSpec;
impl crate::RegisterSpec for SpTrxCounterStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_trx_counter_status::R`](R) reader structure"]
impl crate::Readable for SpTrxCounterStatusSpec {}
#[doc = "`reset()` method sets SP_TRX_COUNTER_STATUS to value 0"]
impl crate::Resettable for SpTrxCounterStatusSpec {}
