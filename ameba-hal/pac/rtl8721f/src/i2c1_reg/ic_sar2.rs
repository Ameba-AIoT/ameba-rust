#[doc = "Register `IC_SAR2` reader"]
pub type R = crate::R<IcSar2Spec>;
#[doc = "Register `IC_SAR2` writer"]
pub type W = crate::W<IcSar2Spec>;
#[doc = "Field `IC_SAR2` reader - The IC_SAR2 holds the slave address when the I2C is operating as slave2. Slave2 only supports 7 bit address. The register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect."]
pub type IcSar2R = crate::FieldReader;
#[doc = "Field `IC_SAR2` writer - The IC_SAR2 holds the slave address when the I2C is operating as slave2. Slave2 only supports 7 bit address. The register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect."]
pub type IcSar2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The IC_SAR2 holds the slave address when the I2C is operating as slave2. Slave2 only supports 7 bit address. The register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect."]
    #[inline(always)]
    pub fn ic_sar2(&self) -> IcSar2R {
        IcSar2R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The IC_SAR2 holds the slave address when the I2C is operating as slave2. Slave2 only supports 7 bit address. The register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect."]
    #[inline(always)]
    pub fn ic_sar2(&mut self) -> IcSar2W<'_, IcSar2Spec> {
        IcSar2W::new(self, 0)
    }
}
#[doc = "I2C Slave2 Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_sar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_sar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcSar2Spec;
impl crate::RegisterSpec for IcSar2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_sar2::R`](R) reader structure"]
impl crate::Readable for IcSar2Spec {}
#[doc = "`write(|w| ..)` method takes [`ic_sar2::W`](W) writer structure"]
impl crate::Writable for IcSar2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_SAR2 to value 0x12"]
impl crate::Resettable for IcSar2Spec {
    const RESET_VALUE: u32 = 0x12;
}
