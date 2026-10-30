#[doc = "Register `TxCDO3` reader"]
pub type R = crate::R<TxCdo3Spec>;
#[doc = "Field `TxCDO3` reader - Tx 3st priority Current Descriptor Offset: FDP+CDO = current descriptor pointer. CDO increments by 16 bytes each time."]
pub type TxCdo3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Tx 3st priority Current Descriptor Offset: FDP+CDO = current descriptor pointer. CDO increments by 16 bytes each time."]
    #[inline(always)]
    pub fn tx_cdo3(&self) -> TxCdo3R {
        TxCdo3R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_cdo3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxCdo3Spec;
impl crate::RegisterSpec for TxCdo3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_cdo3::R`](R) reader structure"]
impl crate::Readable for TxCdo3Spec {}
#[doc = "`reset()` method sets TxCDO3 to value 0"]
impl crate::Resettable for TxCdo3Spec {}
