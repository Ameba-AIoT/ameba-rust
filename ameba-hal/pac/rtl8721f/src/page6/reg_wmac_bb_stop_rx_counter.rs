#[doc = "Register `REG_WMAC_BB_STOP_RX_COUNTER` reader"]
pub type R = crate::R<RegWmacBbStopRxCounterSpec>;
#[doc = "Field `BIT_vbon_early_falling_counter` reader - Increase when VBON fall & no rx plcp error & rx_parser is waiting for rdrdy which is expected but not received . 0x120\\[29\\] / 0x124\\[29\\] : BB_STOP_RX mask/interrupt 0x0B8\\[0\\] / 0x0BC\\[0\\] : BB_STOP_RX mask/interrupt Interrupts will be trigger when mac detect any counter of 0x1690 increase."]
pub type BitVbonEarlyFallingCounterR = crate::FieldReader;
#[doc = "Field `BIT_abort_rx_rdrdy_counter` reader - Increase when mac_abort_rx_rdrdy trigger 0x120\\[29\\] / 0x124\\[29\\] : BB_STOP_RX mask/interrupt 0x0B8\\[0\\] / 0x0BC\\[0\\] : BB_STOP_RX mask/interrupt Interrupts will be trigger when mac detect any counter of 0x1690 increase."]
pub type BitAbortRxRdrdyCounterR = crate::FieldReader;
#[doc = "Field `BIT_abort_rx_vbon_counter` reader - Increase when mac_abort_rx_vbon trigger. 0x120\\[29\\] / 0x124\\[29\\] : BB_STOP_RX mask/interrupt 0x0B8\\[0\\] / 0x0BC\\[0\\] : BB_STOP_RX mask/interrupt Interrupts will be trigger when mac detect any counter of 0x1690 increase."]
pub type BitAbortRxVbonCounterR = crate::FieldReader;
#[doc = "Field `BIT_rst_all_counter` reader - Write one to reset all counters to zero."]
pub type BitRstAllCounterR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Increase when VBON fall & no rx plcp error & rx_parser is waiting for rdrdy which is expected but not received . 0x120\\[29\\] / 0x124\\[29\\] : BB_STOP_RX mask/interrupt 0x0B8\\[0\\] / 0x0BC\\[0\\] : BB_STOP_RX mask/interrupt Interrupts will be trigger when mac detect any counter of 0x1690 increase."]
    #[inline(always)]
    pub fn bit_vbon_early_falling_counter(&self) -> BitVbonEarlyFallingCounterR {
        BitVbonEarlyFallingCounterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Increase when mac_abort_rx_rdrdy trigger 0x120\\[29\\] / 0x124\\[29\\] : BB_STOP_RX mask/interrupt 0x0B8\\[0\\] / 0x0BC\\[0\\] : BB_STOP_RX mask/interrupt Interrupts will be trigger when mac detect any counter of 0x1690 increase."]
    #[inline(always)]
    pub fn bit_abort_rx_rdrdy_counter(&self) -> BitAbortRxRdrdyCounterR {
        BitAbortRxRdrdyCounterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Increase when mac_abort_rx_vbon trigger. 0x120\\[29\\] / 0x124\\[29\\] : BB_STOP_RX mask/interrupt 0x0B8\\[0\\] / 0x0BC\\[0\\] : BB_STOP_RX mask/interrupt Interrupts will be trigger when mac detect any counter of 0x1690 increase."]
    #[inline(always)]
    pub fn bit_abort_rx_vbon_counter(&self) -> BitAbortRxVbonCounterR {
        BitAbortRxVbonCounterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Write one to reset all counters to zero."]
    #[inline(always)]
    pub fn bit_rst_all_counter(&self) -> BitRstAllCounterR {
        BitRstAllCounterR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wmac_bb_stop_rx_counter::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWmacBbStopRxCounterSpec;
impl crate::RegisterSpec for RegWmacBbStopRxCounterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wmac_bb_stop_rx_counter::R`](R) reader structure"]
impl crate::Readable for RegWmacBbStopRxCounterSpec {}
#[doc = "`reset()` method sets REG_WMAC_BB_STOP_RX_COUNTER to value 0"]
impl crate::Resettable for RegWmacBbStopRxCounterSpec {}
