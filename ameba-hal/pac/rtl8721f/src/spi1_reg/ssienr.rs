#[doc = "Register `SSIENR` reader"]
pub type R = crate::R<SsienrSpec>;
#[doc = "Register `SSIENR` writer"]
pub type W = crate::W<SsienrSpec>;
#[doc = "Field `SSI_EN` reader - SPI Enable. Enables and disables all SPI operations. When disabled, all serial transfers are halted immediately. Transmit and receive FIFO buffers are cleared when the device is disabled. It is impossible to program some of the SPI control registers when enabled. When disabled, the ssi_sleep output is set (after delay) to inform the system that it is safe to remove the ssi_clk, thus saving power consumption in the system."]
pub type SsiEnR = crate::BitReader;
#[doc = "Field `SSI_EN` writer - SPI Enable. Enables and disables all SPI operations. When disabled, all serial transfers are halted immediately. Transmit and receive FIFO buffers are cleared when the device is disabled. It is impossible to program some of the SPI control registers when enabled. When disabled, the ssi_sleep output is set (after delay) to inform the system that it is safe to remove the ssi_clk, thus saving power consumption in the system."]
pub type SsiEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI Enable. Enables and disables all SPI operations. When disabled, all serial transfers are halted immediately. Transmit and receive FIFO buffers are cleared when the device is disabled. It is impossible to program some of the SPI control registers when enabled. When disabled, the ssi_sleep output is set (after delay) to inform the system that it is safe to remove the ssi_clk, thus saving power consumption in the system."]
    #[inline(always)]
    pub fn ssi_en(&self) -> SsiEnR {
        SsiEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Enable. Enables and disables all SPI operations. When disabled, all serial transfers are halted immediately. Transmit and receive FIFO buffers are cleared when the device is disabled. It is impossible to program some of the SPI control registers when enabled. When disabled, the ssi_sleep output is set (after delay) to inform the system that it is safe to remove the ssi_clk, thus saving power consumption in the system."]
    #[inline(always)]
    pub fn ssi_en(&mut self) -> SsiEnW<'_, SsienrSpec> {
        SsiEnW::new(self, 0)
    }
}
#[doc = "SSI Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssienr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssienr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsienrSpec;
impl crate::RegisterSpec for SsienrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssienr::R`](R) reader structure"]
impl crate::Readable for SsienrSpec {}
#[doc = "`write(|w| ..)` method takes [`ssienr::W`](W) writer structure"]
impl crate::Writable for SsienrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSIENR to value 0"]
impl crate::Resettable for SsienrSpec {}
