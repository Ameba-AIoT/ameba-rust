#[doc = "Register `OSC131K_STATE` reader"]
pub type R = crate::R<Osc131kStateSpec>;
#[doc = "Register `OSC131K_STATE` writer"]
pub type W = crate::W<Osc131kStateSpec>;
#[doc = "Field `first_pon` reader - Default is 0 , after SW write 1 , it will be tie to 1 , until next reset . This indicate RTC first pon state"]
pub type FirstPonR = crate::BitReader;
#[doc = "Field `first_pon` writer - Default is 0 , after SW write 1 , it will be tie to 1 , until next reset . This indicate RTC first pon state"]
pub type FirstPonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cksl_rtc` reader - rtc clock select. 0: sdm32k 1: osc131k div4 Note:when aon power off, it will be reset to 1 because sdm is off. when aon power on again , SW must set this bit to 0 after SDM config done."]
pub type CkslRtcR = crate::BitReader;
#[doc = "Field `cksl_rtc` writer - rtc clock select. 0: sdm32k 1: osc131k div4 Note:when aon power off, it will be reset to 1 because sdm is off. when aon power on again , SW must set this bit to 0 after SDM config done."]
pub type CkslRtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dbg_sel` reader - rtc debug select for RTC domain PAD"]
pub type DbgSelR = crate::FieldReader;
#[doc = "Field `dbg_sel` writer - rtc debug select for RTC domain PAD"]
pub type DbgSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `scan_mode` reader - Scan mode enable for RTC domain only ."]
pub type ScanModeR = crate::BitReader;
#[doc = "Field `scan_mode` writer - Scan mode enable for RTC domain only ."]
pub type ScanModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Default is 0 , after SW write 1 , it will be tie to 1 , until next reset . This indicate RTC first pon state"]
    #[inline(always)]
    pub fn first_pon(&self) -> FirstPonR {
        FirstPonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - rtc clock select. 0: sdm32k 1: osc131k div4 Note:when aon power off, it will be reset to 1 because sdm is off. when aon power on again , SW must set this bit to 0 after SDM config done."]
    #[inline(always)]
    pub fn cksl_rtc(&self) -> CkslRtcR {
        CkslRtcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:12 - rtc debug select for RTC domain PAD"]
    #[inline(always)]
    pub fn dbg_sel(&self) -> DbgSelR {
        DbgSelR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Scan mode enable for RTC domain only ."]
    #[inline(always)]
    pub fn scan_mode(&self) -> ScanModeR {
        ScanModeR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Default is 0 , after SW write 1 , it will be tie to 1 , until next reset . This indicate RTC first pon state"]
    #[inline(always)]
    pub fn first_pon(&mut self) -> FirstPonW<'_, Osc131kStateSpec> {
        FirstPonW::new(self, 0)
    }
    #[doc = "Bit 1 - rtc clock select. 0: sdm32k 1: osc131k div4 Note:when aon power off, it will be reset to 1 because sdm is off. when aon power on again , SW must set this bit to 0 after SDM config done."]
    #[inline(always)]
    pub fn cksl_rtc(&mut self) -> CkslRtcW<'_, Osc131kStateSpec> {
        CkslRtcW::new(self, 1)
    }
    #[doc = "Bits 8:12 - rtc debug select for RTC domain PAD"]
    #[inline(always)]
    pub fn dbg_sel(&mut self) -> DbgSelW<'_, Osc131kStateSpec> {
        DbgSelW::new(self, 8)
    }
    #[doc = "Bit 16 - Scan mode enable for RTC domain only ."]
    #[inline(always)]
    pub fn scan_mode(&mut self) -> ScanModeW<'_, Osc131kStateSpec> {
        ScanModeW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`osc131k_state::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osc131k_state::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Osc131kStateSpec;
impl crate::RegisterSpec for Osc131kStateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osc131k_state::R`](R) reader structure"]
impl crate::Readable for Osc131kStateSpec {}
#[doc = "`write(|w| ..)` method takes [`osc131k_state::W`](W) writer structure"]
impl crate::Writable for Osc131kStateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSC131K_STATE to value 0x02"]
impl crate::Resettable for Osc131kStateSpec {
    const RESET_VALUE: u32 = 0x02;
}
