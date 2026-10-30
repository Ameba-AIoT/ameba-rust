#[doc = "Register `audio_control_1` reader"]
pub type R = crate::R<AudioControl1Spec>;
#[doc = "Register `audio_control_1` writer"]
pub type W = crate::W<AudioControl1Spec>;
#[doc = "Field `i2s_data_rnd_en` reader - i2s_data_rnd_en"]
pub type I2sDataRndEnR = crate::BitReader;
#[doc = "Field `i2s_data_rnd_en` writer - i2s_data_rnd_en"]
pub type I2sDataRndEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `audio_control_1_dummy` reader - Dummy"]
pub type AudioControl1DummyR = crate::FieldReader;
#[doc = "Field `audio_control_1_dummy` writer - Dummy"]
pub type AudioControl1DummyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 12 - i2s_data_rnd_en"]
    #[inline(always)]
    pub fn i2s_data_rnd_en(&self) -> I2sDataRndEnR {
        I2sDataRndEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - Dummy"]
    #[inline(always)]
    pub fn audio_control_1_dummy(&self) -> AudioControl1DummyR {
        AudioControl1DummyR::new(((self.bits >> 13) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 12 - i2s_data_rnd_en"]
    #[inline(always)]
    pub fn i2s_data_rnd_en(&mut self) -> I2sDataRndEnW<'_, AudioControl1Spec> {
        I2sDataRndEnW::new(self, 12)
    }
    #[doc = "Bits 13:16 - Dummy"]
    #[inline(always)]
    pub fn audio_control_1_dummy(&mut self) -> AudioControl1DummyW<'_, AudioControl1Spec> {
        AudioControl1DummyW::new(self, 13)
    }
}
#[doc = "Audio Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_control_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_control_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudioControl1Spec;
impl crate::RegisterSpec for AudioControl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_control_1::R`](R) reader structure"]
impl crate::Readable for AudioControl1Spec {}
#[doc = "`write(|w| ..)` method takes [`audio_control_1::W`](W) writer structure"]
impl crate::Writable for AudioControl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets audio_control_1 to value 0x1000"]
impl crate::Resettable for AudioControl1Spec {
    const RESET_VALUE: u32 = 0x1000;
}
