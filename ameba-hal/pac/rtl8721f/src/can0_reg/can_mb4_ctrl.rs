#[doc = "Register `CAN_MB4_CTRL` reader"]
pub type R = crate::R<CanMb4CtrlSpec>;
#[doc = "Register `CAN_MB4_CTRL` writer"]
pub type W = crate::W<CanMb4CtrlSpec>;
#[doc = "Field `can_msg4_ba` reader - message buffer base address in RAM"]
pub type CanMsg4BaR = crate::FieldReader<u16>;
#[doc = "Field `can_msg4_ba` writer - message buffer base address in RAM"]
pub type CanMsg4BaW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `can_msg4_rxdma_en` reader - dma enable when the message buffer used as rx"]
pub type CanMsg4RxdmaEnR = crate::BitReader;
#[doc = "Field `can_msg4_rxdma_en` writer - dma enable when the message buffer used as rx"]
pub type CanMsg4RxdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg4_ba(&self) -> CanMsg4BaR {
        CanMsg4BaR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg4_rxdma_en(&self) -> CanMsg4RxdmaEnR {
        CanMsg4RxdmaEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg4_ba(&mut self) -> CanMsg4BaW<'_, CanMb4CtrlSpec> {
        CanMsg4BaW::new(self, 0)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg4_rxdma_en(&mut self) -> CanMsg4RxdmaEnW<'_, CanMb4CtrlSpec> {
        CanMsg4RxdmaEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mb4_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_mb4_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanMb4CtrlSpec;
impl crate::RegisterSpec for CanMb4CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mb4_ctrl::R`](R) reader structure"]
impl crate::Readable for CanMb4CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`can_mb4_ctrl::W`](W) writer structure"]
impl crate::Writable for CanMb4CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_MB4_CTRL to value 0x18"]
impl crate::Resettable for CanMb4CtrlSpec {
    const RESET_VALUE: u32 = 0x18;
}
