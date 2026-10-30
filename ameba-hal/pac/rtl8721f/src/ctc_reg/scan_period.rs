#[doc = "Register `SCAN_PERIOD` reader"]
pub type R = crate::R<ScanPeriodSpec>;
#[doc = "Register `SCAN_PERIOD` writer"]
pub type W = crate::W<ScanPeriodSpec>;
#[doc = "Field `sleep_time_cnt` reader - Code: 0~4095 (0~0xFFF), unit is 1.024KHz cycle (1/32 32.768KHz). When this register is set to 0 , HW will scan continuously and have no sleep time. Recommend value: 60~480ms"]
pub type SleepTimeCntR = crate::FieldReader<u16>;
#[doc = "Field `sleep_time_cnt` writer - Code: 0~4095 (0~0xFFF), unit is 1.024KHz cycle (1/32 32.768KHz). When this register is set to 0 , HW will scan continuously and have no sleep time. Recommend value: 60~480ms"]
pub type SleepTimeCntW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `sample_ave_ctrl` reader - ADC sampled number for average function average number=2^(sample_ave_ctrl+2)"]
pub type SampleAveCtrlR = crate::FieldReader;
#[doc = "Field `sample_ave_ctrl` writer - ADC sampled number for average function average number=2^(sample_ave_ctrl+2)"]
pub type SampleAveCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:11 - Code: 0~4095 (0~0xFFF), unit is 1.024KHz cycle (1/32 32.768KHz). When this register is set to 0 , HW will scan continuously and have no sleep time. Recommend value: 60~480ms"]
    #[inline(always)]
    pub fn sleep_time_cnt(&self) -> SleepTimeCntR {
        SleepTimeCntR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:18 - ADC sampled number for average function average number=2^(sample_ave_ctrl+2)"]
    #[inline(always)]
    pub fn sample_ave_ctrl(&self) -> SampleAveCtrlR {
        SampleAveCtrlR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Code: 0~4095 (0~0xFFF), unit is 1.024KHz cycle (1/32 32.768KHz). When this register is set to 0 , HW will scan continuously and have no sleep time. Recommend value: 60~480ms"]
    #[inline(always)]
    pub fn sleep_time_cnt(&mut self) -> SleepTimeCntW<'_, ScanPeriodSpec> {
        SleepTimeCntW::new(self, 0)
    }
    #[doc = "Bits 16:18 - ADC sampled number for average function average number=2^(sample_ave_ctrl+2)"]
    #[inline(always)]
    pub fn sample_ave_ctrl(&mut self) -> SampleAveCtrlW<'_, ScanPeriodSpec> {
        SampleAveCtrlW::new(self, 16)
    }
}
#[doc = "Scan Parameters Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scan_period::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scan_period::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScanPeriodSpec;
impl crate::RegisterSpec for ScanPeriodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scan_period::R`](R) reader structure"]
impl crate::Readable for ScanPeriodSpec {}
#[doc = "`write(|w| ..)` method takes [`scan_period::W`](W) writer structure"]
impl crate::Writable for ScanPeriodSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCAN_PERIOD to value 0x0006_003c"]
impl crate::Resettable for ScanPeriodSpec {
    const RESET_VALUE: u32 = 0x0006_003c;
}
