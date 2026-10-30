#[doc = "Register `CAN_MB11_CTRL` reader"]
pub type R = crate::R<CanMb11CtrlSpec>;
#[doc = "Register `CAN_MB11_CTRL` writer"]
pub type W = crate::W<CanMb11CtrlSpec>;
#[doc = "Field `can_msg11_ba` reader - message buffer base address in RAM"]
pub type CanMsg11BaR = crate::FieldReader<u16>;
#[doc = "Field `can_msg11_ba` writer - message buffer base address in RAM"]
pub type CanMsg11BaW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `can_msg11_rxdma_en` reader - dma enable when the message buffer used as rx"]
pub type CanMsg11RxdmaEnR = crate::BitReader;
#[doc = "Field `can_msg11_rxdma_en` writer - dma enable when the message buffer used as rx"]
pub type CanMsg11RxdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg11_ba(&self) -> CanMsg11BaR {
        CanMsg11BaR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg11_rxdma_en(&self) -> CanMsg11RxdmaEnR {
        CanMsg11RxdmaEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg11_ba(&mut self) -> CanMsg11BaW<'_, CanMb11CtrlSpec> {
        CanMsg11BaW::new(self, 0)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg11_rxdma_en(&mut self) -> CanMsg11RxdmaEnW<'_, CanMb11CtrlSpec> {
        CanMsg11RxdmaEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mb11_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_mb11_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanMb11CtrlSpec;
impl crate::RegisterSpec for CanMb11CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mb11_ctrl::R`](R) reader structure"]
impl crate::Readable for CanMb11CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`can_mb11_ctrl::W`](W) writer structure"]
impl crate::Writable for CanMb11CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_MB11_CTRL to value 0x42"]
impl crate::Resettable for CanMb11CtrlSpec {
    const RESET_VALUE: u32 = 0x42;
}
