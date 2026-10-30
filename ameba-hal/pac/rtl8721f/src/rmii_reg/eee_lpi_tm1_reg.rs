#[doc = "Register `EEE_LPI_TM1_REG` reader"]
pub type R = crate::R<EeeLpiTm1RegSpec>;
#[doc = "Register `EEE_LPI_TM1_REG` writer"]
pub type W = crate::W<EeeLpiTm1RegSpec>;
#[doc = "Field `EEE_TXLPI_Time` reader - TX in LPI time. (unit: 1us ) (accumulation)Valid as (PORT_MASK\\[0\\]==1&&TXTimer_EN==1)"]
pub type EeeTxlpiTimeR = crate::FieldReader<u32>;
#[doc = "Field `eee_trxctl_level` reader - The gmac_rgmii_txe and gmac_rgmii_rxdv level when gmac is in lpi. 1: tx_ctl and rx_ctl is high when in lpi. 0: reserved."]
pub type EeeTrxctlLevelR = crate::BitReader;
#[doc = "Field `eee_trxctl_level` writer - The gmac_rgmii_txe and gmac_rgmii_rxdv level when gmac is in lpi. 1: tx_ctl and rx_ctl is high when in lpi. 0: reserved."]
pub type EeeTrxctlLevelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `eee_stop_txc` reader - This bit works in rgmii_1000 and rgmii_100 modes only. 1: gmac halts gmac_rgmii_txc_out when gmac enters lpi. 0: gmac does not halt gmac_rgmii_txc_out when gmac enters lpi. This bit should always be b0 for RLE0390(formerly RL6166). Gmac can halt gmac_rgmii_txc_out in the next version."]
pub type EeeStopTxcR = crate::BitReader;
#[doc = "Field `eee_stop_txc` writer - This bit works in rgmii_1000 and rgmii_100 modes only. 1: gmac halts gmac_rgmii_txc_out when gmac enters lpi. 0: gmac does not halt gmac_rgmii_txc_out when gmac enters lpi. This bit should always be b0 for RLE0390(formerly RL6166). Gmac can halt gmac_rgmii_txc_out in the next version."]
pub type EeeStopTxcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:21 - TX in LPI time. (unit: 1us ) (accumulation)Valid as (PORT_MASK\\[0\\]==1&&TXTimer_EN==1)"]
    #[inline(always)]
    pub fn eee_txlpi_time(&self) -> EeeTxlpiTimeR {
        EeeTxlpiTimeR::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bit 22 - The gmac_rgmii_txe and gmac_rgmii_rxdv level when gmac is in lpi. 1: tx_ctl and rx_ctl is high when in lpi. 0: reserved."]
    #[inline(always)]
    pub fn eee_trxctl_level(&self) -> EeeTrxctlLevelR {
        EeeTrxctlLevelR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - This bit works in rgmii_1000 and rgmii_100 modes only. 1: gmac halts gmac_rgmii_txc_out when gmac enters lpi. 0: gmac does not halt gmac_rgmii_txc_out when gmac enters lpi. This bit should always be b0 for RLE0390(formerly RL6166). Gmac can halt gmac_rgmii_txc_out in the next version."]
    #[inline(always)]
    pub fn eee_stop_txc(&self) -> EeeStopTxcR {
        EeeStopTxcR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 22 - The gmac_rgmii_txe and gmac_rgmii_rxdv level when gmac is in lpi. 1: tx_ctl and rx_ctl is high when in lpi. 0: reserved."]
    #[inline(always)]
    pub fn eee_trxctl_level(&mut self) -> EeeTrxctlLevelW<'_, EeeLpiTm1RegSpec> {
        EeeTrxctlLevelW::new(self, 22)
    }
    #[doc = "Bit 23 - This bit works in rgmii_1000 and rgmii_100 modes only. 1: gmac halts gmac_rgmii_txc_out when gmac enters lpi. 0: gmac does not halt gmac_rgmii_txc_out when gmac enters lpi. This bit should always be b0 for RLE0390(formerly RL6166). Gmac can halt gmac_rgmii_txc_out in the next version."]
    #[inline(always)]
    pub fn eee_stop_txc(&mut self) -> EeeStopTxcW<'_, EeeLpiTm1RegSpec> {
        EeeStopTxcW::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`eee_lpi_tm1_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eee_lpi_tm1_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EeeLpiTm1RegSpec;
impl crate::RegisterSpec for EeeLpiTm1RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eee_lpi_tm1_reg::R`](R) reader structure"]
impl crate::Readable for EeeLpiTm1RegSpec {}
#[doc = "`write(|w| ..)` method takes [`eee_lpi_tm1_reg::W`](W) writer structure"]
impl crate::Writable for EeeLpiTm1RegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EEE_LPI_TM1_REG to value 0"]
impl crate::Resettable for EeeLpiTm1RegSpec {}
