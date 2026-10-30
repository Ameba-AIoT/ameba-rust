#[doc = "Register `REG_AXI_CTRL` reader"]
pub type R = crate::R<RegAxiCtrlSpec>;
#[doc = "Register `REG_AXI_CTRL` writer"]
pub type W = crate::W<RegAxiCtrlSpec>;
#[doc = "Field `BIT_MQ_STOP` reader - Stop MQ DMA"]
pub type BitMqStopR = crate::FieldReader;
#[doc = "Field `BIT_MQ_STOP` writer - Stop MQ DMA"]
pub type BitMqStopW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_STOPRX` reader - RX DMA STOP"]
pub type BitStoprxR = crate::BitReader;
#[doc = "Field `BIT_STOPRX` writer - RX DMA STOP"]
pub type BitStoprxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_STOPBK` reader - BKQ DMA STOP"]
pub type BitStopbkR = crate::BitReader;
#[doc = "Field `BIT_STOPBK` writer - BKQ DMA STOP"]
pub type BitStopbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_STOPBE` reader - BEQ DMA STOP"]
pub type BitStopbeR = crate::BitReader;
#[doc = "Field `BIT_STOPBE` writer - BEQ DMA STOP"]
pub type BitStopbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_STOPVI` reader - VIQ DMA STOP"]
pub type BitStopviR = crate::BitReader;
#[doc = "Field `BIT_STOPVI` writer - VIQ DMA STOP"]
pub type BitStopviW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_STOPVO` reader - VOQ DMA STOP"]
pub type BitStopvoR = crate::BitReader;
#[doc = "Field `BIT_STOPVO` writer - VOQ DMA STOP"]
pub type BitStopvoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_STOPMG` reader - MGQ DMA STOP"]
pub type BitStopmgR = crate::BitReader;
#[doc = "Field `BIT_STOPMG` writer - MGQ DMA STOP"]
pub type BitStopmgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_STOPBCN` reader - BCNQ DMA STOP"]
pub type BitStopbcnR = crate::BitReader;
#[doc = "Field `BIT_STOPBCN` writer - BCNQ DMA STOP"]
pub type BitStopbcnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_WT_RXTAG` reader - Display Dword0\\[32:16\\] type of Rx buffer descriptor 0: TotalLen 1: Rxtag"]
pub type BitEnWtRxtagR = crate::BitReader;
#[doc = "Field `BIT_EN_WT_RXTAG` writer - Display Dword0\\[32:16\\] type of Rx buffer descriptor 0: TotalLen 1: Rxtag"]
pub type BitEnWtRxtagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MBSSID_ENSWBCN_BACKDOOR` reader - MBSSID enable software becon"]
pub type BitMbssidEnswbcnBackdoorR = crate::BitReader;
#[doc = "Field `BIT_MBSSID_ENSWBCN_BACKDOOR` writer - MBSSID enable software becon"]
pub type BitMbssidEnswbcnBackdoorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FIX_RXDMA_EN` reader - rxdma error fix enable"]
pub type BitFixRxdmaEnR = crate::BitReader;
#[doc = "Field `BIT_FIX_RXDMA_EN` writer - rxdma error fix enable"]
pub type BitFixRxdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TXDMA_ERROR_FLAG` reader - {length_err,page_err,arb_err,prefetch_err,txqsel_err} length_err: txbd length and txdesc length unmatch page_err: txbd page num and txdesc page num unmatch arb_err: queue arb error prefetch_err: next queue arb error txqsel_err: tx queue select unmatch with txdesc queue select"]
pub type BitTxdmaErrorFlagR = crate::FieldReader;
#[doc = "Field `BIT_REG_TXDMA_FAIL_PS` writer - Write 1 to cause a txdma fail ps"]
pub type BitRegTxdmaFailPsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MAX_TXDMA` reader - TXDMA Burst Size selection default 3. 0: 4bytes; 1: 16bytes; 2: 32bytes; 3: 64bytes."]
pub type BitMaxTxdmaR = crate::FieldReader;
#[doc = "Field `BIT_MAX_TXDMA` writer - TXDMA Burst Size selection default 3. 0: 4bytes; 1: 16bytes; 2: 32bytes; 3: 64bytes."]
pub type BitMaxTxdmaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_MAX_RXDMA` reader - RXDMA Burst Size selection default 3. 0: 4bytes; 1: 16bytes; 2: 32bytes; 3: 64bytes"]
pub type BitMaxRxdmaR = crate::FieldReader;
#[doc = "Field `BIT_MAX_RXDMA` writer - RXDMA Burst Size selection default 3. 0: 4bytes; 1: 16bytes; 2: 32bytes; 3: 64bytes"]
pub type BitMaxRxdmaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Stop MQ DMA"]
    #[inline(always)]
    pub fn bit_mq_stop(&self) -> BitMqStopR {
        BitMqStopR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - RX DMA STOP"]
    #[inline(always)]
    pub fn bit_stoprx(&self) -> BitStoprxR {
        BitStoprxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BKQ DMA STOP"]
    #[inline(always)]
    pub fn bit_stopbk(&self) -> BitStopbkR {
        BitStopbkR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BEQ DMA STOP"]
    #[inline(always)]
    pub fn bit_stopbe(&self) -> BitStopbeR {
        BitStopbeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - VIQ DMA STOP"]
    #[inline(always)]
    pub fn bit_stopvi(&self) -> BitStopviR {
        BitStopviR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - VOQ DMA STOP"]
    #[inline(always)]
    pub fn bit_stopvo(&self) -> BitStopvoR {
        BitStopvoR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MGQ DMA STOP"]
    #[inline(always)]
    pub fn bit_stopmg(&self) -> BitStopmgR {
        BitStopmgR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - BCNQ DMA STOP"]
    #[inline(always)]
    pub fn bit_stopbcn(&self) -> BitStopbcnR {
        BitStopbcnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Display Dword0\\[32:16\\] type of Rx buffer descriptor 0: TotalLen 1: Rxtag"]
    #[inline(always)]
    pub fn bit_en_wt_rxtag(&self) -> BitEnWtRxtagR {
        BitEnWtRxtagR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MBSSID enable software becon"]
    #[inline(always)]
    pub fn bit_mbssid_enswbcn_backdoor(&self) -> BitMbssidEnswbcnBackdoorR {
        BitMbssidEnswbcnBackdoorR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - rxdma error fix enable"]
    #[inline(always)]
    pub fn bit_fix_rxdma_en(&self) -> BitFixRxdmaEnR {
        BitFixRxdmaEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:22 - {length_err,page_err,arb_err,prefetch_err,txqsel_err} length_err: txbd length and txdesc length unmatch page_err: txbd page num and txdesc page num unmatch arb_err: queue arb error prefetch_err: next queue arb error txqsel_err: tx queue select unmatch with txdesc queue select"]
    #[inline(always)]
    pub fn bit_txdma_error_flag(&self) -> BitTxdmaErrorFlagR {
        BitTxdmaErrorFlagR::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 24:25 - TXDMA Burst Size selection default 3. 0: 4bytes; 1: 16bytes; 2: 32bytes; 3: 64bytes."]
    #[inline(always)]
    pub fn bit_max_txdma(&self) -> BitMaxTxdmaR {
        BitMaxTxdmaR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - RXDMA Burst Size selection default 3. 0: 4bytes; 1: 16bytes; 2: 32bytes; 3: 64bytes"]
    #[inline(always)]
    pub fn bit_max_rxdma(&self) -> BitMaxRxdmaR {
        BitMaxRxdmaR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Stop MQ DMA"]
    #[inline(always)]
    pub fn bit_mq_stop(&mut self) -> BitMqStopW<'_, RegAxiCtrlSpec> {
        BitMqStopW::new(self, 0)
    }
    #[doc = "Bit 8 - RX DMA STOP"]
    #[inline(always)]
    pub fn bit_stoprx(&mut self) -> BitStoprxW<'_, RegAxiCtrlSpec> {
        BitStoprxW::new(self, 8)
    }
    #[doc = "Bit 9 - BKQ DMA STOP"]
    #[inline(always)]
    pub fn bit_stopbk(&mut self) -> BitStopbkW<'_, RegAxiCtrlSpec> {
        BitStopbkW::new(self, 9)
    }
    #[doc = "Bit 10 - BEQ DMA STOP"]
    #[inline(always)]
    pub fn bit_stopbe(&mut self) -> BitStopbeW<'_, RegAxiCtrlSpec> {
        BitStopbeW::new(self, 10)
    }
    #[doc = "Bit 11 - VIQ DMA STOP"]
    #[inline(always)]
    pub fn bit_stopvi(&mut self) -> BitStopviW<'_, RegAxiCtrlSpec> {
        BitStopviW::new(self, 11)
    }
    #[doc = "Bit 12 - VOQ DMA STOP"]
    #[inline(always)]
    pub fn bit_stopvo(&mut self) -> BitStopvoW<'_, RegAxiCtrlSpec> {
        BitStopvoW::new(self, 12)
    }
    #[doc = "Bit 13 - MGQ DMA STOP"]
    #[inline(always)]
    pub fn bit_stopmg(&mut self) -> BitStopmgW<'_, RegAxiCtrlSpec> {
        BitStopmgW::new(self, 13)
    }
    #[doc = "Bit 14 - BCNQ DMA STOP"]
    #[inline(always)]
    pub fn bit_stopbcn(&mut self) -> BitStopbcnW<'_, RegAxiCtrlSpec> {
        BitStopbcnW::new(self, 14)
    }
    #[doc = "Bit 15 - Display Dword0\\[32:16\\] type of Rx buffer descriptor 0: TotalLen 1: Rxtag"]
    #[inline(always)]
    pub fn bit_en_wt_rxtag(&mut self) -> BitEnWtRxtagW<'_, RegAxiCtrlSpec> {
        BitEnWtRxtagW::new(self, 15)
    }
    #[doc = "Bit 16 - MBSSID enable software becon"]
    #[inline(always)]
    pub fn bit_mbssid_enswbcn_backdoor(&mut self) -> BitMbssidEnswbcnBackdoorW<'_, RegAxiCtrlSpec> {
        BitMbssidEnswbcnBackdoorW::new(self, 16)
    }
    #[doc = "Bit 17 - rxdma error fix enable"]
    #[inline(always)]
    pub fn bit_fix_rxdma_en(&mut self) -> BitFixRxdmaEnW<'_, RegAxiCtrlSpec> {
        BitFixRxdmaEnW::new(self, 17)
    }
    #[doc = "Bit 23 - Write 1 to cause a txdma fail ps"]
    #[inline(always)]
    pub fn bit_reg_txdma_fail_ps(&mut self) -> BitRegTxdmaFailPsW<'_, RegAxiCtrlSpec> {
        BitRegTxdmaFailPsW::new(self, 23)
    }
    #[doc = "Bits 24:25 - TXDMA Burst Size selection default 3. 0: 4bytes; 1: 16bytes; 2: 32bytes; 3: 64bytes."]
    #[inline(always)]
    pub fn bit_max_txdma(&mut self) -> BitMaxTxdmaW<'_, RegAxiCtrlSpec> {
        BitMaxTxdmaW::new(self, 24)
    }
    #[doc = "Bits 28:29 - RXDMA Burst Size selection default 3. 0: 4bytes; 1: 16bytes; 2: 32bytes; 3: 64bytes"]
    #[inline(always)]
    pub fn bit_max_rxdma(&mut self) -> BitMaxRxdmaW<'_, RegAxiCtrlSpec> {
        BitMaxRxdmaW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_axi_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_axi_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxiCtrlSpec;
impl crate::RegisterSpec for RegAxiCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_axi_ctrl::R`](R) reader structure"]
impl crate::Readable for RegAxiCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_axi_ctrl::W`](W) writer structure"]
impl crate::Writable for RegAxiCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AXI_CTRL to value 0x3302_8000"]
impl crate::Resettable for RegAxiCtrlSpec {
    const RESET_VALUE: u32 = 0x3302_8000;
}
