#[doc = "Register `LOGUART_INTERRUPT_MASK` reader"]
pub type R = crate::R<LoguartInterruptMaskSpec>;
#[doc = "Register `LOGUART_INTERRUPT_MASK` writer"]
pub type W = crate::W<LoguartInterruptMaskSpec>;
#[doc = "Field `loguart_intr_mask_np` reader - Mask loguart interrupt to NP 1:unmask 0:mask"]
pub type LoguartIntrMaskNpR = crate::BitReader;
#[doc = "Field `loguart_intr_mask_np` writer - Mask loguart interrupt to NP 1:unmask 0:mask"]
pub type LoguartIntrMaskNpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `loguart_intr_mask_ap` reader - Mask loguart interrupt to AP 1:unmask 0:mask"]
pub type LoguartIntrMaskApR = crate::BitReader;
#[doc = "Field `loguart_intr_mask_ap` writer - Mask loguart interrupt to AP 1:unmask 0:mask"]
pub type LoguartIntrMaskApW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask loguart interrupt to NP 1:unmask 0:mask"]
    #[inline(always)]
    pub fn loguart_intr_mask_np(&self) -> LoguartIntrMaskNpR {
        LoguartIntrMaskNpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask loguart interrupt to AP 1:unmask 0:mask"]
    #[inline(always)]
    pub fn loguart_intr_mask_ap(&self) -> LoguartIntrMaskApR {
        LoguartIntrMaskApR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask loguart interrupt to NP 1:unmask 0:mask"]
    #[inline(always)]
    pub fn loguart_intr_mask_np(&mut self) -> LoguartIntrMaskNpW<'_, LoguartInterruptMaskSpec> {
        LoguartIntrMaskNpW::new(self, 0)
    }
    #[doc = "Bit 1 - Mask loguart interrupt to AP 1:unmask 0:mask"]
    #[inline(always)]
    pub fn loguart_intr_mask_ap(&mut self) -> LoguartIntrMaskApW<'_, LoguartInterruptMaskSpec> {
        LoguartIntrMaskApW::new(self, 1)
    }
}
#[doc = "loguart interrupt mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`loguart_interrupt_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`loguart_interrupt_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoguartInterruptMaskSpec;
impl crate::RegisterSpec for LoguartInterruptMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`loguart_interrupt_mask::R`](R) reader structure"]
impl crate::Readable for LoguartInterruptMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`loguart_interrupt_mask::W`](W) writer structure"]
impl crate::Writable for LoguartInterruptMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOGUART_INTERRUPT_MASK to value 0x03"]
impl crate::Resettable for LoguartInterruptMaskSpec {
    const RESET_VALUE: u32 = 0x03;
}
