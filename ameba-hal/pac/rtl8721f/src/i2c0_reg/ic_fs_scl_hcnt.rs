#[doc = "Register `IC_FS_SCL_HCNT` reader"]
pub type R = crate::R<IcFsSclHcntSpec>;
#[doc = "Register `IC_FS_SCL_HCNT` writer"]
pub type W = crate::W<IcFsSclHcntSpec>;
#[doc = "Field `IC_FS_SCL_HCNT` reader - This register sets the SCL clock high-period count for fast speed. This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect."]
pub type IcFsSclHcntR = crate::FieldReader<u16>;
#[doc = "Field `IC_FS_SCL_HCNT` writer - This register sets the SCL clock high-period count for fast speed. This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect."]
pub type IcFsSclHcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register sets the SCL clock high-period count for fast speed. This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect."]
    #[inline(always)]
    pub fn ic_fs_scl_hcnt(&self) -> IcFsSclHcntR {
        IcFsSclHcntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register sets the SCL clock high-period count for fast speed. This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect."]
    #[inline(always)]
    pub fn ic_fs_scl_hcnt(&mut self) -> IcFsSclHcntW<'_, IcFsSclHcntSpec> {
        IcFsSclHcntW::new(self, 0)
    }
}
#[doc = "Fast Speed I2C Clock SCL High Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_fs_scl_hcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_fs_scl_hcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcFsSclHcntSpec;
impl crate::RegisterSpec for IcFsSclHcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_fs_scl_hcnt::R`](R) reader structure"]
impl crate::Readable for IcFsSclHcntSpec {}
#[doc = "`write(|w| ..)` method takes [`ic_fs_scl_hcnt::W`](W) writer structure"]
impl crate::Writable for IcFsSclHcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_FS_SCL_HCNT to value 0x3c"]
impl crate::Resettable for IcFsSclHcntSpec {
    const RESET_VALUE: u32 = 0x3c;
}
