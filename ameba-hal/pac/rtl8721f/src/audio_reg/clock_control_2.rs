#[doc = "Register `clock_control_2` reader"]
pub type R = crate::R<ClockControl2Spec>;
#[doc = "Register `clock_control_2` writer"]
pub type W = crate::W<ClockControl2Spec>;
#[doc = "Field `dmic_0_en` reader - ADC filter channel 0 clock enable: DMIC path"]
pub type Dmic0EnR = crate::BitReader;
#[doc = "Field `dmic_0_en` writer - ADC filter channel 0 clock enable: DMIC path"]
pub type Dmic0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dmic_1_en` reader - ADC filter channel 1 clock enable: DMIC path"]
pub type Dmic1EnR = crate::BitReader;
#[doc = "Field `dmic_1_en` writer - ADC filter channel 1 clock enable: DMIC path"]
pub type Dmic1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ad_0_eq_en` reader - ADC channel 0 EQ clock enable"]
pub type Ad0EqEnR = crate::BitReader;
#[doc = "Field `ad_0_eq_en` writer - ADC channel 0 EQ clock enable"]
pub type Ad0EqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ad_1_eq_en` reader - ADC channel 1 EQ clock enable"]
pub type Ad1EqEnR = crate::BitReader;
#[doc = "Field `ad_1_eq_en` writer - ADC channel 1 EQ clock enable"]
pub type Ad1EqEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - ADC filter channel 0 clock enable: DMIC path"]
    #[inline(always)]
    pub fn dmic_0_en(&self) -> Dmic0EnR {
        Dmic0EnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC filter channel 1 clock enable: DMIC path"]
    #[inline(always)]
    pub fn dmic_1_en(&self) -> Dmic1EnR {
        Dmic1EnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC channel 0 EQ clock enable"]
    #[inline(always)]
    pub fn ad_0_eq_en(&self) -> Ad0EqEnR {
        Ad0EqEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC channel 1 EQ clock enable"]
    #[inline(always)]
    pub fn ad_1_eq_en(&self) -> Ad1EqEnR {
        Ad1EqEnR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - ADC filter channel 0 clock enable: DMIC path"]
    #[inline(always)]
    pub fn dmic_0_en(&mut self) -> Dmic0EnW<'_, ClockControl2Spec> {
        Dmic0EnW::new(self, 8)
    }
    #[doc = "Bit 9 - ADC filter channel 1 clock enable: DMIC path"]
    #[inline(always)]
    pub fn dmic_1_en(&mut self) -> Dmic1EnW<'_, ClockControl2Spec> {
        Dmic1EnW::new(self, 9)
    }
    #[doc = "Bit 16 - ADC channel 0 EQ clock enable"]
    #[inline(always)]
    pub fn ad_0_eq_en(&mut self) -> Ad0EqEnW<'_, ClockControl2Spec> {
        Ad0EqEnW::new(self, 16)
    }
    #[doc = "Bit 17 - ADC channel 1 EQ clock enable"]
    #[inline(always)]
    pub fn ad_1_eq_en(&mut self) -> Ad1EqEnW<'_, ClockControl2Spec> {
        Ad1EqEnW::new(self, 17)
    }
}
#[doc = "Clock Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_control_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_control_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockControl2Spec;
impl crate::RegisterSpec for ClockControl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock_control_2::R`](R) reader structure"]
impl crate::Readable for ClockControl2Spec {}
#[doc = "`write(|w| ..)` method takes [`clock_control_2::W`](W) writer structure"]
impl crate::Writable for ClockControl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets clock_control_2 to value 0"]
impl crate::Resettable for ClockControl2Spec {}
