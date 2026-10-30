#[doc = "Register `CAN_MB10_CTRL` reader"]
pub type R = crate::R<CanMb10CtrlSpec>;
#[doc = "Register `CAN_MB10_CTRL` writer"]
pub type W = crate::W<CanMb10CtrlSpec>;
#[doc = "Field `can_msg10_ba` reader - message buffer base address in RAM"]
pub type CanMsg10BaR = crate::FieldReader<u16>;
#[doc = "Field `can_msg10_ba` writer - message buffer base address in RAM"]
pub type CanMsg10BaW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `can_msg10_rxdma_en` reader - dma enable when the message buffer used as rx"]
pub type CanMsg10RxdmaEnR = crate::BitReader;
#[doc = "Field `can_msg10_rxdma_en` writer - dma enable when the message buffer used as rx"]
pub type CanMsg10RxdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg10_ba(&self) -> CanMsg10BaR {
        CanMsg10BaR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg10_rxdma_en(&self) -> CanMsg10RxdmaEnR {
        CanMsg10RxdmaEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg10_ba(&mut self) -> CanMsg10BaW<'_, CanMb10CtrlSpec> {
        CanMsg10BaW::new(self, 0)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg10_rxdma_en(&mut self) -> CanMsg10RxdmaEnW<'_, CanMb10CtrlSpec> {
        CanMsg10RxdmaEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mb10_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_mb10_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanMb10CtrlSpec;
impl crate::RegisterSpec for CanMb10CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mb10_ctrl::R`](R) reader structure"]
impl crate::Readable for CanMb10CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`can_mb10_ctrl::W`](W) writer structure"]
impl crate::Writable for CanMb10CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_MB10_CTRL to value 0x3c"]
impl crate::Resettable for CanMb10CtrlSpec {
    const RESET_VALUE: u32 = 0x3c;
}
