#[doc = "Register `SPDIO_CCPWM` reader"]
pub type R = crate::R<SpdioCcpwmSpec>;
#[doc = "Register `SPDIO_CCPWM` writer"]
pub type W = crate::W<SpdioCcpwmSpec>;
#[doc = "Field `SPDIO_CCPWM` reader - ccpwm"]
pub type SpdioCcpwmR = crate::FieldReader;
#[doc = "Field `SPDIO_CCPWM` writer - ccpwm"]
pub type SpdioCcpwmW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SPDIO_CCPWM_PS` reader - device write 1 to trigger interrupt to host"]
pub type SpdioCcpwmPsR = crate::BitReader;
#[doc = "Field `SPDIO_CCPWM_PS` writer - device write 1 to trigger interrupt to host"]
pub type SpdioCcpwmPsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - ccpwm"]
    #[inline(always)]
    pub fn spdio_ccpwm(&self) -> SpdioCcpwmR {
        SpdioCcpwmR::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - device write 1 to trigger interrupt to host"]
    #[inline(always)]
    pub fn spdio_ccpwm_ps(&self) -> SpdioCcpwmPsR {
        SpdioCcpwmPsR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - ccpwm"]
    #[inline(always)]
    pub fn spdio_ccpwm(&mut self) -> SpdioCcpwmW<'_, SpdioCcpwmSpec> {
        SpdioCcpwmW::new(self, 0)
    }
    #[doc = "Bit 7 - device write 1 to trigger interrupt to host"]
    #[inline(always)]
    pub fn spdio_ccpwm_ps(&mut self) -> SpdioCcpwmPsW<'_, SpdioCcpwmSpec> {
        SpdioCcpwmPsW::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_ccpwm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdio_ccpwm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioCcpwmSpec;
impl crate::RegisterSpec for SpdioCcpwmSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spdio_ccpwm::R`](R) reader structure"]
impl crate::Readable for SpdioCcpwmSpec {}
#[doc = "`write(|w| ..)` method takes [`spdio_ccpwm::W`](W) writer structure"]
impl crate::Writable for SpdioCcpwmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPDIO_CCPWM to value 0"]
impl crate::Resettable for SpdioCcpwmSpec {}
