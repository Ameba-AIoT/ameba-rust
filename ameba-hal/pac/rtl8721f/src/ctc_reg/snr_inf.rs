#[doc = "Register `SNR_INF` reader"]
pub type R = crate::R<SnrInfSpec>;
#[doc = "Register `SNR_INF` writer"]
pub type W = crate::W<SnrInfSpec>;
#[doc = "Field `snr_touch_data` reader - Raw data of touch(signal-baseline)for SNR monitor"]
pub type SnrTouchDataR = crate::FieldReader<u16>;
#[doc = "Field `snr_touch_data` writer - Raw data of touch(signal-baseline)for SNR monitor"]
pub type SnrTouchDataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `snr_noise_data` reader - Noise peak to peak signal raw data for SNR monitor"]
pub type SnrNoiseDataR = crate::FieldReader<u16>;
#[doc = "Field `snr_noise_data` writer - Noise peak to peak signal raw data for SNR monitor"]
pub type SnrNoiseDataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `snr_upd_always` reader - 1: Update SNR info no matter touch or not (used only in debug function)"]
pub type SnrUpdAlwaysR = crate::BitReader;
#[doc = "Field `snr_upd_always` writer - 1: Update SNR info no matter touch or not (used only in debug function)"]
pub type SnrUpdAlwaysW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Raw data of touch(signal-baseline)for SNR monitor"]
    #[inline(always)]
    pub fn snr_touch_data(&self) -> SnrTouchDataR {
        SnrTouchDataR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Noise peak to peak signal raw data for SNR monitor"]
    #[inline(always)]
    pub fn snr_noise_data(&self) -> SnrNoiseDataR {
        SnrNoiseDataR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - 1: Update SNR info no matter touch or not (used only in debug function)"]
    #[inline(always)]
    pub fn snr_upd_always(&self) -> SnrUpdAlwaysR {
        SnrUpdAlwaysR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Raw data of touch(signal-baseline)for SNR monitor"]
    #[inline(always)]
    pub fn snr_touch_data(&mut self) -> SnrTouchDataW<'_, SnrInfSpec> {
        SnrTouchDataW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Noise peak to peak signal raw data for SNR monitor"]
    #[inline(always)]
    pub fn snr_noise_data(&mut self) -> SnrNoiseDataW<'_, SnrInfSpec> {
        SnrNoiseDataW::new(self, 16)
    }
    #[doc = "Bit 31 - 1: Update SNR info no matter touch or not (used only in debug function)"]
    #[inline(always)]
    pub fn snr_upd_always(&mut self) -> SnrUpdAlwaysW<'_, SnrInfSpec> {
        SnrUpdAlwaysW::new(self, 31)
    }
}
#[doc = "Signal Noise Ratio Register\n\nYou can [`read`](crate::Reg::read) this register and get [`snr_inf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snr_inf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SnrInfSpec;
impl crate::RegisterSpec for SnrInfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`snr_inf::R`](R) reader structure"]
impl crate::Readable for SnrInfSpec {}
#[doc = "`write(|w| ..)` method takes [`snr_inf::W`](W) writer structure"]
impl crate::Writable for SnrInfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SNR_INF to value 0"]
impl crate::Resettable for SnrInfSpec {}
