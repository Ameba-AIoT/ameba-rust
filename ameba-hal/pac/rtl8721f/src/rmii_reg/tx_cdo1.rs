#[doc = "Register `TxCDO1` reader"]
pub type R = crate::R<TxCdo1Spec>;
#[doc = "Field `TxCDO1` reader - Tx 1st priority Current Descriptor Offset: FDP+CDO = current descriptor pointer. CDO increments by 16 bytes each time."]
pub type TxCdo1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Tx 1st priority Current Descriptor Offset: FDP+CDO = current descriptor pointer. CDO increments by 16 bytes each time."]
    #[inline(always)]
    pub fn tx_cdo1(&self) -> TxCdo1R {
        TxCdo1R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_cdo1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxCdo1Spec;
impl crate::RegisterSpec for TxCdo1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_cdo1::R`](R) reader structure"]
impl crate::Readable for TxCdo1Spec {}
#[doc = "`reset()` method sets TxCDO1 to value 0"]
impl crate::Resettable for TxCdo1Spec {}
