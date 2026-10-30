#[doc = "Register `SDIO_BUS_TA_TIME` reader"]
pub type R = crate::R<SdioBusTaTimeSpec>;
#[doc = "Register `SDIO_BUS_TA_TIME` writer"]
pub type W = crate::W<SdioBusTaTimeSpec>;
#[doc = "Field `sdio_bus_ta_time_cfg` reader - Bus_TA_Time These bits are used to define the number of SDCLK periods that HW should wait for SD bus turn-around time. After sending the end bit of command, HW will wait at least Bus_TA_Time to start to receive the start bit of response from card. After sending the end bit of write block, HW will also wait at least Bus_TA_Time to start to receive the start bit of CRC status from card."]
pub type SdioBusTaTimeCfgR = crate::FieldReader;
#[doc = "Field `sdio_bus_ta_time_cfg` writer - Bus_TA_Time These bits are used to define the number of SDCLK periods that HW should wait for SD bus turn-around time. After sending the end bit of command, HW will wait at least Bus_TA_Time to start to receive the start bit of response from card. After sending the end bit of write block, HW will also wait at least Bus_TA_Time to start to receive the start bit of CRC status from card."]
pub type SdioBusTaTimeCfgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sdio_bus_ta_time_set` reader - Turn-Around Time enable When this bit is b1, Bus_TA_Time(bit\\[2:0\\]) is in use. Otherwise HW will determine the bus turn-around time automatically."]
pub type SdioBusTaTimeSetR = crate::BitReader;
#[doc = "Field `sdio_bus_ta_time_set` writer - Turn-Around Time enable When this bit is b1, Bus_TA_Time(bit\\[2:0\\]) is in use. Otherwise HW will determine the bus turn-around time automatically."]
pub type SdioBusTaTimeSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `stop_clk_ctrl_cycle` reader - stop_clk_ctrl_cycle Stop SDCLK some clocks ahead before receiving the end of the read-block 3'd0: no advance 3'd1:advance 1 clock cycle 3'd2: advance 2 clock cycles The other value more than 2: And so on"]
pub type StopClkCtrlCycleR = crate::FieldReader;
#[doc = "Field `stop_clk_ctrl_cycle` writer - stop_clk_ctrl_cycle Stop SDCLK some clocks ahead before receiving the end of the read-block 3'd0: no advance 3'd1:advance 1 clock cycle 3'd2: advance 2 clock cycles The other value more than 2: And so on"]
pub type StopClkCtrlCycleW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Bus_TA_Time These bits are used to define the number of SDCLK periods that HW should wait for SD bus turn-around time. After sending the end bit of command, HW will wait at least Bus_TA_Time to start to receive the start bit of response from card. After sending the end bit of write block, HW will also wait at least Bus_TA_Time to start to receive the start bit of CRC status from card."]
    #[inline(always)]
    pub fn sdio_bus_ta_time_cfg(&self) -> SdioBusTaTimeCfgR {
        SdioBusTaTimeCfgR::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Turn-Around Time enable When this bit is b1, Bus_TA_Time(bit\\[2:0\\]) is in use. Otherwise HW will determine the bus turn-around time automatically."]
    #[inline(always)]
    pub fn sdio_bus_ta_time_set(&self) -> SdioBusTaTimeSetR {
        SdioBusTaTimeSetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - stop_clk_ctrl_cycle Stop SDCLK some clocks ahead before receiving the end of the read-block 3'd0: no advance 3'd1:advance 1 clock cycle 3'd2: advance 2 clock cycles The other value more than 2: And so on"]
    #[inline(always)]
    pub fn stop_clk_ctrl_cycle(&self) -> StopClkCtrlCycleR {
        StopClkCtrlCycleR::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Bus_TA_Time These bits are used to define the number of SDCLK periods that HW should wait for SD bus turn-around time. After sending the end bit of command, HW will wait at least Bus_TA_Time to start to receive the start bit of response from card. After sending the end bit of write block, HW will also wait at least Bus_TA_Time to start to receive the start bit of CRC status from card."]
    #[inline(always)]
    pub fn sdio_bus_ta_time_cfg(&mut self) -> SdioBusTaTimeCfgW<'_, SdioBusTaTimeSpec> {
        SdioBusTaTimeCfgW::new(self, 0)
    }
    #[doc = "Bit 3 - Turn-Around Time enable When this bit is b1, Bus_TA_Time(bit\\[2:0\\]) is in use. Otherwise HW will determine the bus turn-around time automatically."]
    #[inline(always)]
    pub fn sdio_bus_ta_time_set(&mut self) -> SdioBusTaTimeSetW<'_, SdioBusTaTimeSpec> {
        SdioBusTaTimeSetW::new(self, 3)
    }
    #[doc = "Bits 4:6 - stop_clk_ctrl_cycle Stop SDCLK some clocks ahead before receiving the end of the read-block 3'd0: no advance 3'd1:advance 1 clock cycle 3'd2: advance 2 clock cycles The other value more than 2: And so on"]
    #[inline(always)]
    pub fn stop_clk_ctrl_cycle(&mut self) -> StopClkCtrlCycleW<'_, SdioBusTaTimeSpec> {
        StopClkCtrlCycleW::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_bus_ta_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_bus_ta_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdioBusTaTimeSpec;
impl crate::RegisterSpec for SdioBusTaTimeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sdio_bus_ta_time::R`](R) reader structure"]
impl crate::Readable for SdioBusTaTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`sdio_bus_ta_time::W`](W) writer structure"]
impl crate::Writable for SdioBusTaTimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDIO_BUS_TA_TIME to value 0x03"]
impl crate::Resettable for SdioBusTaTimeSpec {
    const RESET_VALUE: u8 = 0x03;
}
