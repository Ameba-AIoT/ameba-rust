#[doc = "Register `REG_ANAPAR_MAC_2` reader"]
pub type R = crate::R<RegAnaparMac2Spec>;
#[doc = "Register `REG_ANAPAR_MAC_2` writer"]
pub type W = crate::W<RegAnaparMac2Spec>;
#[doc = "Field `F0N` reader - SDM divider sel"]
pub type F0nR = crate::FieldReader;
#[doc = "Field `F0N` writer - SDM divider sel"]
pub type F0nW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DITHER` reader - Dither funtion"]
pub type DitherR = crate::FieldReader;
#[doc = "Field `DITHER` writer - Dither funtion"]
pub type DitherW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SW_LDO2PWRCUT` reader - 1 power cut mode 0 ldo mode (ADDA LDO)"]
pub type SwLdo2pwrcutR = crate::BitReader;
#[doc = "Field `SW_LDO2PWRCUT` writer - 1 power cut mode 0 ldo mode (ADDA LDO)"]
pub type SwLdo2pwrcutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_LDO_OUT` reader - Ldo voltage 00/01/10/11: 0.85V/0.9V/0.95V/1V (ADDA LDO)"]
pub type SwLdoOutR = crate::FieldReader;
#[doc = "Field `SW_LDO_OUT` writer - Ldo voltage 00/01/10/11: 0.85V/0.9V/0.95V/1V (ADDA LDO)"]
pub type SwLdoOutW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VPLUSE_LDO_ADC` reader - For ADC(WL ADC LDO and RSSI ADC LDO)"]
pub type VpluseLdoAdcR = crate::BitReader;
#[doc = "Field `VPLUSE_LDO_ADC` writer - For ADC(WL ADC LDO and RSSI ADC LDO)"]
pub type VpluseLdoAdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VPLUSE_LDO_DAC` reader - For DAC(WL DAC LDO)"]
pub type VpluseLdoDacR = crate::BitReader;
#[doc = "Field `VPLUSE_LDO_DAC` writer - For DAC(WL DAC LDO)"]
pub type VpluseLdoDacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `null1` reader - "]
pub type Null1R = crate::FieldReader;
#[doc = "Field `null1` writer - "]
pub type Null1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CK320M_PS` reader - Debug"]
pub type Ck320mPsR = crate::FieldReader;
#[doc = "Field `CK320M_PS` writer - Debug"]
pub type Ck320mPsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_cp_idouble` reader - 1 for cp currnet double"]
pub type RegCpIdoubleR = crate::BitReader;
#[doc = "Field `reg_cp_idouble` writer - 1 for cp currnet double"]
pub type RegCpIdoubleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpe` reader - gpe=1 and gpo=1=>output gpe=1 and gpo=0=>input"]
pub type GpeR = crate::BitReader;
#[doc = "Field `gpe` writer - gpe=1 and gpo=1=>output gpe=1 and gpo=0=>input"]
pub type GpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpo` reader - "]
pub type GpoR = crate::BitReader;
#[doc = "Field `gpo` writer - "]
pub type GpoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_DCB` reader - Driving control buffer 11:larger current"]
pub type SDcbR = crate::FieldReader;
#[doc = "Field `S_DCB` writer - Driving control buffer 11:larger current"]
pub type SDcbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ORDER_SDM` reader - SDM order 1: 3th order 0: 2nd order"]
pub type OrderSdmR = crate::BitReader;
#[doc = "Field `ORDER_SDM` writer - SDM order 1: 3th order 0: 2nd order"]
pub type OrderSdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_8715_LUT` reader - 1 -> 8 phase 500MHz 0 -> 6 phase 640MHz"]
pub type Sel8715LutR = crate::BitReader;
#[doc = "Field `SEL_8715_LUT` writer - 1 -> 8 phase 500MHz 0 -> 6 phase 640MHz"]
pub type Sel8715LutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_LDO2PWRCUT_PLL` reader - 1 power cut mode 0 ldo mode(PLL LDO)"]
pub type SwLdo2pwrcutPllR = crate::BitReader;
#[doc = "Field `SW_LDO2PWRCUT_PLL` writer - 1 power cut mode 0 ldo mode(PLL LDO)"]
pub type SwLdo2pwrcutPllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_LDO_OUT_PLL` reader - Ldo voltage 00/01/10/11 0.85/0.9/0.95/1V(PLL LDO)"]
pub type SwLdoOutPllR = crate::FieldReader;
#[doc = "Field `SW_LDO_OUT_PLL` writer - Ldo voltage 00/01/10/11 0.85/0.9/0.95/1V(PLL LDO)"]
pub type SwLdoOutPllW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VPLUSE_LDO_PLL` reader - For PLL(WL PLL LDO)"]
pub type VpluseLdoPllR = crate::BitReader;
#[doc = "Field `VPLUSE_LDO_PLL` writer - For PLL(WL PLL LDO)"]
pub type VpluseLdoPllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_WLAFE_LDO_ST` reader - stable time select for WLAFE LDO 2'b00:10us, 2'b01:20us, 2'b10:100us, 2'b11:200us"]
pub type RWlafeLdoStR = crate::FieldReader;
#[doc = "Field `r_WLAFE_LDO_ST` writer - stable time select for WLAFE LDO 2'b00:10us, 2'b01:20us, 2'b10:100us, 2'b11:200us"]
pub type RWlafeLdoStW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - SDM divider sel"]
    #[inline(always)]
    pub fn f0n(&self) -> F0nR {
        F0nR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:6 - Dither funtion"]
    #[inline(always)]
    pub fn dither(&self) -> DitherR {
        DitherR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - 1 power cut mode 0 ldo mode (ADDA LDO)"]
    #[inline(always)]
    pub fn sw_ldo2pwrcut(&self) -> SwLdo2pwrcutR {
        SwLdo2pwrcutR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Ldo voltage 00/01/10/11: 0.85V/0.9V/0.95V/1V (ADDA LDO)"]
    #[inline(always)]
    pub fn sw_ldo_out(&self) -> SwLdoOutR {
        SwLdoOutR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - For ADC(WL ADC LDO and RSSI ADC LDO)"]
    #[inline(always)]
    pub fn vpluse_ldo_adc(&self) -> VpluseLdoAdcR {
        VpluseLdoAdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - For DAC(WL DAC LDO)"]
    #[inline(always)]
    pub fn vpluse_ldo_dac(&self) -> VpluseLdoDacR {
        VpluseLdoDacR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn null1(&self) -> Null1R {
        Null1R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Debug"]
    #[inline(always)]
    pub fn ck320m_ps(&self) -> Ck320mPsR {
        Ck320mPsR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - 1 for cp currnet double"]
    #[inline(always)]
    pub fn reg_cp_idouble(&self) -> RegCpIdoubleR {
        RegCpIdoubleR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - gpe=1 and gpo=1=>output gpe=1 and gpo=0=>input"]
    #[inline(always)]
    pub fn gpe(&self) -> GpeR {
        GpeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpo(&self) -> GpoR {
        GpoR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - Driving control buffer 11:larger current"]
    #[inline(always)]
    pub fn s_dcb(&self) -> SDcbR {
        SDcbR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - SDM order 1: 3th order 0: 2nd order"]
    #[inline(always)]
    pub fn order_sdm(&self) -> OrderSdmR {
        OrderSdmR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1 -> 8 phase 500MHz 0 -> 6 phase 640MHz"]
    #[inline(always)]
    pub fn sel_8715_lut(&self) -> Sel8715LutR {
        Sel8715LutR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1 power cut mode 0 ldo mode(PLL LDO)"]
    #[inline(always)]
    pub fn sw_ldo2pwrcut_pll(&self) -> SwLdo2pwrcutPllR {
        SwLdo2pwrcutPllR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Ldo voltage 00/01/10/11 0.85/0.9/0.95/1V(PLL LDO)"]
    #[inline(always)]
    pub fn sw_ldo_out_pll(&self) -> SwLdoOutPllR {
        SwLdoOutPllR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - For PLL(WL PLL LDO)"]
    #[inline(always)]
    pub fn vpluse_ldo_pll(&self) -> VpluseLdoPllR {
        VpluseLdoPllR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - stable time select for WLAFE LDO 2'b00:10us, 2'b01:20us, 2'b10:100us, 2'b11:200us"]
    #[inline(always)]
    pub fn r_wlafe_ldo_st(&self) -> RWlafeLdoStR {
        RWlafeLdoStR::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SDM divider sel"]
    #[inline(always)]
    pub fn f0n(&mut self) -> F0nW<'_, RegAnaparMac2Spec> {
        F0nW::new(self, 0)
    }
    #[doc = "Bits 3:6 - Dither funtion"]
    #[inline(always)]
    pub fn dither(&mut self) -> DitherW<'_, RegAnaparMac2Spec> {
        DitherW::new(self, 3)
    }
    #[doc = "Bit 7 - 1 power cut mode 0 ldo mode (ADDA LDO)"]
    #[inline(always)]
    pub fn sw_ldo2pwrcut(&mut self) -> SwLdo2pwrcutW<'_, RegAnaparMac2Spec> {
        SwLdo2pwrcutW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Ldo voltage 00/01/10/11: 0.85V/0.9V/0.95V/1V (ADDA LDO)"]
    #[inline(always)]
    pub fn sw_ldo_out(&mut self) -> SwLdoOutW<'_, RegAnaparMac2Spec> {
        SwLdoOutW::new(self, 8)
    }
    #[doc = "Bit 10 - For ADC(WL ADC LDO and RSSI ADC LDO)"]
    #[inline(always)]
    pub fn vpluse_ldo_adc(&mut self) -> VpluseLdoAdcW<'_, RegAnaparMac2Spec> {
        VpluseLdoAdcW::new(self, 10)
    }
    #[doc = "Bit 11 - For DAC(WL DAC LDO)"]
    #[inline(always)]
    pub fn vpluse_ldo_dac(&mut self) -> VpluseLdoDacW<'_, RegAnaparMac2Spec> {
        VpluseLdoDacW::new(self, 11)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn null1(&mut self) -> Null1W<'_, RegAnaparMac2Spec> {
        Null1W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Debug"]
    #[inline(always)]
    pub fn ck320m_ps(&mut self) -> Ck320mPsW<'_, RegAnaparMac2Spec> {
        Ck320mPsW::new(self, 14)
    }
    #[doc = "Bit 16 - 1 for cp currnet double"]
    #[inline(always)]
    pub fn reg_cp_idouble(&mut self) -> RegCpIdoubleW<'_, RegAnaparMac2Spec> {
        RegCpIdoubleW::new(self, 16)
    }
    #[doc = "Bit 17 - gpe=1 and gpo=1=>output gpe=1 and gpo=0=>input"]
    #[inline(always)]
    pub fn gpe(&mut self) -> GpeW<'_, RegAnaparMac2Spec> {
        GpeW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpo(&mut self) -> GpoW<'_, RegAnaparMac2Spec> {
        GpoW::new(self, 18)
    }
    #[doc = "Bits 19:20 - Driving control buffer 11:larger current"]
    #[inline(always)]
    pub fn s_dcb(&mut self) -> SDcbW<'_, RegAnaparMac2Spec> {
        SDcbW::new(self, 19)
    }
    #[doc = "Bit 21 - SDM order 1: 3th order 0: 2nd order"]
    #[inline(always)]
    pub fn order_sdm(&mut self) -> OrderSdmW<'_, RegAnaparMac2Spec> {
        OrderSdmW::new(self, 21)
    }
    #[doc = "Bit 22 - 1 -> 8 phase 500MHz 0 -> 6 phase 640MHz"]
    #[inline(always)]
    pub fn sel_8715_lut(&mut self) -> Sel8715LutW<'_, RegAnaparMac2Spec> {
        Sel8715LutW::new(self, 22)
    }
    #[doc = "Bit 23 - 1 power cut mode 0 ldo mode(PLL LDO)"]
    #[inline(always)]
    pub fn sw_ldo2pwrcut_pll(&mut self) -> SwLdo2pwrcutPllW<'_, RegAnaparMac2Spec> {
        SwLdo2pwrcutPllW::new(self, 23)
    }
    #[doc = "Bits 24:25 - Ldo voltage 00/01/10/11 0.85/0.9/0.95/1V(PLL LDO)"]
    #[inline(always)]
    pub fn sw_ldo_out_pll(&mut self) -> SwLdoOutPllW<'_, RegAnaparMac2Spec> {
        SwLdoOutPllW::new(self, 24)
    }
    #[doc = "Bit 26 - For PLL(WL PLL LDO)"]
    #[inline(always)]
    pub fn vpluse_ldo_pll(&mut self) -> VpluseLdoPllW<'_, RegAnaparMac2Spec> {
        VpluseLdoPllW::new(self, 26)
    }
    #[doc = "Bits 27:28 - stable time select for WLAFE LDO 2'b00:10us, 2'b01:20us, 2'b10:100us, 2'b11:200us"]
    #[inline(always)]
    pub fn r_wlafe_ldo_st(&mut self) -> RWlafeLdoStW<'_, RegAnaparMac2Spec> {
        RWlafeLdoStW::new(self, 27)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_anapar_mac_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_anapar_mac_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAnaparMac2Spec;
impl crate::RegisterSpec for RegAnaparMac2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_anapar_mac_2::R`](R) reader structure"]
impl crate::Readable for RegAnaparMac2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_anapar_mac_2::W`](W) writer structure"]
impl crate::Writable for RegAnaparMac2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ANAPAR_MAC_2 to value 0x1500_0100"]
impl crate::Resettable for RegAnaparMac2Spec {
    const RESET_VALUE: u32 = 0x1500_0100;
}
