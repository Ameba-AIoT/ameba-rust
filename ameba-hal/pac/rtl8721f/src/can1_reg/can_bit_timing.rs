#[doc = "Register `CAN_BIT_TIMING` reader"]
pub type R = crate::R<CanBitTimingSpec>;
#[doc = "Register `CAN_BIT_TIMING` writer"]
pub type W = crate::W<CanBitTimingSpec>;
#[doc = "Field `can_tseg1` reader - TSEG1 length = <$bit:can_tseg1+1>"]
pub type CanTseg1R = crate::FieldReader;
#[doc = "Field `can_tseg1` writer - TSEG1 length = <$bit:can_tseg1+1>"]
pub type CanTseg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_tseg2` reader - TSEG2 length = <$bit:can_tseg2+1>"]
pub type CanTseg2R = crate::FieldReader;
#[doc = "Field `can_tseg2` writer - TSEG2 length = <$bit:can_tseg2+1>"]
pub type CanTseg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_sjw` reader - SJW length = <$bit:can_sjw>"]
pub type CanSjwR = crate::FieldReader;
#[doc = "Field `can_sjw` writer - SJW length = <$bit:can_sjw>"]
pub type CanSjwW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `can_brp` reader - CAN bit timing perscaler = <$bit:can_brp>+1"]
pub type CanBrpR = crate::FieldReader;
#[doc = "Field `can_brp` writer - CAN bit timing perscaler = <$bit:can_brp>+1"]
pub type CanBrpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TSEG1 length = <$bit:can_tseg1+1>"]
    #[inline(always)]
    pub fn can_tseg1(&self) -> CanTseg1R {
        CanTseg1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TSEG2 length = <$bit:can_tseg2+1>"]
    #[inline(always)]
    pub fn can_tseg2(&self) -> CanTseg2R {
        CanTseg2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - SJW length = <$bit:can_sjw>"]
    #[inline(always)]
    pub fn can_sjw(&self) -> CanSjwR {
        CanSjwR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:31 - CAN bit timing perscaler = <$bit:can_brp>+1"]
    #[inline(always)]
    pub fn can_brp(&self) -> CanBrpR {
        CanBrpR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TSEG1 length = <$bit:can_tseg1+1>"]
    #[inline(always)]
    pub fn can_tseg1(&mut self) -> CanTseg1W<'_, CanBitTimingSpec> {
        CanTseg1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - TSEG2 length = <$bit:can_tseg2+1>"]
    #[inline(always)]
    pub fn can_tseg2(&mut self) -> CanTseg2W<'_, CanBitTimingSpec> {
        CanTseg2W::new(self, 8)
    }
    #[doc = "Bits 16:18 - SJW length = <$bit:can_sjw>"]
    #[inline(always)]
    pub fn can_sjw(&mut self) -> CanSjwW<'_, CanBitTimingSpec> {
        CanSjwW::new(self, 16)
    }
    #[doc = "Bits 24:31 - CAN bit timing perscaler = <$bit:can_brp>+1"]
    #[inline(always)]
    pub fn can_brp(&mut self) -> CanBrpW<'_, CanBitTimingSpec> {
        CanBrpW::new(self, 24)
    }
}
#[doc = "CAN2.0 bit timing\n\nYou can [`read`](crate::Reg::read) this register and get [`can_bit_timing::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_bit_timing::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanBitTimingSpec;
impl crate::RegisterSpec for CanBitTimingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_bit_timing::R`](R) reader structure"]
impl crate::Readable for CanBitTimingSpec {}
#[doc = "`write(|w| ..)` method takes [`can_bit_timing::W`](W) writer structure"]
impl crate::Writable for CanBitTimingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_BIT_TIMING to value 0"]
impl crate::Resettable for CanBitTimingSpec {}
