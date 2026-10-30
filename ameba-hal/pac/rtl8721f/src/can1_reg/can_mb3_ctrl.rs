#[doc = "Register `CAN_MB3_CTRL` reader"]
pub type R = crate::R<CanMb3CtrlSpec>;
#[doc = "Register `CAN_MB3_CTRL` writer"]
pub type W = crate::W<CanMb3CtrlSpec>;
#[doc = "Field `can_msg3_ba` reader - message buffer base address in RAM"]
pub type CanMsg3BaR = crate::FieldReader<u16>;
#[doc = "Field `can_msg3_ba` writer - message buffer base address in RAM"]
pub type CanMsg3BaW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `can_msg3_rxdma_en` reader - dma enable when the message buffer used as rx"]
pub type CanMsg3RxdmaEnR = crate::BitReader;
#[doc = "Field `can_msg3_rxdma_en` writer - dma enable when the message buffer used as rx"]
pub type CanMsg3RxdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg3_ba(&self) -> CanMsg3BaR {
        CanMsg3BaR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg3_rxdma_en(&self) -> CanMsg3RxdmaEnR {
        CanMsg3RxdmaEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg3_ba(&mut self) -> CanMsg3BaW<'_, CanMb3CtrlSpec> {
        CanMsg3BaW::new(self, 0)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg3_rxdma_en(&mut self) -> CanMsg3RxdmaEnW<'_, CanMb3CtrlSpec> {
        CanMsg3RxdmaEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mb3_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_mb3_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanMb3CtrlSpec;
impl crate::RegisterSpec for CanMb3CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mb3_ctrl::R`](R) reader structure"]
impl crate::Readable for CanMb3CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`can_mb3_ctrl::W`](W) writer structure"]
impl crate::Writable for CanMb3CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_MB3_CTRL to value 0x12"]
impl crate::Resettable for CanMb3CtrlSpec {
    const RESET_VALUE: u32 = 0x12;
}
