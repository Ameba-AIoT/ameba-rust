#[doc = "Register `ELR_INTR_CLR` writer"]
pub type W = crate::W<ElrIntrClrSpec>;
#[doc = "Field `ELR_INTR_CLR` writer - Error log interrupt clear. Writing 1'b1 to the register will trigger a 1-cycle pulse interrupt clear. It has to write 1'b1 to this register every time when the error exception occurs. Note that this register is write-only, the return value of reading this register is meaningless."]
pub type ElrIntrClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Error log interrupt clear. Writing 1'b1 to the register will trigger a 1-cycle pulse interrupt clear. It has to write 1'b1 to this register every time when the error exception occurs. Note that this register is write-only, the return value of reading this register is meaningless."]
    #[inline(always)]
    pub fn elr_intr_clr(&mut self) -> ElrIntrClrW<'_, ElrIntrClrSpec> {
        ElrIntrClrW::new(self, 0)
    }
}
#[doc = "Error log clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elr_intr_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ElrIntrClrSpec;
impl crate::RegisterSpec for ElrIntrClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`elr_intr_clr::W`](W) writer structure"]
impl crate::Writable for ElrIntrClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ELR_INTR_CLR to value 0"]
impl crate::Resettable for ElrIntrClrSpec {}
