#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `READ_ERROR_INTERRUPT_CLEAR` writer - "]
pub type ReadErrorInterruptClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCT0_FAIL_INTERRUPT_CLEAR` writer - Clear the this interrupt also clear the error flags of RCT0 health tests"]
pub type Rct0FailInterruptClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCT1_FAIL_INTERRUPT_CLEAR` writer - Clear the this interrupt also clear the error flags of RCT1 health tests"]
pub type Rct1FailInterruptClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APT0_FAIL_INTERRUPT_CLEAR` writer - Clear the this interrupt also clear the error flags of APT0 health tests"]
pub type Apt0FailInterruptClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFFER_FULL_INTERRUPT_CLEAR` writer - "]
pub type BufferFullInterruptClearW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn read_error_interrupt_clear(&mut self) -> ReadErrorInterruptClearW<'_, IcrSpec> {
        ReadErrorInterruptClearW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear the this interrupt also clear the error flags of RCT0 health tests"]
    #[inline(always)]
    pub fn rct0_fail_interrupt_clear(&mut self) -> Rct0FailInterruptClearW<'_, IcrSpec> {
        Rct0FailInterruptClearW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear the this interrupt also clear the error flags of RCT1 health tests"]
    #[inline(always)]
    pub fn rct1_fail_interrupt_clear(&mut self) -> Rct1FailInterruptClearW<'_, IcrSpec> {
        Rct1FailInterruptClearW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear the this interrupt also clear the error flags of APT0 health tests"]
    #[inline(always)]
    pub fn apt0_fail_interrupt_clear(&mut self) -> Apt0FailInterruptClearW<'_, IcrSpec> {
        Apt0FailInterruptClearW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn buffer_full_interrupt_clear(&mut self) -> BufferFullInterruptClearW<'_, IcrSpec> {
        BufferFullInterruptClearW::new(self, 4)
    }
}
#[doc = "Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
