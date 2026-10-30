#[doc = "Register `UART_RP_RX_PATH_CTRL` reader"]
pub type R = crate::R<UartRpRxPathCtrlSpec>;
#[doc = "Register `UART_RP_RX_PATH_CTRL` writer"]
pub type W = crate::W<UartRpRxPathCtrlSpec>;
#[doc = "Field `r_rst_rp_rx_n` reader - Reset Relay Rx path, low active"]
pub type RRstRpRxNR = crate::BitReader;
#[doc = "Field `r_rst_rp_rx_n` writer - Reset Relay Rx path, low active"]
pub type RRstRpRxNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rp_rxbaud_adj_10_0` reader - One factor of Baud rate calculation for Relay path, similar with xfactor_adj"]
pub type RpRxbaudAdj10_0R = crate::FieldReader<u16>;
#[doc = "Field `rp_rxbaud_adj_10_0` writer - One factor of Baud rate calculation for Relay path, similar with xfactor_adj"]
pub type RpRxbaudAdj10_0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bit 2 - Reset Relay Rx path, low active"]
    #[inline(always)]
    pub fn r_rst_rp_rx_n(&self) -> RRstRpRxNR {
        RRstRpRxNR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:13 - One factor of Baud rate calculation for Relay path, similar with xfactor_adj"]
    #[inline(always)]
    pub fn rp_rxbaud_adj_10_0(&self) -> RpRxbaudAdj10_0R {
        RpRxbaudAdj10_0R::new(((self.bits >> 3) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 2 - Reset Relay Rx path, low active"]
    #[inline(always)]
    pub fn r_rst_rp_rx_n(&mut self) -> RRstRpRxNW<'_, UartRpRxPathCtrlSpec> {
        RRstRpRxNW::new(self, 2)
    }
    #[doc = "Bits 3:13 - One factor of Baud rate calculation for Relay path, similar with xfactor_adj"]
    #[inline(always)]
    pub fn rp_rxbaud_adj_10_0(&mut self) -> RpRxbaudAdj10_0W<'_, UartRpRxPathCtrlSpec> {
        RpRxbaudAdj10_0W::new(self, 3)
    }
}
#[doc = "UART Relay Rx path control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_rp_rx_path_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_rp_rx_path_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartRpRxPathCtrlSpec;
impl crate::RegisterSpec for UartRpRxPathCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_rp_rx_path_ctrl::R`](R) reader structure"]
impl crate::Readable for UartRpRxPathCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_rp_rx_path_ctrl::W`](W) writer structure"]
impl crate::Writable for UartRpRxPathCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_RP_RX_PATH_CTRL to value 0"]
impl crate::Resettable for UartRpRxPathCtrlSpec {}
