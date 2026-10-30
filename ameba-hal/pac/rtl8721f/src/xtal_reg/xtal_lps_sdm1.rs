#[doc = "Register `XTAL_LPS_SDM1` reader"]
pub type R = crate::R<XtalLpsSdm1Spec>;
#[doc = "Register `XTAL_LPS_SDM1` writer"]
pub type W = crate::W<XtalLpsSdm1Spec>;
#[doc = "Field `xtal_sdm1_freq` reader - xtal lps sdm1 output frequency, unit is Hz. Note : sdm1 used for system and wifi, default is 32000Hz."]
pub type XtalSdm1FreqR = crate::FieldReader<u16>;
#[doc = "Field `xtal_sdm1_freq` writer - xtal lps sdm1 output frequency, unit is Hz. Note : sdm1 used for system and wifi, default is 32000Hz."]
pub type XtalSdm1FreqW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `xtal_sdm1_en` reader - xtal lps sdm1 output enable Note : sdm1 used for bt, default is 32000Hz."]
pub type XtalSdm1EnR = crate::BitReader;
#[doc = "Field `xtal_sdm1_en` writer - xtal lps sdm1 output enable Note : sdm1 used for bt, default is 32000Hz."]
pub type XtalSdm1EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - xtal lps sdm1 output frequency, unit is Hz. Note : sdm1 used for system and wifi, default is 32000Hz."]
    #[inline(always)]
    pub fn xtal_sdm1_freq(&self) -> XtalSdm1FreqR {
        XtalSdm1FreqR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - xtal lps sdm1 output enable Note : sdm1 used for bt, default is 32000Hz."]
    #[inline(always)]
    pub fn xtal_sdm1_en(&self) -> XtalSdm1EnR {
        XtalSdm1EnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - xtal lps sdm1 output frequency, unit is Hz. Note : sdm1 used for system and wifi, default is 32000Hz."]
    #[inline(always)]
    pub fn xtal_sdm1_freq(&mut self) -> XtalSdm1FreqW<'_, XtalLpsSdm1Spec> {
        XtalSdm1FreqW::new(self, 0)
    }
    #[doc = "Bit 16 - xtal lps sdm1 output enable Note : sdm1 used for bt, default is 32000Hz."]
    #[inline(always)]
    pub fn xtal_sdm1_en(&mut self) -> XtalSdm1EnW<'_, XtalLpsSdm1Spec> {
        XtalSdm1EnW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal_lps_sdm1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal_lps_sdm1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtalLpsSdm1Spec;
impl crate::RegisterSpec for XtalLpsSdm1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtal_lps_sdm1::R`](R) reader structure"]
impl crate::Readable for XtalLpsSdm1Spec {}
#[doc = "`write(|w| ..)` method takes [`xtal_lps_sdm1::W`](W) writer structure"]
impl crate::Writable for XtalLpsSdm1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTAL_LPS_SDM1 to value 0x0001_7d00"]
impl crate::Resettable for XtalLpsSdm1Spec {
    const RESET_VALUE: u32 = 0x0001_7d00;
}
