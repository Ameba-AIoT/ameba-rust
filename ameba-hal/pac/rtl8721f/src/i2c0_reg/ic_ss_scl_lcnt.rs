#[doc = "Register `IC_SS_SCL_LCNT` reader"]
pub type R = crate::R<IcSsSclLcntSpec>;
#[doc = "Register `IC_SS_SCL_LCNT` writer"]
pub type W = crate::W<IcSsSclLcntSpec>;
#[doc = "Field `IC_SS_SCL_LCNT` reader - This register sets the SCL clock low-period count for standard speed. This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect."]
pub type IcSsSclLcntR = crate::FieldReader<u16>;
#[doc = "Field `IC_SS_SCL_LCNT` writer - This register sets the SCL clock low-period count for standard speed. This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect."]
pub type IcSsSclLcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register sets the SCL clock low-period count for standard speed. This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect."]
    #[inline(always)]
    pub fn ic_ss_scl_lcnt(&self) -> IcSsSclLcntR {
        IcSsSclLcntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register sets the SCL clock low-period count for standard speed. This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect."]
    #[inline(always)]
    pub fn ic_ss_scl_lcnt(&mut self) -> IcSsSclLcntW<'_, IcSsSclLcntSpec> {
        IcSsSclLcntW::new(self, 0)
    }
}
#[doc = "Standard Speed I2C Clock SCL Low Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_ss_scl_lcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_ss_scl_lcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcSsSclLcntSpec;
impl crate::RegisterSpec for IcSsSclLcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_ss_scl_lcnt::R`](R) reader structure"]
impl crate::Readable for IcSsSclLcntSpec {}
#[doc = "`write(|w| ..)` method takes [`ic_ss_scl_lcnt::W`](W) writer structure"]
impl crate::Writable for IcSsSclLcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_SS_SCL_LCNT to value 0x01d6"]
impl crate::Resettable for IcSsSclLcntSpec {
    const RESET_VALUE: u32 = 0x01d6;
}
