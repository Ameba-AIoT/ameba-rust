#[doc = "Register `XTAL_LPS_FREQ_NM` reader"]
pub type R = crate::R<XtalLpsFreqNmSpec>;
#[doc = "Register `XTAL_LPS_FREQ_NM` writer"]
pub type W = crate::W<XtalLpsFreqNmSpec>;
#[doc = "Field `xtal_clk_freq_nm` reader - xtal 312.5K freq in normal mode , all cap are valid , it is exactly 312.5K."]
pub type XtalClkFreqNmR = crate::FieldReader<u32>;
#[doc = "Field `xtal_clk_freq_nm` writer - xtal 312.5K freq in normal mode , all cap are valid , it is exactly 312.5K."]
pub type XtalClkFreqNmW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:18 - xtal 312.5K freq in normal mode , all cap are valid , it is exactly 312.5K."]
    #[inline(always)]
    pub fn xtal_clk_freq_nm(&self) -> XtalClkFreqNmR {
        XtalClkFreqNmR::new(self.bits & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:18 - xtal 312.5K freq in normal mode , all cap are valid , it is exactly 312.5K."]
    #[inline(always)]
    pub fn xtal_clk_freq_nm(&mut self) -> XtalClkFreqNmW<'_, XtalLpsFreqNmSpec> {
        XtalClkFreqNmW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal_lps_freq_nm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal_lps_freq_nm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtalLpsFreqNmSpec;
impl crate::RegisterSpec for XtalLpsFreqNmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtal_lps_freq_nm::R`](R) reader structure"]
impl crate::Readable for XtalLpsFreqNmSpec {}
#[doc = "`write(|w| ..)` method takes [`xtal_lps_freq_nm::W`](W) writer structure"]
impl crate::Writable for XtalLpsFreqNmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTAL_LPS_FREQ_NM to value 0x0004_c4b4"]
impl crate::Resettable for XtalLpsFreqNmSpec {
    const RESET_VALUE: u32 = 0x0004_c4b4;
}
