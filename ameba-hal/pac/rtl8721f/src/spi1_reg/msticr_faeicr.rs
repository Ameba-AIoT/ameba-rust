#[doc = "Register `MSTICR_FAEICR` reader"]
pub type R = crate::R<MsticrFaeicrSpec>;
#[doc = "Field `MSTICR_FAEICR` reader - Multi-Master Interrupt Clear Register/Frame Alignment Error Interrupt Clear Register. When SPI is configured as serial-master, this bit field is used to Clear Multi-Master Contention Interrupt. A read from this register clears the ssi_mst_intr interrupt; writing has no effect. When SPI is configured as serial-slave, this bit field is used to Clear Frame Alignment Interrupt. A read from this register clears the ssi_fae_intr interrupt; writing has no effect."]
pub type MsticrFaeicrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Multi-Master Interrupt Clear Register/Frame Alignment Error Interrupt Clear Register. When SPI is configured as serial-master, this bit field is used to Clear Multi-Master Contention Interrupt. A read from this register clears the ssi_mst_intr interrupt; writing has no effect. When SPI is configured as serial-slave, this bit field is used to Clear Frame Alignment Interrupt. A read from this register clears the ssi_fae_intr interrupt; writing has no effect."]
    #[inline(always)]
    pub fn msticr_faeicr(&self) -> MsticrFaeicrR {
        MsticrFaeicrR::new((self.bits & 1) != 0)
    }
}
#[doc = "Multi-Master & Frame Alignment Error Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msticr_faeicr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsticrFaeicrSpec;
impl crate::RegisterSpec for MsticrFaeicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msticr_faeicr::R`](R) reader structure"]
impl crate::Readable for MsticrFaeicrSpec {}
#[doc = "`reset()` method sets MSTICR_FAEICR to value 0"]
impl crate::Resettable for MsticrFaeicrSpec {}
