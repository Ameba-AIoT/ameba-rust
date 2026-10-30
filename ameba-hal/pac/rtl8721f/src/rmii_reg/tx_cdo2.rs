#[doc = "Register `TxCDO2` reader"]
pub type R = crate::R<TxCdo2Spec>;
#[doc = "Field `TxCDO2` reader - Tx 2st priority Current Descriptor Offset: FDP+CDO = current descriptor pointer. CDO increments by 16 bytes each time."]
pub type TxCdo2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Tx 2st priority Current Descriptor Offset: FDP+CDO = current descriptor pointer. CDO increments by 16 bytes each time."]
    #[inline(always)]
    pub fn tx_cdo2(&self) -> TxCdo2R {
        TxCdo2R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_cdo2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxCdo2Spec;
impl crate::RegisterSpec for TxCdo2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_cdo2::R`](R) reader structure"]
impl crate::Readable for TxCdo2Spec {}
#[doc = "`reset()` method sets TxCDO2 to value 0"]
impl crate::Resettable for TxCdo2Spec {}
