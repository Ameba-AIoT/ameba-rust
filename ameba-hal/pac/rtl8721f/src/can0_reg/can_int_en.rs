#[doc = "Register `CAN_INT_EN` reader"]
pub type R = crate::R<CanIntEnSpec>;
#[doc = "Register `CAN_INT_EN` writer"]
pub type W = crate::W<CanIntEnSpec>;
#[doc = "Field `tx_int_en` reader - tx interrupt enable"]
pub type TxIntEnR = crate::BitReader;
#[doc = "Field `tx_int_en` writer - tx interrupt enable"]
pub type TxIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_int_en` reader - rx interrupt enable"]
pub type RxIntEnR = crate::BitReader;
#[doc = "Field `rx_int_en` writer - rx interrupt enable"]
pub type RxIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `error_int_en` reader - error interrupt enable"]
pub type ErrorIntEnR = crate::BitReader;
#[doc = "Field `error_int_en` writer - error interrupt enable"]
pub type ErrorIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wakeup_int_en` reader - wakeup interrupt enbale"]
pub type WakeupIntEnR = crate::BitReader;
#[doc = "Field `wakeup_int_en` writer - wakeup interrupt enbale"]
pub type WakeupIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `busoff_int_en` reader - wakeup interrupt enbale"]
pub type BusoffIntEnR = crate::BitReader;
#[doc = "Field `busoff_int_en` writer - wakeup interrupt enbale"]
pub type BusoffIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ram_move_done_int_en` reader - ram move done interrupt enable"]
pub type RamMoveDoneIntEnR = crate::BitReader;
#[doc = "Field `ram_move_done_int_en` writer - ram move done interrupt enable"]
pub type RamMoveDoneIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_int_mask` reader - tx interrupt mask 1: mask 0: unmask"]
pub type TxIntMaskR = crate::BitReader;
#[doc = "Field `tx_int_mask` writer - tx interrupt mask 1: mask 0: unmask"]
pub type TxIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_int_mask` reader - rx interrupt mask 1: mask 0: unmask"]
pub type RxIntMaskR = crate::BitReader;
#[doc = "Field `rx_int_mask` writer - rx interrupt mask 1: mask 0: unmask"]
pub type RxIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `error_int_mask` reader - error interrupt mask 1: mask 0: unmask"]
pub type ErrorIntMaskR = crate::BitReader;
#[doc = "Field `error_int_mask` writer - error interrupt mask 1: mask 0: unmask"]
pub type ErrorIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wakeup_int_mask` reader - wakeup interrupt mask 1: mask 0: unmask"]
pub type WakeupIntMaskR = crate::BitReader;
#[doc = "Field `wakeup_int_mask` writer - wakeup interrupt mask 1: mask 0: unmask"]
pub type WakeupIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `busoff_int_mask` reader - wakeup interrupt mask 1: mask 0: unmask"]
pub type BusoffIntMaskR = crate::BitReader;
#[doc = "Field `busoff_int_mask` writer - wakeup interrupt mask 1: mask 0: unmask"]
pub type BusoffIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ram_move_done_int_mask` reader - ram move done interrupt mask 1: mask 0: unmask"]
pub type RamMoveDoneIntMaskR = crate::BitReader;
#[doc = "Field `ram_move_done_int_mask` writer - ram move done interrupt mask 1: mask 0: unmask"]
pub type RamMoveDoneIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - tx interrupt enable"]
    #[inline(always)]
    pub fn tx_int_en(&self) -> TxIntEnR {
        TxIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - rx interrupt enable"]
    #[inline(always)]
    pub fn rx_int_en(&self) -> RxIntEnR {
        RxIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - error interrupt enable"]
    #[inline(always)]
    pub fn error_int_en(&self) -> ErrorIntEnR {
        ErrorIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - wakeup interrupt enbale"]
    #[inline(always)]
    pub fn wakeup_int_en(&self) -> WakeupIntEnR {
        WakeupIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - wakeup interrupt enbale"]
    #[inline(always)]
    pub fn busoff_int_en(&self) -> BusoffIntEnR {
        BusoffIntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ram move done interrupt enable"]
    #[inline(always)]
    pub fn ram_move_done_int_en(&self) -> RamMoveDoneIntEnR {
        RamMoveDoneIntEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - tx interrupt mask 1: mask 0: unmask"]
    #[inline(always)]
    pub fn tx_int_mask(&self) -> TxIntMaskR {
        TxIntMaskR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - rx interrupt mask 1: mask 0: unmask"]
    #[inline(always)]
    pub fn rx_int_mask(&self) -> RxIntMaskR {
        RxIntMaskR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - error interrupt mask 1: mask 0: unmask"]
    #[inline(always)]
    pub fn error_int_mask(&self) -> ErrorIntMaskR {
        ErrorIntMaskR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - wakeup interrupt mask 1: mask 0: unmask"]
    #[inline(always)]
    pub fn wakeup_int_mask(&self) -> WakeupIntMaskR {
        WakeupIntMaskR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - wakeup interrupt mask 1: mask 0: unmask"]
    #[inline(always)]
    pub fn busoff_int_mask(&self) -> BusoffIntMaskR {
        BusoffIntMaskR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ram move done interrupt mask 1: mask 0: unmask"]
    #[inline(always)]
    pub fn ram_move_done_int_mask(&self) -> RamMoveDoneIntMaskR {
        RamMoveDoneIntMaskR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - tx interrupt enable"]
    #[inline(always)]
    pub fn tx_int_en(&mut self) -> TxIntEnW<'_, CanIntEnSpec> {
        TxIntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - rx interrupt enable"]
    #[inline(always)]
    pub fn rx_int_en(&mut self) -> RxIntEnW<'_, CanIntEnSpec> {
        RxIntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - error interrupt enable"]
    #[inline(always)]
    pub fn error_int_en(&mut self) -> ErrorIntEnW<'_, CanIntEnSpec> {
        ErrorIntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - wakeup interrupt enbale"]
    #[inline(always)]
    pub fn wakeup_int_en(&mut self) -> WakeupIntEnW<'_, CanIntEnSpec> {
        WakeupIntEnW::new(self, 3)
    }
    #[doc = "Bit 4 - wakeup interrupt enbale"]
    #[inline(always)]
    pub fn busoff_int_en(&mut self) -> BusoffIntEnW<'_, CanIntEnSpec> {
        BusoffIntEnW::new(self, 4)
    }
    #[doc = "Bit 5 - ram move done interrupt enable"]
    #[inline(always)]
    pub fn ram_move_done_int_en(&mut self) -> RamMoveDoneIntEnW<'_, CanIntEnSpec> {
        RamMoveDoneIntEnW::new(self, 5)
    }
    #[doc = "Bit 16 - tx interrupt mask 1: mask 0: unmask"]
    #[inline(always)]
    pub fn tx_int_mask(&mut self) -> TxIntMaskW<'_, CanIntEnSpec> {
        TxIntMaskW::new(self, 16)
    }
    #[doc = "Bit 17 - rx interrupt mask 1: mask 0: unmask"]
    #[inline(always)]
    pub fn rx_int_mask(&mut self) -> RxIntMaskW<'_, CanIntEnSpec> {
        RxIntMaskW::new(self, 17)
    }
    #[doc = "Bit 18 - error interrupt mask 1: mask 0: unmask"]
    #[inline(always)]
    pub fn error_int_mask(&mut self) -> ErrorIntMaskW<'_, CanIntEnSpec> {
        ErrorIntMaskW::new(self, 18)
    }
    #[doc = "Bit 19 - wakeup interrupt mask 1: mask 0: unmask"]
    #[inline(always)]
    pub fn wakeup_int_mask(&mut self) -> WakeupIntMaskW<'_, CanIntEnSpec> {
        WakeupIntMaskW::new(self, 19)
    }
    #[doc = "Bit 20 - wakeup interrupt mask 1: mask 0: unmask"]
    #[inline(always)]
    pub fn busoff_int_mask(&mut self) -> BusoffIntMaskW<'_, CanIntEnSpec> {
        BusoffIntMaskW::new(self, 20)
    }
    #[doc = "Bit 21 - ram move done interrupt mask 1: mask 0: unmask"]
    #[inline(always)]
    pub fn ram_move_done_int_mask(&mut self) -> RamMoveDoneIntMaskW<'_, CanIntEnSpec> {
        RamMoveDoneIntMaskW::new(self, 21)
    }
}
#[doc = "CAN interrupt control\n\nYou can [`read`](crate::Reg::read) this register and get [`can_int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanIntEnSpec;
impl crate::RegisterSpec for CanIntEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_int_en::R`](R) reader structure"]
impl crate::Readable for CanIntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`can_int_en::W`](W) writer structure"]
impl crate::Writable for CanIntEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_INT_EN to value 0"]
impl crate::Resettable for CanIntEnSpec {}
