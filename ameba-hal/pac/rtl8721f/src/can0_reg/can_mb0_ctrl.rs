#[doc = "Register `CAN_MB0_CTRL` reader"]
pub type R = crate::R<CanMb0CtrlSpec>;
#[doc = "Register `CAN_MB0_CTRL` writer"]
pub type W = crate::W<CanMb0CtrlSpec>;
#[doc = "Field `can_msg0_ba` reader - message buffer base address in RAM"]
pub type CanMsg0BaR = crate::FieldReader<u16>;
#[doc = "Field `can_msg0_ba` writer - message buffer base address in RAM"]
pub type CanMsg0BaW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `can_msg0_rxdma_en` reader - dma enable when the message buffer used as rx"]
pub type CanMsg0RxdmaEnR = crate::BitReader;
#[doc = "Field `can_msg0_rxdma_en` writer - dma enable when the message buffer used as rx"]
pub type CanMsg0RxdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg0_ba(&self) -> CanMsg0BaR {
        CanMsg0BaR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg0_rxdma_en(&self) -> CanMsg0RxdmaEnR {
        CanMsg0RxdmaEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg0_ba(&mut self) -> CanMsg0BaW<'_, CanMb0CtrlSpec> {
        CanMsg0BaW::new(self, 0)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg0_rxdma_en(&mut self) -> CanMsg0RxdmaEnW<'_, CanMb0CtrlSpec> {
        CanMsg0RxdmaEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mb0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_mb0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanMb0CtrlSpec;
impl crate::RegisterSpec for CanMb0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mb0_ctrl::R`](R) reader structure"]
impl crate::Readable for CanMb0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`can_mb0_ctrl::W`](W) writer structure"]
impl crate::Writable for CanMb0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_MB0_CTRL to value 0"]
impl crate::Resettable for CanMb0CtrlSpec {}
