#[doc = "Register `EEE_CR4` reader"]
pub type R = crate::R<EeeCr4Spec>;
#[doc = "Register `EEE_CR4` writer"]
pub type W = crate::W<EeeCr4Spec>;
#[doc = "Field `EEE_TX_THR_100` reader - Set the TX threshold in EEE function. Unit is Byte. EEE_TX_THR=Round \\[(T_tx_rate*Rate) / 8 \\]. If you set T_tx_rate=10us, Rate=100Mbps, so set EEE_TX_THR = 0x7D (125bytes). Default value is 0x3E (~half rate)."]
pub type EeeTxThr100R = crate::FieldReader<u16>;
#[doc = "Field `EEE_TX_THR_100` writer - Set the TX threshold in EEE function. Unit is Byte. EEE_TX_THR=Round \\[(T_tx_rate*Rate) / 8 \\]. If you set T_tx_rate=10us, Rate=100Mbps, so set EEE_TX_THR = 0x7D (125bytes). Default value is 0x3E (~half rate)."]
pub type EeeTxThr100W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EEE_TX_THR_Giga` reader - Set the TX threshold in EEE function. Unit is Byte. EEE_TX_THR=Round \\[(T_tx_rate*Rate) / 8 \\]. If you set T_tx_rate=10us, Rate=1000Mbps, so set EEE_TX_THR = 0x4E2 (1250bytes). Default value is 0x271(~half rate)."]
pub type EeeTxThrGigaR = crate::FieldReader<u16>;
#[doc = "Field `EEE_TX_THR_Giga` writer - Set the TX threshold in EEE function. Unit is Byte. EEE_TX_THR=Round \\[(T_tx_rate*Rate) / 8 \\]. If you set T_tx_rate=10us, Rate=1000Mbps, so set EEE_TX_THR = 0x4E2 (1250bytes). Default value is 0x271(~half rate)."]
pub type EeeTxThrGigaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Set the TX threshold in EEE function. Unit is Byte. EEE_TX_THR=Round \\[(T_tx_rate*Rate) / 8 \\]. If you set T_tx_rate=10us, Rate=100Mbps, so set EEE_TX_THR = 0x7D (125bytes). Default value is 0x3E (~half rate)."]
    #[inline(always)]
    pub fn eee_tx_thr_100(&self) -> EeeTxThr100R {
        EeeTxThr100R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Set the TX threshold in EEE function. Unit is Byte. EEE_TX_THR=Round \\[(T_tx_rate*Rate) / 8 \\]. If you set T_tx_rate=10us, Rate=1000Mbps, so set EEE_TX_THR = 0x4E2 (1250bytes). Default value is 0x271(~half rate)."]
    #[inline(always)]
    pub fn eee_tx_thr_giga(&self) -> EeeTxThrGigaR {
        EeeTxThrGigaR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Set the TX threshold in EEE function. Unit is Byte. EEE_TX_THR=Round \\[(T_tx_rate*Rate) / 8 \\]. If you set T_tx_rate=10us, Rate=100Mbps, so set EEE_TX_THR = 0x7D (125bytes). Default value is 0x3E (~half rate)."]
    #[inline(always)]
    pub fn eee_tx_thr_100(&mut self) -> EeeTxThr100W<'_, EeeCr4Spec> {
        EeeTxThr100W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Set the TX threshold in EEE function. Unit is Byte. EEE_TX_THR=Round \\[(T_tx_rate*Rate) / 8 \\]. If you set T_tx_rate=10us, Rate=1000Mbps, so set EEE_TX_THR = 0x4E2 (1250bytes). Default value is 0x271(~half rate)."]
    #[inline(always)]
    pub fn eee_tx_thr_giga(&mut self) -> EeeTxThrGigaW<'_, EeeCr4Spec> {
        EeeTxThrGigaW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`eee_cr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eee_cr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EeeCr4Spec;
impl crate::RegisterSpec for EeeCr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eee_cr4::R`](R) reader structure"]
impl crate::Readable for EeeCr4Spec {}
#[doc = "`write(|w| ..)` method takes [`eee_cr4::W`](W) writer structure"]
impl crate::Writable for EeeCr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EEE_CR4 to value 0x0271_003e"]
impl crate::Resettable for EeeCr4Spec {
    const RESET_VALUE: u32 = 0x0271_003e;
}
