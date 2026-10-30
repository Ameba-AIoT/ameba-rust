#[doc = "Register `ADC_TEST_MUX` reader"]
pub type R = crate::R<AdcTestMuxSpec>;
#[doc = "Register `ADC_TEST_MUX` writer"]
pub type W = crate::W<AdcTestMuxSpec>;
#[doc = "Field `lp_adc_ch_num` reader - sw select adc test channel"]
pub type LpAdcChNumR = crate::FieldReader;
#[doc = "Field `lp_adc_ch_num` writer - sw select adc test channel"]
pub type LpAdcChNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `lp_adc_mode` reader - sw_select adc mode"]
pub type LpAdcModeR = crate::FieldReader;
#[doc = "Field `lp_adc_mode` writer - sw_select adc mode"]
pub type LpAdcModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lp_adc_force` reader - force sw select instead of pinmux input when adc test mode 1:using \\[5:0\\] for adc channel / mode select 0:using io input for adc channel / mode"]
pub type LpAdcForceR = crate::BitReader;
#[doc = "Field `lp_adc_force` writer - force sw select instead of pinmux input when adc test mode 1:using \\[5:0\\] for adc channel / mode select 0:using io input for adc channel / mode"]
pub type LpAdcForceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - sw select adc test channel"]
    #[inline(always)]
    pub fn lp_adc_ch_num(&self) -> LpAdcChNumR {
        LpAdcChNumR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - sw_select adc mode"]
    #[inline(always)]
    pub fn lp_adc_mode(&self) -> LpAdcModeR {
        LpAdcModeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - force sw select instead of pinmux input when adc test mode 1:using \\[5:0\\] for adc channel / mode select 0:using io input for adc channel / mode"]
    #[inline(always)]
    pub fn lp_adc_force(&self) -> LpAdcForceR {
        LpAdcForceR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - sw select adc test channel"]
    #[inline(always)]
    pub fn lp_adc_ch_num(&mut self) -> LpAdcChNumW<'_, AdcTestMuxSpec> {
        LpAdcChNumW::new(self, 0)
    }
    #[doc = "Bits 4:5 - sw_select adc mode"]
    #[inline(always)]
    pub fn lp_adc_mode(&mut self) -> LpAdcModeW<'_, AdcTestMuxSpec> {
        LpAdcModeW::new(self, 4)
    }
    #[doc = "Bit 8 - force sw select instead of pinmux input when adc test mode 1:using \\[5:0\\] for adc channel / mode select 0:using io input for adc channel / mode"]
    #[inline(always)]
    pub fn lp_adc_force(&mut self) -> LpAdcForceW<'_, AdcTestMuxSpec> {
        LpAdcForceW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_test_mux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_test_mux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcTestMuxSpec;
impl crate::RegisterSpec for AdcTestMuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_test_mux::R`](R) reader structure"]
impl crate::Readable for AdcTestMuxSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_test_mux::W`](W) writer structure"]
impl crate::Writable for AdcTestMuxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC_TEST_MUX to value 0"]
impl crate::Resettable for AdcTestMuxSpec {}
