#[doc = "Register `adc_1_control_1` reader"]
pub type R = crate::R<Adc1Control1Spec>;
#[doc = "Register `adc_1_control_1` writer"]
pub type W = crate::W<Adc1Control1Spec>;
#[doc = "Field `adc_1_ad_gain` reader - Channel 1 ADC digital volume -17.625dB ~ 48dB in 0.375dB step 8'h00: -17.625dB ... 8'h2f: 0dB 8'h30: 0.375dB ... 8'haf: 48dB"]
pub type Adc1AdGainR = crate::FieldReader;
#[doc = "Field `adc_1_ad_gain` writer - Channel 1 ADC digital volume -17.625dB ~ 48dB in 0.375dB step 8'h00: -17.625dB ... 8'h2f: 0dB 8'h30: 0.375dB ... 8'haf: 48dB"]
pub type Adc1AdGainW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `adc_1_boost_gain` reader - Channel 1 ADC path boost gain control 00: 0dB 01: 12dB 10: 24dB 11: 36dB"]
pub type Adc1BoostGainR = crate::FieldReader;
#[doc = "Field `adc_1_boost_gain` writer - Channel 1 ADC path boost gain control 00: 0dB 01: 12dB 10: 24dB 11: 36dB"]
pub type Adc1BoostGainW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DUMMY` reader - Dummy"]
pub type DummyR = crate::FieldReader;
#[doc = "Field `DUMMY` writer - Dummy"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc_1_rptr_hold` reader - Channel 1 I2S read point hold number rptr_hold = (I2S_fs/ad_fs) - 1"]
pub type Adc1RptrHoldR = crate::FieldReader;
#[doc = "Field `adc_1_rptr_hold` writer - Channel 1 I2S read point hold number rptr_hold = (I2S_fs/ad_fs) - 1"]
pub type Adc1RptrHoldW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc_1_fifo_keep_one` reader - Channel 1 I2S sample buffering 0: 4 samples 1: 1 sample"]
pub type Adc1FifoKeepOneR = crate::BitReader;
#[doc = "Field `adc_1_fifo_keep_one` writer - Channel 1 I2S sample buffering 0: 4 samples 1: 1 sample"]
pub type Adc1FifoKeepOneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Channel 1 ADC digital volume -17.625dB ~ 48dB in 0.375dB step 8'h00: -17.625dB ... 8'h2f: 0dB 8'h30: 0.375dB ... 8'haf: 48dB"]
    #[inline(always)]
    pub fn adc_1_ad_gain(&self) -> Adc1AdGainR {
        Adc1AdGainR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Channel 1 ADC path boost gain control 00: 0dB 01: 12dB 10: 24dB 11: 36dB"]
    #[inline(always)]
    pub fn adc_1_boost_gain(&self) -> Adc1BoostGainR {
        Adc1BoostGainR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Dummy"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Channel 1 I2S read point hold number rptr_hold = (I2S_fs/ad_fs) - 1"]
    #[inline(always)]
    pub fn adc_1_rptr_hold(&self) -> Adc1RptrHoldR {
        Adc1RptrHoldR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Channel 1 I2S sample buffering 0: 4 samples 1: 1 sample"]
    #[inline(always)]
    pub fn adc_1_fifo_keep_one(&self) -> Adc1FifoKeepOneR {
        Adc1FifoKeepOneR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel 1 ADC digital volume -17.625dB ~ 48dB in 0.375dB step 8'h00: -17.625dB ... 8'h2f: 0dB 8'h30: 0.375dB ... 8'haf: 48dB"]
    #[inline(always)]
    pub fn adc_1_ad_gain(&mut self) -> Adc1AdGainW<'_, Adc1Control1Spec> {
        Adc1AdGainW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Channel 1 ADC path boost gain control 00: 0dB 01: 12dB 10: 24dB 11: 36dB"]
    #[inline(always)]
    pub fn adc_1_boost_gain(&mut self) -> Adc1BoostGainW<'_, Adc1Control1Spec> {
        Adc1BoostGainW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Dummy"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, Adc1Control1Spec> {
        DummyW::new(self, 10)
    }
    #[doc = "Bits 12:15 - Channel 1 I2S read point hold number rptr_hold = (I2S_fs/ad_fs) - 1"]
    #[inline(always)]
    pub fn adc_1_rptr_hold(&mut self) -> Adc1RptrHoldW<'_, Adc1Control1Spec> {
        Adc1RptrHoldW::new(self, 12)
    }
    #[doc = "Bit 16 - Channel 1 I2S sample buffering 0: 4 samples 1: 1 sample"]
    #[inline(always)]
    pub fn adc_1_fifo_keep_one(&mut self) -> Adc1FifoKeepOneW<'_, Adc1Control1Spec> {
        Adc1FifoKeepOneW::new(self, 16)
    }
}
#[doc = "ADC Channel 1 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_1_control_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_1_control_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc1Control1Spec;
impl crate::RegisterSpec for Adc1Control1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_1_control_1::R`](R) reader structure"]
impl crate::Readable for Adc1Control1Spec {}
#[doc = "`write(|w| ..)` method takes [`adc_1_control_1::W`](W) writer structure"]
impl crate::Writable for Adc1Control1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_1_control_1 to value 0x2f"]
impl crate::Resettable for Adc1Control1Spec {
    const RESET_VALUE: u32 = 0x2f;
}
