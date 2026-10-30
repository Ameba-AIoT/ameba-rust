#[doc = "Register `UART_SIR_RX_PFC` reader"]
pub type R = crate::R<UartSirRxPfcSpec>;
#[doc = "Register `UART_SIR_RX_PFC` writer"]
pub type W = crate::W<UartSirRxPfcSpec>;
#[doc = "Field `R_SIR_RX_FILTER_EN` reader - IrDA SIR Rx filter enable"]
pub type RSirRxFilterEnR = crate::BitReader;
#[doc = "Field `R_SIR_RX_FILTER_EN` writer - IrDA SIR Rx filter enable"]
pub type RSirRxFilterEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R_SIR_RX_FILTER_THRS` reader - The threshold of SIR Rx filter, Rx pulse is valid only when Rx pulse width is larger than the threshold."]
pub type RSirRxFilterThrsR = crate::FieldReader<u16>;
#[doc = "Field `R_SIR_RX_FILTER_THRS` writer - The threshold of SIR Rx filter, Rx pulse is valid only when Rx pulse width is larger than the threshold."]
pub type RSirRxFilterThrsW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bit 0 - IrDA SIR Rx filter enable"]
    #[inline(always)]
    pub fn r_sir_rx_filter_en(&self) -> RSirRxFilterEnR {
        RSirRxFilterEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:15 - The threshold of SIR Rx filter, Rx pulse is valid only when Rx pulse width is larger than the threshold."]
    #[inline(always)]
    pub fn r_sir_rx_filter_thrs(&self) -> RSirRxFilterThrsR {
        RSirRxFilterThrsR::new(((self.bits >> 1) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - IrDA SIR Rx filter enable"]
    #[inline(always)]
    pub fn r_sir_rx_filter_en(&mut self) -> RSirRxFilterEnW<'_, UartSirRxPfcSpec> {
        RSirRxFilterEnW::new(self, 0)
    }
    #[doc = "Bits 1:15 - The threshold of SIR Rx filter, Rx pulse is valid only when Rx pulse width is larger than the threshold."]
    #[inline(always)]
    pub fn r_sir_rx_filter_thrs(&mut self) -> RSirRxFilterThrsW<'_, UartSirRxPfcSpec> {
        RSirRxFilterThrsW::new(self, 1)
    }
}
#[doc = "UART IRDA SIR Rx pulse filter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_sir_rx_pfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_sir_rx_pfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartSirRxPfcSpec;
impl crate::RegisterSpec for UartSirRxPfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_sir_rx_pfc::R`](R) reader structure"]
impl crate::Readable for UartSirRxPfcSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_sir_rx_pfc::W`](W) writer structure"]
impl crate::Writable for UartSirRxPfcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_SIR_RX_PFC to value 0"]
impl crate::Resettable for UartSirRxPfcSpec {}
