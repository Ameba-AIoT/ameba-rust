#[doc = "Register `XTAL_LPS_FREQ_LPS` reader"]
pub type R = crate::R<XtalLpsFreqLpsSpec>;
#[doc = "Register `XTAL_LPS_FREQ_LPS` writer"]
pub type W = crate::W<XtalLpsFreqLpsSpec>;
#[doc = "Field `xtal_clk_freq_lps` reader - xtal 312.5K freq in lps mode , cap maybe disable for power save , maximumly , freq will be 312.5K + 250ppm"]
pub type XtalClkFreqLpsR = crate::FieldReader<u32>;
#[doc = "Field `xtal_clk_freq_lps` writer - xtal 312.5K freq in lps mode , cap maybe disable for power save , maximumly , freq will be 312.5K + 250ppm"]
pub type XtalClkFreqLpsW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:18 - xtal 312.5K freq in lps mode , cap maybe disable for power save , maximumly , freq will be 312.5K + 250ppm"]
    #[inline(always)]
    pub fn xtal_clk_freq_lps(&self) -> XtalClkFreqLpsR {
        XtalClkFreqLpsR::new(self.bits & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:18 - xtal 312.5K freq in lps mode , cap maybe disable for power save , maximumly , freq will be 312.5K + 250ppm"]
    #[inline(always)]
    pub fn xtal_clk_freq_lps(&mut self) -> XtalClkFreqLpsW<'_, XtalLpsFreqLpsSpec> {
        XtalClkFreqLpsW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal_lps_freq_lps::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal_lps_freq_lps::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtalLpsFreqLpsSpec;
impl crate::RegisterSpec for XtalLpsFreqLpsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtal_lps_freq_lps::R`](R) reader structure"]
impl crate::Readable for XtalLpsFreqLpsSpec {}
#[doc = "`write(|w| ..)` method takes [`xtal_lps_freq_lps::W`](W) writer structure"]
impl crate::Writable for XtalLpsFreqLpsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTAL_LPS_FREQ_LPS to value 0x0004_c4b4"]
impl crate::Resettable for XtalLpsFreqLpsSpec {
    const RESET_VALUE: u32 = 0x0004_c4b4;
}
