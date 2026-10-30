#[doc = "Register `BT_FTC_MEM_CTRL` reader"]
pub type R = crate::R<BtFtcMemCtrlSpec>;
#[doc = "Register `BT_FTC_MEM_CTRL` writer"]
pub type W = crate::W<BtFtcMemCtrlSpec>;
#[doc = "Field `BT_1PRF0_DVS` reader - FSNOU_A_SYNL"]
pub type Bt1prf0DvsR = crate::FieldReader;
#[doc = "Field `BT_1PRF0_DVS` writer - FSNOU_A_SYNL"]
pub type Bt1prf0DvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BT_1PRF0_DVSE` reader - "]
pub type Bt1prf0DvseR = crate::BitReader;
#[doc = "Field `BT_1PRF0_DVSE` writer - "]
pub type Bt1prf0DvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_1PRF1_DVS` reader - FSNOU_A_SYNL"]
pub type Bt1prf1DvsR = crate::FieldReader;
#[doc = "Field `BT_1PRF1_DVS` writer - FSNOU_A_SYNL"]
pub type Bt1prf1DvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BT_1PRF1_DVSE` reader - "]
pub type Bt1prf1DvseR = crate::BitReader;
#[doc = "Field `BT_1PRF1_DVSE` writer - "]
pub type Bt1prf1DvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_1PSR_DVS` reader - Not used in Lite"]
pub type Bt1psrDvsR = crate::FieldReader;
#[doc = "Field `BT_1PSR_DVS` writer - Not used in Lite"]
pub type Bt1psrDvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BT_1PSR_DVSE` reader - Not used in Lite"]
pub type Bt1psrDvseR = crate::BitReader;
#[doc = "Field `BT_1PSR_DVSE` writer - Not used in Lite"]
pub type Bt1psrDvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_ROM_DVS` reader - FSNOU_A_SPNN"]
pub type BtRomDvsR = crate::FieldReader;
#[doc = "Field `BT_ROM_DVS` writer - FSNOU_A_SPNN"]
pub type BtRomDvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BT_ROM_DVSE` reader - "]
pub type BtRomDvseR = crate::BitReader;
#[doc = "Field `BT_ROM_DVSE` writer - "]
pub type BtRomDvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_TMCK_ALL` reader - "]
pub type BtTmckAllR = crate::BitReader;
#[doc = "Field `BT_TMCK_ALL` writer - "]
pub type BtTmckAllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_LVDRF_CLKDIS` reader - "]
pub type BtLvdrfClkdisR = crate::BitReader;
#[doc = "Field `BT_LVDRF_CLKDIS` writer - "]
pub type BtLvdrfClkdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_DS_ALL` reader - 1 - All BT memory switch to Deep Sleep mode 0 - Power mode of BT memory are controlled by BTPMC"]
pub type BtDsAllR = crate::BitReader;
#[doc = "Field `BT_DS_ALL` writer - 1 - All BT memory switch to Deep Sleep mode 0 - Power mode of BT memory are controlled by BTPMC"]
pub type BtDsAllW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - FSNOU_A_SYNL"]
    #[inline(always)]
    pub fn bt_1prf0_dvs(&self) -> Bt1prf0DvsR {
        Bt1prf0DvsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bt_1prf0_dvse(&self) -> Bt1prf0DvseR {
        Bt1prf0DvseR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - FSNOU_A_SYNL"]
    #[inline(always)]
    pub fn bt_1prf1_dvs(&self) -> Bt1prf1DvsR {
        Bt1prf1DvsR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bt_1prf1_dvse(&self) -> Bt1prf1DvseR {
        Bt1prf1DvseR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Not used in Lite"]
    #[inline(always)]
    pub fn bt_1psr_dvs(&self) -> Bt1psrDvsR {
        Bt1psrDvsR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Not used in Lite"]
    #[inline(always)]
    pub fn bt_1psr_dvse(&self) -> Bt1psrDvseR {
        Bt1psrDvseR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:18 - FSNOU_A_SPNN"]
    #[inline(always)]
    pub fn bt_rom_dvs(&self) -> BtRomDvsR {
        BtRomDvsR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn bt_rom_dvse(&self) -> BtRomDvseR {
        BtRomDvseR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn bt_tmck_all(&self) -> BtTmckAllR {
        BtTmckAllR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn bt_lvdrf_clkdis(&self) -> BtLvdrfClkdisR {
        BtLvdrfClkdisR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1 - All BT memory switch to Deep Sleep mode 0 - Power mode of BT memory are controlled by BTPMC"]
    #[inline(always)]
    pub fn bt_ds_all(&self) -> BtDsAllR {
        BtDsAllR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - FSNOU_A_SYNL"]
    #[inline(always)]
    pub fn bt_1prf0_dvs(&mut self) -> Bt1prf0DvsW<'_, BtFtcMemCtrlSpec> {
        Bt1prf0DvsW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bt_1prf0_dvse(&mut self) -> Bt1prf0DvseW<'_, BtFtcMemCtrlSpec> {
        Bt1prf0DvseW::new(self, 4)
    }
    #[doc = "Bits 5:8 - FSNOU_A_SYNL"]
    #[inline(always)]
    pub fn bt_1prf1_dvs(&mut self) -> Bt1prf1DvsW<'_, BtFtcMemCtrlSpec> {
        Bt1prf1DvsW::new(self, 5)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bt_1prf1_dvse(&mut self) -> Bt1prf1DvseW<'_, BtFtcMemCtrlSpec> {
        Bt1prf1DvseW::new(self, 9)
    }
    #[doc = "Bits 10:13 - Not used in Lite"]
    #[inline(always)]
    pub fn bt_1psr_dvs(&mut self) -> Bt1psrDvsW<'_, BtFtcMemCtrlSpec> {
        Bt1psrDvsW::new(self, 10)
    }
    #[doc = "Bit 14 - Not used in Lite"]
    #[inline(always)]
    pub fn bt_1psr_dvse(&mut self) -> Bt1psrDvseW<'_, BtFtcMemCtrlSpec> {
        Bt1psrDvseW::new(self, 14)
    }
    #[doc = "Bits 15:18 - FSNOU_A_SPNN"]
    #[inline(always)]
    pub fn bt_rom_dvs(&mut self) -> BtRomDvsW<'_, BtFtcMemCtrlSpec> {
        BtRomDvsW::new(self, 15)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn bt_rom_dvse(&mut self) -> BtRomDvseW<'_, BtFtcMemCtrlSpec> {
        BtRomDvseW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn bt_tmck_all(&mut self) -> BtTmckAllW<'_, BtFtcMemCtrlSpec> {
        BtTmckAllW::new(self, 20)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn bt_lvdrf_clkdis(&mut self) -> BtLvdrfClkdisW<'_, BtFtcMemCtrlSpec> {
        BtLvdrfClkdisW::new(self, 22)
    }
    #[doc = "Bit 23 - 1 - All BT memory switch to Deep Sleep mode 0 - Power mode of BT memory are controlled by BTPMC"]
    #[inline(always)]
    pub fn bt_ds_all(&mut self) -> BtDsAllW<'_, BtFtcMemCtrlSpec> {
        BtDsAllW::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`bt_ftc_mem_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bt_ftc_mem_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BtFtcMemCtrlSpec;
impl crate::RegisterSpec for BtFtcMemCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bt_ftc_mem_ctrl::R`](R) reader structure"]
impl crate::Readable for BtFtcMemCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`bt_ftc_mem_ctrl::W`](W) writer structure"]
impl crate::Writable for BtFtcMemCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BT_FTC_MEM_CTRL to value 0x0002_ad8c"]
impl crate::Resettable for BtFtcMemCtrlSpec {
    const RESET_VALUE: u32 = 0x0002_ad8c;
}
