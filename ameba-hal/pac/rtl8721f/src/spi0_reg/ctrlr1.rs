#[doc = "Register `CTRLR1` reader"]
pub type R = crate::R<Ctrlr1Spec>;
#[doc = "Register `CTRLR1` writer"]
pub type W = crate::W<Ctrlr1Spec>;
#[doc = "Field `NDF` reader - Number of Data Frames. When TMOD = 10 or TMOD = 11, this register field sets the number of data frames to be continuously received by the SPI. The SPI continues to receive serial data until the number of data frames received is equal to this register value plus 1, which enables you to receive up to 64 KB of data in a continuous transfer. When the SPI is configured as a serial slave, the transfer continues for as long as the slave is selected. Therefore, this register serves no purpose and is not present when the SPI is configured as a serial slave. When CFS_EN is enabled, this bit field will be taken by SPI master to calculate the amount of data bits for all TMOD."]
pub type NdfR = crate::FieldReader<u16>;
#[doc = "Field `NDF` writer - Number of Data Frames. When TMOD = 10 or TMOD = 11, this register field sets the number of data frames to be continuously received by the SPI. The SPI continues to receive serial data until the number of data frames received is equal to this register value plus 1, which enables you to receive up to 64 KB of data in a continuous transfer. When the SPI is configured as a serial slave, the transfer continues for as long as the slave is selected. Therefore, this register serves no purpose and is not present when the SPI is configured as a serial slave. When CFS_EN is enabled, this bit field will be taken by SPI master to calculate the amount of data bits for all TMOD."]
pub type NdfW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of Data Frames. When TMOD = 10 or TMOD = 11, this register field sets the number of data frames to be continuously received by the SPI. The SPI continues to receive serial data until the number of data frames received is equal to this register value plus 1, which enables you to receive up to 64 KB of data in a continuous transfer. When the SPI is configured as a serial slave, the transfer continues for as long as the slave is selected. Therefore, this register serves no purpose and is not present when the SPI is configured as a serial slave. When CFS_EN is enabled, this bit field will be taken by SPI master to calculate the amount of data bits for all TMOD."]
    #[inline(always)]
    pub fn ndf(&self) -> NdfR {
        NdfR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of Data Frames. When TMOD = 10 or TMOD = 11, this register field sets the number of data frames to be continuously received by the SPI. The SPI continues to receive serial data until the number of data frames received is equal to this register value plus 1, which enables you to receive up to 64 KB of data in a continuous transfer. When the SPI is configured as a serial slave, the transfer continues for as long as the slave is selected. Therefore, this register serves no purpose and is not present when the SPI is configured as a serial slave. When CFS_EN is enabled, this bit field will be taken by SPI master to calculate the amount of data bits for all TMOD."]
    #[inline(always)]
    pub fn ndf(&mut self) -> NdfW<'_, Ctrlr1Spec> {
        NdfW::new(self, 0)
    }
}
#[doc = "Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrlr1Spec;
impl crate::RegisterSpec for Ctrlr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlr1::R`](R) reader structure"]
impl crate::Readable for Ctrlr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrlr1::W`](W) writer structure"]
impl crate::Writable for Ctrlr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLR1 to value 0"]
impl crate::Resettable for Ctrlr1Spec {}
