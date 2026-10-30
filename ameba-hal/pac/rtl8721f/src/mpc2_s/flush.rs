#[doc = "Register `FLUSH` writer"]
pub type W = crate::W<FlushSpec>;
#[doc = "Field `ELR_INTR_CLR` writer - Flush FIFO"]
pub type ElrIntrClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVENT_CNT_CLEAR` writer - Clear event counter"]
pub type EventCntClearW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Flush FIFO"]
    #[inline(always)]
    pub fn elr_intr_clr(&mut self) -> ElrIntrClrW<'_, FlushSpec> {
        ElrIntrClrW::new(self, 0)
    }
    #[doc = "Bit 8 - Clear event counter"]
    #[inline(always)]
    pub fn event_cnt_clear(&mut self) -> EventCntClearW<'_, FlushSpec> {
        EventCntClearW::new(self, 8)
    }
}
#[doc = "Flush FIFO or event counter\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flush::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlushSpec;
impl crate::RegisterSpec for FlushSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flush::W`](W) writer structure"]
impl crate::Writable for FlushSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLUSH to value 0"]
impl crate::Resettable for FlushSpec {}
