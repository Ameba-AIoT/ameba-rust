#[doc = "Register `REG_RTC_CTRL` reader"]
pub type R = crate::R<RegRtcCtrlSpec>;
#[doc = "Register `REG_RTC_CTRL` writer"]
pub type W = crate::W<RegRtcCtrlSpec>;
#[doc = "Field `reg_rtc_dmy` reader - "]
pub type RegRtcDmyR = crate::FieldReader;
#[doc = "Field `reg_rtc_dmy` writer - "]
pub type RegRtcDmyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_rtc_ldo` reader - "]
pub type RegRtcLdoR = crate::FieldReader;
#[doc = "Field `reg_rtc_ldo` writer - "]
pub type RegRtcLdoW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_rtc_en_por_hy` reader - "]
pub type RegRtcEnPorHyR = crate::BitReader;
#[doc = "Field `reg_rtc_en_por_hy` writer - "]
pub type RegRtcEnPorHyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn reg_rtc_dmy(&self) -> RegRtcDmyR {
        RegRtcDmyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn reg_rtc_ldo(&self) -> RegRtcLdoR {
        RegRtcLdoR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_rtc_en_por_hy(&self) -> RegRtcEnPorHyR {
        RegRtcEnPorHyR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn reg_rtc_dmy(&mut self) -> RegRtcDmyW<'_, RegRtcCtrlSpec> {
        RegRtcDmyW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn reg_rtc_ldo(&mut self) -> RegRtcLdoW<'_, RegRtcCtrlSpec> {
        RegRtcLdoW::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_rtc_en_por_hy(&mut self) -> RegRtcEnPorHyW<'_, RegRtcCtrlSpec> {
        RegRtcEnPorHyW::new(self, 6)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rtc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rtc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRtcCtrlSpec;
impl crate::RegisterSpec for RegRtcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rtc_ctrl::R`](R) reader structure"]
impl crate::Readable for RegRtcCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rtc_ctrl::W`](W) writer structure"]
impl crate::Writable for RegRtcCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RTC_CTRL to value 0"]
impl crate::Resettable for RegRtcCtrlSpec {}
