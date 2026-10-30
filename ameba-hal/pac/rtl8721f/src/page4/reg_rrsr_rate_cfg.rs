#[doc = "Register `REG_RRSR_RATE_CFG` reader"]
pub type R = crate::R<RegRrsrRateCfgSpec>;
#[doc = "Register `REG_RRSR_RATE_CFG` writer"]
pub type W = crate::W<RegRrsrRateCfgSpec>;
#[doc = "Field `BIT_RRSR_OFDM` reader - Response rate BIT\\[11:4\\] = { (msb) OFDM54, OFDM48, ...,OFDM6}"]
pub type BitRrsrOfdmR = crate::FieldReader;
#[doc = "Field `BIT_RRSR_OFDM` writer - Response rate BIT\\[11:4\\] = { (msb) OFDM54, OFDM48, ...,OFDM6}"]
pub type BitRrsrOfdmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_RRSR_HT` reader - Response rate BIT\\[19:12\\]={(msb) HT MCS7, MCS6, ..., MCS0}"]
pub type BitRrsrHtR = crate::FieldReader;
#[doc = "Field `BIT_RRSR_HT` writer - Response rate BIT\\[19:12\\]={(msb) HT MCS7, MCS6, ..., MCS0}"]
pub type BitRrsrHtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_RRSR_VHT` reader - Response rate BIT\\[27:20\\] = { (msb) VHT MCS7, MCS6, ..., MCS0}"]
pub type BitRrsrVhtR = crate::FieldReader;
#[doc = "Field `BIT_RRSR_VHT` writer - Response rate BIT\\[27:20\\] = { (msb) VHT MCS7, MCS6, ..., MCS0}"]
pub type BitRrsrVhtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_RRSR_HE` reader - Response rate BIT\\[35:28\\]={(msb) HE MCS7, MCS6, ..., MCS0}"]
pub type BitRrsrHeR = crate::FieldReader;
#[doc = "Field `BIT_RRSR_HE` writer - Response rate BIT\\[35:28\\]={(msb) HE MCS7, MCS6, ..., MCS0}"]
pub type BitRrsrHeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Response rate BIT\\[11:4\\] = { (msb) OFDM54, OFDM48, ...,OFDM6}"]
    #[inline(always)]
    pub fn bit_rrsr_ofdm(&self) -> BitRrsrOfdmR {
        BitRrsrOfdmR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Response rate BIT\\[19:12\\]={(msb) HT MCS7, MCS6, ..., MCS0}"]
    #[inline(always)]
    pub fn bit_rrsr_ht(&self) -> BitRrsrHtR {
        BitRrsrHtR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Response rate BIT\\[27:20\\] = { (msb) VHT MCS7, MCS6, ..., MCS0}"]
    #[inline(always)]
    pub fn bit_rrsr_vht(&self) -> BitRrsrVhtR {
        BitRrsrVhtR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Response rate BIT\\[35:28\\]={(msb) HE MCS7, MCS6, ..., MCS0}"]
    #[inline(always)]
    pub fn bit_rrsr_he(&self) -> BitRrsrHeR {
        BitRrsrHeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Response rate BIT\\[11:4\\] = { (msb) OFDM54, OFDM48, ...,OFDM6}"]
    #[inline(always)]
    pub fn bit_rrsr_ofdm(&mut self) -> BitRrsrOfdmW<'_, RegRrsrRateCfgSpec> {
        BitRrsrOfdmW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Response rate BIT\\[19:12\\]={(msb) HT MCS7, MCS6, ..., MCS0}"]
    #[inline(always)]
    pub fn bit_rrsr_ht(&mut self) -> BitRrsrHtW<'_, RegRrsrRateCfgSpec> {
        BitRrsrHtW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Response rate BIT\\[27:20\\] = { (msb) VHT MCS7, MCS6, ..., MCS0}"]
    #[inline(always)]
    pub fn bit_rrsr_vht(&mut self) -> BitRrsrVhtW<'_, RegRrsrRateCfgSpec> {
        BitRrsrVhtW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Response rate BIT\\[35:28\\]={(msb) HE MCS7, MCS6, ..., MCS0}"]
    #[inline(always)]
    pub fn bit_rrsr_he(&mut self) -> BitRrsrHeW<'_, RegRrsrRateCfgSpec> {
        BitRrsrHeW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rrsr_rate_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rrsr_rate_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRrsrRateCfgSpec;
impl crate::RegisterSpec for RegRrsrRateCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rrsr_rate_cfg::R`](R) reader structure"]
impl crate::Readable for RegRrsrRateCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rrsr_rate_cfg::W`](W) writer structure"]
impl crate::Writable for RegRrsrRateCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RRSR_RATE_CFG to value 0x15"]
impl crate::Resettable for RegRrsrRateCfgSpec {
    const RESET_VALUE: u32 = 0x15;
}
