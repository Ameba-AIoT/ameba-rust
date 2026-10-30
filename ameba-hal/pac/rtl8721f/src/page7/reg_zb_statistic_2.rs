#[doc = "Register `REG_ZB_STATISTIC_2` reader"]
pub type R = crate::R<RegZbStatistic2Spec>;
#[doc = "Field `BIT_ZB_LO_TX` reader - "]
pub type BitZbLoTxR = crate::FieldReader<u16>;
#[doc = "Field `BIT_ZB_LO_RX` reader - for statistic"]
pub type BitZbLoRxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bit_zb_lo_tx(&self) -> BitZbLoTxR {
        BitZbLoTxR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - for statistic"]
    #[inline(always)]
    pub fn bit_zb_lo_rx(&self) -> BitZbLoRxR {
        BitZbLoRxR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_zb_statistic_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegZbStatistic2Spec;
impl crate::RegisterSpec for RegZbStatistic2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_zb_statistic_2::R`](R) reader structure"]
impl crate::Readable for RegZbStatistic2Spec {}
#[doc = "`reset()` method sets REG_ZB_STATISTIC_2 to value 0"]
impl crate::Resettable for RegZbStatistic2Spec {}
