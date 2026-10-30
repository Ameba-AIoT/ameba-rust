#[doc = "Register `IPC_ISR` reader"]
pub type R = crate::R<IpcIsrSpec>;
#[doc = "Field `isr_rx0_full_status0` reader - Rx channel 0 full interrupt status of CPU1. The corresponding Rx full status bit will be set by CPU0's Tx data register (tx0_data) channel 0. It will be cleared by software writing 1."]
pub type IsrRx0FullStatus0R = crate::BitReader;
#[doc = "Field `isr_rx0_full_status1` reader - Refer to the description of the isr_rx0_full_status0."]
pub type IsrRx0FullStatus1R = crate::BitReader;
#[doc = "Field `isr_rx0_full_status2` reader - Refer to the description of the isr_rx0_full_status0."]
pub type IsrRx0FullStatus2R = crate::BitReader;
#[doc = "Field `isr_rx0_full_status3` reader - Refer to the description of the isr_rx0_full_status0."]
pub type IsrRx0FullStatus3R = crate::BitReader;
#[doc = "Field `isr_rx0_full_status4` reader - Refer to the description of the isr_rx0_full_status0."]
pub type IsrRx0FullStatus4R = crate::BitReader;
#[doc = "Field `isr_rx0_full_status5` reader - Refer to the description of the isr_rx0_full_status0."]
pub type IsrRx0FullStatus5R = crate::BitReader;
#[doc = "Field `isr_rx0_full_status6` reader - Refer to the description of the isr_rx0_full_status0."]
pub type IsrRx0FullStatus6R = crate::BitReader;
#[doc = "Field `isr_rx0_full_status7` reader - Refer to the description of the isr_rx0_full_status0."]
pub type IsrRx0FullStatus7R = crate::BitReader;
#[doc = "Field `isr_rx0_full_status8` reader - Refer to the description of the isr_rx0_full_status0."]
pub type IsrRx0FullStatus8R = crate::BitReader;
#[doc = "Field `isr_rx0_full_status9` reader - Refer to the description of the isr_rx0_full_status0."]
pub type IsrRx0FullStatus9R = crate::BitReader;
#[doc = "Field `isr_rx0_full_status10` reader - Refer to the description of the isr_rx0_full_status0."]
pub type IsrRx0FullStatus10R = crate::BitReader;
#[doc = "Field `isr_rx0_full_status11` reader - Refer to the description of the isr_rx0_full_status0."]
pub type IsrRx0FullStatus11R = crate::BitReader;
#[doc = "Field `isr_rx0_full_status12` reader - Refer to the description of the isr_rx0_full_status0."]
pub type IsrRx0FullStatus12R = crate::BitReader;
#[doc = "Field `isr_rx0_full_status13` reader - Refer to the description of the isr_rx0_full_status0."]
pub type IsrRx0FullStatus13R = crate::BitReader;
#[doc = "Field `isr_rx0_full_status14` reader - Refer to the description of the isr_rx0_full_status0."]
pub type IsrRx0FullStatus14R = crate::BitReader;
#[doc = "Field `isr_rx0_full_status15` reader - Refer to the description of the isr_rx0_full_status0."]
pub type IsrRx0FullStatus15R = crate::BitReader;
#[doc = "Field `isr_tx0_empty_status0` reader - Tx channel 0 empty interrupt status of CPU1 transmit to CPU0. If the corresponding Rx full status bit in REG_ISR_CPU0 (eg.isr_rx0_full_status0) is cleared, the corresponding Tx empty status bit will be set automatically. It will be cleared by software writing 1."]
pub type IsrTx0EmptyStatus0R = crate::BitReader;
#[doc = "Field `isr_tx0_empty_status1` reader - Refer to the description of the isr_tx0_empty_status0"]
pub type IsrTx0EmptyStatus1R = crate::BitReader;
#[doc = "Field `isr_tx0_empty_status2` reader - Refer to the description of the isr_tx0_empty_status0"]
pub type IsrTx0EmptyStatus2R = crate::BitReader;
#[doc = "Field `isr_tx0_empty_status3` reader - Refer to the description of the isr_tx0_empty_status0"]
pub type IsrTx0EmptyStatus3R = crate::BitReader;
#[doc = "Field `isr_tx0_empty_status4` reader - Refer to the description of the isr_tx0_empty_status0"]
pub type IsrTx0EmptyStatus4R = crate::BitReader;
#[doc = "Field `isr_tx0_empty_status5` reader - Refer to the description of the isr_tx0_empty_status0"]
pub type IsrTx0EmptyStatus5R = crate::BitReader;
#[doc = "Field `isr_tx0_empty_status6` reader - Refer to the description of the isr_tx0_empty_status0"]
pub type IsrTx0EmptyStatus6R = crate::BitReader;
#[doc = "Field `isr_tx0_empty_status7` reader - Refer to the description of the isr_tx0_empty_status0"]
pub type IsrTx0EmptyStatus7R = crate::BitReader;
#[doc = "Field `isr_tx0_empty_status8` reader - Refer to the description of the isr_tx0_empty_status0"]
pub type IsrTx0EmptyStatus8R = crate::BitReader;
#[doc = "Field `isr_tx0_empty_status9` reader - Refer to the description of the isr_tx0_empty_status0"]
pub type IsrTx0EmptyStatus9R = crate::BitReader;
#[doc = "Field `isr_tx0_empty_status10` reader - Refer to the description of the isr_tx0_empty_status0"]
pub type IsrTx0EmptyStatus10R = crate::BitReader;
#[doc = "Field `isr_tx0_empty_status11` reader - Refer to the description of the isr_tx0_empty_status0"]
pub type IsrTx0EmptyStatus11R = crate::BitReader;
#[doc = "Field `isr_tx0_empty_status12` reader - Refer to the description of the isr_tx0_empty_status0"]
pub type IsrTx0EmptyStatus12R = crate::BitReader;
#[doc = "Field `isr_tx0_empty_status13` reader - Refer to the description of the isr_tx0_empty_status0"]
pub type IsrTx0EmptyStatus13R = crate::BitReader;
#[doc = "Field `isr_tx0_empty_status14` reader - Refer to the description of the isr_tx0_empty_status0"]
pub type IsrTx0EmptyStatus14R = crate::BitReader;
#[doc = "Field `isr_tx0_empty_status15` reader - Refer to the description of the isr_tx0_empty_status0"]
pub type IsrTx0EmptyStatus15R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Rx channel 0 full interrupt status of CPU1. The corresponding Rx full status bit will be set by CPU0's Tx data register (tx0_data) channel 0. It will be cleared by software writing 1."]
    #[inline(always)]
    pub fn isr_rx0_full_status0(&self) -> IsrRx0FullStatus0R {
        IsrRx0FullStatus0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Refer to the description of the isr_rx0_full_status0."]
    #[inline(always)]
    pub fn isr_rx0_full_status1(&self) -> IsrRx0FullStatus1R {
        IsrRx0FullStatus1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Refer to the description of the isr_rx0_full_status0."]
    #[inline(always)]
    pub fn isr_rx0_full_status2(&self) -> IsrRx0FullStatus2R {
        IsrRx0FullStatus2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Refer to the description of the isr_rx0_full_status0."]
    #[inline(always)]
    pub fn isr_rx0_full_status3(&self) -> IsrRx0FullStatus3R {
        IsrRx0FullStatus3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Refer to the description of the isr_rx0_full_status0."]
    #[inline(always)]
    pub fn isr_rx0_full_status4(&self) -> IsrRx0FullStatus4R {
        IsrRx0FullStatus4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Refer to the description of the isr_rx0_full_status0."]
    #[inline(always)]
    pub fn isr_rx0_full_status5(&self) -> IsrRx0FullStatus5R {
        IsrRx0FullStatus5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Refer to the description of the isr_rx0_full_status0."]
    #[inline(always)]
    pub fn isr_rx0_full_status6(&self) -> IsrRx0FullStatus6R {
        IsrRx0FullStatus6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Refer to the description of the isr_rx0_full_status0."]
    #[inline(always)]
    pub fn isr_rx0_full_status7(&self) -> IsrRx0FullStatus7R {
        IsrRx0FullStatus7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Refer to the description of the isr_rx0_full_status0."]
    #[inline(always)]
    pub fn isr_rx0_full_status8(&self) -> IsrRx0FullStatus8R {
        IsrRx0FullStatus8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Refer to the description of the isr_rx0_full_status0."]
    #[inline(always)]
    pub fn isr_rx0_full_status9(&self) -> IsrRx0FullStatus9R {
        IsrRx0FullStatus9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Refer to the description of the isr_rx0_full_status0."]
    #[inline(always)]
    pub fn isr_rx0_full_status10(&self) -> IsrRx0FullStatus10R {
        IsrRx0FullStatus10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Refer to the description of the isr_rx0_full_status0."]
    #[inline(always)]
    pub fn isr_rx0_full_status11(&self) -> IsrRx0FullStatus11R {
        IsrRx0FullStatus11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Refer to the description of the isr_rx0_full_status0."]
    #[inline(always)]
    pub fn isr_rx0_full_status12(&self) -> IsrRx0FullStatus12R {
        IsrRx0FullStatus12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Refer to the description of the isr_rx0_full_status0."]
    #[inline(always)]
    pub fn isr_rx0_full_status13(&self) -> IsrRx0FullStatus13R {
        IsrRx0FullStatus13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Refer to the description of the isr_rx0_full_status0."]
    #[inline(always)]
    pub fn isr_rx0_full_status14(&self) -> IsrRx0FullStatus14R {
        IsrRx0FullStatus14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Refer to the description of the isr_rx0_full_status0."]
    #[inline(always)]
    pub fn isr_rx0_full_status15(&self) -> IsrRx0FullStatus15R {
        IsrRx0FullStatus15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Tx channel 0 empty interrupt status of CPU1 transmit to CPU0. If the corresponding Rx full status bit in REG_ISR_CPU0 (eg.isr_rx0_full_status0) is cleared, the corresponding Tx empty status bit will be set automatically. It will be cleared by software writing 1."]
    #[inline(always)]
    pub fn isr_tx0_empty_status0(&self) -> IsrTx0EmptyStatus0R {
        IsrTx0EmptyStatus0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Refer to the description of the isr_tx0_empty_status0"]
    #[inline(always)]
    pub fn isr_tx0_empty_status1(&self) -> IsrTx0EmptyStatus1R {
        IsrTx0EmptyStatus1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Refer to the description of the isr_tx0_empty_status0"]
    #[inline(always)]
    pub fn isr_tx0_empty_status2(&self) -> IsrTx0EmptyStatus2R {
        IsrTx0EmptyStatus2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Refer to the description of the isr_tx0_empty_status0"]
    #[inline(always)]
    pub fn isr_tx0_empty_status3(&self) -> IsrTx0EmptyStatus3R {
        IsrTx0EmptyStatus3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Refer to the description of the isr_tx0_empty_status0"]
    #[inline(always)]
    pub fn isr_tx0_empty_status4(&self) -> IsrTx0EmptyStatus4R {
        IsrTx0EmptyStatus4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Refer to the description of the isr_tx0_empty_status0"]
    #[inline(always)]
    pub fn isr_tx0_empty_status5(&self) -> IsrTx0EmptyStatus5R {
        IsrTx0EmptyStatus5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Refer to the description of the isr_tx0_empty_status0"]
    #[inline(always)]
    pub fn isr_tx0_empty_status6(&self) -> IsrTx0EmptyStatus6R {
        IsrTx0EmptyStatus6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Refer to the description of the isr_tx0_empty_status0"]
    #[inline(always)]
    pub fn isr_tx0_empty_status7(&self) -> IsrTx0EmptyStatus7R {
        IsrTx0EmptyStatus7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Refer to the description of the isr_tx0_empty_status0"]
    #[inline(always)]
    pub fn isr_tx0_empty_status8(&self) -> IsrTx0EmptyStatus8R {
        IsrTx0EmptyStatus8R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Refer to the description of the isr_tx0_empty_status0"]
    #[inline(always)]
    pub fn isr_tx0_empty_status9(&self) -> IsrTx0EmptyStatus9R {
        IsrTx0EmptyStatus9R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Refer to the description of the isr_tx0_empty_status0"]
    #[inline(always)]
    pub fn isr_tx0_empty_status10(&self) -> IsrTx0EmptyStatus10R {
        IsrTx0EmptyStatus10R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Refer to the description of the isr_tx0_empty_status0"]
    #[inline(always)]
    pub fn isr_tx0_empty_status11(&self) -> IsrTx0EmptyStatus11R {
        IsrTx0EmptyStatus11R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Refer to the description of the isr_tx0_empty_status0"]
    #[inline(always)]
    pub fn isr_tx0_empty_status12(&self) -> IsrTx0EmptyStatus12R {
        IsrTx0EmptyStatus12R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Refer to the description of the isr_tx0_empty_status0"]
    #[inline(always)]
    pub fn isr_tx0_empty_status13(&self) -> IsrTx0EmptyStatus13R {
        IsrTx0EmptyStatus13R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Refer to the description of the isr_tx0_empty_status0"]
    #[inline(always)]
    pub fn isr_tx0_empty_status14(&self) -> IsrTx0EmptyStatus14R {
        IsrTx0EmptyStatus14R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Refer to the description of the isr_tx0_empty_status0"]
    #[inline(always)]
    pub fn isr_tx0_empty_status15(&self) -> IsrTx0EmptyStatus15R {
        IsrTx0EmptyStatus15R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Interrupt Empty Full Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc_isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpcIsrSpec;
impl crate::RegisterSpec for IpcIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc_isr::R`](R) reader structure"]
impl crate::Readable for IpcIsrSpec {}
#[doc = "`reset()` method sets IPC_ISR to value 0"]
impl crate::Resettable for IpcIsrSpec {}
