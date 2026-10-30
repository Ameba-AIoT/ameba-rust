#[doc = "Register `INTERRUPT_ALL_CLR` reader"]
pub type R = crate::R<InterruptAllClrSpec>;
#[doc = "Register `INTERRUPT_ALL_CLR` writer"]
pub type W = crate::W<InterruptAllClrSpec>;
#[doc = "Field `intr_all_clr` reader - Write \"1\" to this register to clear the combined interrupts, all individual interrupts, interrupt status register and raw interrupt status register."]
pub type IntrAllClrR = crate::BitReader;
#[doc = "Field `intr_all_clr` writer - Write \"1\" to this register to clear the combined interrupts, all individual interrupts, interrupt status register and raw interrupt status register."]
pub type IntrAllClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write \"1\" to this register to clear the combined interrupts, all individual interrupts, interrupt status register and raw interrupt status register."]
    #[inline(always)]
    pub fn intr_all_clr(&self) -> IntrAllClrR {
        IntrAllClrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write \"1\" to this register to clear the combined interrupts, all individual interrupts, interrupt status register and raw interrupt status register."]
    #[inline(always)]
    pub fn intr_all_clr(&mut self) -> IntrAllClrW<'_, InterruptAllClrSpec> {
        IntrAllClrW::new(self, 0)
    }
}
#[doc = "Interrupt All Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_all_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_all_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptAllClrSpec;
impl crate::RegisterSpec for InterruptAllClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_all_clr::R`](R) reader structure"]
impl crate::Readable for InterruptAllClrSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_all_clr::W`](W) writer structure"]
impl crate::Writable for InterruptAllClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTERRUPT_ALL_CLR to value 0"]
impl crate::Resettable for InterruptAllClrSpec {}
