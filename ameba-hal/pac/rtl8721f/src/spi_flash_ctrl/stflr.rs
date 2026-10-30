#[doc = "Register `STFLR` reader"]
pub type R = crate::R<StflrSpec>;
#[doc = "Field `STFLR` reader - Status FIFO level. Indicates the FIFO entries of valid data in status data. Note: Available only if configuration of NAND_EN is defined."]
pub type StflrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Status FIFO level. Indicates the FIFO entries of valid data in status data. Note: Available only if configuration of NAND_EN is defined."]
    #[inline(always)]
    pub fn stflr(&self) -> StflrR {
        StflrR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Status FIFO level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stflr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StflrSpec;
impl crate::RegisterSpec for StflrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stflr::R`](R) reader structure"]
impl crate::Readable for StflrSpec {}
#[doc = "`reset()` method sets STFLR to value 0"]
impl crate::Resettable for StflrSpec {}
