#[doc = "Register `LCDC_MCU_CFG` reader"]
pub type R = crate::R<LcdcMcuCfgSpec>;
#[doc = "Register `LCDC_MCU_CFG` writer"]
pub type W = crate::W<LcdcMcuCfgSpec>;
#[doc = "Field `cspl` reader - The CS pulse polarity. * 0: Low level for active pulse * 1: High level for active pulse"]
pub type CsplR = crate::BitReader;
#[doc = "Field `cspl` writer - The CS pulse polarity. * 0: Low level for active pulse * 1: High level for active pulse"]
pub type CsplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rspl` reader - The RS pulse polarity. * 0: Low level for command address * 1: High level for command parameter"]
pub type RsplR = crate::BitReader;
#[doc = "Field `rspl` writer - The RS pulse polarity. * 0: Low level for command address * 1: High level for command parameter"]
pub type RsplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wrpl` reader - The WR pulse polarity. * 0: Data fetched at rising edge * 1: Data fetched at falling edge"]
pub type WrplR = crate::BitReader;
#[doc = "Field `wrpl` writer - The WR pulse polarity. * 0: Data fetched at rising edge * 1: Data fetched at falling edge"]
pub type WrplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rdpl` reader - The RD pulse polarity. * 0: Data fetched at rising edge * 1: Data fetched at falling edge"]
pub type RdplR = crate::BitReader;
#[doc = "Field `rdpl` writer - The RD pulse polarity. * 0: Data fetched at rising edge * 1: Data fetched at falling edge"]
pub type RdplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mcu_datpl` reader - The Data pulse polarity. * 0: Normal * 1: Inverted"]
pub type McuDatplR = crate::BitReader;
#[doc = "Field `mcu_datpl` writer - The Data pulse polarity. * 0: Normal * 1: Inverted"]
pub type McuDatplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tepl` reader - The TE pulse polarity. * 0: Low level for active pulse * 1: High level for active pulse"]
pub type TeplR = crate::BitReader;
#[doc = "Field `tepl` writer - The TE pulse polarity. * 0: Low level for active pulse * 1: High level for active pulse"]
pub type TeplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mcusypl` reader - The MCU VSYNC pulse polarity * 0: Low level for active pulse * 1: High level for active pulse"]
pub type McusyplR = crate::BitReader;
#[doc = "Field `mcusypl` writer - The MCU VSYNC pulse polarity * 0: Low level for active pulse * 1: High level for active pulse"]
pub type McusyplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mcuif_update` reader - Force Hardware to update MCU I/F Timing shadow register at specific timing. CPU writes 1 to force Hardware updating. After Hardware updating finished, this bit is cleared. Software can't write related shadow registers when MCUIFUPDATE is still active. When the LCDC is running, if the following values related with MCU I/F mode are modified dynamically, only writing 1 to this bit can the newer value be used by hardware after the current frame refresh done. * The TEDELAY field in the LCDC_MCU_CFG register * The MCUVSW, MCUVSPD fields in the LCDC_MCU_VSYNC_CFG register * The WRPULW, RDACTW, RDINACTW fields in the LCDC_MCU_TIMING_CFG register"]
pub type McuifUpdateR = crate::BitReader;
#[doc = "Field `mcuif_update` writer - Force Hardware to update MCU I/F Timing shadow register at specific timing. CPU writes 1 to force Hardware updating. After Hardware updating finished, this bit is cleared. Software can't write related shadow registers when MCUIFUPDATE is still active. When the LCDC is running, if the following values related with MCU I/F mode are modified dynamically, only writing 1 to this bit can the newer value be used by hardware after the current frame refresh done. * The TEDELAY field in the LCDC_MCU_CFG register * The MCUVSW, MCUVSPD fields in the LCDC_MCU_VSYNC_CFG register * The WRPULW, RDACTW, RDINACTW fields in the LCDC_MCU_TIMING_CFG register"]
pub type McuifUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mcu_sync_mode` reader - * 00: Synchronized with the internal clock * 01: Synchronized with VSYNC input * 10: Tearing effect line on (Mode1, VSYNC) * Others: Reserved"]
pub type McuSyncModeR = crate::FieldReader;
#[doc = "Field `mcu_sync_mode` writer - * 00: Synchronized with the internal clock * 01: Synchronized with VSYNC input * 10: Tearing effect line on (Mode1, VSYNC) * Others: Reserved"]
pub type McuSyncModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mcu_io_mode_en` reader - MCU I/F I/O mode enable * 0: Disable I/O mode and open DMA mode after I/O mode FIFO empty. * 1: Enable I/O mode after current frame refresh. Poll MCU_IO_MODE_RUN after updating this bit."]
pub type McuIoModeEnR = crate::BitReader;
#[doc = "Field `mcu_io_mode_en` writer - MCU I/F I/O mode enable * 0: Disable I/O mode and open DMA mode after I/O mode FIFO empty. * 1: Enable I/O mode after current frame refresh. Poll MCU_IO_MODE_RUN after updating this bit."]
pub type McuIoModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mcu_io_mode_run` reader - MCU I/F I/O mode run * 0: DMA mode run * 1: I/O mode run"]
pub type McuIoModeRunR = crate::BitReader;
#[doc = "Field `dat_swap` reader - swap dat\\[15:8\\] with dat\\[7:0\\] in mcu 8-bit RGB565"]
pub type DatSwapR = crate::BitReader;
#[doc = "Field `dat_swap` writer - swap dat\\[15:8\\] with dat\\[7:0\\] in mcu 8-bit RGB565"]
pub type DatSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tedelay` reader - The delay interval -5. This interval is from detected TE signal to starting frame transfer. Unit: WR pulse width. The maximum value is 65535. The real delay interval is TEDELAY + 4 or TEDELAY + 5."]
pub type TedelayR = crate::FieldReader<u16>;
#[doc = "Field `tedelay` writer - The delay interval -5. This interval is from detected TE signal to starting frame transfer. Unit: WR pulse width. The maximum value is 65535. The real delay interval is TEDELAY + 4 or TEDELAY + 5."]
pub type TedelayW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - The CS pulse polarity. * 0: Low level for active pulse * 1: High level for active pulse"]
    #[inline(always)]
    pub fn cspl(&self) -> CsplR {
        CsplR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The RS pulse polarity. * 0: Low level for command address * 1: High level for command parameter"]
    #[inline(always)]
    pub fn rspl(&self) -> RsplR {
        RsplR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The WR pulse polarity. * 0: Data fetched at rising edge * 1: Data fetched at falling edge"]
    #[inline(always)]
    pub fn wrpl(&self) -> WrplR {
        WrplR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The RD pulse polarity. * 0: Data fetched at rising edge * 1: Data fetched at falling edge"]
    #[inline(always)]
    pub fn rdpl(&self) -> RdplR {
        RdplR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The Data pulse polarity. * 0: Normal * 1: Inverted"]
    #[inline(always)]
    pub fn mcu_datpl(&self) -> McuDatplR {
        McuDatplR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The TE pulse polarity. * 0: Low level for active pulse * 1: High level for active pulse"]
    #[inline(always)]
    pub fn tepl(&self) -> TeplR {
        TeplR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The MCU VSYNC pulse polarity * 0: Low level for active pulse * 1: High level for active pulse"]
    #[inline(always)]
    pub fn mcusypl(&self) -> McusyplR {
        McusyplR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Force Hardware to update MCU I/F Timing shadow register at specific timing. CPU writes 1 to force Hardware updating. After Hardware updating finished, this bit is cleared. Software can't write related shadow registers when MCUIFUPDATE is still active. When the LCDC is running, if the following values related with MCU I/F mode are modified dynamically, only writing 1 to this bit can the newer value be used by hardware after the current frame refresh done. * The TEDELAY field in the LCDC_MCU_CFG register * The MCUVSW, MCUVSPD fields in the LCDC_MCU_VSYNC_CFG register * The WRPULW, RDACTW, RDINACTW fields in the LCDC_MCU_TIMING_CFG register"]
    #[inline(always)]
    pub fn mcuif_update(&self) -> McuifUpdateR {
        McuifUpdateR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - * 00: Synchronized with the internal clock * 01: Synchronized with VSYNC input * 10: Tearing effect line on (Mode1, VSYNC) * Others: Reserved"]
    #[inline(always)]
    pub fn mcu_sync_mode(&self) -> McuSyncModeR {
        McuSyncModeR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - MCU I/F I/O mode enable * 0: Disable I/O mode and open DMA mode after I/O mode FIFO empty. * 1: Enable I/O mode after current frame refresh. Poll MCU_IO_MODE_RUN after updating this bit."]
    #[inline(always)]
    pub fn mcu_io_mode_en(&self) -> McuIoModeEnR {
        McuIoModeEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MCU I/F I/O mode run * 0: DMA mode run * 1: I/O mode run"]
    #[inline(always)]
    pub fn mcu_io_mode_run(&self) -> McuIoModeRunR {
        McuIoModeRunR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - swap dat\\[15:8\\] with dat\\[7:0\\] in mcu 8-bit RGB565"]
    #[inline(always)]
    pub fn dat_swap(&self) -> DatSwapR {
        DatSwapR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - The delay interval -5. This interval is from detected TE signal to starting frame transfer. Unit: WR pulse width. The maximum value is 65535. The real delay interval is TEDELAY + 4 or TEDELAY + 5."]
    #[inline(always)]
    pub fn tedelay(&self) -> TedelayR {
        TedelayR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - The CS pulse polarity. * 0: Low level for active pulse * 1: High level for active pulse"]
    #[inline(always)]
    pub fn cspl(&mut self) -> CsplW<'_, LcdcMcuCfgSpec> {
        CsplW::new(self, 0)
    }
    #[doc = "Bit 1 - The RS pulse polarity. * 0: Low level for command address * 1: High level for command parameter"]
    #[inline(always)]
    pub fn rspl(&mut self) -> RsplW<'_, LcdcMcuCfgSpec> {
        RsplW::new(self, 1)
    }
    #[doc = "Bit 2 - The WR pulse polarity. * 0: Data fetched at rising edge * 1: Data fetched at falling edge"]
    #[inline(always)]
    pub fn wrpl(&mut self) -> WrplW<'_, LcdcMcuCfgSpec> {
        WrplW::new(self, 2)
    }
    #[doc = "Bit 3 - The RD pulse polarity. * 0: Data fetched at rising edge * 1: Data fetched at falling edge"]
    #[inline(always)]
    pub fn rdpl(&mut self) -> RdplW<'_, LcdcMcuCfgSpec> {
        RdplW::new(self, 3)
    }
    #[doc = "Bit 4 - The Data pulse polarity. * 0: Normal * 1: Inverted"]
    #[inline(always)]
    pub fn mcu_datpl(&mut self) -> McuDatplW<'_, LcdcMcuCfgSpec> {
        McuDatplW::new(self, 4)
    }
    #[doc = "Bit 5 - The TE pulse polarity. * 0: Low level for active pulse * 1: High level for active pulse"]
    #[inline(always)]
    pub fn tepl(&mut self) -> TeplW<'_, LcdcMcuCfgSpec> {
        TeplW::new(self, 5)
    }
    #[doc = "Bit 6 - The MCU VSYNC pulse polarity * 0: Low level for active pulse * 1: High level for active pulse"]
    #[inline(always)]
    pub fn mcusypl(&mut self) -> McusyplW<'_, LcdcMcuCfgSpec> {
        McusyplW::new(self, 6)
    }
    #[doc = "Bit 8 - Force Hardware to update MCU I/F Timing shadow register at specific timing. CPU writes 1 to force Hardware updating. After Hardware updating finished, this bit is cleared. Software can't write related shadow registers when MCUIFUPDATE is still active. When the LCDC is running, if the following values related with MCU I/F mode are modified dynamically, only writing 1 to this bit can the newer value be used by hardware after the current frame refresh done. * The TEDELAY field in the LCDC_MCU_CFG register * The MCUVSW, MCUVSPD fields in the LCDC_MCU_VSYNC_CFG register * The WRPULW, RDACTW, RDINACTW fields in the LCDC_MCU_TIMING_CFG register"]
    #[inline(always)]
    pub fn mcuif_update(&mut self) -> McuifUpdateW<'_, LcdcMcuCfgSpec> {
        McuifUpdateW::new(self, 8)
    }
    #[doc = "Bits 9:10 - * 00: Synchronized with the internal clock * 01: Synchronized with VSYNC input * 10: Tearing effect line on (Mode1, VSYNC) * Others: Reserved"]
    #[inline(always)]
    pub fn mcu_sync_mode(&mut self) -> McuSyncModeW<'_, LcdcMcuCfgSpec> {
        McuSyncModeW::new(self, 9)
    }
    #[doc = "Bit 11 - MCU I/F I/O mode enable * 0: Disable I/O mode and open DMA mode after I/O mode FIFO empty. * 1: Enable I/O mode after current frame refresh. Poll MCU_IO_MODE_RUN after updating this bit."]
    #[inline(always)]
    pub fn mcu_io_mode_en(&mut self) -> McuIoModeEnW<'_, LcdcMcuCfgSpec> {
        McuIoModeEnW::new(self, 11)
    }
    #[doc = "Bit 15 - swap dat\\[15:8\\] with dat\\[7:0\\] in mcu 8-bit RGB565"]
    #[inline(always)]
    pub fn dat_swap(&mut self) -> DatSwapW<'_, LcdcMcuCfgSpec> {
        DatSwapW::new(self, 15)
    }
    #[doc = "Bits 16:31 - The delay interval -5. This interval is from detected TE signal to starting frame transfer. Unit: WR pulse width. The maximum value is 65535. The real delay interval is TEDELAY + 4 or TEDELAY + 5."]
    #[inline(always)]
    pub fn tedelay(&mut self) -> TedelayW<'_, LcdcMcuCfgSpec> {
        TedelayW::new(self, 16)
    }
}
#[doc = "LCDC mcu mode config\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_mcu_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc_mcu_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcMcuCfgSpec;
impl crate::RegisterSpec for LcdcMcuCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_mcu_cfg::R`](R) reader structure"]
impl crate::Readable for LcdcMcuCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdc_mcu_cfg::W`](W) writer structure"]
impl crate::Writable for LcdcMcuCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDC_MCU_CFG to value 0x0220"]
impl crate::Resettable for LcdcMcuCfgSpec {
    const RESET_VALUE: u32 = 0x0220;
}
