#[doc = "Register `TxCDO5` reader"]
pub type R = crate::R<TxCdo5Spec>;
#[doc = "Field `TxCDO5` reader - Tx 5st priority Current Descriptor Offset: FDP+CDO = current descriptor pointer. CDO increments by 16 bytes each time."]
pub type TxCdo5R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Tx 5st priority Current Descriptor Offset: FDP+CDO = current descriptor pointer. CDO increments by 16 bytes each time."]
    #[inline(always)]
    pub fn tx_cdo5(&self) -> TxCdo5R {
        TxCdo5R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_cdo5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxCdo5Spec;
impl crate::RegisterSpec for TxCdo5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_cdo5::R`](R) reader structure"]
impl crate::Readable for TxCdo5Spec {}
#[doc = "`reset()` method sets TxCDO5 to value 0"]
impl crate::Resettable for TxCdo5Spec {}
