#[doc = "Register `clock_control_1` reader"]
pub type R = crate::R<ClockControl1Spec>;
#[doc = "Register `clock_control_1` writer"]
pub type W = crate::W<ClockControl1Spec>;
#[doc = "Field `ad_0_en` reader - ADC channel 0 clock enable"]
pub type Ad0EnR = crate::BitReader;
#[doc = "Field `ad_0_en` writer - ADC channel 0 clock enable"]
pub type Ad0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ad_1_en` reader - ADC channel 1 clock enable"]
pub type Ad1EnR = crate::BitReader;
#[doc = "Field `ad_1_en` writer - ADC channel 1 clock enable"]
pub type Ad1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ad_0_fifo_en` reader - ADC channel 0 FIFO clock enable"]
pub type Ad0FifoEnR = crate::BitReader;
#[doc = "Field `ad_0_fifo_en` writer - ADC channel 0 FIFO clock enable"]
pub type Ad0FifoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ad_1_fifo_en` reader - ADC channel 1 FIFO clock enable"]
pub type Ad1FifoEnR = crate::BitReader;
#[doc = "Field `ad_1_fifo_en` writer - ADC channel 1 FIFO clock enable"]
pub type Ad1FifoEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - ADC channel 0 clock enable"]
    #[inline(always)]
    pub fn ad_0_en(&self) -> Ad0EnR {
        Ad0EnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC channel 1 clock enable"]
    #[inline(always)]
    pub fn ad_1_en(&self) -> Ad1EnR {
        Ad1EnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC channel 0 FIFO clock enable"]
    #[inline(always)]
    pub fn ad_0_fifo_en(&self) -> Ad0FifoEnR {
        Ad0FifoEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC channel 1 FIFO clock enable"]
    #[inline(always)]
    pub fn ad_1_fifo_en(&self) -> Ad1FifoEnR {
        Ad1FifoEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - ADC channel 0 clock enable"]
    #[inline(always)]
    pub fn ad_0_en(&mut self) -> Ad0EnW<'_, ClockControl1Spec> {
        Ad0EnW::new(self, 6)
    }
    #[doc = "Bit 7 - ADC channel 1 clock enable"]
    #[inline(always)]
    pub fn ad_1_en(&mut self) -> Ad1EnW<'_, ClockControl1Spec> {
        Ad1EnW::new(self, 7)
    }
    #[doc = "Bit 14 - ADC channel 0 FIFO clock enable"]
    #[inline(always)]
    pub fn ad_0_fifo_en(&mut self) -> Ad0FifoEnW<'_, ClockControl1Spec> {
        Ad0FifoEnW::new(self, 14)
    }
    #[doc = "Bit 15 - ADC channel 1 FIFO clock enable"]
    #[inline(always)]
    pub fn ad_1_fifo_en(&mut self) -> Ad1FifoEnW<'_, ClockControl1Spec> {
        Ad1FifoEnW::new(self, 15)
    }
}
#[doc = "Clock Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_control_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_control_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockControl1Spec;
impl crate::RegisterSpec for ClockControl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock_control_1::R`](R) reader structure"]
impl crate::Readable for ClockControl1Spec {}
#[doc = "`write(|w| ..)` method takes [`clock_control_1::W`](W) writer structure"]
impl crate::Writable for ClockControl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets clock_control_1 to value 0"]
impl crate::Resettable for ClockControl1Spec {}
