#[doc = "Register `CAN_MB1_CTRL` reader"]
pub type R = crate::R<CanMb1CtrlSpec>;
#[doc = "Register `CAN_MB1_CTRL` writer"]
pub type W = crate::W<CanMb1CtrlSpec>;
#[doc = "Field `can_msg1_ba` reader - message buffer base address in RAM"]
pub type CanMsg1BaR = crate::FieldReader<u16>;
#[doc = "Field `can_msg1_ba` writer - message buffer base address in RAM"]
pub type CanMsg1BaW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `can_msg1_rxdma_en` reader - dma enable when the message buffer used as rx"]
pub type CanMsg1RxdmaEnR = crate::BitReader;
#[doc = "Field `can_msg1_rxdma_en` writer - dma enable when the message buffer used as rx"]
pub type CanMsg1RxdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg1_ba(&self) -> CanMsg1BaR {
        CanMsg1BaR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg1_rxdma_en(&self) -> CanMsg1RxdmaEnR {
        CanMsg1RxdmaEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg1_ba(&mut self) -> CanMsg1BaW<'_, CanMb1CtrlSpec> {
        CanMsg1BaW::new(self, 0)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg1_rxdma_en(&mut self) -> CanMsg1RxdmaEnW<'_, CanMb1CtrlSpec> {
        CanMsg1RxdmaEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mb1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_mb1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanMb1CtrlSpec;
impl crate::RegisterSpec for CanMb1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mb1_ctrl::R`](R) reader structure"]
impl crate::Readable for CanMb1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`can_mb1_ctrl::W`](W) writer structure"]
impl crate::Writable for CanMb1CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_MB1_CTRL to value 0x06"]
impl crate::Resettable for CanMb1CtrlSpec {
    const RESET_VALUE: u32 = 0x06;
}
