#[doc = "Register `REG_TM_TIMER_OFFSET` reader"]
pub type R = crate::R<RegTmTimerOffsetSpec>;
#[doc = "Register `REG_TM_TIMER_OFFSET` writer"]
pub type W = crate::W<RegTmTimerOffsetSpec>;
#[doc = "Field `BIT_TM_RX_PATH_TIME_OFFSET` reader - Compensate the time, from receive antenna connector to DATAON rising edge. unit: 1us"]
pub type BitTmRxPathTimeOffsetR = crate::FieldReader<u16>;
#[doc = "Field `BIT_TM_RX_PATH_TIME_OFFSET` writer - Compensate the time, from receive antenna connector to DATAON rising edge. unit: 1us"]
pub type BitTmRxPathTimeOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_TM_TX_PATH_TIME_OFFSET` reader - Compensate the time, from PHYTXON rising edge to transmit antenna connector. unit: 1us"]
pub type BitTmTxPathTimeOffsetR = crate::FieldReader<u16>;
#[doc = "Field `BIT_TM_TX_PATH_TIME_OFFSET` writer - Compensate the time, from PHYTXON rising edge to transmit antenna connector. unit: 1us"]
pub type BitTmTxPathTimeOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Compensate the time, from receive antenna connector to DATAON rising edge. unit: 1us"]
    #[inline(always)]
    pub fn bit_tm_rx_path_time_offset(&self) -> BitTmRxPathTimeOffsetR {
        BitTmRxPathTimeOffsetR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Compensate the time, from PHYTXON rising edge to transmit antenna connector. unit: 1us"]
    #[inline(always)]
    pub fn bit_tm_tx_path_time_offset(&self) -> BitTmTxPathTimeOffsetR {
        BitTmTxPathTimeOffsetR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compensate the time, from receive antenna connector to DATAON rising edge. unit: 1us"]
    #[inline(always)]
    pub fn bit_tm_rx_path_time_offset(
        &mut self,
    ) -> BitTmRxPathTimeOffsetW<'_, RegTmTimerOffsetSpec> {
        BitTmRxPathTimeOffsetW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Compensate the time, from PHYTXON rising edge to transmit antenna connector. unit: 1us"]
    #[inline(always)]
    pub fn bit_tm_tx_path_time_offset(
        &mut self,
    ) -> BitTmTxPathTimeOffsetW<'_, RegTmTimerOffsetSpec> {
        BitTmTxPathTimeOffsetW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tm_timer_offset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tm_timer_offset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTmTimerOffsetSpec;
impl crate::RegisterSpec for RegTmTimerOffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tm_timer_offset::R`](R) reader structure"]
impl crate::Readable for RegTmTimerOffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tm_timer_offset::W`](W) writer structure"]
impl crate::Writable for RegTmTimerOffsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TM_TIMER_OFFSET to value 0"]
impl crate::Resettable for RegTmTimerOffsetSpec {}
