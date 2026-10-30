#[doc = "Register `UART_REG_MON_BAUD_STS` reader"]
pub type R = crate::R<UartRegMonBaudStsSpec>;
#[doc = "Register `UART_REG_MON_BAUD_STS` writer"]
pub type W = crate::W<UartRegMonBaudStsSpec>;
#[doc = "Field `r_cycnum_perbit_xtal` reader - Indicate the fractional xtal clock cycle number of one bit, for Rx path xtal clk Note: Sortware sets the initial value; hardware updates it depending on the monitor result."]
pub type RCycnumPerbitXtalR = crate::FieldReader<u32>;
#[doc = "Field `r_cycnum_perbit_xtal` writer - Indicate the fractional xtal clock cycle number of one bit, for Rx path xtal clk Note: Sortware sets the initial value; hardware updates it depending on the monitor result."]
pub type RCycnumPerbitXtalW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `ro_mon_rdy` reader - Indicate the finish of calculation of actual cycle number per bit. It's cleared when r_mon_baud_en is 0."]
pub type RoMonRdyR = crate::BitReader;
#[doc = "Field `ro_mon_total_bit` reader - Indicate the number of bits actually monitored"]
pub type RoMonTotalBitR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:19 - Indicate the fractional xtal clock cycle number of one bit, for Rx path xtal clk Note: Sortware sets the initial value; hardware updates it depending on the monitor result."]
    #[inline(always)]
    pub fn r_cycnum_perbit_xtal(&self) -> RCycnumPerbitXtalR {
        RCycnumPerbitXtalR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 20 - Indicate the finish of calculation of actual cycle number per bit. It's cleared when r_mon_baud_en is 0."]
    #[inline(always)]
    pub fn ro_mon_rdy(&self) -> RoMonRdyR {
        RoMonRdyR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:28 - Indicate the number of bits actually monitored"]
    #[inline(always)]
    pub fn ro_mon_total_bit(&self) -> RoMonTotalBitR {
        RoMonTotalBitR::new(((self.bits >> 21) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:19 - Indicate the fractional xtal clock cycle number of one bit, for Rx path xtal clk Note: Sortware sets the initial value; hardware updates it depending on the monitor result."]
    #[inline(always)]
    pub fn r_cycnum_perbit_xtal(&mut self) -> RCycnumPerbitXtalW<'_, UartRegMonBaudStsSpec> {
        RCycnumPerbitXtalW::new(self, 0)
    }
}
#[doc = "UART monitor baud rate status register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_reg_mon_baud_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_reg_mon_baud_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartRegMonBaudStsSpec;
impl crate::RegisterSpec for UartRegMonBaudStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_reg_mon_baud_sts::R`](R) reader structure"]
impl crate::Readable for UartRegMonBaudStsSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_reg_mon_baud_sts::W`](W) writer structure"]
impl crate::Writable for UartRegMonBaudStsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_REG_MON_BAUD_STS to value 0"]
impl crate::Resettable for UartRegMonBaudStsSpec {}
