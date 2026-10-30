#[doc = "Register `REG_MGQ_FIFO_LIFETIME_VALID` reader"]
pub type R = crate::R<RegMgqFifoLifetimeValidSpec>;
#[doc = "Register `REG_MGQ_FIFO_LIFETIME_VALID` writer"]
pub type W = crate::W<RegMgqFifoLifetimeValidSpec>;
#[doc = "Field `BIT_MGQ_FIFO_VALID_CLR_0` reader - Clear the valid map of the MGQ FIFO PKT, low \\[7:0\\] bits"]
pub type BitMgqFifoValidClr0R = crate::FieldReader;
#[doc = "Field `BIT_MGQ_FIFO_VALID_CLR_1` reader - Clear the valid map of the MGQ FIFO PKT, high \\[15:8\\] bits"]
pub type BitMgqFifoValidClr1R = crate::FieldReader;
#[doc = "Field `BIT_MGQ_FIFO_LIFETIME` reader - MGQ FIFO packet lifetime unit: 256us"]
pub type BitMgqFifoLifetimeR = crate::FieldReader<u16>;
#[doc = "Field `BIT_MGQ_FIFO_LIFETIME` writer - MGQ FIFO packet lifetime unit: 256us"]
pub type BitMgqFifoLifetimeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - Clear the valid map of the MGQ FIFO PKT, low \\[7:0\\] bits"]
    #[inline(always)]
    pub fn bit_mgq_fifo_valid_clr_0(&self) -> BitMgqFifoValidClr0R {
        BitMgqFifoValidClr0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clear the valid map of the MGQ FIFO PKT, high \\[15:8\\] bits"]
    #[inline(always)]
    pub fn bit_mgq_fifo_valid_clr_1(&self) -> BitMgqFifoValidClr1R {
        BitMgqFifoValidClr1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - MGQ FIFO packet lifetime unit: 256us"]
    #[inline(always)]
    pub fn bit_mgq_fifo_lifetime(&self) -> BitMgqFifoLifetimeR {
        BitMgqFifoLifetimeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - MGQ FIFO packet lifetime unit: 256us"]
    #[inline(always)]
    pub fn bit_mgq_fifo_lifetime(
        &mut self,
    ) -> BitMgqFifoLifetimeW<'_, RegMgqFifoLifetimeValidSpec> {
        BitMgqFifoLifetimeW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mgq_fifo_lifetime_valid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mgq_fifo_lifetime_valid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMgqFifoLifetimeValidSpec;
impl crate::RegisterSpec for RegMgqFifoLifetimeValidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mgq_fifo_lifetime_valid::R`](R) reader structure"]
impl crate::Readable for RegMgqFifoLifetimeValidSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mgq_fifo_lifetime_valid::W`](W) writer structure"]
impl crate::Writable for RegMgqFifoLifetimeValidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MGQ_FIFO_LIFETIME_VALID to value 0x1000_0000"]
impl crate::Resettable for RegMgqFifoLifetimeValidSpec {
    const RESET_VALUE: u32 = 0x1000_0000;
}
