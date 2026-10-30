#[doc = "Register `CAN_TIME_STAMP` reader"]
pub type R = crate::R<CanTimeStampSpec>;
#[doc = "Register `CAN_TIME_STAMP` writer"]
pub type W = crate::W<CanTimeStampSpec>;
#[doc = "Field `can_time_stamp` reader - time stamp counter value, up count"]
pub type CanTimeStampR = crate::FieldReader<u16>;
#[doc = "Field `can_time_stamp_div` reader - timer stamp conter prescalar"]
pub type CanTimeStampDivR = crate::FieldReader;
#[doc = "Field `can_time_stamp_div` writer - timer stamp conter prescalar"]
pub type CanTimeStampDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_time_stamp_en` reader - time stamp function enable and time stamp counter start"]
pub type CanTimeStampEnR = crate::BitReader;
#[doc = "Field `can_time_stamp_en` writer - time stamp function enable and time stamp counter start"]
pub type CanTimeStampEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - time stamp counter value, up count"]
    #[inline(always)]
    pub fn can_time_stamp(&self) -> CanTimeStampR {
        CanTimeStampR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - timer stamp conter prescalar"]
    #[inline(always)]
    pub fn can_time_stamp_div(&self) -> CanTimeStampDivR {
        CanTimeStampDivR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - time stamp function enable and time stamp counter start"]
    #[inline(always)]
    pub fn can_time_stamp_en(&self) -> CanTimeStampEnR {
        CanTimeStampEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:23 - timer stamp conter prescalar"]
    #[inline(always)]
    pub fn can_time_stamp_div(&mut self) -> CanTimeStampDivW<'_, CanTimeStampSpec> {
        CanTimeStampDivW::new(self, 16)
    }
    #[doc = "Bit 31 - time stamp function enable and time stamp counter start"]
    #[inline(always)]
    pub fn can_time_stamp_en(&mut self) -> CanTimeStampEnW<'_, CanTimeStampSpec> {
        CanTimeStampEnW::new(self, 31)
    }
}
#[doc = "Timestamp function\n\nYou can [`read`](crate::Reg::read) this register and get [`can_time_stamp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_time_stamp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanTimeStampSpec;
impl crate::RegisterSpec for CanTimeStampSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_time_stamp::R`](R) reader structure"]
impl crate::Readable for CanTimeStampSpec {}
#[doc = "`write(|w| ..)` method takes [`can_time_stamp::W`](W) writer structure"]
impl crate::Writable for CanTimeStampSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_TIME_STAMP to value 0"]
impl crate::Resettable for CanTimeStampSpec {}
