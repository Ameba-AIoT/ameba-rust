#[doc = "Register `adc_align_control_reg_0` reader"]
pub type R = crate::R<AdcAlignControlReg0Spec>;
#[doc = "Register `adc_align_control_reg_0` writer"]
pub type W = crate::W<AdcAlignControlReg0Spec>;
#[doc = "Field `sp_ad_fifo_align_en` reader - 1: Uplink FIFO will align with I2S 0: Disable"]
pub type SpAdFifoAlignEnR = crate::BitReader;
#[doc = "Field `sp_ad_fifo_align_en` writer - 1: Uplink FIFO will align with I2S 0: Disable"]
pub type SpAdFifoAlignEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_ad_align_en` reader - 1: Uplink channel will align with I2S 0: Disable"]
pub type SpAdAlignEnR = crate::BitReader;
#[doc = "Field `sp_ad_align_en` writer - 1: Uplink channel will align with I2S 0: Disable"]
pub type SpAdAlignEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: Uplink FIFO will align with I2S 0: Disable"]
    #[inline(always)]
    pub fn sp_ad_fifo_align_en(&self) -> SpAdFifoAlignEnR {
        SpAdFifoAlignEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Uplink channel will align with I2S 0: Disable"]
    #[inline(always)]
    pub fn sp_ad_align_en(&self) -> SpAdAlignEnR {
        SpAdAlignEnR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Uplink FIFO will align with I2S 0: Disable"]
    #[inline(always)]
    pub fn sp_ad_fifo_align_en(&mut self) -> SpAdFifoAlignEnW<'_, AdcAlignControlReg0Spec> {
        SpAdFifoAlignEnW::new(self, 0)
    }
    #[doc = "Bit 2 - 1: Uplink channel will align with I2S 0: Disable"]
    #[inline(always)]
    pub fn sp_ad_align_en(&mut self) -> SpAdAlignEnW<'_, AdcAlignControlReg0Spec> {
        SpAdAlignEnW::new(self, 2)
    }
}
#[doc = "ADC Align Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_align_control_reg_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_align_control_reg_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcAlignControlReg0Spec;
impl crate::RegisterSpec for AdcAlignControlReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_align_control_reg_0::R`](R) reader structure"]
impl crate::Readable for AdcAlignControlReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`adc_align_control_reg_0::W`](W) writer structure"]
impl crate::Writable for AdcAlignControlReg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_align_control_reg_0 to value 0"]
impl crate::Resettable for AdcAlignControlReg0Spec {}
