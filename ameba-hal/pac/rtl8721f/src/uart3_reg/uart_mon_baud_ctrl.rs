#[doc = "Register `UART_MON_BAUD_CTRL` reader"]
pub type R = crate::R<UartMonBaudCtrlSpec>;
#[doc = "Register `UART_MON_BAUD_CTRL` writer"]
pub type W = crate::W<UartMonBaudCtrlSpec>;
#[doc = "Field `R_MON_BAUD_EN` reader - Monitor Rx baud enable"]
pub type RMonBaudEnR = crate::BitReader;
#[doc = "Field `R_MON_BAUD_EN` writer - Monitor Rx baud enable"]
pub type RMonBaudEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R_BIT_NUM_THRES` reader - Indicate the bit number threshold of one monitor period, max value is 127"]
pub type RBitNumThresR = crate::FieldReader;
#[doc = "Field `R_BIT_NUM_THRES` writer - Indicate the bit number threshold of one monitor period, max value is 127"]
pub type RBitNumThresW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R_CYCNUM_PERBIT_OSC` reader - Indicate the average OSC clock cycle number of one bit, for Rx path OSC clk NOTE: SW sets the initial value, HW updates it depending on the monitor result."]
pub type RCycnumPerbitOscR = crate::FieldReader<u32>;
#[doc = "Field `R_CYCNUM_PERBIT_OSC` writer - Indicate the average OSC clock cycle number of one bit, for Rx path OSC clk NOTE: SW sets the initial value, HW updates it depending on the monitor result."]
pub type RCycnumPerbitOscW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `R_UPD_OSC_IN_XTAL` reader - Update parameter r_cycnum_perbit_osc (during the update of parameter r_cycnum_perbit_xtal) enable"]
pub type RUpdOscInXtalR = crate::BitReader;
#[doc = "Field `R_UPD_OSC_IN_XTAL` writer - Update parameter r_cycnum_perbit_osc (during the update of parameter r_cycnum_perbit_xtal) enable"]
pub type RUpdOscInXtalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Monitor Rx baud enable"]
    #[inline(always)]
    pub fn r_mon_baud_en(&self) -> RMonBaudEnR {
        RMonBaudEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - Indicate the bit number threshold of one monitor period, max value is 127"]
    #[inline(always)]
    pub fn r_bit_num_thres(&self) -> RBitNumThresR {
        RBitNumThresR::new(((self.bits >> 1) & 0xff) as u8)
    }
    #[doc = "Bits 9:28 - Indicate the average OSC clock cycle number of one bit, for Rx path OSC clk NOTE: SW sets the initial value, HW updates it depending on the monitor result."]
    #[inline(always)]
    pub fn r_cycnum_perbit_osc(&self) -> RCycnumPerbitOscR {
        RCycnumPerbitOscR::new((self.bits >> 9) & 0x000f_ffff)
    }
    #[doc = "Bit 29 - Update parameter r_cycnum_perbit_osc (during the update of parameter r_cycnum_perbit_xtal) enable"]
    #[inline(always)]
    pub fn r_upd_osc_in_xtal(&self) -> RUpdOscInXtalR {
        RUpdOscInXtalR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Monitor Rx baud enable"]
    #[inline(always)]
    pub fn r_mon_baud_en(&mut self) -> RMonBaudEnW<'_, UartMonBaudCtrlSpec> {
        RMonBaudEnW::new(self, 0)
    }
    #[doc = "Bits 1:8 - Indicate the bit number threshold of one monitor period, max value is 127"]
    #[inline(always)]
    pub fn r_bit_num_thres(&mut self) -> RBitNumThresW<'_, UartMonBaudCtrlSpec> {
        RBitNumThresW::new(self, 1)
    }
    #[doc = "Bits 9:28 - Indicate the average OSC clock cycle number of one bit, for Rx path OSC clk NOTE: SW sets the initial value, HW updates it depending on the monitor result."]
    #[inline(always)]
    pub fn r_cycnum_perbit_osc(&mut self) -> RCycnumPerbitOscW<'_, UartMonBaudCtrlSpec> {
        RCycnumPerbitOscW::new(self, 9)
    }
    #[doc = "Bit 29 - Update parameter r_cycnum_perbit_osc (during the update of parameter r_cycnum_perbit_xtal) enable"]
    #[inline(always)]
    pub fn r_upd_osc_in_xtal(&mut self) -> RUpdOscInXtalW<'_, UartMonBaudCtrlSpec> {
        RUpdOscInXtalW::new(self, 29)
    }
}
#[doc = "UART monitor baud rate control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_mon_baud_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_mon_baud_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartMonBaudCtrlSpec;
impl crate::RegisterSpec for UartMonBaudCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_mon_baud_ctrl::R`](R) reader structure"]
impl crate::Readable for UartMonBaudCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_mon_baud_ctrl::W`](W) writer structure"]
impl crate::Writable for UartMonBaudCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_MON_BAUD_CTRL to value 0"]
impl crate::Resettable for UartMonBaudCtrlSpec {}
