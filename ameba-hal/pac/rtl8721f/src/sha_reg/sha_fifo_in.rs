#[doc = "Register `SHA_FIFO_IN` writer"]
pub type W = crate::W<ShaFifoInSpec>;
#[doc = "Field `SHA_FIFO_IN` writer - Input data register for slave mode."]
pub type ShaFifoInW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Input data register for slave mode."]
    #[inline(always)]
    pub fn sha_fifo_in(&mut self) -> ShaFifoInW<'_, ShaFifoInSpec> {
        ShaFifoInW::new(self, 0)
    }
}
#[doc = "FIFO input data register for slave mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_fifo_in::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShaFifoInSpec;
impl crate::RegisterSpec for ShaFifoInSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sha_fifo_in::W`](W) writer structure"]
impl crate::Writable for ShaFifoInSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA_FIFO_IN to value 0"]
impl crate::Resettable for ShaFifoInSpec {}
