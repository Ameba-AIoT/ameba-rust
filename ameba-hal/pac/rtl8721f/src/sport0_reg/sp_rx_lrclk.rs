#[doc = "Register `SP_RX_LRCLK` reader"]
pub type R = crate::R<SpRxLrclkSpec>;
#[doc = "Register `SP_RX_LRCLK` writer"]
pub type W = crate::W<SpRxLrclkSpec>;
#[doc = "Field `tx_sport_compare_val` reader - X = (tx_sport_compare_val). When counter equal X. Sport will send tx_sport_interrupt to DSP. FW should take care X={32~134217727}"]
pub type TxSportCompareValR = crate::FieldReader<u32>;
#[doc = "Field `tx_sport_compare_val` writer - X = (tx_sport_compare_val). When counter equal X. Sport will send tx_sport_interrupt to DSP. FW should take care X={32~134217727}"]
pub type TxSportCompareValW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
#[doc = "Field `tx_fs_phase_rpt_b5` reader - tx_fs_phase_rptp\\[5\\] when using div64 fs_phase_trace_cnt"]
pub type TxFsPhaseRptB5R = crate::BitReader;
#[doc = "Field `en_tsft_trig_latch` reader - Enable wifi tsft trigger fs_phase_latch"]
pub type EnTsftTrigLatchR = crate::BitReader;
#[doc = "Field `en_tsft_trig_latch` writer - Enable wifi tsft trigger fs_phase_latch"]
pub type EnTsftTrigLatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `en_fs_phase_latch` writer - 0x001: Latch the value of tx_fs_phase_rpt, rx_fs_phase_rpt, tx_sport_sounter, rx_sport_sounter at the same time. This control bit is \"write 1 clear\" type"]
pub type EnFsPhaseLatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `en_tx_sport_interrupt` reader - Enable tx_sport_interrupt"]
pub type EnTxSportInterruptR = crate::BitReader;
#[doc = "Field `en_tx_sport_interrupt` writer - Enable tx_sport_interrupt"]
pub type EnTxSportInterruptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clr_tx_sport_rdy` writer - 0x001: clear tx_sport_interrupt signal This control bit is \"write 1 clear\" type For read, the read data is from clr_tx_sport_rdy"]
pub type ClrTxSportRdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:26 - X = (tx_sport_compare_val). When counter equal X. Sport will send tx_sport_interrupt to DSP. FW should take care X={32~134217727}"]
    #[inline(always)]
    pub fn tx_sport_compare_val(&self) -> TxSportCompareValR {
        TxSportCompareValR::new(self.bits & 0x07ff_ffff)
    }
    #[doc = "Bit 27 - tx_fs_phase_rptp\\[5\\] when using div64 fs_phase_trace_cnt"]
    #[inline(always)]
    pub fn tx_fs_phase_rpt_b5(&self) -> TxFsPhaseRptB5R {
        TxFsPhaseRptB5R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable wifi tsft trigger fs_phase_latch"]
    #[inline(always)]
    pub fn en_tsft_trig_latch(&self) -> EnTsftTrigLatchR {
        EnTsftTrigLatchR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable tx_sport_interrupt"]
    #[inline(always)]
    pub fn en_tx_sport_interrupt(&self) -> EnTxSportInterruptR {
        EnTxSportInterruptR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:26 - X = (tx_sport_compare_val). When counter equal X. Sport will send tx_sport_interrupt to DSP. FW should take care X={32~134217727}"]
    #[inline(always)]
    pub fn tx_sport_compare_val(&mut self) -> TxSportCompareValW<'_, SpRxLrclkSpec> {
        TxSportCompareValW::new(self, 0)
    }
    #[doc = "Bit 28 - Enable wifi tsft trigger fs_phase_latch"]
    #[inline(always)]
    pub fn en_tsft_trig_latch(&mut self) -> EnTsftTrigLatchW<'_, SpRxLrclkSpec> {
        EnTsftTrigLatchW::new(self, 28)
    }
    #[doc = "Bit 29 - 0x001: Latch the value of tx_fs_phase_rpt, rx_fs_phase_rpt, tx_sport_sounter, rx_sport_sounter at the same time. This control bit is \"write 1 clear\" type"]
    #[inline(always)]
    pub fn en_fs_phase_latch(&mut self) -> EnFsPhaseLatchW<'_, SpRxLrclkSpec> {
        EnFsPhaseLatchW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable tx_sport_interrupt"]
    #[inline(always)]
    pub fn en_tx_sport_interrupt(&mut self) -> EnTxSportInterruptW<'_, SpRxLrclkSpec> {
        EnTxSportInterruptW::new(self, 30)
    }
    #[doc = "Bit 31 - 0x001: clear tx_sport_interrupt signal This control bit is \"write 1 clear\" type For read, the read data is from clr_tx_sport_rdy"]
    #[inline(always)]
    pub fn clr_tx_sport_rdy(&mut self) -> ClrTxSportRdyW<'_, SpRxLrclkSpec> {
        ClrTxSportRdyW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_rx_lrclk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sp_rx_lrclk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpRxLrclkSpec;
impl crate::RegisterSpec for SpRxLrclkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_rx_lrclk::R`](R) reader structure"]
impl crate::Readable for SpRxLrclkSpec {}
#[doc = "`write(|w| ..)` method takes [`sp_rx_lrclk::W`](W) writer structure"]
impl crate::Writable for SpRxLrclkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SP_RX_LRCLK to value 0x40"]
impl crate::Resettable for SpRxLrclkSpec {
    const RESET_VALUE: u32 = 0x40;
}
