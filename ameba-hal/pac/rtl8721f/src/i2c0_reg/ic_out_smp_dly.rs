#[doc = "Register `IC_OUT_SMP_DLY` reader"]
pub type R = crate::R<IcOutSmpDlySpec>;
#[doc = "Register `IC_OUT_SMP_DLY` writer"]
pub type W = crate::W<IcOutSmpDlySpec>;
#[doc = "Field `IC_OUT_SMP_DLY` reader - Delay cycles for sample of master drived SCL value. The sample of master drived SCL is used for slave stretch. Slave stretch is valid when master drive SCL High, but slave drive SCL low. Valid value of IC_OUT_SMP_DLY is 0-7. Inicate a delay of 1-8 cycles."]
pub type IcOutSmpDlyR = crate::FieldReader;
#[doc = "Field `IC_OUT_SMP_DLY` writer - Delay cycles for sample of master drived SCL value. The sample of master drived SCL is used for slave stretch. Slave stretch is valid when master drive SCL High, but slave drive SCL low. Valid value of IC_OUT_SMP_DLY is 0-7. Inicate a delay of 1-8 cycles."]
pub type IcOutSmpDlyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Delay cycles for sample of master drived SCL value. The sample of master drived SCL is used for slave stretch. Slave stretch is valid when master drive SCL High, but slave drive SCL low. Valid value of IC_OUT_SMP_DLY is 0-7. Inicate a delay of 1-8 cycles."]
    #[inline(always)]
    pub fn ic_out_smp_dly(&self) -> IcOutSmpDlyR {
        IcOutSmpDlyR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Delay cycles for sample of master drived SCL value. The sample of master drived SCL is used for slave stretch. Slave stretch is valid when master drive SCL High, but slave drive SCL low. Valid value of IC_OUT_SMP_DLY is 0-7. Inicate a delay of 1-8 cycles."]
    #[inline(always)]
    pub fn ic_out_smp_dly(&mut self) -> IcOutSmpDlyW<'_, IcOutSmpDlySpec> {
        IcOutSmpDlyW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_out_smp_dly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_out_smp_dly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcOutSmpDlySpec;
impl crate::RegisterSpec for IcOutSmpDlySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_out_smp_dly::R`](R) reader structure"]
impl crate::Readable for IcOutSmpDlySpec {}
#[doc = "`write(|w| ..)` method takes [`ic_out_smp_dly::W`](W) writer structure"]
impl crate::Writable for IcOutSmpDlySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_OUT_SMP_DLY to value 0x02"]
impl crate::Resettable for IcOutSmpDlySpec {
    const RESET_VALUE: u32 = 0x02;
}
