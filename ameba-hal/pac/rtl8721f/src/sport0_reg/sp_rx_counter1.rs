#[doc = "Register `SP_RX_COUNTER1` reader"]
pub type R = crate::R<SpRxCounter1Spec>;
#[doc = "Register `SP_RX_COUNTER1` writer"]
pub type W = crate::W<SpRxCounter1Spec>;
#[doc = "Field `rx_sport_compare_val` reader - X = (rx_sport_compare_val). When counter equal X. Sport will send rx_sport_interrupt to DSP. FW should take care X={32~134217727}"]
pub type RxSportCompareValR = crate::FieldReader<u32>;
#[doc = "Field `rx_sport_compare_val` writer - X = (rx_sport_compare_val). When counter equal X. Sport will send rx_sport_interrupt to DSP. FW should take care X={32~134217727}"]
pub type RxSportCompareValW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
#[doc = "Field `rx_fs_phase_rpt_b5` reader - rx_fs_phase_rptp\\[5\\] when using div64 fs_phase_trace_cnt"]
pub type RxFsPhaseRptB5R = crate::BitReader;
#[doc = "Field `fs_phase_en_d64` reader - Enable div64 fs_phase_trace_cnt for both tx and rx"]
pub type FsPhaseEnD64R = crate::BitReader;
#[doc = "Field `fs_phase_en_d64` writer - Enable div64 fs_phase_trace_cnt for both tx and rx"]
pub type FsPhaseEnD64W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUMMY16` reader - "]
pub type Dummy16R = crate::BitReader;
#[doc = "Field `DUMMY16` writer - "]
pub type Dummy16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `en_rx_sport_interrupt` reader - Enable rx sport interrupt."]
pub type EnRxSportInterruptR = crate::BitReader;
#[doc = "Field `en_rx_sport_interrupt` writer - Enable rx sport interrupt."]
pub type EnRxSportInterruptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clr_rx_sport_rdy` writer - X = (tx_sport_compare_val). When counter equal X. Sport will send tx_sport_interrupt to DSP. FW should take care X={32~8191}"]
pub type ClrRxSportRdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:26 - X = (rx_sport_compare_val). When counter equal X. Sport will send rx_sport_interrupt to DSP. FW should take care X={32~134217727}"]
    #[inline(always)]
    pub fn rx_sport_compare_val(&self) -> RxSportCompareValR {
        RxSportCompareValR::new(self.bits & 0x07ff_ffff)
    }
    #[doc = "Bit 27 - rx_fs_phase_rptp\\[5\\] when using div64 fs_phase_trace_cnt"]
    #[inline(always)]
    pub fn rx_fs_phase_rpt_b5(&self) -> RxFsPhaseRptB5R {
        RxFsPhaseRptB5R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable div64 fs_phase_trace_cnt for both tx and rx"]
    #[inline(always)]
    pub fn fs_phase_en_d64(&self) -> FsPhaseEnD64R {
        FsPhaseEnD64R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dummy16(&self) -> Dummy16R {
        Dummy16R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable rx sport interrupt."]
    #[inline(always)]
    pub fn en_rx_sport_interrupt(&self) -> EnRxSportInterruptR {
        EnRxSportInterruptR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:26 - X = (rx_sport_compare_val). When counter equal X. Sport will send rx_sport_interrupt to DSP. FW should take care X={32~134217727}"]
    #[inline(always)]
    pub fn rx_sport_compare_val(&mut self) -> RxSportCompareValW<'_, SpRxCounter1Spec> {
        RxSportCompareValW::new(self, 0)
    }
    #[doc = "Bit 28 - Enable div64 fs_phase_trace_cnt for both tx and rx"]
    #[inline(always)]
    pub fn fs_phase_en_d64(&mut self) -> FsPhaseEnD64W<'_, SpRxCounter1Spec> {
        FsPhaseEnD64W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dummy16(&mut self) -> Dummy16W<'_, SpRxCounter1Spec> {
        Dummy16W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable rx sport interrupt."]
    #[inline(always)]
    pub fn en_rx_sport_interrupt(&mut self) -> EnRxSportInterruptW<'_, SpRxCounter1Spec> {
        EnRxSportInterruptW::new(self, 30)
    }
    #[doc = "Bit 31 - X = (tx_sport_compare_val). When counter equal X. Sport will send tx_sport_interrupt to DSP. FW should take care X={32~8191}"]
    #[inline(always)]
    pub fn clr_rx_sport_rdy(&mut self) -> ClrRxSportRdyW<'_, SpRxCounter1Spec> {
        ClrRxSportRdyW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_rx_counter1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sp_rx_counter1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpRxCounter1Spec;
impl crate::RegisterSpec for SpRxCounter1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_rx_counter1::R`](R) reader structure"]
impl crate::Readable for SpRxCounter1Spec {}
#[doc = "`write(|w| ..)` method takes [`sp_rx_counter1::W`](W) writer structure"]
impl crate::Writable for SpRxCounter1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SP_RX_COUNTER1 to value 0x40"]
impl crate::Resettable for SpRxCounter1Spec {
    const RESET_VALUE: u32 = 0x40;
}
