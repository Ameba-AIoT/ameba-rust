#[doc = "Register `CAN_MB12_CTRL` reader"]
pub type R = crate::R<CanMb12CtrlSpec>;
#[doc = "Register `CAN_MB12_CTRL` writer"]
pub type W = crate::W<CanMb12CtrlSpec>;
#[doc = "Field `can_msg12_ba` reader - message buffer base address in RAM"]
pub type CanMsg12BaR = crate::FieldReader<u16>;
#[doc = "Field `can_msg12_ba` writer - message buffer base address in RAM"]
pub type CanMsg12BaW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `can_msg12_rxdma_en` reader - dma enable when the message buffer used as rx"]
pub type CanMsg12RxdmaEnR = crate::BitReader;
#[doc = "Field `can_msg12_rxdma_en` writer - dma enable when the message buffer used as rx"]
pub type CanMsg12RxdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg12_ba(&self) -> CanMsg12BaR {
        CanMsg12BaR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg12_rxdma_en(&self) -> CanMsg12RxdmaEnR {
        CanMsg12RxdmaEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg12_ba(&mut self) -> CanMsg12BaW<'_, CanMb12CtrlSpec> {
        CanMsg12BaW::new(self, 0)
    }
    #[doc = "Bit 24 - dma enable when the message buffer used as rx"]
    #[inline(always)]
    pub fn can_msg12_rxdma_en(&mut self) -> CanMsg12RxdmaEnW<'_, CanMb12CtrlSpec> {
        CanMsg12RxdmaEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mb12_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_mb12_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanMb12CtrlSpec;
impl crate::RegisterSpec for CanMb12CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mb12_ctrl::R`](R) reader structure"]
impl crate::Readable for CanMb12CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`can_mb12_ctrl::W`](W) writer structure"]
impl crate::Writable for CanMb12CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_MB12_CTRL to value 0x48"]
impl crate::Resettable for CanMb12CtrlSpec {
    const RESET_VALUE: u32 = 0x48;
}
