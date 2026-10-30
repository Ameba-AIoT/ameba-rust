#[doc = "Register `UART_RP_LCR` reader"]
pub type R = crate::R<UartRpLcrSpec>;
#[doc = "Register `UART_RP_LCR` writer"]
pub type W = crate::W<UartRpLcrSpec>;
#[doc = "Field `rp_wls0` reader - Word length selection * 0: 7 bits data * 1: 8 bits data"]
pub type RpWls0R = crate::BitReader;
#[doc = "Field `rp_wls0` writer - Word length selection * 0: 7 bits data * 1: 8 bits data"]
pub type RpWls0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rp_stb` reader - Stop bits length selection * 0: 1 stop bit * 1: 2 stop bits Note: the receiver always checks the first stop bit only."]
pub type RpStbR = crate::BitReader;
#[doc = "Field `rp_stb` writer - Stop bits length selection * 0: 1 stop bit * 1: 2 stop bits Note: the receiver always checks the first stop bit only."]
pub type RpStbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rp_pen` reader - Parity Enable * 0: No parity bit * 1: Parity bit is generated on each outgoing character and is checked on each incoming one."]
pub type RpPenR = crate::BitReader;
#[doc = "Field `rp_pen` writer - Parity Enable * 0: No parity bit * 1: Parity bit is generated on each outgoing character and is checked on each incoming one."]
pub type RpPenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rp_eps` reader - Even Parity select * 0: Odd parity, the number of Logic \"1\" is odd. * 1: Even parity, the number of Logic \"1\" is even."]
pub type RpEpsR = crate::BitReader;
#[doc = "Field `rp_eps` writer - Even Parity select * 0: Odd parity, the number of Logic \"1\" is odd. * 1: Even parity, the number of Logic \"1\" is even."]
pub type RpEpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rp_stkp` reader - Stick Parity bit * 0: Not stick parity, the parity type is ODD or EVEN (depend on eps) * 1: Stick parity, eps=1: low parity (1'b0); eps=0: high parity (1'b1)"]
pub type RpStkpR = crate::BitReader;
#[doc = "Field `rp_stkp` writer - Stick Parity bit * 0: Not stick parity, the parity type is ODD or EVEN (depend on eps) * 1: Stick parity, eps=1: low parity (1'b0); eps=0: high parity (1'b1)"]
pub type RpStkpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rp_dlab` reader - Divisor Latch Access bit, and control the accessibility of the DLL, DLM and IER registers * 0: The divisor latches not accessible; IER accessible * 1: The divisor latches accessible; DLL and DLM accessible Note: THR/RBR don not care the value of dlab bit."]
pub type RpDlabR = crate::BitReader;
#[doc = "Field `rp_dlab` writer - Divisor Latch Access bit, and control the accessibility of the DLL, DLM and IER registers * 0: The divisor latches not accessible; IER accessible * 1: The divisor latches accessible; DLL and DLM accessible Note: THR/RBR don not care the value of dlab bit."]
pub type RpDlabW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rp_rxbaud_19_0` reader - Factor of Baud rate calculation for Relay path"]
pub type RpRxbaud19_0R = crate::FieldReader<u32>;
#[doc = "Field `rp_rxbaud_19_0` writer - Factor of Baud rate calculation for Relay path"]
pub type RpRxbaud19_0W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bit 0 - Word length selection * 0: 7 bits data * 1: 8 bits data"]
    #[inline(always)]
    pub fn rp_wls0(&self) -> RpWls0R {
        RpWls0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Stop bits length selection * 0: 1 stop bit * 1: 2 stop bits Note: the receiver always checks the first stop bit only."]
    #[inline(always)]
    pub fn rp_stb(&self) -> RpStbR {
        RpStbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Enable * 0: No parity bit * 1: Parity bit is generated on each outgoing character and is checked on each incoming one."]
    #[inline(always)]
    pub fn rp_pen(&self) -> RpPenR {
        RpPenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Even Parity select * 0: Odd parity, the number of Logic \"1\" is odd. * 1: Even parity, the number of Logic \"1\" is even."]
    #[inline(always)]
    pub fn rp_eps(&self) -> RpEpsR {
        RpEpsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stick Parity bit * 0: Not stick parity, the parity type is ODD or EVEN (depend on eps) * 1: Stick parity, eps=1: low parity (1'b0); eps=0: high parity (1'b1)"]
    #[inline(always)]
    pub fn rp_stkp(&self) -> RpStkpR {
        RpStkpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Divisor Latch Access bit, and control the accessibility of the DLL, DLM and IER registers * 0: The divisor latches not accessible; IER accessible * 1: The divisor latches accessible; DLL and DLM accessible Note: THR/RBR don not care the value of dlab bit."]
    #[inline(always)]
    pub fn rp_dlab(&self) -> RpDlabR {
        RpDlabR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 12:31 - Factor of Baud rate calculation for Relay path"]
    #[inline(always)]
    pub fn rp_rxbaud_19_0(&self) -> RpRxbaud19_0R {
        RpRxbaud19_0R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Word length selection * 0: 7 bits data * 1: 8 bits data"]
    #[inline(always)]
    pub fn rp_wls0(&mut self) -> RpWls0W<'_, UartRpLcrSpec> {
        RpWls0W::new(self, 0)
    }
    #[doc = "Bit 2 - Stop bits length selection * 0: 1 stop bit * 1: 2 stop bits Note: the receiver always checks the first stop bit only."]
    #[inline(always)]
    pub fn rp_stb(&mut self) -> RpStbW<'_, UartRpLcrSpec> {
        RpStbW::new(self, 2)
    }
    #[doc = "Bit 3 - Parity Enable * 0: No parity bit * 1: Parity bit is generated on each outgoing character and is checked on each incoming one."]
    #[inline(always)]
    pub fn rp_pen(&mut self) -> RpPenW<'_, UartRpLcrSpec> {
        RpPenW::new(self, 3)
    }
    #[doc = "Bit 4 - Even Parity select * 0: Odd parity, the number of Logic \"1\" is odd. * 1: Even parity, the number of Logic \"1\" is even."]
    #[inline(always)]
    pub fn rp_eps(&mut self) -> RpEpsW<'_, UartRpLcrSpec> {
        RpEpsW::new(self, 4)
    }
    #[doc = "Bit 5 - Stick Parity bit * 0: Not stick parity, the parity type is ODD or EVEN (depend on eps) * 1: Stick parity, eps=1: low parity (1'b0); eps=0: high parity (1'b1)"]
    #[inline(always)]
    pub fn rp_stkp(&mut self) -> RpStkpW<'_, UartRpLcrSpec> {
        RpStkpW::new(self, 5)
    }
    #[doc = "Bit 7 - Divisor Latch Access bit, and control the accessibility of the DLL, DLM and IER registers * 0: The divisor latches not accessible; IER accessible * 1: The divisor latches accessible; DLL and DLM accessible Note: THR/RBR don not care the value of dlab bit."]
    #[inline(always)]
    pub fn rp_dlab(&mut self) -> RpDlabW<'_, UartRpLcrSpec> {
        RpDlabW::new(self, 7)
    }
    #[doc = "Bits 12:31 - Factor of Baud rate calculation for Relay path"]
    #[inline(always)]
    pub fn rp_rxbaud_19_0(&mut self) -> RpRxbaud19_0W<'_, UartRpLcrSpec> {
        RpRxbaud19_0W::new(self, 12)
    }
}
#[doc = "UART Relay Rx Path line control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_rp_lcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_rp_lcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartRpLcrSpec;
impl crate::RegisterSpec for UartRpLcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_rp_lcr::R`](R) reader structure"]
impl crate::Readable for UartRpLcrSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_rp_lcr::W`](W) writer structure"]
impl crate::Writable for UartRpLcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_RP_LCR to value 0x01"]
impl crate::Resettable for UartRpLcrSpec {
    const RESET_VALUE: u32 = 0x01;
}
