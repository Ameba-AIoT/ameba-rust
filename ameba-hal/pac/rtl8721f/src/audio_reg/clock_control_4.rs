#[doc = "Register `clock_control_4` reader"]
pub type R = crate::R<ClockControl4Spec>;
#[doc = "Register `clock_control_4` writer"]
pub type W = crate::W<ClockControl4Spec>;
#[doc = "Field `sample_rate_0` reader - Set sample rate source 0 4'h0: 48K 4'h1: 96K 4'h2: 192K 4'h3: 32K 4'h4: 176.4K 4'h5: 16K 4'h6: Reserved 4'h7: 8K 4'h8: 44.1K 4'h9: 88.2K 4'ha:24K 4'hb: 12K 4'hc: 22.05K 4'hd: 11.025K 4'he~4'hf: Reserved"]
pub type SampleRate0R = crate::FieldReader;
#[doc = "Field `sample_rate_0` writer - Set sample rate source 0 4'h0: 48K 4'h1: 96K 4'h2: 192K 4'h3: 32K 4'h4: 176.4K 4'h5: 16K 4'h6: Reserved 4'h7: 8K 4'h8: 44.1K 4'h9: 88.2K 4'ha:24K 4'hb: 12K 4'hc: 22.05K 4'hd: 11.025K 4'he~4'hf: Reserved"]
pub type SampleRate0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `sample_rate_1` reader - Set sample rate source 1 4'h0: 48K 4'h1: 96K 4'h2: 192K 4'h3: 32K 4'h4: 176.4K 4'h5: 16K 4'h6: Reserved 4'h7: 8K 4'h8: 44.1K 4'h9: 88.2K 4'ha:24K 4'hb: 12K 4'hc: 22.05K 4'hd: 11.025K 4'he~4'hf: Reserved"]
pub type SampleRate1R = crate::FieldReader;
#[doc = "Field `sample_rate_1` writer - Set sample rate source 1 4'h0: 48K 4'h1: 96K 4'h2: 192K 4'h3: 32K 4'h4: 176.4K 4'h5: 16K 4'h6: Reserved 4'h7: 8K 4'h8: 44.1K 4'h9: 88.2K 4'ha:24K 4'hb: 12K 4'hc: 22.05K 4'hd: 11.025K 4'he~4'hf: Reserved"]
pub type SampleRate1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Set sample rate source 0 4'h0: 48K 4'h1: 96K 4'h2: 192K 4'h3: 32K 4'h4: 176.4K 4'h5: 16K 4'h6: Reserved 4'h7: 8K 4'h8: 44.1K 4'h9: 88.2K 4'ha:24K 4'hb: 12K 4'hc: 22.05K 4'hd: 11.025K 4'he~4'hf: Reserved"]
    #[inline(always)]
    pub fn sample_rate_0(&self) -> SampleRate0R {
        SampleRate0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Set sample rate source 1 4'h0: 48K 4'h1: 96K 4'h2: 192K 4'h3: 32K 4'h4: 176.4K 4'h5: 16K 4'h6: Reserved 4'h7: 8K 4'h8: 44.1K 4'h9: 88.2K 4'ha:24K 4'hb: 12K 4'hc: 22.05K 4'hd: 11.025K 4'he~4'hf: Reserved"]
    #[inline(always)]
    pub fn sample_rate_1(&self) -> SampleRate1R {
        SampleRate1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Set sample rate source 0 4'h0: 48K 4'h1: 96K 4'h2: 192K 4'h3: 32K 4'h4: 176.4K 4'h5: 16K 4'h6: Reserved 4'h7: 8K 4'h8: 44.1K 4'h9: 88.2K 4'ha:24K 4'hb: 12K 4'hc: 22.05K 4'hd: 11.025K 4'he~4'hf: Reserved"]
    #[inline(always)]
    pub fn sample_rate_0(&mut self) -> SampleRate0W<'_, ClockControl4Spec> {
        SampleRate0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Set sample rate source 1 4'h0: 48K 4'h1: 96K 4'h2: 192K 4'h3: 32K 4'h4: 176.4K 4'h5: 16K 4'h6: Reserved 4'h7: 8K 4'h8: 44.1K 4'h9: 88.2K 4'ha:24K 4'hb: 12K 4'hc: 22.05K 4'hd: 11.025K 4'he~4'hf: Reserved"]
    #[inline(always)]
    pub fn sample_rate_1(&mut self) -> SampleRate1W<'_, ClockControl4Spec> {
        SampleRate1W::new(self, 4)
    }
}
#[doc = "Clock Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_control_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_control_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockControl4Spec;
impl crate::RegisterSpec for ClockControl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock_control_4::R`](R) reader structure"]
impl crate::Readable for ClockControl4Spec {}
#[doc = "`write(|w| ..)` method takes [`clock_control_4::W`](W) writer structure"]
impl crate::Writable for ClockControl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets clock_control_4 to value 0"]
impl crate::Resettable for ClockControl4Spec {}
