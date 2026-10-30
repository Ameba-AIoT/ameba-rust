#[doc = "Register `SP_INT_CTRL` reader"]
pub type R = crate::R<SpIntCtrlSpec>;
#[doc = "Register `SP_INT_CTRL` writer"]
pub type W = crate::W<SpIntCtrlSpec>;
#[doc = "Field `tx_dsp_clear_int_0` reader - For DSP mode (bypass GDMA), F/W writes 1'b1 and then 1'b0 to clear TX interrupt. Note: TX interrupt is to indicate that DSP can write audio data to TX FIFO_0"]
pub type TxDspClearInt0R = crate::BitReader;
#[doc = "Field `tx_dsp_clear_int_0` writer - For DSP mode (bypass GDMA), F/W writes 1'b1 and then 1'b0 to clear TX interrupt. Note: TX interrupt is to indicate that DSP can write audio data to TX FIFO_0"]
pub type TxDspClearInt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_dsp_clear_int_0` reader - For DSP mode (bypass GDMA), F/W writes 1'b1 and then 1'b0 to clear RX interrupt. Note: RX interrupt is to indicate that DSP can get audio data from RX FIFO_0"]
pub type RxDspClearInt0R = crate::BitReader;
#[doc = "Field `rx_dsp_clear_int_0` writer - For DSP mode (bypass GDMA), F/W writes 1'b1 and then 1'b0 to clear RX interrupt. Note: RX interrupt is to indicate that DSP can get audio data from RX FIFO_0"]
pub type RxDspClearInt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `intr_clr_0` reader - Bit\\[2\\]: for the interrupt of \"tx_fifo_full_intr\" Bit\\[3\\]: for the interrupt of \"rx_fifo_full_intr\" Bit\\[4\\]: for the interrupt of \"tx_fifo_empty_intr\" Bit\\[5\\]: for the interrupt of \"rx_fifo_empty_intr\" Bit\\[6\\]: Reserved"]
pub type IntrClr0R = crate::FieldReader;
#[doc = "Field `intr_clr_0` writer - Bit\\[2\\]: for the interrupt of \"tx_fifo_full_intr\" Bit\\[3\\]: for the interrupt of \"rx_fifo_full_intr\" Bit\\[4\\]: for the interrupt of \"tx_fifo_empty_intr\" Bit\\[5\\]: for the interrupt of \"rx_fifo_empty_intr\" Bit\\[6\\]: Reserved"]
pub type IntrClr0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `tx_dsp_clear_int_1` reader - For DSP mode (bypass GDMA), F/W writes 1'b1 and then 1'b0 to clear TX interrupt. Note: TX interrupt is to indicate that DSP can write audio data to TX FIFO_1"]
pub type TxDspClearInt1R = crate::BitReader;
#[doc = "Field `tx_dsp_clear_int_1` writer - For DSP mode (bypass GDMA), F/W writes 1'b1 and then 1'b0 to clear TX interrupt. Note: TX interrupt is to indicate that DSP can write audio data to TX FIFO_1"]
pub type TxDspClearInt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_dsp_clear_int_1` reader - For DSP mode (bypass GDMA), F/W writes 1'b1 and then 1'b0 to clear RX interrupt. Note: RX interrupt is to indicate that DSP can get audio data from RX FIFO_1"]
pub type RxDspClearInt1R = crate::BitReader;
#[doc = "Field `rx_dsp_clear_int_1` writer - For DSP mode (bypass GDMA), F/W writes 1'b1 and then 1'b0 to clear RX interrupt. Note: RX interrupt is to indicate that DSP can get audio data from RX FIFO_1"]
pub type RxDspClearInt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `intr_clr_1` reader - Bit\\[9\\]: for the interrupt of \"tx_fifo_full_intr_1\" Bit\\[10\\]: for the interrupt of \"rx_fifo_full_intr_1\" Bit\\[11\\]: for the interrupt of \"tx_fifo_empty_intr_1\" Bit\\[12\\]: for the interrupt of \"rx_fifo_empty_intr_1\" Bit\\[13\\]: Reserved"]
pub type IntrClr1R = crate::FieldReader;
#[doc = "Field `intr_clr_1` writer - Bit\\[9\\]: for the interrupt of \"tx_fifo_full_intr_1\" Bit\\[10\\]: for the interrupt of \"rx_fifo_full_intr_1\" Bit\\[11\\]: for the interrupt of \"tx_fifo_empty_intr_1\" Bit\\[12\\]: for the interrupt of \"rx_fifo_empty_intr_1\" Bit\\[13\\]: Reserved"]
pub type IntrClr1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `lrc_start_intr` reader - "]
pub type LrcStartIntrR = crate::BitReader;
#[doc = "Field `lrc_start_intr` writer - "]
pub type LrcStartIntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lrc_stop_intr` reader - "]
pub type LrcStopIntrR = crate::BitReader;
#[doc = "Field `lrc_stop_intr` writer - "]
pub type LrcStopIntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `int_enable_dsp_0` reader - Bit\\[16\\]: for the interrupt of \"sp_ready_to_tx\" Bit\\[17\\]: for the interrupt of \"sp_ready_to_rx\" Bit\\[18\\]: for the interrupt of \"tx_fifo_full_intr\" Bit\\[19\\]: for the interrupt of \"rx_fifo_full_intr\" Bit\\[20\\]: for the interrupt of \"tx_fifo_empty_intr\" Bit\\[21\\]: for the interrupt of \"rx_fifo_empty_intr\" Bit\\[22\\]: for the interrupt of \"tx_i2s_idle\" Bit\\[23\\]: Reserved"]
pub type IntEnableDsp0R = crate::FieldReader;
#[doc = "Field `int_enable_dsp_0` writer - Bit\\[16\\]: for the interrupt of \"sp_ready_to_tx\" Bit\\[17\\]: for the interrupt of \"sp_ready_to_rx\" Bit\\[18\\]: for the interrupt of \"tx_fifo_full_intr\" Bit\\[19\\]: for the interrupt of \"rx_fifo_full_intr\" Bit\\[20\\]: for the interrupt of \"tx_fifo_empty_intr\" Bit\\[21\\]: for the interrupt of \"rx_fifo_empty_intr\" Bit\\[22\\]: for the interrupt of \"tx_i2s_idle\" Bit\\[23\\]: Reserved"]
pub type IntEnableDsp0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `int_enable_dsp_1` reader - Bit\\[24\\]: for the interrupt of \"sp_ready_to_tx_1\" Bit\\[25\\]: for the interrupt of \"sp_ready_to_rx_1\" Bit\\[26\\]: for the interrupt of \"tx_fifo_full_intr_1\" Bit\\[27\\]: for the interrupt of \"rx_fifo_full_intr_1\" Bit\\[28\\]: for the interrupt of \"tx_fifo_empty_intr_1\" Bit\\[29\\]: for the interrupt of \"rx_fifo_empty_intr_1\" Bit\\[30\\]: for the interrupt of \"tx_i2s_idle_1\" Bit\\[31\\]: Reserved"]
pub type IntEnableDsp1R = crate::FieldReader;
#[doc = "Field `int_enable_dsp_1` writer - Bit\\[24\\]: for the interrupt of \"sp_ready_to_tx_1\" Bit\\[25\\]: for the interrupt of \"sp_ready_to_rx_1\" Bit\\[26\\]: for the interrupt of \"tx_fifo_full_intr_1\" Bit\\[27\\]: for the interrupt of \"rx_fifo_full_intr_1\" Bit\\[28\\]: for the interrupt of \"tx_fifo_empty_intr_1\" Bit\\[29\\]: for the interrupt of \"rx_fifo_empty_intr_1\" Bit\\[30\\]: for the interrupt of \"tx_i2s_idle_1\" Bit\\[31\\]: Reserved"]
pub type IntEnableDsp1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - For DSP mode (bypass GDMA), F/W writes 1'b1 and then 1'b0 to clear TX interrupt. Note: TX interrupt is to indicate that DSP can write audio data to TX FIFO_0"]
    #[inline(always)]
    pub fn tx_dsp_clear_int_0(&self) -> TxDspClearInt0R {
        TxDspClearInt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - For DSP mode (bypass GDMA), F/W writes 1'b1 and then 1'b0 to clear RX interrupt. Note: RX interrupt is to indicate that DSP can get audio data from RX FIFO_0"]
    #[inline(always)]
    pub fn rx_dsp_clear_int_0(&self) -> RxDspClearInt0R {
        RxDspClearInt0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Bit\\[2\\]: for the interrupt of \"tx_fifo_full_intr\" Bit\\[3\\]: for the interrupt of \"rx_fifo_full_intr\" Bit\\[4\\]: for the interrupt of \"tx_fifo_empty_intr\" Bit\\[5\\]: for the interrupt of \"rx_fifo_empty_intr\" Bit\\[6\\]: Reserved"]
    #[inline(always)]
    pub fn intr_clr_0(&self) -> IntrClr0R {
        IntrClr0R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - For DSP mode (bypass GDMA), F/W writes 1'b1 and then 1'b0 to clear TX interrupt. Note: TX interrupt is to indicate that DSP can write audio data to TX FIFO_1"]
    #[inline(always)]
    pub fn tx_dsp_clear_int_1(&self) -> TxDspClearInt1R {
        TxDspClearInt1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - For DSP mode (bypass GDMA), F/W writes 1'b1 and then 1'b0 to clear RX interrupt. Note: RX interrupt is to indicate that DSP can get audio data from RX FIFO_1"]
    #[inline(always)]
    pub fn rx_dsp_clear_int_1(&self) -> RxDspClearInt1R {
        RxDspClearInt1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:13 - Bit\\[9\\]: for the interrupt of \"tx_fifo_full_intr_1\" Bit\\[10\\]: for the interrupt of \"rx_fifo_full_intr_1\" Bit\\[11\\]: for the interrupt of \"tx_fifo_empty_intr_1\" Bit\\[12\\]: for the interrupt of \"rx_fifo_empty_intr_1\" Bit\\[13\\]: Reserved"]
    #[inline(always)]
    pub fn intr_clr_1(&self) -> IntrClr1R {
        IntrClr1R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn lrc_start_intr(&self) -> LrcStartIntrR {
        LrcStartIntrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn lrc_stop_intr(&self) -> LrcStopIntrR {
        LrcStopIntrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Bit\\[16\\]: for the interrupt of \"sp_ready_to_tx\" Bit\\[17\\]: for the interrupt of \"sp_ready_to_rx\" Bit\\[18\\]: for the interrupt of \"tx_fifo_full_intr\" Bit\\[19\\]: for the interrupt of \"rx_fifo_full_intr\" Bit\\[20\\]: for the interrupt of \"tx_fifo_empty_intr\" Bit\\[21\\]: for the interrupt of \"rx_fifo_empty_intr\" Bit\\[22\\]: for the interrupt of \"tx_i2s_idle\" Bit\\[23\\]: Reserved"]
    #[inline(always)]
    pub fn int_enable_dsp_0(&self) -> IntEnableDsp0R {
        IntEnableDsp0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Bit\\[24\\]: for the interrupt of \"sp_ready_to_tx_1\" Bit\\[25\\]: for the interrupt of \"sp_ready_to_rx_1\" Bit\\[26\\]: for the interrupt of \"tx_fifo_full_intr_1\" Bit\\[27\\]: for the interrupt of \"rx_fifo_full_intr_1\" Bit\\[28\\]: for the interrupt of \"tx_fifo_empty_intr_1\" Bit\\[29\\]: for the interrupt of \"rx_fifo_empty_intr_1\" Bit\\[30\\]: for the interrupt of \"tx_i2s_idle_1\" Bit\\[31\\]: Reserved"]
    #[inline(always)]
    pub fn int_enable_dsp_1(&self) -> IntEnableDsp1R {
        IntEnableDsp1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - For DSP mode (bypass GDMA), F/W writes 1'b1 and then 1'b0 to clear TX interrupt. Note: TX interrupt is to indicate that DSP can write audio data to TX FIFO_0"]
    #[inline(always)]
    pub fn tx_dsp_clear_int_0(&mut self) -> TxDspClearInt0W<'_, SpIntCtrlSpec> {
        TxDspClearInt0W::new(self, 0)
    }
    #[doc = "Bit 1 - For DSP mode (bypass GDMA), F/W writes 1'b1 and then 1'b0 to clear RX interrupt. Note: RX interrupt is to indicate that DSP can get audio data from RX FIFO_0"]
    #[inline(always)]
    pub fn rx_dsp_clear_int_0(&mut self) -> RxDspClearInt0W<'_, SpIntCtrlSpec> {
        RxDspClearInt0W::new(self, 1)
    }
    #[doc = "Bits 2:6 - Bit\\[2\\]: for the interrupt of \"tx_fifo_full_intr\" Bit\\[3\\]: for the interrupt of \"rx_fifo_full_intr\" Bit\\[4\\]: for the interrupt of \"tx_fifo_empty_intr\" Bit\\[5\\]: for the interrupt of \"rx_fifo_empty_intr\" Bit\\[6\\]: Reserved"]
    #[inline(always)]
    pub fn intr_clr_0(&mut self) -> IntrClr0W<'_, SpIntCtrlSpec> {
        IntrClr0W::new(self, 2)
    }
    #[doc = "Bit 7 - For DSP mode (bypass GDMA), F/W writes 1'b1 and then 1'b0 to clear TX interrupt. Note: TX interrupt is to indicate that DSP can write audio data to TX FIFO_1"]
    #[inline(always)]
    pub fn tx_dsp_clear_int_1(&mut self) -> TxDspClearInt1W<'_, SpIntCtrlSpec> {
        TxDspClearInt1W::new(self, 7)
    }
    #[doc = "Bit 8 - For DSP mode (bypass GDMA), F/W writes 1'b1 and then 1'b0 to clear RX interrupt. Note: RX interrupt is to indicate that DSP can get audio data from RX FIFO_1"]
    #[inline(always)]
    pub fn rx_dsp_clear_int_1(&mut self) -> RxDspClearInt1W<'_, SpIntCtrlSpec> {
        RxDspClearInt1W::new(self, 8)
    }
    #[doc = "Bits 9:13 - Bit\\[9\\]: for the interrupt of \"tx_fifo_full_intr_1\" Bit\\[10\\]: for the interrupt of \"rx_fifo_full_intr_1\" Bit\\[11\\]: for the interrupt of \"tx_fifo_empty_intr_1\" Bit\\[12\\]: for the interrupt of \"rx_fifo_empty_intr_1\" Bit\\[13\\]: Reserved"]
    #[inline(always)]
    pub fn intr_clr_1(&mut self) -> IntrClr1W<'_, SpIntCtrlSpec> {
        IntrClr1W::new(self, 9)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn lrc_start_intr(&mut self) -> LrcStartIntrW<'_, SpIntCtrlSpec> {
        LrcStartIntrW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn lrc_stop_intr(&mut self) -> LrcStopIntrW<'_, SpIntCtrlSpec> {
        LrcStopIntrW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Bit\\[16\\]: for the interrupt of \"sp_ready_to_tx\" Bit\\[17\\]: for the interrupt of \"sp_ready_to_rx\" Bit\\[18\\]: for the interrupt of \"tx_fifo_full_intr\" Bit\\[19\\]: for the interrupt of \"rx_fifo_full_intr\" Bit\\[20\\]: for the interrupt of \"tx_fifo_empty_intr\" Bit\\[21\\]: for the interrupt of \"rx_fifo_empty_intr\" Bit\\[22\\]: for the interrupt of \"tx_i2s_idle\" Bit\\[23\\]: Reserved"]
    #[inline(always)]
    pub fn int_enable_dsp_0(&mut self) -> IntEnableDsp0W<'_, SpIntCtrlSpec> {
        IntEnableDsp0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Bit\\[24\\]: for the interrupt of \"sp_ready_to_tx_1\" Bit\\[25\\]: for the interrupt of \"sp_ready_to_rx_1\" Bit\\[26\\]: for the interrupt of \"tx_fifo_full_intr_1\" Bit\\[27\\]: for the interrupt of \"rx_fifo_full_intr_1\" Bit\\[28\\]: for the interrupt of \"tx_fifo_empty_intr_1\" Bit\\[29\\]: for the interrupt of \"rx_fifo_empty_intr_1\" Bit\\[30\\]: for the interrupt of \"tx_i2s_idle_1\" Bit\\[31\\]: Reserved"]
    #[inline(always)]
    pub fn int_enable_dsp_1(&mut self) -> IntEnableDsp1W<'_, SpIntCtrlSpec> {
        IntEnableDsp1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_int_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sp_int_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpIntCtrlSpec;
impl crate::RegisterSpec for SpIntCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_int_ctrl::R`](R) reader structure"]
impl crate::Readable for SpIntCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sp_int_ctrl::W`](W) writer structure"]
impl crate::Writable for SpIntCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SP_INT_CTRL to value 0"]
impl crate::Resettable for SpIntCtrlSpec {}
