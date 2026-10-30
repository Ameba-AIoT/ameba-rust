#[doc = "Register `CAN_MB8_CTRL` reader"]
pub type R = crate::R<CanMb8CtrlSpec>;
#[doc = "Register `CAN_MB8_CTRL` writer"]
pub type W = crate::W<CanMb8CtrlSpec>;
#[doc = "Field `can_msg8_ba` reader - message buffer base address in RAM"]
pub type CanMsg8BaR = crate::FieldReader<u16>;
#[doc = "Field `can_msg8_ba` writer - message buffer base address in RAM"]
pub type CanMsg8BaW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `can_msg8_rxdma_en` reader - dma enable when the message buffer used as rx"]
pub type CanMsg8RxdmaEnR = crate::BitReader;
#[doc = "Field `can_msg8_rxdma_en` writer - dma enable when the message buffer used as rx"]
pub type CanMsg8RxdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg8_ba(&self) -> CanMsg8BaR {
        CanMsg8BaR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg8_rxdma_en(&self) -> CanMsg8RxdmaEnR {
        CanMsg8RxdmaEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg8_ba(&mut self) -> CanMsg8BaW<'_, CanMb8CtrlSpec> {
        CanMsg8BaW::new(self, 0)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg8_rxdma_en(&mut self) -> CanMsg8RxdmaEnW<'_, CanMb8CtrlSpec> {
        CanMsg8RxdmaEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mb8_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_mb8_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanMb8CtrlSpec;
impl crate::RegisterSpec for CanMb8CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mb8_ctrl::R`](R) reader structure"]
impl crate::Readable for CanMb8CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`can_mb8_ctrl::W`](W) writer structure"]
impl crate::Writable for CanMb8CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_MB8_CTRL to value 0x30"]
impl crate::Resettable for CanMb8CtrlSpec {
    const RESET_VALUE: u32 = 0x30;
}
