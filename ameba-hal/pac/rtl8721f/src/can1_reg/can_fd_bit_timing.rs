#[doc = "Register `CAN_FD_BIT_TIMING` reader"]
pub type R = crate::R<CanFdBitTimingSpec>;
#[doc = "Register `CAN_FD_BIT_TIMING` writer"]
pub type W = crate::W<CanFdBitTimingSpec>;
#[doc = "Field `can_fd_tseg1` reader - TSEG1 length = <$bit:can_fd_tseg1>+1"]
pub type CanFdTseg1R = crate::FieldReader;
#[doc = "Field `can_fd_tseg1` writer - TSEG1 length = <$bit:can_fd_tseg1>+1"]
pub type CanFdTseg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_fd_tseg2` reader - TSEG2 length = <$bit:can_fd_tseg2+1>"]
pub type CanFdTseg2R = crate::FieldReader;
#[doc = "Field `can_fd_tseg2` writer - TSEG2 length = <$bit:can_fd_tseg2+1>"]
pub type CanFdTseg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_fd_sjw` reader - SJW length = <$bit:can_fd_sjw>"]
pub type CanFdSjwR = crate::FieldReader;
#[doc = "Field `can_fd_sjw` writer - SJW length = <$bit:can_fd_sjw>"]
pub type CanFdSjwW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `can_fd_brp` reader - CAN bit timing perscaler = <$bit:can_fd_brp>+1"]
pub type CanFdBrpR = crate::FieldReader;
#[doc = "Field `can_fd_brp` writer - CAN bit timing perscaler = <$bit:can_fd_brp>+1"]
pub type CanFdBrpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TSEG1 length = <$bit:can_fd_tseg1>+1"]
    #[inline(always)]
    pub fn can_fd_tseg1(&self) -> CanFdTseg1R {
        CanFdTseg1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TSEG2 length = <$bit:can_fd_tseg2+1>"]
    #[inline(always)]
    pub fn can_fd_tseg2(&self) -> CanFdTseg2R {
        CanFdTseg2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - SJW length = <$bit:can_fd_sjw>"]
    #[inline(always)]
    pub fn can_fd_sjw(&self) -> CanFdSjwR {
        CanFdSjwR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:31 - CAN bit timing perscaler = <$bit:can_fd_brp>+1"]
    #[inline(always)]
    pub fn can_fd_brp(&self) -> CanFdBrpR {
        CanFdBrpR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TSEG1 length = <$bit:can_fd_tseg1>+1"]
    #[inline(always)]
    pub fn can_fd_tseg1(&mut self) -> CanFdTseg1W<'_, CanFdBitTimingSpec> {
        CanFdTseg1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - TSEG2 length = <$bit:can_fd_tseg2+1>"]
    #[inline(always)]
    pub fn can_fd_tseg2(&mut self) -> CanFdTseg2W<'_, CanFdBitTimingSpec> {
        CanFdTseg2W::new(self, 8)
    }
    #[doc = "Bits 16:18 - SJW length = <$bit:can_fd_sjw>"]
    #[inline(always)]
    pub fn can_fd_sjw(&mut self) -> CanFdSjwW<'_, CanFdBitTimingSpec> {
        CanFdSjwW::new(self, 16)
    }
    #[doc = "Bits 24:31 - CAN bit timing perscaler = <$bit:can_fd_brp>+1"]
    #[inline(always)]
    pub fn can_fd_brp(&mut self) -> CanFdBrpW<'_, CanFdBitTimingSpec> {
        CanFdBrpW::new(self, 24)
    }
}
#[doc = "CAN FD DATA field bit timing\n\nYou can [`read`](crate::Reg::read) this register and get [`can_fd_bit_timing::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_fd_bit_timing::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanFdBitTimingSpec;
impl crate::RegisterSpec for CanFdBitTimingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_fd_bit_timing::R`](R) reader structure"]
impl crate::Readable for CanFdBitTimingSpec {}
#[doc = "`write(|w| ..)` method takes [`can_fd_bit_timing::W`](W) writer structure"]
impl crate::Writable for CanFdBitTimingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_FD_BIT_TIMING to value 0"]
impl crate::Resettable for CanFdBitTimingSpec {}
