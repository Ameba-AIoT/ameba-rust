#[doc = "Register `REG_USTIME_EDCA` reader"]
pub type R = crate::R<RegUstimeEdcaSpec>;
#[doc = "Register `REG_USTIME_EDCA` writer"]
pub type W = crate::W<RegUstimeEdcaSpec>;
#[doc = "Field `BIT_USTIME_EDCA` reader - Sets the microsecond time unit used by MAC EDCA clock."]
pub type BitUstimeEdcaR = crate::FieldReader;
#[doc = "Field `BIT_USTIME_EDCA` writer - Sets the microsecond time unit used by MAC EDCA clock."]
pub type BitUstimeEdcaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_SPEC_SIFS_CCK` reader - The spec SIFS value for duration calculation"]
pub type BitSpecSifsCckR = crate::FieldReader;
#[doc = "Field `BIT_SPEC_SIFS_CCK` writer - The spec SIFS value for duration calculation"]
pub type BitSpecSifsCckW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_SPEC_SIFS_OFDM` reader - The spec SIFS value for duration calculation"]
pub type BitSpecSifsOfdmR = crate::FieldReader;
#[doc = "Field `BIT_SPEC_SIFS_OFDM` writer - The spec SIFS value for duration calculation"]
pub type BitSpecSifsOfdmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sets the microsecond time unit used by MAC EDCA clock."]
    #[inline(always)]
    pub fn bit_ustime_edca(&self) -> BitUstimeEdcaR {
        BitUstimeEdcaR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - The spec SIFS value for duration calculation"]
    #[inline(always)]
    pub fn bit_spec_sifs_cck(&self) -> BitSpecSifsCckR {
        BitSpecSifsCckR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - The spec SIFS value for duration calculation"]
    #[inline(always)]
    pub fn bit_spec_sifs_ofdm(&self) -> BitSpecSifsOfdmR {
        BitSpecSifsOfdmR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sets the microsecond time unit used by MAC EDCA clock."]
    #[inline(always)]
    pub fn bit_ustime_edca(&mut self) -> BitUstimeEdcaW<'_, RegUstimeEdcaSpec> {
        BitUstimeEdcaW::new(self, 0)
    }
    #[doc = "Bits 16:23 - The spec SIFS value for duration calculation"]
    #[inline(always)]
    pub fn bit_spec_sifs_cck(&mut self) -> BitSpecSifsCckW<'_, RegUstimeEdcaSpec> {
        BitSpecSifsCckW::new(self, 16)
    }
    #[doc = "Bits 24:31 - The spec SIFS value for duration calculation"]
    #[inline(always)]
    pub fn bit_spec_sifs_ofdm(&mut self) -> BitSpecSifsOfdmW<'_, RegUstimeEdcaSpec> {
        BitSpecSifsOfdmW::new(self, 24)
    }
}
#[doc = "US Time Tuning for EDCA Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ustime_edca::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ustime_edca::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegUstimeEdcaSpec;
impl crate::RegisterSpec for RegUstimeEdcaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ustime_edca::R`](R) reader structure"]
impl crate::Readable for RegUstimeEdcaSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ustime_edca::W`](W) writer structure"]
impl crate::Writable for RegUstimeEdcaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_USTIME_EDCA to value 0x100a_0028"]
impl crate::Resettable for RegUstimeEdcaSpec {
    const RESET_VALUE: u32 = 0x100a_0028;
}
