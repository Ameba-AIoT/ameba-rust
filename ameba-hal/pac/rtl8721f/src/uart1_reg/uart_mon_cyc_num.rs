#[doc = "Register `UART_MON_CYC_NUM` reader"]
pub type R = crate::R<UartMonCycNumSpec>;
#[doc = "Field `RO_MON_TOTAL_CYCLE` reader - Indicate the number of clock cycle actually monitored"]
pub type RoMonTotalCycleR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:27 - Indicate the number of clock cycle actually monitored"]
    #[inline(always)]
    pub fn ro_mon_total_cycle(&self) -> RoMonTotalCycleR {
        RoMonTotalCycleR::new(self.bits & 0x0fff_ffff)
    }
}
#[doc = "UART monitored cycle number register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_mon_cyc_num::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartMonCycNumSpec;
impl crate::RegisterSpec for UartMonCycNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_mon_cyc_num::R`](R) reader structure"]
impl crate::Readable for UartMonCycNumSpec {}
#[doc = "`reset()` method sets UART_MON_CYC_NUM to value 0"]
impl crate::Resettable for UartMonCycNumSpec {}
