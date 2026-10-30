#[doc = "Register `IC_SAR` reader"]
pub type R = crate::R<IcSarSpec>;
#[doc = "Register `IC_SAR` writer"]
pub type W = crate::W<IcSarSpec>;
#[doc = "Field `IC_SAR` reader - The IC_SAR holds the slave address when the I2C is operating as slave1. For 7-bit addressing, only IC_SAR\\[6:0\\] is used. The register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect."]
pub type IcSarR = crate::FieldReader<u16>;
#[doc = "Field `IC_SAR` writer - The IC_SAR holds the slave address when the I2C is operating as slave1. For 7-bit addressing, only IC_SAR\\[6:0\\] is used. The register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect."]
pub type IcSarW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - The IC_SAR holds the slave address when the I2C is operating as slave1. For 7-bit addressing, only IC_SAR\\[6:0\\] is used. The register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect."]
    #[inline(always)]
    pub fn ic_sar(&self) -> IcSarR {
        IcSarR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - The IC_SAR holds the slave address when the I2C is operating as slave1. For 7-bit addressing, only IC_SAR\\[6:0\\] is used. The register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect."]
    #[inline(always)]
    pub fn ic_sar(&mut self) -> IcSarW<'_, IcSarSpec> {
        IcSarW::new(self, 0)
    }
}
#[doc = "I2C Slave Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_sar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_sar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcSarSpec;
impl crate::RegisterSpec for IcSarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_sar::R`](R) reader structure"]
impl crate::Readable for IcSarSpec {}
#[doc = "`write(|w| ..)` method takes [`ic_sar::W`](W) writer structure"]
impl crate::Writable for IcSarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_SAR to value 0x11"]
impl crate::Resettable for IcSarSpec {
    const RESET_VALUE: u32 = 0x11;
}
