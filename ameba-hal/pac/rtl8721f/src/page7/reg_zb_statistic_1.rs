#[doc = "Register `REG_ZB_STATISTIC_1` reader"]
pub type R = crate::R<RegZbStatistic1Spec>;
#[doc = "Field `BIT_ZB_HI_TX` reader - "]
pub type BitZbHiTxR = crate::FieldReader<u16>;
#[doc = "Field `BIT_ZB_HI_RX` reader - for statistic"]
pub type BitZbHiRxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bit_zb_hi_tx(&self) -> BitZbHiTxR {
        BitZbHiTxR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - for statistic"]
    #[inline(always)]
    pub fn bit_zb_hi_rx(&self) -> BitZbHiRxR {
        BitZbHiRxR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_zb_statistic_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegZbStatistic1Spec;
impl crate::RegisterSpec for RegZbStatistic1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_zb_statistic_1::R`](R) reader structure"]
impl crate::Readable for RegZbStatistic1Spec {}
#[doc = "`reset()` method sets REG_ZB_STATISTIC_1 to value 0"]
impl crate::Resettable for RegZbStatistic1Spec {}
