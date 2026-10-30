#[doc = "Register `REG_AUDIO_LAT_TSFT` reader"]
pub type R = crate::R<RegAudioLatTsftSpec>;
#[doc = "Field `BIT_AUDIO_LAT_TSFT_NS` reader - audio latch tsft_ns"]
pub type BitAudioLatTsftNsR = crate::FieldReader<u16>;
#[doc = "Field `BIT_AUDIO_LAT_TSFT_US` reader - low 22bits of audio latch tsft_us"]
pub type BitAudioLatTsftUsR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - audio latch tsft_ns"]
    #[inline(always)]
    pub fn bit_audio_lat_tsft_ns(&self) -> BitAudioLatTsftNsR {
        BitAudioLatTsftNsR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 20:31 - low 22bits of audio latch tsft_us"]
    #[inline(always)]
    pub fn bit_audio_lat_tsft_us(&self) -> BitAudioLatTsftUsR {
        BitAudioLatTsftUsR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_audio_lat_tsft::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAudioLatTsftSpec;
impl crate::RegisterSpec for RegAudioLatTsftSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_audio_lat_tsft::R`](R) reader structure"]
impl crate::Readable for RegAudioLatTsftSpec {}
#[doc = "`reset()` method sets REG_AUDIO_LAT_TSFT to value 0"]
impl crate::Resettable for RegAudioLatTsftSpec {}
