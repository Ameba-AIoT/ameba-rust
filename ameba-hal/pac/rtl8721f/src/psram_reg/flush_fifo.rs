#[doc = "Register `FLUSH_FIFO` writer"]
pub type W = crate::W<FlushFifoSpec>;
#[doc = "Field `FLUSH_ALL` writer - Clear all data in all FIFO (include TX_FIFO, RX_FIFO and ST_FIFO) and reset all FSM when writing to FLUSH_ALL register."]
pub type FlushAllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLUSH_DR_FIFO` writer - Clear all data in the TX_FIFO and RX_FIFO when writing to FLUSH_DR_FIFO register."]
pub type FlushDrFifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLUSH_ST_FIFO` writer - Clear all data in the ST_FIFO when writing to FLUSH_ST_FIFO register."]
pub type FlushStFifoW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear all data in all FIFO (include TX_FIFO, RX_FIFO and ST_FIFO) and reset all FSM when writing to FLUSH_ALL register."]
    #[inline(always)]
    pub fn flush_all(&mut self) -> FlushAllW<'_, FlushFifoSpec> {
        FlushAllW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear all data in the TX_FIFO and RX_FIFO when writing to FLUSH_DR_FIFO register."]
    #[inline(always)]
    pub fn flush_dr_fifo(&mut self) -> FlushDrFifoW<'_, FlushFifoSpec> {
        FlushDrFifoW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear all data in the ST_FIFO when writing to FLUSH_ST_FIFO register."]
    #[inline(always)]
    pub fn flush_st_fifo(&mut self) -> FlushStFifoW<'_, FlushFifoSpec> {
        FlushStFifoW::new(self, 2)
    }
}
#[doc = "Flush FIFO Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flush_fifo::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlushFifoSpec;
impl crate::RegisterSpec for FlushFifoSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flush_fifo::W`](W) writer structure"]
impl crate::Writable for FlushFifoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLUSH_FIFO to value 0"]
impl crate::Resettable for FlushFifoSpec {}
