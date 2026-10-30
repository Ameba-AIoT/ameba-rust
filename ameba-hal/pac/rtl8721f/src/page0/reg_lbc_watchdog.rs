#[doc = "Register `REG_LBC_WATCHDOG` reader"]
pub type R = crate::R<RegLbcWatchdogSpec>;
#[doc = "Register `REG_LBC_WATCHDOG` writer"]
pub type W = crate::W<RegLbcWatchdogSpec>;
#[doc = "Field `BIT_EN_WATCH_DOG` reader - Enable lbc timeout watchdog"]
pub type BitEnWatchDogR = crate::BitReader;
#[doc = "Field `BIT_EN_WATCH_DOG` writer - Enable lbc timeout watchdog"]
pub type BitEnWatchDogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_IO_TIMEOUT_FLAG` reader - Lbc timeout flag.Write \"1\" to clear."]
pub type BitIoTimeoutFlagR = crate::BitReader;
#[doc = "Field `BIT_WATCH_DOG_RECORD` reader - Time out register address"]
pub type BitWatchDogRecordR = crate::FieldReader<u16>;
#[doc = "Field `RSVD` reader - Reserved"]
pub type RsvdR = crate::FieldReader<u16>;
#[doc = "Field `RSVD` writer - Reserved"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Enable lbc timeout watchdog"]
    #[inline(always)]
    pub fn bit_en_watch_dog(&self) -> BitEnWatchDogR {
        BitEnWatchDogR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lbc timeout flag.Write \"1\" to clear."]
    #[inline(always)]
    pub fn bit_io_timeout_flag(&self) -> BitIoTimeoutFlagR {
        BitIoTimeoutFlagR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:15 - Time out register address"]
    #[inline(always)]
    pub fn bit_watch_dog_record(&self) -> BitWatchDogRecordR {
        BitWatchDogRecordR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:31 - Reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable lbc timeout watchdog"]
    #[inline(always)]
    pub fn bit_en_watch_dog(&mut self) -> BitEnWatchDogW<'_, RegLbcWatchdogSpec> {
        BitEnWatchDogW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Reserved"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegLbcWatchdogSpec> {
        RsvdW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_lbc_watchdog::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_lbc_watchdog::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegLbcWatchdogSpec;
impl crate::RegisterSpec for RegLbcWatchdogSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_lbc_watchdog::R`](R) reader structure"]
impl crate::Readable for RegLbcWatchdogSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_lbc_watchdog::W`](W) writer structure"]
impl crate::Writable for RegLbcWatchdogSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_LBC_WATCHDOG to value 0x01"]
impl crate::Resettable for RegLbcWatchdogSpec {
    const RESET_VALUE: u32 = 0x01;
}
