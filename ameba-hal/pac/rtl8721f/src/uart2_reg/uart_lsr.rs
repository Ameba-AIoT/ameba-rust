#[doc = "Register `UART_LSR` reader"]
pub type R = crate::R<UartLsrSpec>;
#[doc = "Field `DRDY` reader - * 0: No characters in the Receiver FIFO * 1: At least one character has been received and transferred into the FIFO"]
pub type DrdyR = crate::BitReader;
#[doc = "Field `OVR_ERR` reader - Overrun Error (OE) indicator * 0: No Overrun state * 1: The transmission is continue although the Rx FIFO is full"]
pub type OvrErrR = crate::BitReader;
#[doc = "Field `PAR_ERR` reader - Parity Error (PE) indicator * 0: No parity error in current character * 1: The received character does not have the correct parity bit"]
pub type ParErrR = crate::BitReader;
#[doc = "Field `FRM_ERR` reader - Framing Error (FE) indicator * 0: No framing error in the current character * 1: The received character does not have a valid stop bit"]
pub type FrmErrR = crate::BitReader;
#[doc = "Field `BREAK_INT` reader - Break Interrupt (BI) indicator * 0: No break condition in the current character * 1: The received data input held in the Spacing ( logic 0) state is longer than a full word transmission time"]
pub type BreakIntR = crate::BitReader;
#[doc = "Field `TX_EMPTY` reader - TXFIFO empty indicator"]
pub type TxEmptyR = crate::BitReader;
#[doc = "Field `TX_NOT_FULL` reader - TXFIFO not full indicator"]
pub type TxNotFullR = crate::BitReader;
#[doc = "Field `RXFIFO_ERR` reader - Uart_rx_error, depending on the FIFO control register (FCR) bit\\[0\\] * 0: clear when the CPU reads the LSR, if there are no subsequent errors in the FIFO. * 1: parity error, overrun error, framing error or break indication in the FIFO."]
pub type RxfifoErrR = crate::BitReader;
#[doc = "Field `RXFIFO_INT` reader - This bit is only set when Received Data Available Interrupt interrupt is enabled(ERBI is set) 1: FIFO Trigger level reached or Rx FIFO full"]
pub type RxfifoIntR = crate::BitReader;
#[doc = "Field `TIMEOUT_INT` reader - This bit is only set when Rx Timeout interrupt is enabled(ETOI is set) 1: at least one character in the FIFO but no character has been input to the FIFO or reading from it for the time duration, which depends on the value in register REG_RX_PATH_CTRL\\[31:16\\]"]
pub type TimeoutIntR = crate::BitReader;
#[doc = "Field `MONITOR_DONE_INT` reader - This bit is only set when Rx Path Monitor Done interrupt is enabled(EMDI is set) 1: Rx path monitor done"]
pub type MonitorDoneIntR = crate::BitReader;
#[doc = "Field `MODEM_INT` reader - This bit is only set when Modem status pending 1: Modem interrupt"]
pub type ModemIntR = crate::BitReader;
#[doc = "Field `RXND_INT` reader - This bit is only set when RX debouncer has detected noise 1: Rx path noise detected interrupt"]
pub type RxndIntR = crate::BitReader;
#[doc = "Field `RE_TIMEOUT_INT` reader - This bit is only set when rxfifo empty timeout interrupt is enabled(ERETI is set) 1:Rxfifo is empty for longer than TH_RETI"]
pub type ReTimeoutIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - * 0: No characters in the Receiver FIFO * 1: At least one character has been received and transferred into the FIFO"]
    #[inline(always)]
    pub fn drdy(&self) -> DrdyR {
        DrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun Error (OE) indicator * 0: No Overrun state * 1: The transmission is continue although the Rx FIFO is full"]
    #[inline(always)]
    pub fn ovr_err(&self) -> OvrErrR {
        OvrErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Error (PE) indicator * 0: No parity error in current character * 1: The received character does not have the correct parity bit"]
    #[inline(always)]
    pub fn par_err(&self) -> ParErrR {
        ParErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Framing Error (FE) indicator * 0: No framing error in the current character * 1: The received character does not have a valid stop bit"]
    #[inline(always)]
    pub fn frm_err(&self) -> FrmErrR {
        FrmErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Break Interrupt (BI) indicator * 0: No break condition in the current character * 1: The received data input held in the Spacing ( logic 0) state is longer than a full word transmission time"]
    #[inline(always)]
    pub fn break_int(&self) -> BreakIntR {
        BreakIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXFIFO empty indicator"]
    #[inline(always)]
    pub fn tx_empty(&self) -> TxEmptyR {
        TxEmptyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXFIFO not full indicator"]
    #[inline(always)]
    pub fn tx_not_full(&self) -> TxNotFullR {
        TxNotFullR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Uart_rx_error, depending on the FIFO control register (FCR) bit\\[0\\] * 0: clear when the CPU reads the LSR, if there are no subsequent errors in the FIFO. * 1: parity error, overrun error, framing error or break indication in the FIFO."]
    #[inline(always)]
    pub fn rxfifo_err(&self) -> RxfifoErrR {
        RxfifoErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit is only set when Received Data Available Interrupt interrupt is enabled(ERBI is set) 1: FIFO Trigger level reached or Rx FIFO full"]
    #[inline(always)]
    pub fn rxfifo_int(&self) -> RxfifoIntR {
        RxfifoIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit is only set when Rx Timeout interrupt is enabled(ETOI is set) 1: at least one character in the FIFO but no character has been input to the FIFO or reading from it for the time duration, which depends on the value in register REG_RX_PATH_CTRL\\[31:16\\]"]
    #[inline(always)]
    pub fn timeout_int(&self) -> TimeoutIntR {
        TimeoutIntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit is only set when Rx Path Monitor Done interrupt is enabled(EMDI is set) 1: Rx path monitor done"]
    #[inline(always)]
    pub fn monitor_done_int(&self) -> MonitorDoneIntR {
        MonitorDoneIntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit is only set when Modem status pending 1: Modem interrupt"]
    #[inline(always)]
    pub fn modem_int(&self) -> ModemIntR {
        ModemIntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit is only set when RX debouncer has detected noise 1: Rx path noise detected interrupt"]
    #[inline(always)]
    pub fn rxnd_int(&self) -> RxndIntR {
        RxndIntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit is only set when rxfifo empty timeout interrupt is enabled(ERETI is set) 1:Rxfifo is empty for longer than TH_RETI"]
    #[inline(always)]
    pub fn re_timeout_int(&self) -> ReTimeoutIntR {
        ReTimeoutIntR::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "UART line status register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_lsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartLsrSpec;
impl crate::RegisterSpec for UartLsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_lsr::R`](R) reader structure"]
impl crate::Readable for UartLsrSpec {}
#[doc = "`reset()` method sets UART_LSR to value 0x60"]
impl crate::Resettable for UartLsrSpec {
    const RESET_VALUE: u32 = 0x60;
}
