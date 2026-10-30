#[doc = "Register `SPI_DUMMY` reader"]
pub type R = crate::R<SpiDummySpec>;
#[doc = "Field `dummy` reader - rsvd for hw"]
pub type DummyR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - rsvd for hw"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "SPI Dummy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_dummy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiDummySpec;
impl crate::RegisterSpec for SpiDummySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_dummy::R`](R) reader structure"]
impl crate::Readable for SpiDummySpec {}
#[doc = "`reset()` method sets SPI_DUMMY to value 0"]
impl crate::Resettable for SpiDummySpec {}
