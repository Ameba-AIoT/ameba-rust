#[doc = "Register `UART_RX_PATH_CTRL` reader"]
pub type R = crate::R<UartRxPathCtrlSpec>;
#[doc = "Register `UART_RX_PATH_CTRL` writer"]
pub type W = crate::W<UartRxPathCtrlSpec>;
#[doc = "Field `R_RST_NEWRX_N` reader - Reset new Rx path, low active"]
pub type RRstNewrxNR = crate::BitReader;
#[doc = "Field `R_RST_NEWRX_N` writer - Reset new Rx path, low active"]
pub type RRstNewrxNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBAUD_ADJ_10_0` reader - One factor of Baud rate calculation for Rx path, similar with xfactor_adj"]
pub type RxbaudAdj10_0R = crate::FieldReader<u16>;
#[doc = "Field `RXBAUD_ADJ_10_0` writer - One factor of Baud rate calculation for Rx path, similar with xfactor_adj"]
pub type RxbaudAdj10_0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `R_RXTO_THRS` reader - Indicate the timeout threshold of Rx path, unit in one-bit time"]
pub type RRxtoThrsR = crate::FieldReader<u16>;
#[doc = "Field `R_RXTO_THRS` writer - Indicate the timeout threshold of Rx path, unit in one-bit time"]
pub type RRxtoThrsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 2 - Reset new Rx path, low active"]
    #[inline(always)]
    pub fn r_rst_newrx_n(&self) -> RRstNewrxNR {
        RRstNewrxNR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:13 - One factor of Baud rate calculation for Rx path, similar with xfactor_adj"]
    #[inline(always)]
    pub fn rxbaud_adj_10_0(&self) -> RxbaudAdj10_0R {
        RxbaudAdj10_0R::new(((self.bits >> 3) & 0x07ff) as u16)
    }
    #[doc = "Bits 16:31 - Indicate the timeout threshold of Rx path, unit in one-bit time"]
    #[inline(always)]
    pub fn r_rxto_thrs(&self) -> RRxtoThrsR {
        RRxtoThrsR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 2 - Reset new Rx path, low active"]
    #[inline(always)]
    pub fn r_rst_newrx_n(&mut self) -> RRstNewrxNW<'_, UartRxPathCtrlSpec> {
        RRstNewrxNW::new(self, 2)
    }
    #[doc = "Bits 3:13 - One factor of Baud rate calculation for Rx path, similar with xfactor_adj"]
    #[inline(always)]
    pub fn rxbaud_adj_10_0(&mut self) -> RxbaudAdj10_0W<'_, UartRxPathCtrlSpec> {
        RxbaudAdj10_0W::new(self, 3)
    }
    #[doc = "Bits 16:31 - Indicate the timeout threshold of Rx path, unit in one-bit time"]
    #[inline(always)]
    pub fn r_rxto_thrs(&mut self) -> RRxtoThrsW<'_, UartRxPathCtrlSpec> {
        RRxtoThrsW::new(self, 16)
    }
}
#[doc = "UART Rx path control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_rx_path_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_rx_path_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartRxPathCtrlSpec;
impl crate::RegisterSpec for UartRxPathCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_rx_path_ctrl::R`](R) reader structure"]
impl crate::Readable for UartRxPathCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_rx_path_ctrl::W`](W) writer structure"]
impl crate::Writable for UartRxPathCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_RX_PATH_CTRL to value 0x0040_0000"]
impl crate::Resettable for UartRxPathCtrlSpec {
    const RESET_VALUE: u32 = 0x0040_0000;
}
