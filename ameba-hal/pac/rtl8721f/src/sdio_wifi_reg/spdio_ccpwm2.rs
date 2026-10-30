#[doc = "Register `SPDIO_CCPWM2` reader"]
pub type R = crate::R<SpdioCcpwm2Spec>;
#[doc = "Register `SPDIO_CCPWM2` writer"]
pub type W = crate::W<SpdioCcpwm2Spec>;
#[doc = "Field `SPDIO_CCPWM2` reader - ccpwm2"]
pub type SpdioCcpwm2R = crate::FieldReader<u16>;
#[doc = "Field `SPDIO_CCPWM2` writer - ccpwm2"]
pub type SpdioCcpwm2W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `SPDIO_CCPWM2_PS` reader - device write 1 to trigger interrupt to host"]
pub type SpdioCcpwm2PsR = crate::BitReader;
#[doc = "Field `SPDIO_CCPWM2_PS` writer - device write 1 to trigger interrupt to host"]
pub type SpdioCcpwm2PsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - ccpwm2"]
    #[inline(always)]
    pub fn spdio_ccpwm2(&self) -> SpdioCcpwm2R {
        SpdioCcpwm2R::new(self.bits & 0x7fff)
    }
    #[doc = "Bit 15 - device write 1 to trigger interrupt to host"]
    #[inline(always)]
    pub fn spdio_ccpwm2_ps(&self) -> SpdioCcpwm2PsR {
        SpdioCcpwm2PsR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - ccpwm2"]
    #[inline(always)]
    pub fn spdio_ccpwm2(&mut self) -> SpdioCcpwm2W<'_, SpdioCcpwm2Spec> {
        SpdioCcpwm2W::new(self, 0)
    }
    #[doc = "Bit 15 - device write 1 to trigger interrupt to host"]
    #[inline(always)]
    pub fn spdio_ccpwm2_ps(&mut self) -> SpdioCcpwm2PsW<'_, SpdioCcpwm2Spec> {
        SpdioCcpwm2PsW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_ccpwm2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdio_ccpwm2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioCcpwm2Spec;
impl crate::RegisterSpec for SpdioCcpwm2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spdio_ccpwm2::R`](R) reader structure"]
impl crate::Readable for SpdioCcpwm2Spec {}
#[doc = "`write(|w| ..)` method takes [`spdio_ccpwm2::W`](W) writer structure"]
impl crate::Writable for SpdioCcpwm2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPDIO_CCPWM2 to value 0"]
impl crate::Resettable for SpdioCcpwm2Spec {}
