#[doc = "Register `IC_HS_SCL_LCNT` reader"]
pub type R = crate::R<IcHsSclLcntSpec>;
#[doc = "Register `IC_HS_SCL_LCNT` writer"]
pub type W = crate::W<IcHsSclLcntSpec>;
#[doc = "Field `IC_HS_SCL_LCNT` reader - This register sets the SCL clock low-period count for high speed. This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect. The SCL Low time depends on the loading of the bus. For 100pF loading, the SCL High time is 160ns; for 400pF loading, the SCL High time is 320ns."]
pub type IcHsSclLcntR = crate::FieldReader<u16>;
#[doc = "Field `IC_HS_SCL_LCNT` writer - This register sets the SCL clock low-period count for high speed. This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect. The SCL Low time depends on the loading of the bus. For 100pF loading, the SCL High time is 160ns; for 400pF loading, the SCL High time is 320ns."]
pub type IcHsSclLcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register sets the SCL clock low-period count for high speed. This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect. The SCL Low time depends on the loading of the bus. For 100pF loading, the SCL High time is 160ns; for 400pF loading, the SCL High time is 320ns."]
    #[inline(always)]
    pub fn ic_hs_scl_lcnt(&self) -> IcHsSclLcntR {
        IcHsSclLcntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register sets the SCL clock low-period count for high speed. This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect. The SCL Low time depends on the loading of the bus. For 100pF loading, the SCL High time is 160ns; for 400pF loading, the SCL High time is 320ns."]
    #[inline(always)]
    pub fn ic_hs_scl_lcnt(&mut self) -> IcHsSclLcntW<'_, IcHsSclLcntSpec> {
        IcHsSclLcntW::new(self, 0)
    }
}
#[doc = "High Speed I2C Clock SCL Low Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_hs_scl_lcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_hs_scl_lcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcHsSclLcntSpec;
impl crate::RegisterSpec for IcHsSclLcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_hs_scl_lcnt::R`](R) reader structure"]
impl crate::Readable for IcHsSclLcntSpec {}
#[doc = "`write(|w| ..)` method takes [`ic_hs_scl_lcnt::W`](W) writer structure"]
impl crate::Writable for IcHsSclLcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_HS_SCL_LCNT to value 0x10"]
impl crate::Resettable for IcHsSclLcntSpec {
    const RESET_VALUE: u32 = 0x10;
}
