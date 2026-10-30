#[doc = "Register `REG_PLAT_CTRL` reader"]
pub type R = crate::R<RegPlatCtrlSpec>;
#[doc = "Register `REG_PLAT_CTRL` writer"]
pub type W = crate::W<RegPlatCtrlSpec>;
#[doc = "Field `HUK_PROT` reader - 0: only cpu can read HUK key 1: no one can read HUK key This bit will be latch to high when it was write to 1 until reset again ."]
pub type HukProtR = crate::BitReader;
#[doc = "Field `HUK_PROT` writer - 0: only cpu can read HUK key 1: no one can read HUK key This bit will be latch to high when it was write to 1 until reset again ."]
pub type HukProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hs_sram0_rd_weight` reader - Excuted read command number continuously if both read and write command reached This is for hs_ram 512K"]
pub type HsSram0RdWeightR = crate::FieldReader;
#[doc = "Field `hs_sram0_rd_weight` writer - Excuted read command number continuously if both read and write command reached This is for hs_ram 512K"]
pub type HsSram0RdWeightW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `hs_sram2_rd_weight` reader - Excuted read command number continuously if both read and write command reached This is for bt/wl share_ram"]
pub type HsSram2RdWeightR = crate::FieldReader;
#[doc = "Field `hs_sram2_rd_weight` writer - Excuted read command number continuously if both read and write command reached This is for bt/wl share_ram"]
pub type HsSram2RdWeightW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ppe_clk_opt` reader - ppe clock option"]
pub type PpeClkOptR = crate::FieldReader;
#[doc = "Field `ppe_clk_opt` writer - ppe clock option"]
pub type PpeClkOptW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sys_retention_load` reader - 1: restore data"]
pub type SysRetentionLoadR = crate::BitReader;
#[doc = "Field `sys_retention_load` writer - 1: restore data"]
pub type SysRetentionLoadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sys_retention_save` reader - 1: save data"]
pub type SysRetentionSaveR = crate::BitReader;
#[doc = "Field `sys_retention_save` writer - 1: save data"]
pub type SysRetentionSaveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KM4TZ_RETENTION_MODE` reader - 1: Enable, when CPU0 reset, cache will not be cleared 0: Disable, when CPU0 reset, cache will be cleared by HW"]
pub type Km4tzRetentionModeR = crate::BitReader;
#[doc = "Field `KM4TZ_RETENTION_MODE` writer - 1: Enable, when CPU0 reset, cache will not be cleared 0: Disable, when CPU0 reset, cache will be cleared by HW"]
pub type Km4tzRetentionModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KM4NS_RETENTION_MODE` reader - 1: Enable, when CPU1 reset, cache will not be cleared 0: Disable, when CPU1 reset, cache will be cleared by HW"]
pub type Km4nsRetentionModeR = crate::BitReader;
#[doc = "Field `KM4NS_RETENTION_MODE` writer - 1: Enable, when CPU1 reset, cache will not be cleared 0: Disable, when CPU1 reset, cache will be cleared by HW"]
pub type Km4nsRetentionModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHARE_WL_MEM` reader - 1: Enable KM4 to share WL memory"]
pub type ShareWlMemR = crate::BitReader;
#[doc = "Field `SHARE_WL_MEM` writer - 1: Enable KM4 to share WL memory"]
pub type ShareWlMemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHARE_BT_MEM` reader - 1: Enable KM4 to share BT memory"]
pub type ShareBtMemR = crate::BitReader;
#[doc = "Field `SHARE_BT_MEM` writer - 1: Enable KM4 to share BT memory"]
pub type ShareBtMemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ppe_glb_sus` reader - Assert this bit to suspend PPE, active high"]
pub type PpeGlbSusR = crate::BitReader;
#[doc = "Field `ppe_glb_sus` writer - Assert this bit to suspend PPE, active high"]
pub type PpeGlbSusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gdma0_ccm` reader - GDMA0 clock control mode"]
pub type Gdma0CcmR = crate::FieldReader;
#[doc = "Field `gdma0_ccm` writer - GDMA0 clock control mode"]
pub type Gdma0CcmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `psram_spdup_sim` reader - psramc simulatin speedup enable. Nouse because config form not specified ."]
pub type PsramSpdupSimR = crate::BitReader;
#[doc = "Field `psram_spdup_sim` writer - psramc simulatin speedup enable. Nouse because config form not specified ."]
pub type PsramSpdupSimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIC_SPDUP_SIM` reader - 1: SPIC will wait 256 spic_clk cycles before ready 0: SPIC will wait 2^16 spic_clk cycles before ready"]
pub type SpicSpdupSimR = crate::BitReader;
#[doc = "Field `SPIC_SPDUP_SIM` writer - 1: SPIC will wait 256 spic_clk cycles before ready 0: SPIC will wait 2^16 spic_clk cycles before ready"]
pub type SpicSpdupSimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI0_MST` reader - 1: SPI0 used as master 0: SPI0 used as slaver"]
pub type Spi0MstR = crate::BitReader;
#[doc = "Field `SPI0_MST` writer - 1: SPI0 used as master 0: SPI0 used as slaver"]
pub type Spi0MstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1_MST` reader - 1: SPI1 used as master 0: SPI1 used as slaver"]
pub type Spi1MstR = crate::BitReader;
#[doc = "Field `SPI1_MST` writer - 1: SPI1 used as master 0: SPI1 used as slaver"]
pub type Spi1MstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `km4tz_sev_in` reader - cpu0 sev input, posedge of this bit will trig CPU0 event"]
pub type Km4tzSevInR = crate::BitReader;
#[doc = "Field `km4tz_sev_in` writer - cpu0 sev input, posedge of this bit will trig CPU0 event"]
pub type Km4tzSevInW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `km4ns_sev_in` reader - cpu1 sev input, posedge of this bit will trig cpu1 event"]
pub type Km4nsSevInR = crate::BitReader;
#[doc = "Field `km4ns_sev_in` writer - cpu1 sev input, posedge of this bit will trig cpu1 event"]
pub type Km4nsSevInW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `crypto_clk_opt` reader - 1X: always ON 01: always OFF 00: controlled by internal ICG"]
pub type CryptoClkOptR = crate::FieldReader;
#[doc = "Field `crypto_clk_opt` writer - 1X: always ON 01: always OFF 00: controlled by internal ICG"]
pub type CryptoClkOptW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0: only cpu can read HUK key 1: no one can read HUK key This bit will be latch to high when it was write to 1 until reset again ."]
    #[inline(always)]
    pub fn huk_prot(&self) -> HukProtR {
        HukProtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:4 - Excuted read command number continuously if both read and write command reached This is for hs_ram 512K"]
    #[inline(always)]
    pub fn hs_sram0_rd_weight(&self) -> HsSram0RdWeightR {
        HsSram0RdWeightR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Excuted read command number continuously if both read and write command reached This is for bt/wl share_ram"]
    #[inline(always)]
    pub fn hs_sram2_rd_weight(&self) -> HsSram2RdWeightR {
        HsSram2RdWeightR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - ppe clock option"]
    #[inline(always)]
    pub fn ppe_clk_opt(&self) -> PpeClkOptR {
        PpeClkOptR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - 1: restore data"]
    #[inline(always)]
    pub fn sys_retention_load(&self) -> SysRetentionLoadR {
        SysRetentionLoadR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: save data"]
    #[inline(always)]
    pub fn sys_retention_save(&self) -> SysRetentionSaveR {
        SysRetentionSaveR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: Enable, when CPU0 reset, cache will not be cleared 0: Disable, when CPU0 reset, cache will be cleared by HW"]
    #[inline(always)]
    pub fn km4tz_retention_mode(&self) -> Km4tzRetentionModeR {
        Km4tzRetentionModeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1: Enable, when CPU1 reset, cache will not be cleared 0: Disable, when CPU1 reset, cache will be cleared by HW"]
    #[inline(always)]
    pub fn km4ns_retention_mode(&self) -> Km4nsRetentionModeR {
        Km4nsRetentionModeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1: Enable KM4 to share WL memory"]
    #[inline(always)]
    pub fn share_wl_mem(&self) -> ShareWlMemR {
        ShareWlMemR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: Enable KM4 to share BT memory"]
    #[inline(always)]
    pub fn share_bt_mem(&self) -> ShareBtMemR {
        ShareBtMemR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Assert this bit to suspend PPE, active high"]
    #[inline(always)]
    pub fn ppe_glb_sus(&self) -> PpeGlbSusR {
        PpeGlbSusR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - GDMA0 clock control mode"]
    #[inline(always)]
    pub fn gdma0_ccm(&self) -> Gdma0CcmR {
        Gdma0CcmR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 24 - psramc simulatin speedup enable. Nouse because config form not specified ."]
    #[inline(always)]
    pub fn psram_spdup_sim(&self) -> PsramSpdupSimR {
        PsramSpdupSimR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: SPIC will wait 256 spic_clk cycles before ready 0: SPIC will wait 2^16 spic_clk cycles before ready"]
    #[inline(always)]
    pub fn spic_spdup_sim(&self) -> SpicSpdupSimR {
        SpicSpdupSimR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: SPI0 used as master 0: SPI0 used as slaver"]
    #[inline(always)]
    pub fn spi0_mst(&self) -> Spi0MstR {
        Spi0MstR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1: SPI1 used as master 0: SPI1 used as slaver"]
    #[inline(always)]
    pub fn spi1_mst(&self) -> Spi1MstR {
        Spi1MstR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - cpu0 sev input, posedge of this bit will trig CPU0 event"]
    #[inline(always)]
    pub fn km4tz_sev_in(&self) -> Km4tzSevInR {
        Km4tzSevInR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - cpu1 sev input, posedge of this bit will trig cpu1 event"]
    #[inline(always)]
    pub fn km4ns_sev_in(&self) -> Km4nsSevInR {
        Km4nsSevInR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - 1X: always ON 01: always OFF 00: controlled by internal ICG"]
    #[inline(always)]
    pub fn crypto_clk_opt(&self) -> CryptoClkOptR {
        CryptoClkOptR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0: only cpu can read HUK key 1: no one can read HUK key This bit will be latch to high when it was write to 1 until reset again ."]
    #[inline(always)]
    pub fn huk_prot(&mut self) -> HukProtW<'_, RegPlatCtrlSpec> {
        HukProtW::new(self, 0)
    }
    #[doc = "Bits 2:4 - Excuted read command number continuously if both read and write command reached This is for hs_ram 512K"]
    #[inline(always)]
    pub fn hs_sram0_rd_weight(&mut self) -> HsSram0RdWeightW<'_, RegPlatCtrlSpec> {
        HsSram0RdWeightW::new(self, 2)
    }
    #[doc = "Bits 8:10 - Excuted read command number continuously if both read and write command reached This is for bt/wl share_ram"]
    #[inline(always)]
    pub fn hs_sram2_rd_weight(&mut self) -> HsSram2RdWeightW<'_, RegPlatCtrlSpec> {
        HsSram2RdWeightW::new(self, 8)
    }
    #[doc = "Bits 12:13 - ppe clock option"]
    #[inline(always)]
    pub fn ppe_clk_opt(&mut self) -> PpeClkOptW<'_, RegPlatCtrlSpec> {
        PpeClkOptW::new(self, 12)
    }
    #[doc = "Bit 14 - 1: restore data"]
    #[inline(always)]
    pub fn sys_retention_load(&mut self) -> SysRetentionLoadW<'_, RegPlatCtrlSpec> {
        SysRetentionLoadW::new(self, 14)
    }
    #[doc = "Bit 15 - 1: save data"]
    #[inline(always)]
    pub fn sys_retention_save(&mut self) -> SysRetentionSaveW<'_, RegPlatCtrlSpec> {
        SysRetentionSaveW::new(self, 15)
    }
    #[doc = "Bit 16 - 1: Enable, when CPU0 reset, cache will not be cleared 0: Disable, when CPU0 reset, cache will be cleared by HW"]
    #[inline(always)]
    pub fn km4tz_retention_mode(&mut self) -> Km4tzRetentionModeW<'_, RegPlatCtrlSpec> {
        Km4tzRetentionModeW::new(self, 16)
    }
    #[doc = "Bit 17 - 1: Enable, when CPU1 reset, cache will not be cleared 0: Disable, when CPU1 reset, cache will be cleared by HW"]
    #[inline(always)]
    pub fn km4ns_retention_mode(&mut self) -> Km4nsRetentionModeW<'_, RegPlatCtrlSpec> {
        Km4nsRetentionModeW::new(self, 17)
    }
    #[doc = "Bit 18 - 1: Enable KM4 to share WL memory"]
    #[inline(always)]
    pub fn share_wl_mem(&mut self) -> ShareWlMemW<'_, RegPlatCtrlSpec> {
        ShareWlMemW::new(self, 18)
    }
    #[doc = "Bit 19 - 1: Enable KM4 to share BT memory"]
    #[inline(always)]
    pub fn share_bt_mem(&mut self) -> ShareBtMemW<'_, RegPlatCtrlSpec> {
        ShareBtMemW::new(self, 19)
    }
    #[doc = "Bit 20 - Assert this bit to suspend PPE, active high"]
    #[inline(always)]
    pub fn ppe_glb_sus(&mut self) -> PpeGlbSusW<'_, RegPlatCtrlSpec> {
        PpeGlbSusW::new(self, 20)
    }
    #[doc = "Bits 21:22 - GDMA0 clock control mode"]
    #[inline(always)]
    pub fn gdma0_ccm(&mut self) -> Gdma0CcmW<'_, RegPlatCtrlSpec> {
        Gdma0CcmW::new(self, 21)
    }
    #[doc = "Bit 24 - psramc simulatin speedup enable. Nouse because config form not specified ."]
    #[inline(always)]
    pub fn psram_spdup_sim(&mut self) -> PsramSpdupSimW<'_, RegPlatCtrlSpec> {
        PsramSpdupSimW::new(self, 24)
    }
    #[doc = "Bit 25 - 1: SPIC will wait 256 spic_clk cycles before ready 0: SPIC will wait 2^16 spic_clk cycles before ready"]
    #[inline(always)]
    pub fn spic_spdup_sim(&mut self) -> SpicSpdupSimW<'_, RegPlatCtrlSpec> {
        SpicSpdupSimW::new(self, 25)
    }
    #[doc = "Bit 26 - 1: SPI0 used as master 0: SPI0 used as slaver"]
    #[inline(always)]
    pub fn spi0_mst(&mut self) -> Spi0MstW<'_, RegPlatCtrlSpec> {
        Spi0MstW::new(self, 26)
    }
    #[doc = "Bit 27 - 1: SPI1 used as master 0: SPI1 used as slaver"]
    #[inline(always)]
    pub fn spi1_mst(&mut self) -> Spi1MstW<'_, RegPlatCtrlSpec> {
        Spi1MstW::new(self, 27)
    }
    #[doc = "Bit 28 - cpu0 sev input, posedge of this bit will trig CPU0 event"]
    #[inline(always)]
    pub fn km4tz_sev_in(&mut self) -> Km4tzSevInW<'_, RegPlatCtrlSpec> {
        Km4tzSevInW::new(self, 28)
    }
    #[doc = "Bit 29 - cpu1 sev input, posedge of this bit will trig cpu1 event"]
    #[inline(always)]
    pub fn km4ns_sev_in(&mut self) -> Km4nsSevInW<'_, RegPlatCtrlSpec> {
        Km4nsSevInW::new(self, 29)
    }
    #[doc = "Bits 30:31 - 1X: always ON 01: always OFF 00: controlled by internal ICG"]
    #[inline(always)]
    pub fn crypto_clk_opt(&mut self) -> CryptoClkOptW<'_, RegPlatCtrlSpec> {
        CryptoClkOptW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_plat_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_plat_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPlatCtrlSpec;
impl crate::RegisterSpec for RegPlatCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_plat_ctrl::R`](R) reader structure"]
impl crate::Readable for RegPlatCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_plat_ctrl::W`](W) writer structure"]
impl crate::Writable for RegPlatCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PLAT_CTRL to value 0x0208"]
impl crate::Resettable for RegPlatCtrlSpec {
    const RESET_VALUE: u32 = 0x0208;
}
