#[doc = "Register `CAN_MB7_CTRL` reader"]
pub type R = crate::R<CanMb7CtrlSpec>;
#[doc = "Register `CAN_MB7_CTRL` writer"]
pub type W = crate::W<CanMb7CtrlSpec>;
#[doc = "Field `can_msg7_ba` reader - message buffer base address in RAM"]
pub type CanMsg7BaR = crate::FieldReader<u16>;
#[doc = "Field `can_msg7_ba` writer - message buffer base address in RAM"]
pub type CanMsg7BaW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `can_msg7_rxdma_en` reader - dma enable when the message buffer used as rx"]
pub type CanMsg7RxdmaEnR = crate::BitReader;
#[doc = "Field `can_msg7_rxdma_en` writer - dma enable when the message buffer used as rx"]
pub type CanMsg7RxdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg7_ba(&self) -> CanMsg7BaR {
        CanMsg7BaR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg7_rxdma_en(&self) -> CanMsg7RxdmaEnR {
        CanMsg7RxdmaEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg7_ba(&mut self) -> CanMsg7BaW<'_, CanMb7CtrlSpec> {
        CanMsg7BaW::new(self, 0)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg7_rxdma_en(&mut self) -> CanMsg7RxdmaEnW<'_, CanMb7CtrlSpec> {
        CanMsg7RxdmaEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mb7_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_mb7_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanMb7CtrlSpec;
impl crate::RegisterSpec for CanMb7CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mb7_ctrl::R`](R) reader structure"]
impl crate::Readable for CanMb7CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`can_mb7_ctrl::W`](W) writer structure"]
impl crate::Writable for CanMb7CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_MB7_CTRL to value 0x2a"]
impl crate::Resettable for CanMb7CtrlSpec {
    const RESET_VALUE: u32 = 0x2a;
}
