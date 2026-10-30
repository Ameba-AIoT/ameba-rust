#[doc = "Register `REG_ANAPAR_DCK_0` reader"]
pub type R = crate::R<RegAnaparDck0Spec>;
#[doc = "Register `REG_ANAPAR_DCK_0` writer"]
pub type W = crate::W<RegAnaparDck0Spec>;
#[doc = "Field `DCK_SWAP_FORCE_CR` reader - Force the SWAP from SWAP_Manu_CR"]
pub type DckSwapForceCrR = crate::BitReader;
#[doc = "Field `DCK_SWAP_FORCE_CR` writer - Force the SWAP from SWAP_Manu_CR"]
pub type DckSwapForceCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_SWAP_MANU_CR` reader - SWAP by manual"]
pub type DckSwapManuCrR = crate::BitReader;
#[doc = "Field `DCK_SWAP_MANU_CR` writer - SWAP by manual"]
pub type DckSwapManuCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_RSTB` reader - DCK power on reset"]
pub type DckRstbR = crate::BitReader;
#[doc = "Field `DCK_RSTB` writer - DCK power on reset"]
pub type DckRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_POW_DA_CR` reader - power on for DC-offset calibration"]
pub type DckPowDaCrR = crate::BitReader;
#[doc = "Field `DCK_POW_DA_CR` writer - power on for DC-offset calibration"]
pub type DckPowDaCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_CAL_OS_CR` reader - DC-offset calibration enable, ipath and qpath"]
pub type DckCalOsCrR = crate::BitReader;
#[doc = "Field `DCK_CAL_OS_CR` writer - DC-offset calibration enable, ipath and qpath"]
pub type DckCalOsCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_CAL_OS_I_CR` reader - DC-offset calibration enable ipath"]
pub type DckCalOsICrR = crate::BitReader;
#[doc = "Field `DCK_CAL_OS_I_CR` writer - DC-offset calibration enable ipath"]
pub type DckCalOsICrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_CAL_OS_Q_CR` reader - DC-offset calibration enable qpath"]
pub type DckCalOsQCrR = crate::BitReader;
#[doc = "Field `DCK_CAL_OS_Q_CR` writer - DC-offset calibration enable qpath"]
pub type DckCalOsQCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_LPOW_CR` reader - Low power enable when calibration done"]
pub type DckLpowCrR = crate::BitReader;
#[doc = "Field `DCK_LPOW_CR` writer - Low power enable when calibration done"]
pub type DckLpowCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_ADC_OFS_CR` reader - Compensate ADC offset : adc2cal_AD\\[9:0\\] + ADC_ofs_CR\\[5:0\\]"]
pub type DckAdcOfsCrR = crate::FieldReader;
#[doc = "Field `DCK_ADC_OFS_CR` writer - Compensate ADC offset : adc2cal_AD\\[9:0\\] + ADC_ofs_CR\\[5:0\\]"]
pub type DckAdcOfsCrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DCK_CK_ADC_BAR_CR` reader - 0: original clock 1: inverse clock"]
pub type DckCkAdcBarCrR = crate::BitReader;
#[doc = "Field `DCK_CK_ADC_BAR_CR` writer - 0: original clock 1: inverse clock"]
pub type DckCkAdcBarCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_CK_DAC_BAR_CR` reader - 0: original clock 1: inverse clock"]
pub type DckCkDacBarCrR = crate::BitReader;
#[doc = "Field `DCK_CK_DAC_BAR_CR` writer - 0: original clock 1: inverse clock"]
pub type DckCkDacBarCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_CK_SYS_BAR_CR` reader - 0: original clock 1: inverse clock"]
pub type DckCkSysBarCrR = crate::BitReader;
#[doc = "Field `DCK_CK_SYS_BAR_CR` writer - 0: original clock 1: inverse clock"]
pub type DckCkSysBarCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_WAVE_OFSP_CR` reader - id\\[7:0\\] or qd\\[7:0\\] = wave_mid_val + wave_ofs_val"]
pub type DckWaveOfspCrR = crate::FieldReader;
#[doc = "Field `DCK_WAVE_OFSP_CR` writer - id\\[7:0\\] or qd\\[7:0\\] = wave_mid_val + wave_ofs_val"]
pub type DckWaveOfspCrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DCK_WAVE_MID_SH_CR` reader - DA\\[8:0\\] to shift -15 ~ +15 the middle value"]
pub type DckWaveMidShCrR = crate::FieldReader;
#[doc = "Field `DCK_WAVE_MID_SH_CR` writer - DA\\[8:0\\] to shift -15 ~ +15 the middle value"]
pub type DckWaveMidShCrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DCK_CMP_BAR_CR` reader - 0:AVG_VOS1 - AVG_VOS2 / 1:AVG_VOS2 - AVG_VOS1"]
pub type DckCmpBarCrR = crate::BitReader;
#[doc = "Field `DCK_CMP_BAR_CR` writer - 0:AVG_VOS1 - AVG_VOS2 / 1:AVG_VOS2 - AVG_VOS1"]
pub type DckCmpBarCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_COMP_ZERO_CR` reader - 0: DOS \\[+/-\\] 1: DOS\\[+/0/-\\]"]
pub type DckCompZeroCrR = crate::BitReader;
#[doc = "Field `DCK_COMP_ZERO_CR` writer - 0: DOS \\[+/-\\] 1: DOS\\[+/0/-\\]"]
pub type DckCompZeroCrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Force the SWAP from SWAP_Manu_CR"]
    #[inline(always)]
    pub fn dck_swap_force_cr(&self) -> DckSwapForceCrR {
        DckSwapForceCrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SWAP by manual"]
    #[inline(always)]
    pub fn dck_swap_manu_cr(&self) -> DckSwapManuCrR {
        DckSwapManuCrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DCK power on reset"]
    #[inline(always)]
    pub fn dck_rstb(&self) -> DckRstbR {
        DckRstbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - power on for DC-offset calibration"]
    #[inline(always)]
    pub fn dck_pow_da_cr(&self) -> DckPowDaCrR {
        DckPowDaCrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DC-offset calibration enable, ipath and qpath"]
    #[inline(always)]
    pub fn dck_cal_os_cr(&self) -> DckCalOsCrR {
        DckCalOsCrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DC-offset calibration enable ipath"]
    #[inline(always)]
    pub fn dck_cal_os_i_cr(&self) -> DckCalOsICrR {
        DckCalOsICrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DC-offset calibration enable qpath"]
    #[inline(always)]
    pub fn dck_cal_os_q_cr(&self) -> DckCalOsQCrR {
        DckCalOsQCrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Low power enable when calibration done"]
    #[inline(always)]
    pub fn dck_lpow_cr(&self) -> DckLpowCrR {
        DckLpowCrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Compensate ADC offset : adc2cal_AD\\[9:0\\] + ADC_ofs_CR\\[5:0\\]"]
    #[inline(always)]
    pub fn dck_adc_ofs_cr(&self) -> DckAdcOfsCrR {
        DckAdcOfsCrR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - 0: original clock 1: inverse clock"]
    #[inline(always)]
    pub fn dck_ck_adc_bar_cr(&self) -> DckCkAdcBarCrR {
        DckCkAdcBarCrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 0: original clock 1: inverse clock"]
    #[inline(always)]
    pub fn dck_ck_dac_bar_cr(&self) -> DckCkDacBarCrR {
        DckCkDacBarCrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 0: original clock 1: inverse clock"]
    #[inline(always)]
    pub fn dck_ck_sys_bar_cr(&self) -> DckCkSysBarCrR {
        DckCkSysBarCrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:24 - id\\[7:0\\] or qd\\[7:0\\] = wave_mid_val + wave_ofs_val"]
    #[inline(always)]
    pub fn dck_wave_ofsp_cr(&self) -> DckWaveOfspCrR {
        DckWaveOfspCrR::new(((self.bits >> 17) & 0xff) as u8)
    }
    #[doc = "Bits 25:29 - DA\\[8:0\\] to shift -15 ~ +15 the middle value"]
    #[inline(always)]
    pub fn dck_wave_mid_sh_cr(&self) -> DckWaveMidShCrR {
        DckWaveMidShCrR::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - 0:AVG_VOS1 - AVG_VOS2 / 1:AVG_VOS2 - AVG_VOS1"]
    #[inline(always)]
    pub fn dck_cmp_bar_cr(&self) -> DckCmpBarCrR {
        DckCmpBarCrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 0: DOS \\[+/-\\] 1: DOS\\[+/0/-\\]"]
    #[inline(always)]
    pub fn dck_comp_zero_cr(&self) -> DckCompZeroCrR {
        DckCompZeroCrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force the SWAP from SWAP_Manu_CR"]
    #[inline(always)]
    pub fn dck_swap_force_cr(&mut self) -> DckSwapForceCrW<'_, RegAnaparDck0Spec> {
        DckSwapForceCrW::new(self, 0)
    }
    #[doc = "Bit 1 - SWAP by manual"]
    #[inline(always)]
    pub fn dck_swap_manu_cr(&mut self) -> DckSwapManuCrW<'_, RegAnaparDck0Spec> {
        DckSwapManuCrW::new(self, 1)
    }
    #[doc = "Bit 2 - DCK power on reset"]
    #[inline(always)]
    pub fn dck_rstb(&mut self) -> DckRstbW<'_, RegAnaparDck0Spec> {
        DckRstbW::new(self, 2)
    }
    #[doc = "Bit 3 - power on for DC-offset calibration"]
    #[inline(always)]
    pub fn dck_pow_da_cr(&mut self) -> DckPowDaCrW<'_, RegAnaparDck0Spec> {
        DckPowDaCrW::new(self, 3)
    }
    #[doc = "Bit 4 - DC-offset calibration enable, ipath and qpath"]
    #[inline(always)]
    pub fn dck_cal_os_cr(&mut self) -> DckCalOsCrW<'_, RegAnaparDck0Spec> {
        DckCalOsCrW::new(self, 4)
    }
    #[doc = "Bit 5 - DC-offset calibration enable ipath"]
    #[inline(always)]
    pub fn dck_cal_os_i_cr(&mut self) -> DckCalOsICrW<'_, RegAnaparDck0Spec> {
        DckCalOsICrW::new(self, 5)
    }
    #[doc = "Bit 6 - DC-offset calibration enable qpath"]
    #[inline(always)]
    pub fn dck_cal_os_q_cr(&mut self) -> DckCalOsQCrW<'_, RegAnaparDck0Spec> {
        DckCalOsQCrW::new(self, 6)
    }
    #[doc = "Bit 7 - Low power enable when calibration done"]
    #[inline(always)]
    pub fn dck_lpow_cr(&mut self) -> DckLpowCrW<'_, RegAnaparDck0Spec> {
        DckLpowCrW::new(self, 7)
    }
    #[doc = "Bits 8:13 - Compensate ADC offset : adc2cal_AD\\[9:0\\] + ADC_ofs_CR\\[5:0\\]"]
    #[inline(always)]
    pub fn dck_adc_ofs_cr(&mut self) -> DckAdcOfsCrW<'_, RegAnaparDck0Spec> {
        DckAdcOfsCrW::new(self, 8)
    }
    #[doc = "Bit 14 - 0: original clock 1: inverse clock"]
    #[inline(always)]
    pub fn dck_ck_adc_bar_cr(&mut self) -> DckCkAdcBarCrW<'_, RegAnaparDck0Spec> {
        DckCkAdcBarCrW::new(self, 14)
    }
    #[doc = "Bit 15 - 0: original clock 1: inverse clock"]
    #[inline(always)]
    pub fn dck_ck_dac_bar_cr(&mut self) -> DckCkDacBarCrW<'_, RegAnaparDck0Spec> {
        DckCkDacBarCrW::new(self, 15)
    }
    #[doc = "Bit 16 - 0: original clock 1: inverse clock"]
    #[inline(always)]
    pub fn dck_ck_sys_bar_cr(&mut self) -> DckCkSysBarCrW<'_, RegAnaparDck0Spec> {
        DckCkSysBarCrW::new(self, 16)
    }
    #[doc = "Bits 17:24 - id\\[7:0\\] or qd\\[7:0\\] = wave_mid_val + wave_ofs_val"]
    #[inline(always)]
    pub fn dck_wave_ofsp_cr(&mut self) -> DckWaveOfspCrW<'_, RegAnaparDck0Spec> {
        DckWaveOfspCrW::new(self, 17)
    }
    #[doc = "Bits 25:29 - DA\\[8:0\\] to shift -15 ~ +15 the middle value"]
    #[inline(always)]
    pub fn dck_wave_mid_sh_cr(&mut self) -> DckWaveMidShCrW<'_, RegAnaparDck0Spec> {
        DckWaveMidShCrW::new(self, 25)
    }
    #[doc = "Bit 30 - 0:AVG_VOS1 - AVG_VOS2 / 1:AVG_VOS2 - AVG_VOS1"]
    #[inline(always)]
    pub fn dck_cmp_bar_cr(&mut self) -> DckCmpBarCrW<'_, RegAnaparDck0Spec> {
        DckCmpBarCrW::new(self, 30)
    }
    #[doc = "Bit 31 - 0: DOS \\[+/-\\] 1: DOS\\[+/0/-\\]"]
    #[inline(always)]
    pub fn dck_comp_zero_cr(&mut self) -> DckCompZeroCrW<'_, RegAnaparDck0Spec> {
        DckCompZeroCrW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_anapar_dck_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_anapar_dck_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAnaparDck0Spec;
impl crate::RegisterSpec for RegAnaparDck0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_anapar_dck_0::R`](R) reader structure"]
impl crate::Readable for RegAnaparDck0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_anapar_dck_0::W`](W) writer structure"]
impl crate::Writable for RegAnaparDck0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ANAPAR_DCK_0 to value 0x0092_0000"]
impl crate::Resettable for RegAnaparDck0Spec {
    const RESET_VALUE: u32 = 0x0092_0000;
}
