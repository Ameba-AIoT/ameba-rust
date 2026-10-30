#[doc = "Register `TxCDO4` reader"]
pub type R = crate::R<TxCdo4Spec>;
#[doc = "Field `TxCDO4` reader - Tx 4st priority Current Descriptor Offset: FDP+CDO = current descriptor pointer. CDO increments by 16 bytes each time."]
pub type TxCdo4R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Tx 4st priority Current Descriptor Offset: FDP+CDO = current descriptor pointer. CDO increments by 16 bytes each time."]
    #[inline(always)]
    pub fn tx_cdo4(&self) -> TxCdo4R {
        TxCdo4R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_cdo4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxCdo4Spec;
impl crate::RegisterSpec for TxCdo4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_cdo4::R`](R) reader structure"]
impl crate::Readable for TxCdo4Spec {}
#[doc = "`reset()` method sets TxCDO4 to value 0"]
impl crate::Resettable for TxCdo4Spec {}
