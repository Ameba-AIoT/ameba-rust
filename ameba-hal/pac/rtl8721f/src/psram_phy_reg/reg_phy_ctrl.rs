#[doc = "Register `REG_PHY_CTRL` reader"]
pub type R = crate::R<RegPhyCtrlSpec>;
#[doc = "Register `REG_PHY_CTRL` writer"]
pub type W = crate::W<RegPhyCtrlSpec>;
#[doc = "Field `RFIFO_RDY_DLY` reader - variable latency: delay read data cycles(phy clk) after detecting first DQS rising edge, must < 4 fixed latency: delay read data cycles(phy clk) after asserting spi_rx_data_en, must > 5 4*read_latency - rd_dummy_length + rfifo_rdy_dly = in_physical_cyc"]
pub type RfifoRdyDlyR = crate::FieldReader;
#[doc = "Field `RFIFO_RDY_DLY` writer - variable latency: delay read data cycles(phy clk) after detecting first DQS rising edge, must < 4 fixed latency: delay read data cycles(phy clk) after asserting spi_rx_data_en, must > 5 4*read_latency - rd_dummy_length + rfifo_rdy_dly = in_physical_cyc"]
pub type RfifoRdyDlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CAL_RWDS_PHASE` reader - one-hot phase select, delay cal_rwds to push sampling data into rfifo"]
pub type CalRwdsPhaseR = crate::FieldReader;
#[doc = "Field `CAL_RWDS_PHASE` writer - one-hot phase select, delay cal_rwds to push sampling data into rfifo"]
pub type CalRwdsPhaseW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LATENCY_TYPE` reader - set this bit according to spic & psram latency settings 0: variable latency 1: fixed latency"]
pub type LatencyTypeR = crate::BitReader;
#[doc = "Field `LATENCY_TYPE` writer - set this bit according to spic & psram latency settings 0: variable latency 1: fixed latency"]
pub type LatencyTypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWDS_PD_EN` reader - psram_phy can control PD pin of rwds iopad, bypass high-Z state at CMD/ADDR stage"]
pub type RwdsPdEnR = crate::BitReader;
#[doc = "Field `RWDS_PD_EN` writer - psram_phy can control PD pin of rwds iopad, bypass high-Z state at CMD/ADDR stage"]
pub type RwdsPdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWDS_CHK_OPT` reader - 0: synced psram_rwds_i direct to controller 1: only check psram_rwds_i at last valid cycle of CA stage, sync this pulse to controller"]
pub type RwdsChkOptR = crate::BitReader;
#[doc = "Field `RWDS_CHK_OPT` writer - 0: synced psram_rwds_i direct to controller 1: only check psram_rwds_i at last valid cycle of CA stage, sync this pulse to controller"]
pub type RwdsChkOptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSRAM_DQ16_EN` reader - 1: enable to support DQ16 psram"]
pub type PsramDq16EnR = crate::BitReader;
#[doc = "Field `PSRAM_DQ16_EN` writer - 1: enable to support DQ16 psram"]
pub type PsramDq16EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSRAM_SW_RST_N` reader - low active, except for phy register(using system reset), psram software reset: psram, phy_ctrl, calibration"]
pub type PsramSwRstNR = crate::BitReader;
#[doc = "Field `PSRAM_SW_RST_N` writer - low active, except for phy register(using system reset), psram software reset: psram, phy_ctrl, calibration"]
pub type PsramSwRstNW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - variable latency: delay read data cycles(phy clk) after detecting first DQS rising edge, must < 4 fixed latency: delay read data cycles(phy clk) after asserting spi_rx_data_en, must > 5 4*read_latency - rd_dummy_length + rfifo_rdy_dly = in_physical_cyc"]
    #[inline(always)]
    pub fn rfifo_rdy_dly(&self) -> RfifoRdyDlyR {
        RfifoRdyDlyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - one-hot phase select, delay cal_rwds to push sampling data into rfifo"]
    #[inline(always)]
    pub fn cal_rwds_phase(&self) -> CalRwdsPhaseR {
        CalRwdsPhaseR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - set this bit according to spic & psram latency settings 0: variable latency 1: fixed latency"]
    #[inline(always)]
    pub fn latency_type(&self) -> LatencyTypeR {
        LatencyTypeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - psram_phy can control PD pin of rwds iopad, bypass high-Z state at CMD/ADDR stage"]
    #[inline(always)]
    pub fn rwds_pd_en(&self) -> RwdsPdEnR {
        RwdsPdEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 0: synced psram_rwds_i direct to controller 1: only check psram_rwds_i at last valid cycle of CA stage, sync this pulse to controller"]
    #[inline(always)]
    pub fn rwds_chk_opt(&self) -> RwdsChkOptR {
        RwdsChkOptR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: enable to support DQ16 psram"]
    #[inline(always)]
    pub fn psram_dq16_en(&self) -> PsramDq16EnR {
        PsramDq16EnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - low active, except for phy register(using system reset), psram software reset: psram, phy_ctrl, calibration"]
    #[inline(always)]
    pub fn psram_sw_rst_n(&self) -> PsramSwRstNR {
        PsramSwRstNR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - variable latency: delay read data cycles(phy clk) after detecting first DQS rising edge, must < 4 fixed latency: delay read data cycles(phy clk) after asserting spi_rx_data_en, must > 5 4*read_latency - rd_dummy_length + rfifo_rdy_dly = in_physical_cyc"]
    #[inline(always)]
    pub fn rfifo_rdy_dly(&mut self) -> RfifoRdyDlyW<'_, RegPhyCtrlSpec> {
        RfifoRdyDlyW::new(self, 0)
    }
    #[doc = "Bits 4:7 - one-hot phase select, delay cal_rwds to push sampling data into rfifo"]
    #[inline(always)]
    pub fn cal_rwds_phase(&mut self) -> CalRwdsPhaseW<'_, RegPhyCtrlSpec> {
        CalRwdsPhaseW::new(self, 4)
    }
    #[doc = "Bit 12 - set this bit according to spic & psram latency settings 0: variable latency 1: fixed latency"]
    #[inline(always)]
    pub fn latency_type(&mut self) -> LatencyTypeW<'_, RegPhyCtrlSpec> {
        LatencyTypeW::new(self, 12)
    }
    #[doc = "Bit 13 - psram_phy can control PD pin of rwds iopad, bypass high-Z state at CMD/ADDR stage"]
    #[inline(always)]
    pub fn rwds_pd_en(&mut self) -> RwdsPdEnW<'_, RegPhyCtrlSpec> {
        RwdsPdEnW::new(self, 13)
    }
    #[doc = "Bit 14 - 0: synced psram_rwds_i direct to controller 1: only check psram_rwds_i at last valid cycle of CA stage, sync this pulse to controller"]
    #[inline(always)]
    pub fn rwds_chk_opt(&mut self) -> RwdsChkOptW<'_, RegPhyCtrlSpec> {
        RwdsChkOptW::new(self, 14)
    }
    #[doc = "Bit 15 - 1: enable to support DQ16 psram"]
    #[inline(always)]
    pub fn psram_dq16_en(&mut self) -> PsramDq16EnW<'_, RegPhyCtrlSpec> {
        PsramDq16EnW::new(self, 15)
    }
    #[doc = "Bit 31 - low active, except for phy register(using system reset), psram software reset: psram, phy_ctrl, calibration"]
    #[inline(always)]
    pub fn psram_sw_rst_n(&mut self) -> PsramSwRstNW<'_, RegPhyCtrlSpec> {
        PsramSwRstNW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_phy_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_phy_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPhyCtrlSpec;
impl crate::RegisterSpec for RegPhyCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_phy_ctrl::R`](R) reader structure"]
impl crate::Readable for RegPhyCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_phy_ctrl::W`](W) writer structure"]
impl crate::Writable for RegPhyCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PHY_CTRL to value 0x8000_0010"]
impl crate::Resettable for RegPhyCtrlSpec {
    const RESET_VALUE: u32 = 0x8000_0010;
}
