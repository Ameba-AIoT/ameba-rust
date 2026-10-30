#[doc = "Register `CAN_MB13_CTRL` reader"]
pub type R = crate::R<CanMb13CtrlSpec>;
#[doc = "Register `CAN_MB13_CTRL` writer"]
pub type W = crate::W<CanMb13CtrlSpec>;
#[doc = "Field `can_msg13_ba` reader - message buffer base address in RAM"]
pub type CanMsg13BaR = crate::FieldReader<u16>;
#[doc = "Field `can_msg13_ba` writer - message buffer base address in RAM"]
pub type CanMsg13BaW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `can_msg13_rxdma_en` reader - dma enable when the message buffer used as rx"]
pub type CanMsg13RxdmaEnR = crate::BitReader;
#[doc = "Field `can_msg13_rxdma_en` writer - dma enable when the message buffer used as rx"]
pub type CanMsg13RxdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg13_ba(&self) -> CanMsg13BaR {
        CanMsg13BaR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg13_rxdma_en(&self) -> CanMsg13RxdmaEnR {
        CanMsg13RxdmaEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg13_ba(&mut self) -> CanMsg13BaW<'_, CanMb13CtrlSpec> {
        CanMsg13BaW::new(self, 0)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg13_rxdma_en(&mut self) -> CanMsg13RxdmaEnW<'_, CanMb13CtrlSpec> {
        CanMsg13RxdmaEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mb13_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_mb13_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanMb13CtrlSpec;
impl crate::RegisterSpec for CanMb13CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mb13_ctrl::R`](R) reader structure"]
impl crate::Readable for CanMb13CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`can_mb13_ctrl::W`](W) writer structure"]
impl crate::Writable for CanMb13CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_MB13_CTRL to value 0x4e"]
impl crate::Resettable for CanMb13CtrlSpec {
    const RESET_VALUE: u32 = 0x4e;
}
