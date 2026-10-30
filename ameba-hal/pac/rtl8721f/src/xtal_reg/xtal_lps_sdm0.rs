#[doc = "Register `XTAL_LPS_SDM0` reader"]
pub type R = crate::R<XtalLpsSdm0Spec>;
#[doc = "Register `XTAL_LPS_SDM0` writer"]
pub type W = crate::W<XtalLpsSdm0Spec>;
#[doc = "Field `xtal_sdm0_freq` reader - xtal lps sdm0 output frequency, unit is Hz. Note : sdm0 used for system and wifi, default is 32768Hz."]
pub type XtalSdm0FreqR = crate::FieldReader<u16>;
#[doc = "Field `xtal_sdm0_freq` writer - xtal lps sdm0 output frequency, unit is Hz. Note : sdm0 used for system and wifi, default is 32768Hz."]
pub type XtalSdm0FreqW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `xtal_sdm0_en` reader - xtal lps sdm0 output enable Note : sdm0 used for system and wifi, default is 32768Hz."]
pub type XtalSdm0EnR = crate::BitReader;
#[doc = "Field `xtal_sdm0_en` writer - xtal lps sdm0 output enable Note : sdm0 used for system and wifi, default is 32768Hz."]
pub type XtalSdm0EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - xtal lps sdm0 output frequency, unit is Hz. Note : sdm0 used for system and wifi, default is 32768Hz."]
    #[inline(always)]
    pub fn xtal_sdm0_freq(&self) -> XtalSdm0FreqR {
        XtalSdm0FreqR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - xtal lps sdm0 output enable Note : sdm0 used for system and wifi, default is 32768Hz."]
    #[inline(always)]
    pub fn xtal_sdm0_en(&self) -> XtalSdm0EnR {
        XtalSdm0EnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - xtal lps sdm0 output frequency, unit is Hz. Note : sdm0 used for system and wifi, default is 32768Hz."]
    #[inline(always)]
    pub fn xtal_sdm0_freq(&mut self) -> XtalSdm0FreqW<'_, XtalLpsSdm0Spec> {
        XtalSdm0FreqW::new(self, 0)
    }
    #[doc = "Bit 16 - xtal lps sdm0 output enable Note : sdm0 used for system and wifi, default is 32768Hz."]
    #[inline(always)]
    pub fn xtal_sdm0_en(&mut self) -> XtalSdm0EnW<'_, XtalLpsSdm0Spec> {
        XtalSdm0EnW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal_lps_sdm0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal_lps_sdm0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtalLpsSdm0Spec;
impl crate::RegisterSpec for XtalLpsSdm0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtal_lps_sdm0::R`](R) reader structure"]
impl crate::Readable for XtalLpsSdm0Spec {}
#[doc = "`write(|w| ..)` method takes [`xtal_lps_sdm0::W`](W) writer structure"]
impl crate::Writable for XtalLpsSdm0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTAL_LPS_SDM0 to value 0x0001_8000"]
impl crate::Resettable for XtalLpsSdm0Spec {
    const RESET_VALUE: u32 = 0x0001_8000;
}
