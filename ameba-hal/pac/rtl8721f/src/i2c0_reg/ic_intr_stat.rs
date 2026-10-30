#[doc = "Register `IC_INTR_STAT` reader"]
pub type R = crate::R<IcIntrStatSpec>;
#[doc = "Field `R_RX_UNDER` reader - "]
pub type RRxUnderR = crate::BitReader;
#[doc = "Field `R_RX_OVER` reader - "]
pub type RRxOverR = crate::BitReader;
#[doc = "Field `R_RX_FULL` reader - "]
pub type RRxFullR = crate::BitReader;
#[doc = "Field `R_TX_OVER` reader - "]
pub type RTxOverR = crate::BitReader;
#[doc = "Field `R_TX_EMPTY` reader - "]
pub type RTxEmptyR = crate::BitReader;
#[doc = "Field `R_RD_REQ` reader - "]
pub type RRdReqR = crate::BitReader;
#[doc = "Field `R_TX_ABRT` reader - "]
pub type RTxAbrtR = crate::BitReader;
#[doc = "Field `R_RX_DONE` reader - "]
pub type RRxDoneR = crate::BitReader;
#[doc = "Field `R_ACTIVITY` reader - "]
pub type RActivityR = crate::BitReader;
#[doc = "Field `R_STOP_DET` reader - "]
pub type RStopDetR = crate::BitReader;
#[doc = "Field `R_START_DET` reader - "]
pub type RStartDetR = crate::BitReader;
#[doc = "Field `R_GEN_CALL` reader - "]
pub type RGenCallR = crate::BitReader;
#[doc = "Field `R_LP_WAKE_1` reader - "]
pub type RLpWake1R = crate::BitReader;
#[doc = "Field `R_LP_WAKE_2` reader - "]
pub type RLpWake2R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn r_rx_under(&self) -> RRxUnderR {
        RRxUnderR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn r_rx_over(&self) -> RRxOverR {
        RRxOverR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn r_rx_full(&self) -> RRxFullR {
        RRxFullR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn r_tx_over(&self) -> RTxOverR {
        RTxOverR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn r_tx_empty(&self) -> RTxEmptyR {
        RTxEmptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn r_rd_req(&self) -> RRdReqR {
        RRdReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn r_tx_abrt(&self) -> RTxAbrtR {
        RTxAbrtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn r_rx_done(&self) -> RRxDoneR {
        RRxDoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn r_activity(&self) -> RActivityR {
        RActivityR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn r_stop_det(&self) -> RStopDetR {
        RStopDetR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn r_start_det(&self) -> RStartDetR {
        RStartDetR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn r_gen_call(&self) -> RGenCallR {
        RGenCallR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn r_lp_wake_1(&self) -> RLpWake1R {
        RLpWake1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn r_lp_wake_2(&self) -> RLpWake2R {
        RLpWake2R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "I2C Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_intr_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcIntrStatSpec;
impl crate::RegisterSpec for IcIntrStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_intr_stat::R`](R) reader structure"]
impl crate::Readable for IcIntrStatSpec {}
#[doc = "`reset()` method sets IC_INTR_STAT to value 0"]
impl crate::Resettable for IcIntrStatSpec {}
