#[doc = "Register `TM_CTRL` reader"]
pub type R = crate::R<TmCtrlSpec>;
#[doc = "Register `TM_CTRL` writer"]
pub type W = crate::W<TmCtrlSpec>;
#[doc = "Field `tm_adccksel` reader - Selelct ADC fs * 0: clk_src/256 * 1: clk_src/128"]
pub type TmAdcckselR = crate::BitReader;
#[doc = "Field `tm_adccksel` writer - Selelct ADC fs * 0: clk_src/256 * 1: clk_src/128"]
pub type TmAdcckselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tm_biasdem_en` reader - Bias DEM enable"]
pub type TmBiasdemEnR = crate::BitReader;
#[doc = "Field `tm_biasdem_en` writer - Bias DEM enable"]
pub type TmBiasdemEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tm_bias_sel` reader - Select ADC bias current * 0: 8uA * 1: 10uA * 2: 12uA * 3: 14uA"]
pub type TmBiasSelR = crate::FieldReader;
#[doc = "Field `tm_bias_sel` writer - Select ADC bias current * 0: 8uA * 1: 10uA * 2: 12uA * 3: 14uA"]
pub type TmBiasSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tm_en_latch` reader - Latch thermal sensor output immediately after power on for more accurate test"]
pub type TmEnLatchR = crate::BitReader;
#[doc = "Field `tm_en_latch` writer - Latch thermal sensor output immediately after power on for more accurate test"]
pub type TmEnLatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tm_chop_en` reader - Chop enable"]
pub type TmChopEnR = crate::BitReader;
#[doc = "Field `tm_chop_en` writer - Chop enable"]
pub type TmChopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tm_hold_dly` reader - Hold delay, adjust TM_CLK_OUT width"]
pub type TmHoldDlyR = crate::FieldReader;
#[doc = "Field `tm_hold_dly` writer - Hold delay, adjust TM_CLK_OUT width"]
pub type TmHoldDlyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tm_hold_en` reader - Hold output"]
pub type TmHoldEnR = crate::BitReader;
#[doc = "Field `tm_hold_en` writer - Hold output"]
pub type TmHoldEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tm_osr` reader - Deci filter DSR select * 0: 128 * 1: 256 * 2: 512 * 3: 1024"]
pub type TmOsrR = crate::FieldReader;
#[doc = "Field `tm_osr` writer - Deci filter DSR select * 0: 128 * 1: 256 * 2: 512 * 3: 1024"]
pub type TmOsrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tm_chopfreqsel` reader - Chop frequecy select * 0: fs/64 * 1: fs/128 * 3: fs/256 * 7: fs/512 * 15: fs/1024"]
pub type TmChopfreqselR = crate::FieldReader;
#[doc = "Field `tm_chopfreqsel` writer - Chop frequecy select * 0: fs/64 * 1: fs/128 * 3: fs/256 * 7: fs/512 * 15: fs/1024"]
pub type TmChopfreqselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tm_rstb` reader - Thermal reset signal. * 0: disable thermal * 1: enable thermal"]
pub type TmRstbR = crate::BitReader;
#[doc = "Field `tm_rstb` writer - Thermal reset signal. * 0: disable thermal * 1: enable thermal"]
pub type TmRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tm_clk_inv_sel` reader - Clock phase invert select when read temperature. * 0: not invert (sample data in rising edge) * 1: invert (sample data in falling edge)"]
pub type TmClkInvSelR = crate::BitReader;
#[doc = "Field `tm_clk_inv_sel` writer - Clock phase invert select when read temperature. * 0: not invert (sample data in rising edge) * 1: invert (sample data in falling edge)"]
pub type TmClkInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tm_powcut` reader - Power cut * 0: disable power cut * 1: enable power cut"]
pub type TmPowcutR = crate::BitReader;
#[doc = "Field `tm_powcut` writer - Power cut * 0: disable power cut * 1: enable power cut"]
pub type TmPowcutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tm_pow` reader - thermal poweron signal * 0:power down * 1:power on"]
pub type TmPowR = crate::BitReader;
#[doc = "Field `tm_pow` writer - thermal poweron signal * 0:power down * 1:power on"]
pub type TmPowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tm_pwr` reader - thm protect password. tm_pow,tm_powcut,tm_rstb,tm_highcmp_pt_en,tm_high_pt_thr, iso_thm can be programed only when this value is 8'h69."]
pub type TmPwrR = crate::FieldReader;
#[doc = "Field `tm_pwr` writer - thm protect password. tm_pow,tm_powcut,tm_rstb,tm_highcmp_pt_en,tm_high_pt_thr, iso_thm can be programed only when this value is 8'h69."]
pub type TmPwrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Selelct ADC fs * 0: clk_src/256 * 1: clk_src/128"]
    #[inline(always)]
    pub fn tm_adccksel(&self) -> TmAdcckselR {
        TmAdcckselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Bias DEM enable"]
    #[inline(always)]
    pub fn tm_biasdem_en(&self) -> TmBiasdemEnR {
        TmBiasdemEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Select ADC bias current * 0: 8uA * 1: 10uA * 2: 12uA * 3: 14uA"]
    #[inline(always)]
    pub fn tm_bias_sel(&self) -> TmBiasSelR {
        TmBiasSelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Latch thermal sensor output immediately after power on for more accurate test"]
    #[inline(always)]
    pub fn tm_en_latch(&self) -> TmEnLatchR {
        TmEnLatchR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Chop enable"]
    #[inline(always)]
    pub fn tm_chop_en(&self) -> TmChopEnR {
        TmChopEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Hold delay, adjust TM_CLK_OUT width"]
    #[inline(always)]
    pub fn tm_hold_dly(&self) -> TmHoldDlyR {
        TmHoldDlyR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Hold output"]
    #[inline(always)]
    pub fn tm_hold_en(&self) -> TmHoldEnR {
        TmHoldEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - Deci filter DSR select * 0: 128 * 1: 256 * 2: 512 * 3: 1024"]
    #[inline(always)]
    pub fn tm_osr(&self) -> TmOsrR {
        TmOsrR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:17 - Chop frequecy select * 0: fs/64 * 1: fs/128 * 3: fs/256 * 7: fs/512 * 15: fs/1024"]
    #[inline(always)]
    pub fn tm_chopfreqsel(&self) -> TmChopfreqselR {
        TmChopfreqselR::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - Thermal reset signal. * 0: disable thermal * 1: enable thermal"]
    #[inline(always)]
    pub fn tm_rstb(&self) -> TmRstbR {
        TmRstbR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clock phase invert select when read temperature. * 0: not invert (sample data in rising edge) * 1: invert (sample data in falling edge)"]
    #[inline(always)]
    pub fn tm_clk_inv_sel(&self) -> TmClkInvSelR {
        TmClkInvSelR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Power cut * 0: disable power cut * 1: enable power cut"]
    #[inline(always)]
    pub fn tm_powcut(&self) -> TmPowcutR {
        TmPowcutR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - thermal poweron signal * 0:power down * 1:power on"]
    #[inline(always)]
    pub fn tm_pow(&self) -> TmPowR {
        TmPowR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:31 - thm protect password. tm_pow,tm_powcut,tm_rstb,tm_highcmp_pt_en,tm_high_pt_thr, iso_thm can be programed only when this value is 8'h69."]
    #[inline(always)]
    pub fn tm_pwr(&self) -> TmPwrR {
        TmPwrR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Selelct ADC fs * 0: clk_src/256 * 1: clk_src/128"]
    #[inline(always)]
    pub fn tm_adccksel(&mut self) -> TmAdcckselW<'_, TmCtrlSpec> {
        TmAdcckselW::new(self, 0)
    }
    #[doc = "Bit 3 - Bias DEM enable"]
    #[inline(always)]
    pub fn tm_biasdem_en(&mut self) -> TmBiasdemEnW<'_, TmCtrlSpec> {
        TmBiasdemEnW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Select ADC bias current * 0: 8uA * 1: 10uA * 2: 12uA * 3: 14uA"]
    #[inline(always)]
    pub fn tm_bias_sel(&mut self) -> TmBiasSelW<'_, TmCtrlSpec> {
        TmBiasSelW::new(self, 4)
    }
    #[doc = "Bit 6 - Latch thermal sensor output immediately after power on for more accurate test"]
    #[inline(always)]
    pub fn tm_en_latch(&mut self) -> TmEnLatchW<'_, TmCtrlSpec> {
        TmEnLatchW::new(self, 6)
    }
    #[doc = "Bit 7 - Chop enable"]
    #[inline(always)]
    pub fn tm_chop_en(&mut self) -> TmChopEnW<'_, TmCtrlSpec> {
        TmChopEnW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Hold delay, adjust TM_CLK_OUT width"]
    #[inline(always)]
    pub fn tm_hold_dly(&mut self) -> TmHoldDlyW<'_, TmCtrlSpec> {
        TmHoldDlyW::new(self, 8)
    }
    #[doc = "Bit 10 - Hold output"]
    #[inline(always)]
    pub fn tm_hold_en(&mut self) -> TmHoldEnW<'_, TmCtrlSpec> {
        TmHoldEnW::new(self, 10)
    }
    #[doc = "Bits 11:13 - Deci filter DSR select * 0: 128 * 1: 256 * 2: 512 * 3: 1024"]
    #[inline(always)]
    pub fn tm_osr(&mut self) -> TmOsrW<'_, TmCtrlSpec> {
        TmOsrW::new(self, 11)
    }
    #[doc = "Bits 14:17 - Chop frequecy select * 0: fs/64 * 1: fs/128 * 3: fs/256 * 7: fs/512 * 15: fs/1024"]
    #[inline(always)]
    pub fn tm_chopfreqsel(&mut self) -> TmChopfreqselW<'_, TmCtrlSpec> {
        TmChopfreqselW::new(self, 14)
    }
    #[doc = "Bit 18 - Thermal reset signal. * 0: disable thermal * 1: enable thermal"]
    #[inline(always)]
    pub fn tm_rstb(&mut self) -> TmRstbW<'_, TmCtrlSpec> {
        TmRstbW::new(self, 18)
    }
    #[doc = "Bit 19 - Clock phase invert select when read temperature. * 0: not invert (sample data in rising edge) * 1: invert (sample data in falling edge)"]
    #[inline(always)]
    pub fn tm_clk_inv_sel(&mut self) -> TmClkInvSelW<'_, TmCtrlSpec> {
        TmClkInvSelW::new(self, 19)
    }
    #[doc = "Bit 20 - Power cut * 0: disable power cut * 1: enable power cut"]
    #[inline(always)]
    pub fn tm_powcut(&mut self) -> TmPowcutW<'_, TmCtrlSpec> {
        TmPowcutW::new(self, 20)
    }
    #[doc = "Bit 21 - thermal poweron signal * 0:power down * 1:power on"]
    #[inline(always)]
    pub fn tm_pow(&mut self) -> TmPowW<'_, TmCtrlSpec> {
        TmPowW::new(self, 21)
    }
    #[doc = "Bits 24:31 - thm protect password. tm_pow,tm_powcut,tm_rstb,tm_highcmp_pt_en,tm_high_pt_thr, iso_thm can be programed only when this value is 8'h69."]
    #[inline(always)]
    pub fn tm_pwr(&mut self) -> TmPwrW<'_, TmCtrlSpec> {
        TmPwrW::new(self, 24)
    }
}
#[doc = "Thermal Meter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tm_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tm_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmCtrlSpec;
impl crate::RegisterSpec for TmCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tm_ctrl::R`](R) reader structure"]
impl crate::Readable for TmCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tm_ctrl::W`](W) writer structure"]
impl crate::Writable for TmCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TM_CTRL to value 0x6934_41d9"]
impl crate::Resettable for TmCtrlSpec {
    const RESET_VALUE: u32 = 0x6934_41d9;
}
