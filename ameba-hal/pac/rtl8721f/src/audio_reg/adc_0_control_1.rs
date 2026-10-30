#[doc = "Register `adc_0_control_1` reader"]
pub type R = crate::R<Adc0Control1Spec>;
#[doc = "Register `adc_0_control_1` writer"]
pub type W = crate::W<Adc0Control1Spec>;
#[doc = "Field `adc_0_ad_gain` reader - Channel 0 ADC digital volume -17.625dB ~ 48dB in 0.375dB step 8'h00: -17.625dB ... 8'h2f: 0dB 8'h30: 0.375dB ... 8'haf: 48dB"]
pub type Adc0AdGainR = crate::FieldReader;
#[doc = "Field `adc_0_ad_gain` writer - Channel 0 ADC digital volume -17.625dB ~ 48dB in 0.375dB step 8'h00: -17.625dB ... 8'h2f: 0dB 8'h30: 0.375dB ... 8'haf: 48dB"]
pub type Adc0AdGainW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `adc_0_boost_gain` reader - Channel 0 ADC path boost gain control 00: 0dB 01: 12dB 10: 24dB 11: 36dB"]
pub type Adc0BoostGainR = crate::FieldReader;
#[doc = "Field `adc_0_boost_gain` writer - Channel 0 ADC path boost gain control 00: 0dB 01: 12dB 10: 24dB 11: 36dB"]
pub type Adc0BoostGainW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DUMMY` reader - Dummy"]
pub type DummyR = crate::FieldReader;
#[doc = "Field `DUMMY` writer - Dummy"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc_0_rptr_hold` reader - Channel 0 I2S read point hold number rptr_hold = (I2S_fs/ad_fs) - 1"]
pub type Adc0RptrHoldR = crate::FieldReader;
#[doc = "Field `adc_0_rptr_hold` writer - Channel 0 I2S read point hold number rptr_hold = (I2S_fs/ad_fs) - 1"]
pub type Adc0RptrHoldW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc_0_fifo_keep_one` reader - Channel 0 I2S sample buffering 0: 4 samples 1: 1 sample"]
pub type Adc0FifoKeepOneR = crate::BitReader;
#[doc = "Field `adc_0_fifo_keep_one` writer - Channel 0 I2S sample buffering 0: 4 samples 1: 1 sample"]
pub type Adc0FifoKeepOneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Channel 0 ADC digital volume -17.625dB ~ 48dB in 0.375dB step 8'h00: -17.625dB ... 8'h2f: 0dB 8'h30: 0.375dB ... 8'haf: 48dB"]
    #[inline(always)]
    pub fn adc_0_ad_gain(&self) -> Adc0AdGainR {
        Adc0AdGainR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Channel 0 ADC path boost gain control 00: 0dB 01: 12dB 10: 24dB 11: 36dB"]
    #[inline(always)]
    pub fn adc_0_boost_gain(&self) -> Adc0BoostGainR {
        Adc0BoostGainR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Dummy"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Channel 0 I2S read point hold number rptr_hold = (I2S_fs/ad_fs) - 1"]
    #[inline(always)]
    pub fn adc_0_rptr_hold(&self) -> Adc0RptrHoldR {
        Adc0RptrHoldR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Channel 0 I2S sample buffering 0: 4 samples 1: 1 sample"]
    #[inline(always)]
    pub fn adc_0_fifo_keep_one(&self) -> Adc0FifoKeepOneR {
        Adc0FifoKeepOneR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel 0 ADC digital volume -17.625dB ~ 48dB in 0.375dB step 8'h00: -17.625dB ... 8'h2f: 0dB 8'h30: 0.375dB ... 8'haf: 48dB"]
    #[inline(always)]
    pub fn adc_0_ad_gain(&mut self) -> Adc0AdGainW<'_, Adc0Control1Spec> {
        Adc0AdGainW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Channel 0 ADC path boost gain control 00: 0dB 01: 12dB 10: 24dB 11: 36dB"]
    #[inline(always)]
    pub fn adc_0_boost_gain(&mut self) -> Adc0BoostGainW<'_, Adc0Control1Spec> {
        Adc0BoostGainW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Dummy"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, Adc0Control1Spec> {
        DummyW::new(self, 10)
    }
    #[doc = "Bits 12:15 - Channel 0 I2S read point hold number rptr_hold = (I2S_fs/ad_fs) - 1"]
    #[inline(always)]
    pub fn adc_0_rptr_hold(&mut self) -> Adc0RptrHoldW<'_, Adc0Control1Spec> {
        Adc0RptrHoldW::new(self, 12)
    }
    #[doc = "Bit 16 - Channel 0 I2S sample buffering 0: 4 samples 1: 1 sample"]
    #[inline(always)]
    pub fn adc_0_fifo_keep_one(&mut self) -> Adc0FifoKeepOneW<'_, Adc0Control1Spec> {
        Adc0FifoKeepOneW::new(self, 16)
    }
}
#[doc = "ADC Channel 0 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_0_control_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_0_control_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0Control1Spec;
impl crate::RegisterSpec for Adc0Control1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_0_control_1::R`](R) reader structure"]
impl crate::Readable for Adc0Control1Spec {}
#[doc = "`write(|w| ..)` method takes [`adc_0_control_1::W`](W) writer structure"]
impl crate::Writable for Adc0Control1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_0_control_1 to value 0x2f"]
impl crate::Resettable for Adc0Control1Spec {
    const RESET_VALUE: u32 = 0x2f;
}
