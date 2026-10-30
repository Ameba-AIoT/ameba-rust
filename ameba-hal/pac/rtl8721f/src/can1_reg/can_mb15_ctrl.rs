#[doc = "Register `CAN_MB15_CTRL` reader"]
pub type R = crate::R<CanMb15CtrlSpec>;
#[doc = "Register `CAN_MB15_CTRL` writer"]
pub type W = crate::W<CanMb15CtrlSpec>;
#[doc = "Field `can_msg15_ba` reader - message buffer base address in RAM"]
pub type CanMsg15BaR = crate::FieldReader<u16>;
#[doc = "Field `can_msg15_ba` writer - message buffer base address in RAM"]
pub type CanMsg15BaW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `can_msg15_rxdma_en` reader - dma enable when the message buffer used as rx"]
pub type CanMsg15RxdmaEnR = crate::BitReader;
#[doc = "Field `can_msg15_rxdma_en` writer - dma enable when the message buffer used as rx"]
pub type CanMsg15RxdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg15_ba(&self) -> CanMsg15BaR {
        CanMsg15BaR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg15_rxdma_en(&self) -> CanMsg15RxdmaEnR {
        CanMsg15RxdmaEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg15_ba(&mut self) -> CanMsg15BaW<'_, CanMb15CtrlSpec> {
        CanMsg15BaW::new(self, 0)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg15_rxdma_en(&mut self) -> CanMsg15RxdmaEnW<'_, CanMb15CtrlSpec> {
        CanMsg15RxdmaEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mb15_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_mb15_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanMb15CtrlSpec;
impl crate::RegisterSpec for CanMb15CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mb15_ctrl::R`](R) reader structure"]
impl crate::Readable for CanMb15CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`can_mb15_ctrl::W`](W) writer structure"]
impl crate::Writable for CanMb15CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_MB15_CTRL to value 0x5a"]
impl crate::Resettable for CanMb15CtrlSpec {
    const RESET_VALUE: u32 = 0x5a;
}
