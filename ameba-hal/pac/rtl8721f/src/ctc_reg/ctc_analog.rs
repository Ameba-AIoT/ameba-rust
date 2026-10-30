#[doc = "Register `CTC_ANALOG` reader"]
pub type R = crate::R<CtcAnalogSpec>;
#[doc = "Register `CTC_ANALOG` writer"]
pub type W = crate::W<CtcAnalogSpec>;
#[doc = "Field `ctc_bypass_mode_sel` reader - 0: divide_mode 1: bypass_mode"]
pub type CtcBypassModeSelR = crate::BitReader;
#[doc = "Field `ctc_bypass_mode_sel` writer - 0: divide_mode 1: bypass_mode"]
pub type CtcBypassModeSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_diode` reader - 1: enable diode"]
pub type RegDiodeR = crate::BitReader;
#[doc = "Field `reg_diode` writer - 1: enable diode"]
pub type RegDiodeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: divide_mode 1: bypass_mode"]
    #[inline(always)]
    pub fn ctc_bypass_mode_sel(&self) -> CtcBypassModeSelR {
        CtcBypassModeSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: enable diode"]
    #[inline(always)]
    pub fn reg_diode(&self) -> RegDiodeR {
        RegDiodeR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: divide_mode 1: bypass_mode"]
    #[inline(always)]
    pub fn ctc_bypass_mode_sel(&mut self) -> CtcBypassModeSelW<'_, CtcAnalogSpec> {
        CtcBypassModeSelW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: enable diode"]
    #[inline(always)]
    pub fn reg_diode(&mut self) -> RegDiodeW<'_, CtcAnalogSpec> {
        RegDiodeW::new(self, 1)
    }
}
#[doc = "Captouch BYPASS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctc_analog::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctc_analog::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtcAnalogSpec;
impl crate::RegisterSpec for CtcAnalogSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctc_analog::R`](R) reader structure"]
impl crate::Readable for CtcAnalogSpec {}
#[doc = "`write(|w| ..)` method takes [`ctc_analog::W`](W) writer structure"]
impl crate::Writable for CtcAnalogSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTC_ANALOG to value 0"]
impl crate::Resettable for CtcAnalogSpec {}
