#[doc = "Register `REG_MACTX_MISC` reader"]
pub type R = crate::R<RegMactxMiscSpec>;
#[doc = "Register `REG_MACTX_MISC` writer"]
pub type W = crate::W<RegMactxMiscSpec>;
#[doc = "Field `r_EOFPAD_EN` reader - enable eof pad operation at the end of mpdu"]
pub type REofpadEnR = crate::BitReader;
#[doc = "Field `r_EOFPAD_EN` writer - enable eof pad operation at the end of mpdu"]
pub type REofpadEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_WMAC_MPDU_PADEN` reader - when txdfifo underflow ,eanble the mactx_mpdu moudle to pad dummy between mpdu."]
pub type RWmacMpduPadenR = crate::BitReader;
#[doc = "Field `r_WMAC_MPDU_PADEN` writer - when txdfifo underflow ,eanble the mactx_mpdu moudle to pad dummy between mpdu."]
pub type RWmacMpduPadenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_WMAC_SW_RD` reader - eanble sw read txdfifo"]
pub type RWmacSwRdR = crate::BitReader;
#[doc = "Field `r_WMAC_SW_RD` writer - eanble sw read txdfifo"]
pub type RWmacSwRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_WMAC_FORCE_READ_TXDFIFO` reader - during tx procedure, force read txd fifo, which is only use at the situation when bb is hang."]
pub type RWmacForceReadTxdfifoR = crate::BitReader;
#[doc = "Field `r_WMAC_FORCE_READ_TXDFIFO` writer - during tx procedure, force read txd fifo, which is only use at the situation when bb is hang."]
pub type RWmacForceReadTxdfifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sw_txdfifo_addr` reader - sw read txd fifo address,\\[6:1\\] is address, \\[0\\] : 0 read low 32bit; 1 read high 32bit;"]
pub type SwTxdfifoAddrR = crate::FieldReader;
#[doc = "Field `sw_txdfifo_addr` writer - sw read txd fifo address,\\[6:1\\] is address, \\[0\\] : 0 read low 32bit; 1 read high 32bit;"]
pub type SwTxdfifoAddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `sw_txdfifo_rdy` reader - sw read txd fifo ready"]
pub type SwTxdfifoRdyR = crate::BitReader;
#[doc = "Field `sw_txdfifo_rdy` writer - sw read txd fifo ready"]
pub type SwTxdfifoRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_TXDFIFO_INTERVAL_THRE` reader - the interval threshold between txdfifo write and read pointer, if the interval is smaller than the threshold, mactx_mpdu moudle can pad dummy between mpdu(if eanbled)"]
pub type RTxdfifoIntervalThreR = crate::FieldReader<u16>;
#[doc = "Field `r_TXDFIFO_INTERVAL_THRE` writer - the interval threshold between txdfifo write and read pointer, if the interval is smaller than the threshold, mactx_mpdu moudle can pad dummy between mpdu(if eanbled)"]
pub type RTxdfifoIntervalThreW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `r_WMAC_RST_BB_CNT` reader - macclk cycle number of wmac_rst_bb signal"]
pub type RWmacRstBbCntR = crate::FieldReader;
#[doc = "Field `r_WMAC_RST_BB_CNT` writer - macclk cycle number of wmac_rst_bb signal"]
pub type RWmacRstBbCntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_WMAC_CCK_LOCKED_CLK` reader - indicate that the transmit frequency and symbol clocks are derived from the same oscillator"]
pub type RWmacCckLockedClkR = crate::BitReader;
#[doc = "Field `r_WMAC_CCK_LOCKED_CLK` writer - indicate that the transmit frequency and symbol clocks are derived from the same oscillator"]
pub type RWmacCckLockedClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_TXTIME_CAL_DISGCLK` reader - disable the clk gating for txtime compute module"]
pub type RTxtimeCalDisgclkR = crate::BitReader;
#[doc = "Field `r_TXTIME_CAL_DISGCLK` writer - disable the clk gating for txtime compute module"]
pub type RTxtimeCalDisgclkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable eof pad operation at the end of mpdu"]
    #[inline(always)]
    pub fn r_eofpad_en(&self) -> REofpadEnR {
        REofpadEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - when txdfifo underflow ,eanble the mactx_mpdu moudle to pad dummy between mpdu."]
    #[inline(always)]
    pub fn r_wmac_mpdu_paden(&self) -> RWmacMpduPadenR {
        RWmacMpduPadenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - eanble sw read txdfifo"]
    #[inline(always)]
    pub fn r_wmac_sw_rd(&self) -> RWmacSwRdR {
        RWmacSwRdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - during tx procedure, force read txd fifo, which is only use at the situation when bb is hang."]
    #[inline(always)]
    pub fn r_wmac_force_read_txdfifo(&self) -> RWmacForceReadTxdfifoR {
        RWmacForceReadTxdfifoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:10 - sw read txd fifo address,\\[6:1\\] is address, \\[0\\] : 0 read low 32bit; 1 read high 32bit;"]
    #[inline(always)]
    pub fn sw_txdfifo_addr(&self) -> SwTxdfifoAddrR {
        SwTxdfifoAddrR::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bit 11 - sw read txd fifo ready"]
    #[inline(always)]
    pub fn sw_txdfifo_rdy(&self) -> SwTxdfifoRdyR {
        SwTxdfifoRdyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:25 - the interval threshold between txdfifo write and read pointer, if the interval is smaller than the threshold, mactx_mpdu moudle can pad dummy between mpdu(if eanbled)"]
    #[inline(always)]
    pub fn r_txdfifo_interval_thre(&self) -> RTxdfifoIntervalThreR {
        RTxdfifoIntervalThreR::new(((self.bits >> 12) & 0x3fff) as u16)
    }
    #[doc = "Bits 26:28 - macclk cycle number of wmac_rst_bb signal"]
    #[inline(always)]
    pub fn r_wmac_rst_bb_cnt(&self) -> RWmacRstBbCntR {
        RWmacRstBbCntR::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bit 29 - indicate that the transmit frequency and symbol clocks are derived from the same oscillator"]
    #[inline(always)]
    pub fn r_wmac_cck_locked_clk(&self) -> RWmacCckLockedClkR {
        RWmacCckLockedClkR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - disable the clk gating for txtime compute module"]
    #[inline(always)]
    pub fn r_txtime_cal_disgclk(&self) -> RTxtimeCalDisgclkR {
        RTxtimeCalDisgclkR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable eof pad operation at the end of mpdu"]
    #[inline(always)]
    pub fn r_eofpad_en(&mut self) -> REofpadEnW<'_, RegMactxMiscSpec> {
        REofpadEnW::new(self, 0)
    }
    #[doc = "Bit 1 - when txdfifo underflow ,eanble the mactx_mpdu moudle to pad dummy between mpdu."]
    #[inline(always)]
    pub fn r_wmac_mpdu_paden(&mut self) -> RWmacMpduPadenW<'_, RegMactxMiscSpec> {
        RWmacMpduPadenW::new(self, 1)
    }
    #[doc = "Bit 2 - eanble sw read txdfifo"]
    #[inline(always)]
    pub fn r_wmac_sw_rd(&mut self) -> RWmacSwRdW<'_, RegMactxMiscSpec> {
        RWmacSwRdW::new(self, 2)
    }
    #[doc = "Bit 3 - during tx procedure, force read txd fifo, which is only use at the situation when bb is hang."]
    #[inline(always)]
    pub fn r_wmac_force_read_txdfifo(&mut self) -> RWmacForceReadTxdfifoW<'_, RegMactxMiscSpec> {
        RWmacForceReadTxdfifoW::new(self, 3)
    }
    #[doc = "Bits 4:10 - sw read txd fifo address,\\[6:1\\] is address, \\[0\\] : 0 read low 32bit; 1 read high 32bit;"]
    #[inline(always)]
    pub fn sw_txdfifo_addr(&mut self) -> SwTxdfifoAddrW<'_, RegMactxMiscSpec> {
        SwTxdfifoAddrW::new(self, 4)
    }
    #[doc = "Bit 11 - sw read txd fifo ready"]
    #[inline(always)]
    pub fn sw_txdfifo_rdy(&mut self) -> SwTxdfifoRdyW<'_, RegMactxMiscSpec> {
        SwTxdfifoRdyW::new(self, 11)
    }
    #[doc = "Bits 12:25 - the interval threshold between txdfifo write and read pointer, if the interval is smaller than the threshold, mactx_mpdu moudle can pad dummy between mpdu(if eanbled)"]
    #[inline(always)]
    pub fn r_txdfifo_interval_thre(&mut self) -> RTxdfifoIntervalThreW<'_, RegMactxMiscSpec> {
        RTxdfifoIntervalThreW::new(self, 12)
    }
    #[doc = "Bits 26:28 - macclk cycle number of wmac_rst_bb signal"]
    #[inline(always)]
    pub fn r_wmac_rst_bb_cnt(&mut self) -> RWmacRstBbCntW<'_, RegMactxMiscSpec> {
        RWmacRstBbCntW::new(self, 26)
    }
    #[doc = "Bit 29 - indicate that the transmit frequency and symbol clocks are derived from the same oscillator"]
    #[inline(always)]
    pub fn r_wmac_cck_locked_clk(&mut self) -> RWmacCckLockedClkW<'_, RegMactxMiscSpec> {
        RWmacCckLockedClkW::new(self, 29)
    }
    #[doc = "Bit 30 - disable the clk gating for txtime compute module"]
    #[inline(always)]
    pub fn r_txtime_cal_disgclk(&mut self) -> RTxtimeCalDisgclkW<'_, RegMactxMiscSpec> {
        RTxtimeCalDisgclkW::new(self, 30)
    }
}
#[doc = "misc ctrl cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mactx_misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mactx_misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMactxMiscSpec;
impl crate::RegisterSpec for RegMactxMiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mactx_misc::R`](R) reader structure"]
impl crate::Readable for RegMactxMiscSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mactx_misc::W`](W) writer structure"]
impl crate::Writable for RegMactxMiscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MACTX_MISC to value 0x2c00_5003"]
impl crate::Resettable for RegMactxMiscSpec {
    const RESET_VALUE: u32 = 0x2c00_5003;
}
