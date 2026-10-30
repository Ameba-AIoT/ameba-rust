#[doc = "Register `UART_AGGC` reader"]
pub type R = crate::R<UartAggcSpec>;
#[doc = "Register `UART_AGGC` writer"]
pub type W = crate::W<UartAggcSpec>;
#[doc = "Field `agg_sync_pat` reader - AGG packet Start"]
pub type AggSyncPatR = crate::FieldReader;
#[doc = "Field `agg_sync_pat` writer - AGG packet Start"]
pub type AggSyncPatW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `agg_th` reader - AGG threshold 4'0000: 1 4'0111: 8 Numbers above 4'0111 are reserved."]
pub type AggThR = crate::FieldReader;
#[doc = "Field `agg_th` writer - AGG threshold 4'0000: 1 4'0111: 8 Numbers above 4'0111 are reserved."]
pub type AggThW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `agg_to` reader - AGG timeout Unit is one baudrate period of loguart Tx, default is 1ms if Tx baudrate is 3Mbps. Data should be sent from log FIFO to tx_fifo if idle time > AGG timeout, even if there is just only one byte in log FIFO of the path."]
pub type AggToR = crate::FieldReader<u16>;
#[doc = "Field `agg_to` writer - AGG timeout Unit is one baudrate period of loguart Tx, default is 1ms if Tx baudrate is 3Mbps. Data should be sent from log FIFO to tx_fifo if idle time > AGG timeout, even if there is just only one byte in log FIFO of the path."]
pub type AggToW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `agg_en` reader - Tx AGG function enable"]
pub type AggEnR = crate::BitReader;
#[doc = "Field `agg_en` writer - Tx AGG function enable"]
pub type AggEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tp1_en` reader - Tx Path1 enable *0: Disable *1: Enable"]
pub type Tp1EnR = crate::BitReader;
#[doc = "Field `tp1_en` writer - Tx Path1 enable *0: Disable *1: Enable"]
pub type Tp1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tp2_en` reader - Tx Path2 enable *0: Disable *1: Enable"]
pub type Tp2EnR = crate::BitReader;
#[doc = "Field `tp2_en` writer - Tx Path2 enable *0: Disable *1: Enable"]
pub type Tp2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tp3_en` reader - Tx Path3 enable *0: Disable *1: Enable"]
pub type Tp3EnR = crate::BitReader;
#[doc = "Field `tp3_en` writer - Tx Path3 enable *0: Disable *1: Enable"]
pub type Tp3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tp4_en` reader - Tx Path4 enable *0: Disable *1: Enable"]
pub type Tp4EnR = crate::BitReader;
#[doc = "Field `tp4_en` writer - Tx Path4 enable *0: Disable *1: Enable"]
pub type Tp4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rp_en` reader - Relay Rx Path enable *0: Disable *1: Enable"]
pub type RpEnR = crate::BitReader;
#[doc = "Field `rp_en` writer - Relay Rx Path enable *0: Disable *1: Enable"]
pub type RpEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - AGG packet Start"]
    #[inline(always)]
    pub fn agg_sync_pat(&self) -> AggSyncPatR {
        AggSyncPatR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - AGG threshold 4'0000: 1 4'0111: 8 Numbers above 4'0111 are reserved."]
    #[inline(always)]
    pub fn agg_th(&self) -> AggThR {
        AggThR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:25 - AGG timeout Unit is one baudrate period of loguart Tx, default is 1ms if Tx baudrate is 3Mbps. Data should be sent from log FIFO to tx_fifo if idle time > AGG timeout, even if there is just only one byte in log FIFO of the path."]
    #[inline(always)]
    pub fn agg_to(&self) -> AggToR {
        AggToR::new(((self.bits >> 12) & 0x3fff) as u16)
    }
    #[doc = "Bit 26 - Tx AGG function enable"]
    #[inline(always)]
    pub fn agg_en(&self) -> AggEnR {
        AggEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Tx Path1 enable *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn tp1_en(&self) -> Tp1EnR {
        Tp1EnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Tx Path2 enable *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn tp2_en(&self) -> Tp2EnR {
        Tp2EnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Tx Path3 enable *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn tp3_en(&self) -> Tp3EnR {
        Tp3EnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Tx Path4 enable *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn tp4_en(&self) -> Tp4EnR {
        Tp4EnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Relay Rx Path enable *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn rp_en(&self) -> RpEnR {
        RpEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - AGG packet Start"]
    #[inline(always)]
    pub fn agg_sync_pat(&mut self) -> AggSyncPatW<'_, UartAggcSpec> {
        AggSyncPatW::new(self, 0)
    }
    #[doc = "Bits 8:11 - AGG threshold 4'0000: 1 4'0111: 8 Numbers above 4'0111 are reserved."]
    #[inline(always)]
    pub fn agg_th(&mut self) -> AggThW<'_, UartAggcSpec> {
        AggThW::new(self, 8)
    }
    #[doc = "Bits 12:25 - AGG timeout Unit is one baudrate period of loguart Tx, default is 1ms if Tx baudrate is 3Mbps. Data should be sent from log FIFO to tx_fifo if idle time > AGG timeout, even if there is just only one byte in log FIFO of the path."]
    #[inline(always)]
    pub fn agg_to(&mut self) -> AggToW<'_, UartAggcSpec> {
        AggToW::new(self, 12)
    }
    #[doc = "Bit 26 - Tx AGG function enable"]
    #[inline(always)]
    pub fn agg_en(&mut self) -> AggEnW<'_, UartAggcSpec> {
        AggEnW::new(self, 26)
    }
    #[doc = "Bit 27 - Tx Path1 enable *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn tp1_en(&mut self) -> Tp1EnW<'_, UartAggcSpec> {
        Tp1EnW::new(self, 27)
    }
    #[doc = "Bit 28 - Tx Path2 enable *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn tp2_en(&mut self) -> Tp2EnW<'_, UartAggcSpec> {
        Tp2EnW::new(self, 28)
    }
    #[doc = "Bit 29 - Tx Path3 enable *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn tp3_en(&mut self) -> Tp3EnW<'_, UartAggcSpec> {
        Tp3EnW::new(self, 29)
    }
    #[doc = "Bit 30 - Tx Path4 enable *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn tp4_en(&mut self) -> Tp4EnW<'_, UartAggcSpec> {
        Tp4EnW::new(self, 30)
    }
    #[doc = "Bit 31 - Relay Rx Path enable *0: Disable *1: Enable"]
    #[inline(always)]
    pub fn rp_en(&mut self) -> RpEnW<'_, UartAggcSpec> {
        RpEnW::new(self, 31)
    }
}
#[doc = "UART AGG control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_aggc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_aggc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartAggcSpec;
impl crate::RegisterSpec for UartAggcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_aggc::R`](R) reader structure"]
impl crate::Readable for UartAggcSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_aggc::W`](W) writer structure"]
impl crate::Writable for UartAggcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_AGGC to value 0x00bb_87ff"]
impl crate::Resettable for UartAggcSpec {
    const RESET_VALUE: u32 = 0x00bb_87ff;
}
