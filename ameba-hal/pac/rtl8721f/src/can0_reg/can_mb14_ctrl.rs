#[doc = "Register `CAN_MB14_CTRL` reader"]
pub type R = crate::R<CanMb14CtrlSpec>;
#[doc = "Register `CAN_MB14_CTRL` writer"]
pub type W = crate::W<CanMb14CtrlSpec>;
#[doc = "Field `can_msg14_ba` reader - message buffer base address in RAM"]
pub type CanMsg14BaR = crate::FieldReader<u16>;
#[doc = "Field `can_msg14_ba` writer - message buffer base address in RAM"]
pub type CanMsg14BaW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `can_msg14_rxdma_en` reader - dma enable when the message buffer used as rx"]
pub type CanMsg14RxdmaEnR = crate::BitReader;
#[doc = "Field `can_msg14_rxdma_en` writer - dma enable when the message buffer used as rx"]
pub type CanMsg14RxdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg14_ba(&self) -> CanMsg14BaR {
        CanMsg14BaR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg14_rxdma_en(&self) -> CanMsg14RxdmaEnR {
        CanMsg14RxdmaEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg14_ba(&mut self) -> CanMsg14BaW<'_, CanMb14CtrlSpec> {
        CanMsg14BaW::new(self, 0)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg14_rxdma_en(&mut self) -> CanMsg14RxdmaEnW<'_, CanMb14CtrlSpec> {
        CanMsg14RxdmaEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mb14_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_mb14_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanMb14CtrlSpec;
impl crate::RegisterSpec for CanMb14CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mb14_ctrl::R`](R) reader structure"]
impl crate::Readable for CanMb14CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`can_mb14_ctrl::W`](W) writer structure"]
impl crate::Writable for CanMb14CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_MB14_CTRL to value 0x54"]
impl crate::Resettable for CanMb14CtrlSpec {
    const RESET_VALUE: u32 = 0x54;
}
