#[doc = "Register `IC_HS_SCL_HCNT` reader"]
pub type R = crate::R<IcHsSclHcntSpec>;
#[doc = "Register `IC_HS_SCL_HCNT` writer"]
pub type W = crate::W<IcHsSclHcntSpec>;
#[doc = "Field `IC_HS_SCL_HCNT` reader - This register sets the SCL clock high-period count for high speed. This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect. The SCL High time depends on the loading of the bus. For 100pF loading, the SCL High time is 60ns; for 400pF loading, the SCL High time is 120ns."]
pub type IcHsSclHcntR = crate::FieldReader<u16>;
#[doc = "Field `IC_HS_SCL_HCNT` writer - This register sets the SCL clock high-period count for high speed. This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect. The SCL High time depends on the loading of the bus. For 100pF loading, the SCL High time is 60ns; for 400pF loading, the SCL High time is 120ns."]
pub type IcHsSclHcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register sets the SCL clock high-period count for high speed. This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect. The SCL High time depends on the loading of the bus. For 100pF loading, the SCL High time is 60ns; for 400pF loading, the SCL High time is 120ns."]
    #[inline(always)]
    pub fn ic_hs_scl_hcnt(&self) -> IcHsSclHcntR {
        IcHsSclHcntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register sets the SCL clock high-period count for high speed. This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect. The SCL High time depends on the loading of the bus. For 100pF loading, the SCL High time is 60ns; for 400pF loading, the SCL High time is 120ns."]
    #[inline(always)]
    pub fn ic_hs_scl_hcnt(&mut self) -> IcHsSclHcntW<'_, IcHsSclHcntSpec> {
        IcHsSclHcntW::new(self, 0)
    }
}
#[doc = "High Speed I2C Clock SCL High Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_hs_scl_hcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_hs_scl_hcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcHsSclHcntSpec;
impl crate::RegisterSpec for IcHsSclHcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_hs_scl_hcnt::R`](R) reader structure"]
impl crate::Readable for IcHsSclHcntSpec {}
#[doc = "`write(|w| ..)` method takes [`ic_hs_scl_hcnt::W`](W) writer structure"]
impl crate::Writable for IcHsSclHcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_HS_SCL_HCNT to value 0x06"]
impl crate::Resettable for IcHsSclHcntSpec {
    const RESET_VALUE: u32 = 0x06;
}
