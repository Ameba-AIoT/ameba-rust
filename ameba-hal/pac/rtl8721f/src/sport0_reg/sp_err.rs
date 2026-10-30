#[doc = "Register `SP_ERR` reader"]
pub type R = crate::R<SpErrSpec>;
#[doc = "Field `tx_err_cnt` reader - TX error counter (SPK path) Note: This counter should always be zero if everything works well."]
pub type TxErrCntR = crate::FieldReader<u16>;
#[doc = "Field `rx_err_cnt` reader - RX error counter (MIC path) Note: This counter should always be zero if everything works well."]
pub type RxErrCntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - TX error counter (SPK path) Note: This counter should always be zero if everything works well."]
    #[inline(always)]
    pub fn tx_err_cnt(&self) -> TxErrCntR {
        TxErrCntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - RX error counter (MIC path) Note: This counter should always be zero if everything works well."]
    #[inline(always)]
    pub fn rx_err_cnt(&self) -> RxErrCntR {
        RxErrCntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_err::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpErrSpec;
impl crate::RegisterSpec for SpErrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_err::R`](R) reader structure"]
impl crate::Readable for SpErrSpec {}
#[doc = "`reset()` method sets SP_ERR to value 0"]
impl crate::Resettable for SpErrSpec {}
