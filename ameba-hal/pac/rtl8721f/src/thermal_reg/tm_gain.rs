#[doc = "Register `TM_GAIN` reader"]
pub type R = crate::R<TmGainSpec>;
#[doc = "Register `TM_GAIN` writer"]
pub type W = crate::W<TmGainSpec>;
#[doc = "Field `tm_a` reader - This bit defines thermal meter gain. * Bit\\[28:14\\]: Integer part * Bit\\[13:0\\]: Decimal part"]
pub type TmAR = crate::FieldReader<u32>;
#[doc = "Field `tm_a` writer - This bit defines thermal meter gain. * Bit\\[28:14\\]: Integer part * Bit\\[13:0\\]: Decimal part"]
pub type TmAW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - This bit defines thermal meter gain. * Bit\\[28:14\\]: Integer part * Bit\\[13:0\\]: Decimal part"]
    #[inline(always)]
    pub fn tm_a(&self) -> TmAR {
        TmAR::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - This bit defines thermal meter gain. * Bit\\[28:14\\]: Integer part * Bit\\[13:0\\]: Decimal part"]
    #[inline(always)]
    pub fn tm_a(&mut self) -> TmAW<'_, TmGainSpec> {
        TmAW::new(self, 0)
    }
}
#[doc = "Thermal Meter Gain Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tm_gain::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tm_gain::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmGainSpec;
impl crate::RegisterSpec for TmGainSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tm_gain::R`](R) reader structure"]
impl crate::Readable for TmGainSpec {}
#[doc = "`write(|w| ..)` method takes [`tm_gain::W`](W) writer structure"]
impl crate::Writable for TmGainSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TM_GAIN to value 0x0828_4000"]
impl crate::Resettable for TmGainSpec {
    const RESET_VALUE: u32 = 0x0828_4000;
}
