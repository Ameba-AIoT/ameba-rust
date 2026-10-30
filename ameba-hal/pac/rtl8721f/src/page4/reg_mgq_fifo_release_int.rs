#[doc = "Register `REG_MGQ_FIFO_RELEASE_INT` reader"]
pub type R = crate::R<RegMgqFifoReleaseIntSpec>;
#[doc = "Register `REG_MGQ_FIFO_RELEASE_INT` writer"]
pub type W = crate::W<RegMgqFifoReleaseIntSpec>;
#[doc = "Field `BIT_MGQ_FIFO_INT_MASK` reader - The mask of the MGQ FIFO PKT release interrupt."]
pub type BitMgqFifoIntMaskR = crate::FieldReader<u16>;
#[doc = "Field `BIT_MGQ_FIFO_INT_MASK` writer - The mask of the MGQ FIFO PKT release interrupt."]
pub type BitMgqFifoIntMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_MGQ_FIFO_INT_FLAG` writer - The flag of the MGQ FIFO PKT release interrupt, write one clear."]
pub type BitMgqFifoIntFlagW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The mask of the MGQ FIFO PKT release interrupt."]
    #[inline(always)]
    pub fn bit_mgq_fifo_int_mask(&self) -> BitMgqFifoIntMaskR {
        BitMgqFifoIntMaskR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The mask of the MGQ FIFO PKT release interrupt."]
    #[inline(always)]
    pub fn bit_mgq_fifo_int_mask(&mut self) -> BitMgqFifoIntMaskW<'_, RegMgqFifoReleaseIntSpec> {
        BitMgqFifoIntMaskW::new(self, 0)
    }
    #[doc = "Bits 16:31 - The flag of the MGQ FIFO PKT release interrupt, write one clear."]
    #[inline(always)]
    pub fn bit_mgq_fifo_int_flag(&mut self) -> BitMgqFifoIntFlagW<'_, RegMgqFifoReleaseIntSpec> {
        BitMgqFifoIntFlagW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mgq_fifo_release_int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mgq_fifo_release_int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMgqFifoReleaseIntSpec;
impl crate::RegisterSpec for RegMgqFifoReleaseIntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mgq_fifo_release_int::R`](R) reader structure"]
impl crate::Readable for RegMgqFifoReleaseIntSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mgq_fifo_release_int::W`](W) writer structure"]
impl crate::Writable for RegMgqFifoReleaseIntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MGQ_FIFO_RELEASE_INT to value 0"]
impl crate::Resettable for RegMgqFifoReleaseIntSpec {}
