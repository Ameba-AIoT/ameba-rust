#[doc = "Register `CAN_INT_FLAG` reader"]
pub type R = crate::R<CanIntFlagSpec>;
#[doc = "Field `tx_int_flag` reader - tx interrupt flag, write to clear"]
pub type TxIntFlagR = crate::BitReader;
#[doc = "Field `rx_int_flag` reader - rx interrupt flag, write to clear"]
pub type RxIntFlagR = crate::BitReader;
#[doc = "Field `error_int_flag` reader - error interrupt flag, write to clear"]
pub type ErrorIntFlagR = crate::BitReader;
#[doc = "Field `wakeup_int_flag` reader - wakeup interrupt flag, write one to clear"]
pub type WakeupIntFlagR = crate::BitReader;
#[doc = "Field `busoff_int_flag` reader - busoff interrupt flag, write one to clear"]
pub type BusoffIntFlagR = crate::BitReader;
#[doc = "Field `ram_move_done_int_flag` reader - data move from register to CAN IP internal RAM finished interupt, move from RAM to register done will NOT generate interrupt, write one to clear"]
pub type RamMoveDoneIntFlagR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - tx interrupt flag, write to clear"]
    #[inline(always)]
    pub fn tx_int_flag(&self) -> TxIntFlagR {
        TxIntFlagR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - rx interrupt flag, write to clear"]
    #[inline(always)]
    pub fn rx_int_flag(&self) -> RxIntFlagR {
        RxIntFlagR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - error interrupt flag, write to clear"]
    #[inline(always)]
    pub fn error_int_flag(&self) -> ErrorIntFlagR {
        ErrorIntFlagR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - wakeup interrupt flag, write one to clear"]
    #[inline(always)]
    pub fn wakeup_int_flag(&self) -> WakeupIntFlagR {
        WakeupIntFlagR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - busoff interrupt flag, write one to clear"]
    #[inline(always)]
    pub fn busoff_int_flag(&self) -> BusoffIntFlagR {
        BusoffIntFlagR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - data move from register to CAN IP internal RAM finished interupt, move from RAM to register done will NOT generate interrupt, write one to clear"]
    #[inline(always)]
    pub fn ram_move_done_int_flag(&self) -> RamMoveDoneIntFlagR {
        RamMoveDoneIntFlagR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "CAN interrupt flags\n\nYou can [`read`](crate::Reg::read) this register and get [`can_int_flag::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanIntFlagSpec;
impl crate::RegisterSpec for CanIntFlagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_int_flag::R`](R) reader structure"]
impl crate::Readable for CanIntFlagSpec {}
#[doc = "`reset()` method sets CAN_INT_FLAG to value 0"]
impl crate::Resettable for CanIntFlagSpec {}
