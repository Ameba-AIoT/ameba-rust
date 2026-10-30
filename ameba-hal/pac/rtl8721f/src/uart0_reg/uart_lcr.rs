#[doc = "Register `UART_LCR` reader"]
pub type R = crate::R<UartLcrSpec>;
#[doc = "Register `UART_LCR` writer"]
pub type W = crate::W<UartLcrSpec>;
#[doc = "Field `WLS0` reader - Word length selection * 0: 7 bits data * 1: 8 bits data"]
pub type Wls0R = crate::BitReader;
#[doc = "Field `WLS0` writer - Word length selection * 0: 7 bits data * 1: 8 bits data"]
pub type Wls0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STB` reader - Stop bits length selection * 0: 1 stop bit * 1: 2 stop bits Note: The receiver always checks the first stop bit only."]
pub type StbR = crate::BitReader;
#[doc = "Field `STB` writer - Stop bits length selection * 0: 1 stop bit * 1: 2 stop bits Note: The receiver always checks the first stop bit only."]
pub type StbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN` reader - Parity Enable * 0: No parity bit * 1: Parity bit is generated on each outgoing character and is checked on each incoming one"]
pub type PenR = crate::BitReader;
#[doc = "Field `PEN` writer - Parity Enable * 0: No parity bit * 1: Parity bit is generated on each outgoing character and is checked on each incoming one"]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPS` reader - Even Parity selection * 0: Odd parity, the number of Logic '1' is odd * 1: Even parity, the number of Logic '1' is even"]
pub type EpsR = crate::BitReader;
#[doc = "Field `EPS` writer - Even Parity selection * 0: Odd parity, the number of Logic '1' is odd * 1: Even parity, the number of Logic '1' is even"]
pub type EpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STKP` reader - Stick Parity bit * 0: Not stick parity, the parity type is ODD or EVEN (depending on eps) * 1: Stick parity **eps=1: low parity (1'b0) **eps=0: high parity (1'b1)"]
pub type StkpR = crate::BitReader;
#[doc = "Field `STKP` writer - Stick Parity bit * 0: Not stick parity, the parity type is ODD or EVEN (depending on eps) * 1: Stick parity **eps=1: low parity (1'b0) **eps=0: high parity (1'b1)"]
pub type StkpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRCTL` reader - Break Control bit * 0: break signal disable * 1: break signal enable (logic '0')"]
pub type BrctlR = crate::BitReader;
#[doc = "Field `BRCTL` writer - Break Control bit * 0: break signal disable * 1: break signal enable (logic '0')"]
pub type BrctlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLAB` reader - Divisor Latch Access bit, and control the accessibility of the DLL, DLM and IER registers * 0: The divisor latches not accessible; IER accessible * 1: The divisor latches accessible; DLL and DLM accessible Note: THR/RBR don't care the value of dlab bit."]
pub type DlabR = crate::BitReader;
#[doc = "Field `DLAB` writer - Divisor Latch Access bit, and control the accessibility of the DLL, DLM and IER registers * 0: The divisor latches not accessible; IER accessible * 1: The divisor latches accessible; DLL and DLM accessible Note: THR/RBR don't care the value of dlab bit."]
pub type DlabW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Word length selection * 0: 7 bits data * 1: 8 bits data"]
    #[inline(always)]
    pub fn wls0(&self) -> Wls0R {
        Wls0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Stop bits length selection * 0: 1 stop bit * 1: 2 stop bits Note: The receiver always checks the first stop bit only."]
    #[inline(always)]
    pub fn stb(&self) -> StbR {
        StbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Enable * 0: No parity bit * 1: Parity bit is generated on each outgoing character and is checked on each incoming one"]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Even Parity selection * 0: Odd parity, the number of Logic '1' is odd * 1: Even parity, the number of Logic '1' is even"]
    #[inline(always)]
    pub fn eps(&self) -> EpsR {
        EpsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stick Parity bit * 0: Not stick parity, the parity type is ODD or EVEN (depending on eps) * 1: Stick parity **eps=1: low parity (1'b0) **eps=0: high parity (1'b1)"]
    #[inline(always)]
    pub fn stkp(&self) -> StkpR {
        StkpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Break Control bit * 0: break signal disable * 1: break signal enable (logic '0')"]
    #[inline(always)]
    pub fn brctl(&self) -> BrctlR {
        BrctlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Divisor Latch Access bit, and control the accessibility of the DLL, DLM and IER registers * 0: The divisor latches not accessible; IER accessible * 1: The divisor latches accessible; DLL and DLM accessible Note: THR/RBR don't care the value of dlab bit."]
    #[inline(always)]
    pub fn dlab(&self) -> DlabR {
        DlabR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Word length selection * 0: 7 bits data * 1: 8 bits data"]
    #[inline(always)]
    pub fn wls0(&mut self) -> Wls0W<'_, UartLcrSpec> {
        Wls0W::new(self, 0)
    }
    #[doc = "Bit 2 - Stop bits length selection * 0: 1 stop bit * 1: 2 stop bits Note: The receiver always checks the first stop bit only."]
    #[inline(always)]
    pub fn stb(&mut self) -> StbW<'_, UartLcrSpec> {
        StbW::new(self, 2)
    }
    #[doc = "Bit 3 - Parity Enable * 0: No parity bit * 1: Parity bit is generated on each outgoing character and is checked on each incoming one"]
    #[inline(always)]
    pub fn pen(&mut self) -> PenW<'_, UartLcrSpec> {
        PenW::new(self, 3)
    }
    #[doc = "Bit 4 - Even Parity selection * 0: Odd parity, the number of Logic '1' is odd * 1: Even parity, the number of Logic '1' is even"]
    #[inline(always)]
    pub fn eps(&mut self) -> EpsW<'_, UartLcrSpec> {
        EpsW::new(self, 4)
    }
    #[doc = "Bit 5 - Stick Parity bit * 0: Not stick parity, the parity type is ODD or EVEN (depending on eps) * 1: Stick parity **eps=1: low parity (1'b0) **eps=0: high parity (1'b1)"]
    #[inline(always)]
    pub fn stkp(&mut self) -> StkpW<'_, UartLcrSpec> {
        StkpW::new(self, 5)
    }
    #[doc = "Bit 6 - Break Control bit * 0: break signal disable * 1: break signal enable (logic '0')"]
    #[inline(always)]
    pub fn brctl(&mut self) -> BrctlW<'_, UartLcrSpec> {
        BrctlW::new(self, 6)
    }
    #[doc = "Bit 7 - Divisor Latch Access bit, and control the accessibility of the DLL, DLM and IER registers * 0: The divisor latches not accessible; IER accessible * 1: The divisor latches accessible; DLL and DLM accessible Note: THR/RBR don't care the value of dlab bit."]
    #[inline(always)]
    pub fn dlab(&mut self) -> DlabW<'_, UartLcrSpec> {
        DlabW::new(self, 7)
    }
}
#[doc = "UART line control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_lcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_lcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartLcrSpec;
impl crate::RegisterSpec for UartLcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_lcr::R`](R) reader structure"]
impl crate::Readable for UartLcrSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_lcr::W`](W) writer structure"]
impl crate::Writable for UartLcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_LCR to value 0x01"]
impl crate::Resettable for UartLcrSpec {
    const RESET_VALUE: u32 = 0x01;
}
