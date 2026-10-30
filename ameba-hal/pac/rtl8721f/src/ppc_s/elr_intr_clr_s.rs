#[doc = "Register `ELR_INTR_CLR_S` writer"]
pub type W = crate::W<ElrIntrClrSSpec>;
#[doc = "Field `ELR_INTR_CLR_S` writer - Error log interrupt clear. Writing 1'b1 to the register will trigger a 1-cycle pulse interrupt clear. It has to write 1'b1 to this register every time when the error exception occurs. Note that this register is write-only, the return value of reading this register is meaningless."]
pub type ElrIntrClrSW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Error log interrupt clear. Writing 1'b1 to the register will trigger a 1-cycle pulse interrupt clear. It has to write 1'b1 to this register every time when the error exception occurs. Note that this register is write-only, the return value of reading this register is meaningless."]
    #[inline(always)]
    pub fn elr_intr_clr_s(&mut self) -> ElrIntrClrSW<'_, ElrIntrClrSSpec> {
        ElrIntrClrSW::new(self, 0)
    }
}
#[doc = "Error log clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elr_intr_clr_s::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ElrIntrClrSSpec;
impl crate::RegisterSpec for ElrIntrClrSSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`elr_intr_clr_s::W`](W) writer structure"]
impl crate::Writable for ElrIntrClrSSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ELR_INTR_CLR_S to value 0"]
impl crate::Resettable for ElrIntrClrSSpec {}
