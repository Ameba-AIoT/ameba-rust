#[doc = "Register `CAN_MB5_CTRL` reader"]
pub type R = crate::R<CanMb5CtrlSpec>;
#[doc = "Register `CAN_MB5_CTRL` writer"]
pub type W = crate::W<CanMb5CtrlSpec>;
#[doc = "Field `can_msg5_ba` reader - message buffer base address in RAM"]
pub type CanMsg5BaR = crate::FieldReader<u16>;
#[doc = "Field `can_msg5_ba` writer - message buffer base address in RAM"]
pub type CanMsg5BaW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `can_msg5_rxdma_en` reader - dma enable when the message buffer used as rx"]
pub type CanMsg5RxdmaEnR = crate::BitReader;
#[doc = "Field `can_msg5_rxdma_en` writer - dma enable when the message buffer used as rx"]
pub type CanMsg5RxdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg5_ba(&self) -> CanMsg5BaR {
        CanMsg5BaR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg5_rxdma_en(&self) -> CanMsg5RxdmaEnR {
        CanMsg5RxdmaEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg5_ba(&mut self) -> CanMsg5BaW<'_, CanMb5CtrlSpec> {
        CanMsg5BaW::new(self, 0)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg5_rxdma_en(&mut self) -> CanMsg5RxdmaEnW<'_, CanMb5CtrlSpec> {
        CanMsg5RxdmaEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mb5_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_mb5_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanMb5CtrlSpec;
impl crate::RegisterSpec for CanMb5CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mb5_ctrl::R`](R) reader structure"]
impl crate::Readable for CanMb5CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`can_mb5_ctrl::W`](W) writer structure"]
impl crate::Writable for CanMb5CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_MB5_CTRL to value 0x1e"]
impl crate::Resettable for CanMb5CtrlSpec {
    const RESET_VALUE: u32 = 0x1e;
}
