#[doc = "Register `EEEP_CR2` reader"]
pub type R = crate::R<EeepCr2Spec>;
#[doc = "Register `EEEP_CR2` writer"]
pub type W = crate::W<EeepCr2Spec>;
#[doc = "Field `EEEP_Sleep_Step` reader - Value from 1 to 255, indicate the rx sleep step in EEEP function. Value 0 is the same as 1. EEPROM and register configurable.//???"]
pub type EeepSleepStepR = crate::FieldReader;
#[doc = "Field `EEEP_Sleep_Step` writer - Value from 1 to 255, indicate the rx sleep step in EEEP function. Value 0 is the same as 1. EEPROM and register configurable.//???"]
pub type EeepSleepStepW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Value from 1 to 255, indicate the rx sleep step in EEEP function. Value 0 is the same as 1. EEPROM and register configurable.//???"]
    #[inline(always)]
    pub fn eeep_sleep_step(&self) -> EeepSleepStepR {
        EeepSleepStepR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Value from 1 to 255, indicate the rx sleep step in EEEP function. Value 0 is the same as 1. EEPROM and register configurable.//???"]
    #[inline(always)]
    pub fn eeep_sleep_step(&mut self) -> EeepSleepStepW<'_, EeepCr2Spec> {
        EeepSleepStepW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`eeep_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eeep_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EeepCr2Spec;
impl crate::RegisterSpec for EeepCr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eeep_cr2::R`](R) reader structure"]
impl crate::Readable for EeepCr2Spec {}
#[doc = "`write(|w| ..)` method takes [`eeep_cr2::W`](W) writer structure"]
impl crate::Writable for EeepCr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EEEP_CR2 to value 0x01"]
impl crate::Resettable for EeepCr2Spec {
    const RESET_VALUE: u32 = 0x01;
}
