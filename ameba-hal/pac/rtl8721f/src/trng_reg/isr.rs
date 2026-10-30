#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `READ_ERROR_INTERRUPT_ACTIVE` reader - "]
pub type ReadErrorInterruptActiveR = crate::BitReader;
#[doc = "Field `RCT0_FAIL_INTERRUPT_ACTIVE` reader - "]
pub type Rct0FailInterruptActiveR = crate::BitReader;
#[doc = "Field `RCT1_FAIL_INTERRUPT_ACTIVE` reader - "]
pub type Rct1FailInterruptActiveR = crate::BitReader;
#[doc = "Field `APT0_FAIL_INTERRUPT_ACTIVE` reader - "]
pub type Apt0FailInterruptActiveR = crate::BitReader;
#[doc = "Field `BUFFER_FULL_INTERRUPT_ACTIVE` reader - "]
pub type BufferFullInterruptActiveR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn read_error_interrupt_active(&self) -> ReadErrorInterruptActiveR {
        ReadErrorInterruptActiveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rct0_fail_interrupt_active(&self) -> Rct0FailInterruptActiveR {
        Rct0FailInterruptActiveR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rct1_fail_interrupt_active(&self) -> Rct1FailInterruptActiveR {
        Rct1FailInterruptActiveR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn apt0_fail_interrupt_active(&self) -> Apt0FailInterruptActiveR {
        Apt0FailInterruptActiveR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn buffer_full_interrupt_active(&self) -> BufferFullInterruptActiveR {
        BufferFullInterruptActiveR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
