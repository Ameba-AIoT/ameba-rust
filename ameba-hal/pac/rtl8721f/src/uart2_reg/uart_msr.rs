#[doc = "Register `UART_MSR` reader"]
pub type R = crate::R<UartMsrSpec>;
#[doc = "Field `D_CTS` reader - * 0: Otherwise * 1: The CTS line has changed its state"]
pub type DCtsR = crate::BitReader;
#[doc = "Field `D_DSR` reader - * 0: Otherwise * 1: The DSR line has changed its state"]
pub type DDsrR = crate::BitReader;
#[doc = "Field `TERI` reader - * 0: Otherwise * 1: The RI line has changed its state from low to high state"]
pub type TeriR = crate::BitReader;
#[doc = "Field `D_DCD` reader - * 0: Otherwise * 1: The DCD line has changed its state"]
pub type DDcdR = crate::BitReader;
#[doc = "Field `R_CTS` reader - Complement of the CTS input or equals to RTS in loopback mode."]
pub type RCtsR = crate::BitReader;
#[doc = "Field `R_DSR` reader - Complement of the DSR input or equals to DTR in loopback mode."]
pub type RDsrR = crate::BitReader;
#[doc = "Field `R_RI` reader - Complement of the RI input or equals to Out1 in loopback mode."]
pub type RRiR = crate::BitReader;
#[doc = "Field `R_DCD` reader - Complement of the DCD input or equals to Out2 in loopback mode."]
pub type RDcdR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - * 0: Otherwise * 1: The CTS line has changed its state"]
    #[inline(always)]
    pub fn d_cts(&self) -> DCtsR {
        DCtsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - * 0: Otherwise * 1: The DSR line has changed its state"]
    #[inline(always)]
    pub fn d_dsr(&self) -> DDsrR {
        DDsrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - * 0: Otherwise * 1: The RI line has changed its state from low to high state"]
    #[inline(always)]
    pub fn teri(&self) -> TeriR {
        TeriR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - * 0: Otherwise * 1: The DCD line has changed its state"]
    #[inline(always)]
    pub fn d_dcd(&self) -> DDcdR {
        DDcdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Complement of the CTS input or equals to RTS in loopback mode."]
    #[inline(always)]
    pub fn r_cts(&self) -> RCtsR {
        RCtsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Complement of the DSR input or equals to DTR in loopback mode."]
    #[inline(always)]
    pub fn r_dsr(&self) -> RDsrR {
        RDsrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Complement of the RI input or equals to Out1 in loopback mode."]
    #[inline(always)]
    pub fn r_ri(&self) -> RRiR {
        RRiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Complement of the DCD input or equals to Out2 in loopback mode."]
    #[inline(always)]
    pub fn r_dcd(&self) -> RDcdR {
        RDcdR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "UART modem status register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_msr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartMsrSpec;
impl crate::RegisterSpec for UartMsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_msr::R`](R) reader structure"]
impl crate::Readable for UartMsrSpec {}
#[doc = "`reset()` method sets UART_MSR to value 0"]
impl crate::Resettable for UartMsrSpec {}
