#[doc = "Register `CAN_MB6_CTRL` reader"]
pub type R = crate::R<CanMb6CtrlSpec>;
#[doc = "Register `CAN_MB6_CTRL` writer"]
pub type W = crate::W<CanMb6CtrlSpec>;
#[doc = "Field `can_msg6_ba` reader - message buffer base address in RAM"]
pub type CanMsg6BaR = crate::FieldReader<u16>;
#[doc = "Field `can_msg6_ba` writer - message buffer base address in RAM"]
pub type CanMsg6BaW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `can_msg6_rxdma_en` reader - dma enable when the message buffer used as rx"]
pub type CanMsg6RxdmaEnR = crate::BitReader;
#[doc = "Field `can_msg6_rxdma_en` writer - dma enable when the message buffer used as rx"]
pub type CanMsg6RxdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg6_ba(&self) -> CanMsg6BaR {
        CanMsg6BaR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg6_rxdma_en(&self) -> CanMsg6RxdmaEnR {
        CanMsg6RxdmaEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg6_ba(&mut self) -> CanMsg6BaW<'_, CanMb6CtrlSpec> {
        CanMsg6BaW::new(self, 0)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg6_rxdma_en(&mut self) -> CanMsg6RxdmaEnW<'_, CanMb6CtrlSpec> {
        CanMsg6RxdmaEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mb6_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_mb6_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanMb6CtrlSpec;
impl crate::RegisterSpec for CanMb6CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mb6_ctrl::R`](R) reader structure"]
impl crate::Readable for CanMb6CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`can_mb6_ctrl::W`](W) writer structure"]
impl crate::Writable for CanMb6CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_MB6_CTRL to value 0x24"]
impl crate::Resettable for CanMb6CtrlSpec {
    const RESET_VALUE: u32 = 0x24;
}
