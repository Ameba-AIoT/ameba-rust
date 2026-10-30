#[doc = "Register `RTC_WPR` reader"]
pub type R = crate::R<RtcWprSpec>;
#[doc = "Register `RTC_WPR` writer"]
pub type W = crate::W<RtcWprSpec>;
#[doc = "Field `KEY` reader - Write protection key This byte is written by software. Refer to RTC register write protection for a description of how to unlock RTC register write protection.RTC write protection register,write \"0xCA\"then write \"0x53\" disable write protect,write \"0xFF\" enable write protect"]
pub type KeyR = crate::FieldReader;
#[doc = "Field `KEY` writer - Write protection key This byte is written by software. Refer to RTC register write protection for a description of how to unlock RTC register write protection.RTC write protection register,write \"0xCA\"then write \"0x53\" disable write protect,write \"0xFF\" enable write protect"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Write protection key This byte is written by software. Refer to RTC register write protection for a description of how to unlock RTC register write protection.RTC write protection register,write \"0xCA\"then write \"0x53\" disable write protect,write \"0xFF\" enable write protect"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write protection key This byte is written by software. Refer to RTC register write protection for a description of how to unlock RTC register write protection.RTC write protection register,write \"0xCA\"then write \"0x53\" disable write protect,write \"0xFF\" enable write protect"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, RtcWprSpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "RTC Write Protect Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_wpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_wpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcWprSpec;
impl crate::RegisterSpec for RtcWprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_wpr::R`](R) reader structure"]
impl crate::Readable for RtcWprSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_wpr::W`](W) writer structure"]
impl crate::Writable for RtcWprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_WPR to value 0"]
impl crate::Resettable for RtcWprSpec {}
