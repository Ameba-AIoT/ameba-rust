#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `READ_ERROR_INTERRUPT_ENABLE` reader - Read error interrupt enable. Trigger interrupt when read RNG data register and RNG valid is not 0x1~0x8."]
pub type ReadErrorInterruptEnableR = crate::BitReader;
#[doc = "Field `READ_ERROR_INTERRUPT_ENABLE` writer - Read error interrupt enable. Trigger interrupt when read RNG data register and RNG valid is not 0x1~0x8."]
pub type ReadErrorInterruptEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCT0_FAIL_INTERRUPT_ENABLE` reader - Repetition Count Test 0 fail interrupt enable"]
pub type Rct0FailInterruptEnableR = crate::BitReader;
#[doc = "Field `RCT0_FAIL_INTERRUPT_ENABLE` writer - Repetition Count Test 0 fail interrupt enable"]
pub type Rct0FailInterruptEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCT1_FAIL_INTERRUPT_ENABLE` reader - Repetition Count Test 1 fail interrupt enable"]
pub type Rct1FailInterruptEnableR = crate::BitReader;
#[doc = "Field `RCT1_FAIL_INTERRUPT_ENABLE` writer - Repetition Count Test 1 fail interrupt enable"]
pub type Rct1FailInterruptEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APT0_FAIL_INTERRUPT_ENABLE` reader - Adaptive Proportion Test 0 fail interrupt enable"]
pub type Apt0FailInterruptEnableR = crate::BitReader;
#[doc = "Field `APT0_FAIL_INTERRUPT_ENABLE` writer - Adaptive Proportion Test 0 fail interrupt enable"]
pub type Apt0FailInterruptEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFFER_FULL_INTERRUPT_ENABLE` reader - RNG buffer full interrupt enable"]
pub type BufferFullInterruptEnableR = crate::BitReader;
#[doc = "Field `BUFFER_FULL_INTERRUPT_ENABLE` writer - RNG buffer full interrupt enable"]
pub type BufferFullInterruptEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Read error interrupt enable. Trigger interrupt when read RNG data register and RNG valid is not 0x1~0x8."]
    #[inline(always)]
    pub fn read_error_interrupt_enable(&self) -> ReadErrorInterruptEnableR {
        ReadErrorInterruptEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Repetition Count Test 0 fail interrupt enable"]
    #[inline(always)]
    pub fn rct0_fail_interrupt_enable(&self) -> Rct0FailInterruptEnableR {
        Rct0FailInterruptEnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Repetition Count Test 1 fail interrupt enable"]
    #[inline(always)]
    pub fn rct1_fail_interrupt_enable(&self) -> Rct1FailInterruptEnableR {
        Rct1FailInterruptEnableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Adaptive Proportion Test 0 fail interrupt enable"]
    #[inline(always)]
    pub fn apt0_fail_interrupt_enable(&self) -> Apt0FailInterruptEnableR {
        Apt0FailInterruptEnableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RNG buffer full interrupt enable"]
    #[inline(always)]
    pub fn buffer_full_interrupt_enable(&self) -> BufferFullInterruptEnableR {
        BufferFullInterruptEnableR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read error interrupt enable. Trigger interrupt when read RNG data register and RNG valid is not 0x1~0x8."]
    #[inline(always)]
    pub fn read_error_interrupt_enable(&mut self) -> ReadErrorInterruptEnableW<'_, IerSpec> {
        ReadErrorInterruptEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Repetition Count Test 0 fail interrupt enable"]
    #[inline(always)]
    pub fn rct0_fail_interrupt_enable(&mut self) -> Rct0FailInterruptEnableW<'_, IerSpec> {
        Rct0FailInterruptEnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Repetition Count Test 1 fail interrupt enable"]
    #[inline(always)]
    pub fn rct1_fail_interrupt_enable(&mut self) -> Rct1FailInterruptEnableW<'_, IerSpec> {
        Rct1FailInterruptEnableW::new(self, 2)
    }
    #[doc = "Bit 3 - Adaptive Proportion Test 0 fail interrupt enable"]
    #[inline(always)]
    pub fn apt0_fail_interrupt_enable(&mut self) -> Apt0FailInterruptEnableW<'_, IerSpec> {
        Apt0FailInterruptEnableW::new(self, 3)
    }
    #[doc = "Bit 4 - RNG buffer full interrupt enable"]
    #[inline(always)]
    pub fn buffer_full_interrupt_enable(&mut self) -> BufferFullInterruptEnableW<'_, IerSpec> {
        BufferFullInterruptEnableW::new(self, 4)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0x0e"]
impl crate::Resettable for IerSpec {
    const RESET_VALUE: u32 = 0x0e;
}
