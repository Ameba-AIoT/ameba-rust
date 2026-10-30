#[doc = "Register `REG_CLK` reader"]
pub type R = crate::R<RegClkSpec>;
#[doc = "Register `REG_CLK` writer"]
pub type W = crate::W<RegClkSpec>;
#[doc = "Field `cke_aon` reader - if 0, aon reg(except this bit) , swr/ldo reg will be gated. Note: AON clock can't be shutdown as analog need."]
pub type CkeAonR = crate::BitReader;
#[doc = "Field `cke_aon` writer - if 0, aon reg(except this bit) , swr/ldo reg will be gated. Note: AON clock can't be shutdown as analog need."]
pub type CkeAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_atim` reader - 1: Enable AON timer clock 0: Disable"]
pub type CkeAtimR = crate::BitReader;
#[doc = "Field `cke_atim` writer - 1: Enable AON timer clock 0: Disable"]
pub type CkeAtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_rtc` reader - 1: Enable RTC clock 0: Disable"]
pub type CkeRtcR = crate::BitReader;
#[doc = "Field `cke_rtc` writer - 1: Enable RTC clock 0: Disable"]
pub type CkeRtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_sdm` reader - 1: Enable 32.768K SDM module's clock 0: Disable"]
pub type CkeSdmR = crate::BitReader;
#[doc = "Field `cke_sdm` writer - 1: Enable 32.768K SDM module's clock 0: Disable"]
pub type CkeSdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_iwdg` reader - 1: Enable iwdg bus clock 0: Disable"]
pub type CkeIwdgR = crate::BitReader;
#[doc = "Field `cke_iwdg` writer - 1: Enable iwdg bus clock 0: Disable"]
pub type CkeIwdgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_ledc` reader - 1: Enable ledc module's clock 0: Disable"]
pub type CkeLedcR = crate::BitReader;
#[doc = "Field `cke_ledc` writer - 1: Enable ledc module's clock 0: Disable"]
pub type CkeLedcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - if 0, aon reg(except this bit) , swr/ldo reg will be gated. Note: AON clock can't be shutdown as analog need."]
    #[inline(always)]
    pub fn cke_aon(&self) -> CkeAonR {
        CkeAonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Enable AON timer clock 0: Disable"]
    #[inline(always)]
    pub fn cke_atim(&self) -> CkeAtimR {
        CkeAtimR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Enable RTC clock 0: Disable"]
    #[inline(always)]
    pub fn cke_rtc(&self) -> CkeRtcR {
        CkeRtcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Enable 32.768K SDM module's clock 0: Disable"]
    #[inline(always)]
    pub fn cke_sdm(&self) -> CkeSdmR {
        CkeSdmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Enable iwdg bus clock 0: Disable"]
    #[inline(always)]
    pub fn cke_iwdg(&self) -> CkeIwdgR {
        CkeIwdgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: Enable ledc module's clock 0: Disable"]
    #[inline(always)]
    pub fn cke_ledc(&self) -> CkeLedcR {
        CkeLedcR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - if 0, aon reg(except this bit) , swr/ldo reg will be gated. Note: AON clock can't be shutdown as analog need."]
    #[inline(always)]
    pub fn cke_aon(&mut self) -> CkeAonW<'_, RegClkSpec> {
        CkeAonW::new(self, 0)
    }
    #[doc = "Bit 2 - 1: Enable AON timer clock 0: Disable"]
    #[inline(always)]
    pub fn cke_atim(&mut self) -> CkeAtimW<'_, RegClkSpec> {
        CkeAtimW::new(self, 2)
    }
    #[doc = "Bit 3 - 1: Enable RTC clock 0: Disable"]
    #[inline(always)]
    pub fn cke_rtc(&mut self) -> CkeRtcW<'_, RegClkSpec> {
        CkeRtcW::new(self, 3)
    }
    #[doc = "Bit 4 - 1: Enable 32.768K SDM module's clock 0: Disable"]
    #[inline(always)]
    pub fn cke_sdm(&mut self) -> CkeSdmW<'_, RegClkSpec> {
        CkeSdmW::new(self, 4)
    }
    #[doc = "Bit 5 - 1: Enable iwdg bus clock 0: Disable"]
    #[inline(always)]
    pub fn cke_iwdg(&mut self) -> CkeIwdgW<'_, RegClkSpec> {
        CkeIwdgW::new(self, 5)
    }
    #[doc = "Bit 8 - 1: Enable ledc module's clock 0: Disable"]
    #[inline(always)]
    pub fn cke_ledc(&mut self) -> CkeLedcW<'_, RegClkSpec> {
        CkeLedcW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegClkSpec;
impl crate::RegisterSpec for RegClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_clk::R`](R) reader structure"]
impl crate::Readable for RegClkSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_clk::W`](W) writer structure"]
impl crate::Writable for RegClkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CLK to value 0x35"]
impl crate::Resettable for RegClkSpec {
    const RESET_VALUE: u32 = 0x35;
}
