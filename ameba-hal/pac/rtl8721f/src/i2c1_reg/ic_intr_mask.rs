#[doc = "Register `IC_INTR_MASK` reader"]
pub type R = crate::R<IcIntrMaskSpec>;
#[doc = "Register `IC_INTR_MASK` writer"]
pub type W = crate::W<IcIntrMaskSpec>;
#[doc = "Field `M_RX_UNDER` reader - "]
pub type MRxUnderR = crate::BitReader;
#[doc = "Field `M_RX_UNDER` writer - "]
pub type MRxUnderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M_RX_OVER` reader - "]
pub type MRxOverR = crate::BitReader;
#[doc = "Field `M_RX_OVER` writer - "]
pub type MRxOverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M_RX_FULL` reader - "]
pub type MRxFullR = crate::BitReader;
#[doc = "Field `M_RX_FULL` writer - "]
pub type MRxFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M_TX_OVER` reader - "]
pub type MTxOverR = crate::BitReader;
#[doc = "Field `M_TX_OVER` writer - "]
pub type MTxOverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M_TX_EMPTY` reader - "]
pub type MTxEmptyR = crate::BitReader;
#[doc = "Field `M_TX_EMPTY` writer - "]
pub type MTxEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M_RD_REQ` reader - "]
pub type MRdReqR = crate::BitReader;
#[doc = "Field `M_RD_REQ` writer - "]
pub type MRdReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M_TX_ABRT` reader - "]
pub type MTxAbrtR = crate::BitReader;
#[doc = "Field `M_TX_ABRT` writer - "]
pub type MTxAbrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M_RX_DONE` reader - "]
pub type MRxDoneR = crate::BitReader;
#[doc = "Field `M_RX_DONE` writer - "]
pub type MRxDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M_ACTIVITY` reader - "]
pub type MActivityR = crate::BitReader;
#[doc = "Field `M_ACTIVITY` writer - "]
pub type MActivityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M_STOP_DET` reader - "]
pub type MStopDetR = crate::BitReader;
#[doc = "Field `M_STOP_DET` writer - "]
pub type MStopDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M_START_DET` reader - "]
pub type MStartDetR = crate::BitReader;
#[doc = "Field `M_START_DET` writer - "]
pub type MStartDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M_GEN_CALL` reader - "]
pub type MGenCallR = crate::BitReader;
#[doc = "Field `M_GEN_CALL` writer - "]
pub type MGenCallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M_LP_WAKE_1` reader - "]
pub type MLpWake1R = crate::BitReader;
#[doc = "Field `M_LP_WAKE_1` writer - "]
pub type MLpWake1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M_LP_WAKE_2` reader - "]
pub type MLpWake2R = crate::BitReader;
#[doc = "Field `M_LP_WAKE_2` writer - "]
pub type MLpWake2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn m_rx_under(&self) -> MRxUnderR {
        MRxUnderR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn m_rx_over(&self) -> MRxOverR {
        MRxOverR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn m_rx_full(&self) -> MRxFullR {
        MRxFullR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn m_tx_over(&self) -> MTxOverR {
        MTxOverR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn m_tx_empty(&self) -> MTxEmptyR {
        MTxEmptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn m_rd_req(&self) -> MRdReqR {
        MRdReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn m_tx_abrt(&self) -> MTxAbrtR {
        MTxAbrtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn m_rx_done(&self) -> MRxDoneR {
        MRxDoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn m_activity(&self) -> MActivityR {
        MActivityR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn m_stop_det(&self) -> MStopDetR {
        MStopDetR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn m_start_det(&self) -> MStartDetR {
        MStartDetR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn m_gen_call(&self) -> MGenCallR {
        MGenCallR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn m_lp_wake_1(&self) -> MLpWake1R {
        MLpWake1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn m_lp_wake_2(&self) -> MLpWake2R {
        MLpWake2R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn m_rx_under(&mut self) -> MRxUnderW<'_, IcIntrMaskSpec> {
        MRxUnderW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn m_rx_over(&mut self) -> MRxOverW<'_, IcIntrMaskSpec> {
        MRxOverW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn m_rx_full(&mut self) -> MRxFullW<'_, IcIntrMaskSpec> {
        MRxFullW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn m_tx_over(&mut self) -> MTxOverW<'_, IcIntrMaskSpec> {
        MTxOverW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn m_tx_empty(&mut self) -> MTxEmptyW<'_, IcIntrMaskSpec> {
        MTxEmptyW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn m_rd_req(&mut self) -> MRdReqW<'_, IcIntrMaskSpec> {
        MRdReqW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn m_tx_abrt(&mut self) -> MTxAbrtW<'_, IcIntrMaskSpec> {
        MTxAbrtW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn m_rx_done(&mut self) -> MRxDoneW<'_, IcIntrMaskSpec> {
        MRxDoneW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn m_activity(&mut self) -> MActivityW<'_, IcIntrMaskSpec> {
        MActivityW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn m_stop_det(&mut self) -> MStopDetW<'_, IcIntrMaskSpec> {
        MStopDetW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn m_start_det(&mut self) -> MStartDetW<'_, IcIntrMaskSpec> {
        MStartDetW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn m_gen_call(&mut self) -> MGenCallW<'_, IcIntrMaskSpec> {
        MGenCallW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn m_lp_wake_1(&mut self) -> MLpWake1W<'_, IcIntrMaskSpec> {
        MLpWake1W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn m_lp_wake_2(&mut self) -> MLpWake2W<'_, IcIntrMaskSpec> {
        MLpWake2W::new(self, 13)
    }
}
#[doc = "I2C Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_intr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_intr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcIntrMaskSpec;
impl crate::RegisterSpec for IcIntrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_intr_mask::R`](R) reader structure"]
impl crate::Readable for IcIntrMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`ic_intr_mask::W`](W) writer structure"]
impl crate::Writable for IcIntrMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_INTR_MASK to value 0"]
impl crate::Resettable for IcIntrMaskSpec {}
