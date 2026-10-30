#[doc = "Register `UART_LSR` reader"]
pub type R = crate::R<UartLsrSpec>;
#[doc = "Field `drdy` reader - Data Ready (DR) indicator * 0: No characters in the Receiver FIFO * 1: At least one character has been received and transferred into the FIFO"]
pub type DrdyR = crate::BitReader;
#[doc = "Field `ovr_err` reader - Overrun Error (OE) indicator * 0: No Overrun state * 1: The transmission is continue although the Rx FIFO is full"]
pub type OvrErrR = crate::BitReader;
#[doc = "Field `par_err` reader - Parity Error (PE) indicator * 0: No parity error in current character * 1: The received character does not have the correct parity bit"]
pub type ParErrR = crate::BitReader;
#[doc = "Field `frm_err` reader - Framing Error (FE) indicator * 0: No framing error in the current character * 1: The received character does not have a valid stop bit"]
pub type FrmErrR = crate::BitReader;
#[doc = "Field `break_int` reader - Break Interrupt (BI) indicator * 0: No break condition in the current character * 1: The received data input held in the Spacing ( logic 0) state is longer than a full word transmission time"]
pub type BreakIntR = crate::BitReader;
#[doc = "Field `monitor_done_int` reader - This bit is only set when Rx Path Monitor Done interrupt is enabled(EMDI is set) 1: Rx path monitor done"]
pub type MonitorDoneIntR = crate::BitReader;
#[doc = "Field `tx_empty` reader - TX fifo empty indicator"]
pub type TxEmptyR = crate::BitReader;
#[doc = "Field `rxfifo_err` reader - Uart_rx_error, only valid when the FCR bit \\[0\\] set to 1 * 0: Clear when the CPU reads the LSR, if there are no subsequent errors in the FIFO * 1: Parity error, overrun error,framing error or break indication in the FIFO"]
pub type RxfifoErrR = crate::BitReader;
#[doc = "Field `rpf_drdy` reader - Relay Path FIFO Data Ready (DR) indicator * 0: No characters in the Relay Receiver FIFO * 1: At least one character has been received and transferred into the Relay path FIFO"]
pub type RpfDrdyR = crate::BitReader;
#[doc = "Field `rpf_not_full` reader - Relay Path FIFO Not FULL (NF) indicator * 0: Relay path FIFO is full * 1: Relay path FIFO is not full"]
pub type RpfNotFullR = crate::BitReader;
#[doc = "Field `rpf_ovr_err` reader - Relay Path FIFO Overrun Error (OE) indicator * 0: No Overrun state * 1: The transmission is continue although the Relay FIFO is full"]
pub type RpfOvrErrR = crate::BitReader;
#[doc = "Field `rp_par_err` reader - Relay Path Parity Error (PE) indicator * 0: No parity error in current character * 1: The received character does not have the correct parity bit"]
pub type RpParErrR = crate::BitReader;
#[doc = "Field `rp_frm_err` reader - Relay Path Framing Error (FE) indicator * 0: No framing error in the current character * 1: The received character does not have a valid stop bit"]
pub type RpFrmErrR = crate::BitReader;
#[doc = "Field `RXFIFO_INT` reader - This bit is only set when Received Data Available Interrupt interrupt is enabled(ERBI is set) 1: FIFO Trigger level reached or Rx FIFO full"]
pub type RxfifoIntR = crate::BitReader;
#[doc = "Field `TIMEOUT_INT` reader - This bit is only set when Rx Timeout interrupt is enabled(ETOI is set) 1: at least one character in the FIFO but no character has been input to the FIFO or reading from it for the time duration, which depends on the value in register REG_RX_PATH_CTRL\\[31:16\\]"]
pub type TimeoutIntR = crate::BitReader;
#[doc = "Field `rp_rxfifo_err` reader - Relay Path rx_error,only valid when the FCR\\[12\\] set to 1 * 0: Clear when the CPU reads the LSR, if there are no subsequent errors in the relay path FIFO * 1: Parity error, framing error or break indication in the relay path FIFO"]
pub type RpRxfifoErrR = crate::BitReader;
#[doc = "Field `tp1f_empty` reader - Tx Path1 FIFO empty indicator"]
pub type Tp1fEmptyR = crate::BitReader;
#[doc = "Field `tp2f_empty` reader - Tx Path2 FIFO empty indicator"]
pub type Tp2fEmptyR = crate::BitReader;
#[doc = "Field `tp3f_empty` reader - Tx Path3 FIFO empty indicator"]
pub type Tp3fEmptyR = crate::BitReader;
#[doc = "Field `tp4f_empty` reader - Tx Path4 FIFO empty indicator"]
pub type Tp4fEmptyR = crate::BitReader;
#[doc = "Field `tp1f_not_full` reader - Tx Path1 FIFO not full indicator"]
pub type Tp1fNotFullR = crate::BitReader;
#[doc = "Field `tp2f_not_full` reader - Tx Path2 FIFO not full indicator"]
pub type Tp2fNotFullR = crate::BitReader;
#[doc = "Field `tp3f_not_full` reader - Tx Path3 FIFO not full indicator"]
pub type Tp3fNotFullR = crate::BitReader;
#[doc = "Field `tp4f_not_full` reader - Tx Path4 FIFO not full indicator"]
pub type Tp4fNotFullR = crate::BitReader;
#[doc = "Field `rx_fifo_ptr` reader - Data bytes count in the receive FIFO: the value of rx_fifo_ptr\\[7:0\\] indicates the data bytes number received in the FIFO."]
pub type RxFifoPtrR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Data Ready (DR) indicator * 0: No characters in the Receiver FIFO * 1: At least one character has been received and transferred into the FIFO"]
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
    #[doc = "Bit 5 - This bit is only set when Rx Path Monitor Done interrupt is enabled(EMDI is set) 1: Rx path monitor done"]
    #[inline(always)]
    pub fn monitor_done_int(&self) -> MonitorDoneIntR {
        MonitorDoneIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX fifo empty indicator"]
    #[inline(always)]
    pub fn tx_empty(&self) -> TxEmptyR {
        TxEmptyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Uart_rx_error, only valid when the FCR bit \\[0\\] set to 1 * 0: Clear when the CPU reads the LSR, if there are no subsequent errors in the FIFO * 1: Parity error, overrun error,framing error or break indication in the FIFO"]
    #[inline(always)]
    pub fn rxfifo_err(&self) -> RxfifoErrR {
        RxfifoErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Relay Path FIFO Data Ready (DR) indicator * 0: No characters in the Relay Receiver FIFO * 1: At least one character has been received and transferred into the Relay path FIFO"]
    #[inline(always)]
    pub fn rpf_drdy(&self) -> RpfDrdyR {
        RpfDrdyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Relay Path FIFO Not FULL (NF) indicator * 0: Relay path FIFO is full * 1: Relay path FIFO is not full"]
    #[inline(always)]
    pub fn rpf_not_full(&self) -> RpfNotFullR {
        RpfNotFullR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Relay Path FIFO Overrun Error (OE) indicator * 0: No Overrun state * 1: The transmission is continue although the Relay FIFO is full"]
    #[inline(always)]
    pub fn rpf_ovr_err(&self) -> RpfOvrErrR {
        RpfOvrErrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Relay Path Parity Error (PE) indicator * 0: No parity error in current character * 1: The received character does not have the correct parity bit"]
    #[inline(always)]
    pub fn rp_par_err(&self) -> RpParErrR {
        RpParErrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Relay Path Framing Error (FE) indicator * 0: No framing error in the current character * 1: The received character does not have a valid stop bit"]
    #[inline(always)]
    pub fn rp_frm_err(&self) -> RpFrmErrR {
        RpFrmErrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit is only set when Received Data Available Interrupt interrupt is enabled(ERBI is set) 1: FIFO Trigger level reached or Rx FIFO full"]
    #[inline(always)]
    pub fn rxfifo_int(&self) -> RxfifoIntR {
        RxfifoIntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This bit is only set when Rx Timeout interrupt is enabled(ETOI is set) 1: at least one character in the FIFO but no character has been input to the FIFO or reading from it for the time duration, which depends on the value in register REG_RX_PATH_CTRL\\[31:16\\]"]
    #[inline(always)]
    pub fn timeout_int(&self) -> TimeoutIntR {
        TimeoutIntR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Relay Path rx_error,only valid when the FCR\\[12\\] set to 1 * 0: Clear when the CPU reads the LSR, if there are no subsequent errors in the relay path FIFO * 1: Parity error, framing error or break indication in the relay path FIFO"]
    #[inline(always)]
    pub fn rp_rxfifo_err(&self) -> RpRxfifoErrR {
        RpRxfifoErrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Tx Path1 FIFO empty indicator"]
    #[inline(always)]
    pub fn tp1f_empty(&self) -> Tp1fEmptyR {
        Tp1fEmptyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Tx Path2 FIFO empty indicator"]
    #[inline(always)]
    pub fn tp2f_empty(&self) -> Tp2fEmptyR {
        Tp2fEmptyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Tx Path3 FIFO empty indicator"]
    #[inline(always)]
    pub fn tp3f_empty(&self) -> Tp3fEmptyR {
        Tp3fEmptyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Tx Path4 FIFO empty indicator"]
    #[inline(always)]
    pub fn tp4f_empty(&self) -> Tp4fEmptyR {
        Tp4fEmptyR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Tx Path1 FIFO not full indicator"]
    #[inline(always)]
    pub fn tp1f_not_full(&self) -> Tp1fNotFullR {
        Tp1fNotFullR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Tx Path2 FIFO not full indicator"]
    #[inline(always)]
    pub fn tp2f_not_full(&self) -> Tp2fNotFullR {
        Tp2fNotFullR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Tx Path3 FIFO not full indicator"]
    #[inline(always)]
    pub fn tp3f_not_full(&self) -> Tp3fNotFullR {
        Tp3fNotFullR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Tx Path4 FIFO not full indicator"]
    #[inline(always)]
    pub fn tp4f_not_full(&self) -> Tp4fNotFullR {
        Tp4fNotFullR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Data bytes count in the receive FIFO: the value of rx_fifo_ptr\\[7:0\\] indicates the data bytes number received in the FIFO."]
    #[inline(always)]
    pub fn rx_fifo_ptr(&self) -> RxFifoPtrR {
        RxFifoPtrR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "UART line status register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_lsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartLsrSpec;
impl crate::RegisterSpec for UartLsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_lsr::R`](R) reader structure"]
impl crate::Readable for UartLsrSpec {}
#[doc = "`reset()` method sets UART_LSR to value 0x0200"]
impl crate::Resettable for UartLsrSpec {
    const RESET_VALUE: u32 = 0x0200;
}
