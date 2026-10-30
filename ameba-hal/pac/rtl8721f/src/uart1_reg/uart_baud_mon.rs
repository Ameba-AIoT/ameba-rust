#[doc = "Register `UART_BAUD_MON` reader"]
pub type R = crate::R<UartBaudMonSpec>;
#[doc = "Register `UART_BAUD_MON` writer"]
pub type W = crate::W<UartBaudMonSpec>;
#[doc = "Field `MIN_FALL_SPACE` reader - Minimum space of adjacent falling edge of Rx data"]
pub type MinFallSpaceR = crate::FieldReader<u16>;
#[doc = "Field `MIN_LOW_PERIOD` reader - Minimum space of consecutive logic '0' of Rx data"]
pub type MinLowPeriodR = crate::FieldReader<u16>;
#[doc = "Field `FALLING_THRESH` reader - Indicate the threshold of Rx data rising edge"]
pub type FallingThreshR = crate::FieldReader;
#[doc = "Field `FALLING_THRESH` writer - Indicate the threshold of Rx data rising edge"]
pub type FallingThreshW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MON_DATA_VLD` reader - Indicate valid monitoring data"]
pub type MonDataVldR = crate::BitReader;
#[doc = "Field `TOGGLE_MON_EN` reader - Toggle monitor enable"]
pub type ToggleMonEnR = crate::BitReader;
#[doc = "Field `TOGGLE_MON_EN` writer - Toggle monitor enable"]
pub type ToggleMonEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Minimum space of adjacent falling edge of Rx data"]
    #[inline(always)]
    pub fn min_fall_space(&self) -> MinFallSpaceR {
        MinFallSpaceR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - Minimum space of consecutive logic '0' of Rx data"]
    #[inline(always)]
    pub fn min_low_period(&self) -> MinLowPeriodR {
        MinLowPeriodR::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bits 24:29 - Indicate the threshold of Rx data rising edge"]
    #[inline(always)]
    pub fn falling_thresh(&self) -> FallingThreshR {
        FallingThreshR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Indicate valid monitoring data"]
    #[inline(always)]
    pub fn mon_data_vld(&self) -> MonDataVldR {
        MonDataVldR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Toggle monitor enable"]
    #[inline(always)]
    pub fn toggle_mon_en(&self) -> ToggleMonEnR {
        ToggleMonEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:29 - Indicate the threshold of Rx data rising edge"]
    #[inline(always)]
    pub fn falling_thresh(&mut self) -> FallingThreshW<'_, UartBaudMonSpec> {
        FallingThreshW::new(self, 24)
    }
    #[doc = "Bit 31 - Toggle monitor enable"]
    #[inline(always)]
    pub fn toggle_mon_en(&mut self) -> ToggleMonEnW<'_, UartBaudMonSpec> {
        ToggleMonEnW::new(self, 31)
    }
}
#[doc = "UART baud monitor register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_baud_mon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_baud_mon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartBaudMonSpec;
impl crate::RegisterSpec for UartBaudMonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_baud_mon::R`](R) reader structure"]
impl crate::Readable for UartBaudMonSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_baud_mon::W`](W) writer structure"]
impl crate::Writable for UartBaudMonSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_BAUD_MON to value 0x0700_0000"]
impl crate::Resettable for UartBaudMonSpec {
    const RESET_VALUE: u32 = 0x0700_0000;
}
