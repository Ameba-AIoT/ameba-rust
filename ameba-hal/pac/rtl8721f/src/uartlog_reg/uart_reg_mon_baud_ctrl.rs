#[doc = "Register `UART_REG_MON_BAUD_CTRL` reader"]
pub type R = crate::R<UartRegMonBaudCtrlSpec>;
#[doc = "Register `UART_REG_MON_BAUD_CTRL` writer"]
pub type W = crate::W<UartRegMonBaudCtrlSpec>;
#[doc = "Field `r_mon_baud_en` reader - Monitor Rx baud enable"]
pub type RMonBaudEnR = crate::BitReader;
#[doc = "Field `r_mon_baud_en` writer - Monitor Rx baud enable"]
pub type RMonBaudEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_bit_num_thres` reader - Indicate the bit number threshold of one monitor period, max value is 127."]
pub type RBitNumThresR = crate::FieldReader;
#[doc = "Field `r_bit_num_thres` writer - Indicate the bit number threshold of one monitor period, max value is 127."]
pub type RBitNumThresW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_cycnum_perbit_osc` reader - Indicate the average OSC clock cycle number of one bit, for Rx path OSC clock Note: Sortware sets the initial value; hardware updates it depending on the monitor result."]
pub type RCycnumPerbitOscR = crate::FieldReader<u32>;
#[doc = "Field `r_cycnum_perbit_osc` writer - Indicate the average OSC clock cycle number of one bit, for Rx path OSC clock Note: Sortware sets the initial value; hardware updates it depending on the monitor result."]
pub type RCycnumPerbitOscW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `r_upd_osc_in_xtal` reader - Update parameter r_cycnum_perbit_osc(during the update of parameter r_cycnum_perbit_xtal) enable"]
pub type RUpdOscInXtalR = crate::BitReader;
#[doc = "Field `r_upd_osc_in_xtal` writer - Update parameter r_cycnum_perbit_osc(during the update of parameter r_cycnum_perbit_xtal) enable"]
pub type RUpdOscInXtalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Monitor Rx baud enable"]
    #[inline(always)]
    pub fn r_mon_baud_en(&self) -> RMonBaudEnR {
        RMonBaudEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - Indicate the bit number threshold of one monitor period, max value is 127."]
    #[inline(always)]
    pub fn r_bit_num_thres(&self) -> RBitNumThresR {
        RBitNumThresR::new(((self.bits >> 1) & 0xff) as u8)
    }
    #[doc = "Bits 9:28 - Indicate the average OSC clock cycle number of one bit, for Rx path OSC clock Note: Sortware sets the initial value; hardware updates it depending on the monitor result."]
    #[inline(always)]
    pub fn r_cycnum_perbit_osc(&self) -> RCycnumPerbitOscR {
        RCycnumPerbitOscR::new((self.bits >> 9) & 0x000f_ffff)
    }
    #[doc = "Bit 29 - Update parameter r_cycnum_perbit_osc(during the update of parameter r_cycnum_perbit_xtal) enable"]
    #[inline(always)]
    pub fn r_upd_osc_in_xtal(&self) -> RUpdOscInXtalR {
        RUpdOscInXtalR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Monitor Rx baud enable"]
    #[inline(always)]
    pub fn r_mon_baud_en(&mut self) -> RMonBaudEnW<'_, UartRegMonBaudCtrlSpec> {
        RMonBaudEnW::new(self, 0)
    }
    #[doc = "Bits 1:8 - Indicate the bit number threshold of one monitor period, max value is 127."]
    #[inline(always)]
    pub fn r_bit_num_thres(&mut self) -> RBitNumThresW<'_, UartRegMonBaudCtrlSpec> {
        RBitNumThresW::new(self, 1)
    }
    #[doc = "Bits 9:28 - Indicate the average OSC clock cycle number of one bit, for Rx path OSC clock Note: Sortware sets the initial value; hardware updates it depending on the monitor result."]
    #[inline(always)]
    pub fn r_cycnum_perbit_osc(&mut self) -> RCycnumPerbitOscW<'_, UartRegMonBaudCtrlSpec> {
        RCycnumPerbitOscW::new(self, 9)
    }
    #[doc = "Bit 29 - Update parameter r_cycnum_perbit_osc(during the update of parameter r_cycnum_perbit_xtal) enable"]
    #[inline(always)]
    pub fn r_upd_osc_in_xtal(&mut self) -> RUpdOscInXtalW<'_, UartRegMonBaudCtrlSpec> {
        RUpdOscInXtalW::new(self, 29)
    }
}
#[doc = "UART monitor baud rate control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_reg_mon_baud_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_reg_mon_baud_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartRegMonBaudCtrlSpec;
impl crate::RegisterSpec for UartRegMonBaudCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_reg_mon_baud_ctrl::R`](R) reader structure"]
impl crate::Readable for UartRegMonBaudCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_reg_mon_baud_ctrl::W`](W) writer structure"]
impl crate::Writable for UartRegMonBaudCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_REG_MON_BAUD_CTRL to value 0"]
impl crate::Resettable for UartRegMonBaudCtrlSpec {}
