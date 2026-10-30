#[doc = "Register `EEEP_CR1` reader"]
pub type R = crate::R<EeepCr1Spec>;
#[doc = "Register `EEEP_CR1` writer"]
pub type W = crate::W<EeepCr1Spec>;
#[doc = "Field `Eeep_dbgo_sel` reader - eeep module debug signals selection."]
pub type EeepDbgoSelR = crate::FieldReader;
#[doc = "Field `Eeep_dbgo_sel` writer - eeep module debug signals selection."]
pub type EeepDbgoSelW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_MST_MODE` reader - When PMA is in master clock role, the EEEP_RX can work. SW needs to configure PCS reg9.bit12(Master-slave manual config enable) and reg9.bit11(Master-slave config value). Then setting EEEP_CR1_REG.PHY_MST_MODE."]
pub type PhyMstModeR = crate::BitReader;
#[doc = "Field `PHY_MST_MODE` writer - When PMA is in master clock role, the EEEP_RX can work. SW needs to configure PCS reg9.bit12(Master-slave manual config enable) and reg9.bit11(Master-slave config value). Then setting EEEP_CR1_REG.PHY_MST_MODE."]
pub type PhyMstModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG_EEEP_DEFER_TXLPI` reader - ???"]
pub type CfgEeepDeferTxlpiR = crate::BitReader;
#[doc = "Field `CFG_EEEP_DEFER_TXLPI` writer - ???"]
pub type CfgEeepDeferTxlpiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG_EN_EEEP` reader - 1: Enable EEEP in mode. 0: Disable EEEP in mode."]
pub type CfgEnEeepR = crate::BitReader;
#[doc = "Field `CFG_EN_EEEP` writer - 1: Enable EEEP in mode. 0: Disable EEEP in mode."]
pub type CfgEnEeepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG_EN_EEEP_GIGA` reader - 1: Enable EEEP in giga mode. 0: Disable EEEP in giga mode."]
pub type CfgEnEeepGigaR = crate::BitReader;
#[doc = "Field `CFG_EN_EEEP_GIGA` writer - 1: Enable EEEP in giga mode. 0: Disable EEEP in giga mode."]
pub type CfgEnEeepGigaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG_EN_EEEP_TX` reader - 1: Enable EEEP TX. 0: Disable EEEP TX."]
pub type CfgEnEeepTxR = crate::BitReader;
#[doc = "Field `CFG_EN_EEEP_TX` writer - 1: Enable EEEP TX. 0: Disable EEEP TX."]
pub type CfgEnEeepTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG_EN_EEEP_RX` reader - 1: Enable EEEP RX. 0: Disable EEEP RX."]
pub type CfgEnEeepRxR = crate::BitReader;
#[doc = "Field `CFG_EN_EEEP_RX` writer - 1: Enable EEEP RX. 0: Disable EEEP RX."]
pub type CfgEnEeepRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `En_EEEP` reader - Enable EEEP function 0b0: Disable EEEP function. 0b1: Enable EEEP function."]
pub type EnEeepR = crate::BitReader;
#[doc = "Field `En_EEEP` writer - Enable EEEP function 0b0: Disable EEEP function. 0b1: Enable EEEP function."]
pub type EnEeepW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:22 - eeep module debug signals selection."]
    #[inline(always)]
    pub fn eeep_dbgo_sel(&self) -> EeepDbgoSelR {
        EeepDbgoSelR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - When PMA is in master clock role, the EEEP_RX can work. SW needs to configure PCS reg9.bit12(Master-slave manual config enable) and reg9.bit11(Master-slave config value). Then setting EEEP_CR1_REG.PHY_MST_MODE."]
    #[inline(always)]
    pub fn phy_mst_mode(&self) -> PhyMstModeR {
        PhyMstModeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - ???"]
    #[inline(always)]
    pub fn cfg_eeep_defer_txlpi(&self) -> CfgEeepDeferTxlpiR {
        CfgEeepDeferTxlpiR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1: Enable EEEP in mode. 0: Disable EEEP in mode."]
    #[inline(always)]
    pub fn cfg_en_eeep(&self) -> CfgEnEeepR {
        CfgEnEeepR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: Enable EEEP in giga mode. 0: Disable EEEP in giga mode."]
    #[inline(always)]
    pub fn cfg_en_eeep_giga(&self) -> CfgEnEeepGigaR {
        CfgEnEeepGigaR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: Enable EEEP TX. 0: Disable EEEP TX."]
    #[inline(always)]
    pub fn cfg_en_eeep_tx(&self) -> CfgEnEeepTxR {
        CfgEnEeepTxR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: Enable EEEP RX. 0: Disable EEEP RX."]
    #[inline(always)]
    pub fn cfg_en_eeep_rx(&self) -> CfgEnEeepRxR {
        CfgEnEeepRxR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable EEEP function 0b0: Disable EEEP function. 0b1: Enable EEEP function."]
    #[inline(always)]
    pub fn en_eeep(&self) -> EnEeepR {
        EnEeepR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:22 - eeep module debug signals selection."]
    #[inline(always)]
    pub fn eeep_dbgo_sel(&mut self) -> EeepDbgoSelW<'_, EeepCr1Spec> {
        EeepDbgoSelW::new(self, 16)
    }
    #[doc = "Bit 24 - When PMA is in master clock role, the EEEP_RX can work. SW needs to configure PCS reg9.bit12(Master-slave manual config enable) and reg9.bit11(Master-slave config value). Then setting EEEP_CR1_REG.PHY_MST_MODE."]
    #[inline(always)]
    pub fn phy_mst_mode(&mut self) -> PhyMstModeW<'_, EeepCr1Spec> {
        PhyMstModeW::new(self, 24)
    }
    #[doc = "Bit 26 - ???"]
    #[inline(always)]
    pub fn cfg_eeep_defer_txlpi(&mut self) -> CfgEeepDeferTxlpiW<'_, EeepCr1Spec> {
        CfgEeepDeferTxlpiW::new(self, 26)
    }
    #[doc = "Bit 27 - 1: Enable EEEP in mode. 0: Disable EEEP in mode."]
    #[inline(always)]
    pub fn cfg_en_eeep(&mut self) -> CfgEnEeepW<'_, EeepCr1Spec> {
        CfgEnEeepW::new(self, 27)
    }
    #[doc = "Bit 28 - 1: Enable EEEP in giga mode. 0: Disable EEEP in giga mode."]
    #[inline(always)]
    pub fn cfg_en_eeep_giga(&mut self) -> CfgEnEeepGigaW<'_, EeepCr1Spec> {
        CfgEnEeepGigaW::new(self, 28)
    }
    #[doc = "Bit 29 - 1: Enable EEEP TX. 0: Disable EEEP TX."]
    #[inline(always)]
    pub fn cfg_en_eeep_tx(&mut self) -> CfgEnEeepTxW<'_, EeepCr1Spec> {
        CfgEnEeepTxW::new(self, 29)
    }
    #[doc = "Bit 30 - 1: Enable EEEP RX. 0: Disable EEEP RX."]
    #[inline(always)]
    pub fn cfg_en_eeep_rx(&mut self) -> CfgEnEeepRxW<'_, EeepCr1Spec> {
        CfgEnEeepRxW::new(self, 30)
    }
    #[doc = "Bit 31 - Enable EEEP function 0b0: Disable EEEP function. 0b1: Enable EEEP function."]
    #[inline(always)]
    pub fn en_eeep(&mut self) -> EnEeepW<'_, EeepCr1Spec> {
        EnEeepW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`eeep_cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eeep_cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EeepCr1Spec;
impl crate::RegisterSpec for EeepCr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eeep_cr1::R`](R) reader structure"]
impl crate::Readable for EeepCr1Spec {}
#[doc = "`write(|w| ..)` method takes [`eeep_cr1::W`](W) writer structure"]
impl crate::Writable for EeepCr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EEEP_CR1 to value 0"]
impl crate::Resettable for EeepCr1Spec {}
