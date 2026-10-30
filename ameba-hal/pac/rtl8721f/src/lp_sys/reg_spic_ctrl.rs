#[doc = "Register `REG_SPIC_CTRL` reader"]
pub type R = crate::R<RegSpicCtrlSpec>;
#[doc = "Register `REG_SPIC_CTRL` writer"]
pub type W = crate::W<RegSpicCtrlSpec>;
#[doc = "Field `ecc_icg_disable` reader - Disable NAND ECC auto icg to save power 1:disable , no gating 0:enable"]
pub type EccIcgDisableR = crate::BitReader;
#[doc = "Field `ecc_icg_disable` writer - Disable NAND ECC auto icg to save power 1:disable , no gating 0:enable"]
pub type EccIcgDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_CAL_EN` reader - Flash calibration enable 1: delay flash sout for calibration 0: bypass flash sout to spic"]
pub type FlashCalEnR = crate::BitReader;
#[doc = "Field `FLASH_CAL_EN` writer - Flash calibration enable 1: delay flash sout for calibration 0: bypass flash sout to spic"]
pub type FlashCalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPHY_EN` reader - when set, enable spic_dphy function to read data from flash"]
pub type DphyEnR = crate::BitReader;
#[doc = "Field `DPHY_EN` writer - when set, enable spic_dphy function to read data from flash"]
pub type DphyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPHY_RD_DELAY_CNT` reader - spic_dphy output dealy cnt, when set < 3, output data will occur at fourth or fifth axi clock from the rising edge of spi_rx_data_en; when set >= 3, data will occur after DPHY_RD_DELAY_CNT + 2 axi clock from the rising edge of spi_rx_data_en"]
pub type DphyRdDelayCntR = crate::FieldReader;
#[doc = "Field `DPHY_RD_DELAY_CNT` writer - spic_dphy output dealy cnt, when set < 3, output data will occur at fourth or fifth axi clock from the rising edge of spi_rx_data_en; when set >= 3, data will occur after DPHY_RD_DELAY_CNT + 2 axi clock from the rising edge of spi_rx_data_en"]
pub type DphyRdDelayCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 12 - Disable NAND ECC auto icg to save power 1:disable , no gating 0:enable"]
    #[inline(always)]
    pub fn ecc_icg_disable(&self) -> EccIcgDisableR {
        EccIcgDisableR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Flash calibration enable 1: delay flash sout for calibration 0: bypass flash sout to spic"]
    #[inline(always)]
    pub fn flash_cal_en(&self) -> FlashCalEnR {
        FlashCalEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - when set, enable spic_dphy function to read data from flash"]
    #[inline(always)]
    pub fn dphy_en(&self) -> DphyEnR {
        DphyEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:20 - spic_dphy output dealy cnt, when set < 3, output data will occur at fourth or fifth axi clock from the rising edge of spi_rx_data_en; when set >= 3, data will occur after DPHY_RD_DELAY_CNT + 2 axi clock from the rising edge of spi_rx_data_en"]
    #[inline(always)]
    pub fn dphy_rd_delay_cnt(&self) -> DphyRdDelayCntR {
        DphyRdDelayCntR::new(((self.bits >> 17) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 12 - Disable NAND ECC auto icg to save power 1:disable , no gating 0:enable"]
    #[inline(always)]
    pub fn ecc_icg_disable(&mut self) -> EccIcgDisableW<'_, RegSpicCtrlSpec> {
        EccIcgDisableW::new(self, 12)
    }
    #[doc = "Bit 15 - Flash calibration enable 1: delay flash sout for calibration 0: bypass flash sout to spic"]
    #[inline(always)]
    pub fn flash_cal_en(&mut self) -> FlashCalEnW<'_, RegSpicCtrlSpec> {
        FlashCalEnW::new(self, 15)
    }
    #[doc = "Bit 16 - when set, enable spic_dphy function to read data from flash"]
    #[inline(always)]
    pub fn dphy_en(&mut self) -> DphyEnW<'_, RegSpicCtrlSpec> {
        DphyEnW::new(self, 16)
    }
    #[doc = "Bits 17:20 - spic_dphy output dealy cnt, when set < 3, output data will occur at fourth or fifth axi clock from the rising edge of spi_rx_data_en; when set >= 3, data will occur after DPHY_RD_DELAY_CNT + 2 axi clock from the rising edge of spi_rx_data_en"]
    #[inline(always)]
    pub fn dphy_rd_delay_cnt(&mut self) -> DphyRdDelayCntW<'_, RegSpicCtrlSpec> {
        DphyRdDelayCntW::new(self, 17)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_spic_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_spic_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSpicCtrlSpec;
impl crate::RegisterSpec for RegSpicCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_spic_ctrl::R`](R) reader structure"]
impl crate::Readable for RegSpicCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_spic_ctrl::W`](W) writer structure"]
impl crate::Writable for RegSpicCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SPIC_CTRL to value 0x1000"]
impl crate::Resettable for RegSpicCtrlSpec {
    const RESET_VALUE: u32 = 0x1000;
}
