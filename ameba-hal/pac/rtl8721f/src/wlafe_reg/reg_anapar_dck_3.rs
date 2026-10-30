#[doc = "Register `REG_ANAPAR_DCK_3` reader"]
pub type R = crate::R<RegAnaparDck3Spec>;
#[doc = "Register `REG_ANAPAR_DCK_3` writer"]
pub type W = crate::W<RegAnaparDck3Spec>;
#[doc = "Field `DCK_BT_KOS_EN_FORCE_Q_CR` reader - Manual write KOSEN at BT mode"]
pub type DckBtKosEnForceQCrR = crate::BitReader;
#[doc = "Field `DCK_BT_KOS_EN_FORCE_Q_CR` writer - Manual write KOSEN at BT mode"]
pub type DckBtKosEnForceQCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_WL_DOS_FORCE_I_CR` reader - Manual write DOS data at WL mode"]
pub type DckWlDosForceICrR = crate::BitReader;
#[doc = "Field `DCK_WL_DOS_FORCE_I_CR` writer - Manual write DOS data at WL mode"]
pub type DckWlDosForceICrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_WL_DOS_FORCE_Q_CR` reader - Manual write DOS data at WL mode"]
pub type DckWlDosForceQCrR = crate::BitReader;
#[doc = "Field `DCK_WL_DOS_FORCE_Q_CR` writer - Manual write DOS data at WL mode"]
pub type DckWlDosForceQCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_WL_KOS_EN_FORCE_I_CR` reader - Manual write KOSEN at WL mode"]
pub type DckWlKosEnForceICrR = crate::BitReader;
#[doc = "Field `DCK_WL_KOS_EN_FORCE_I_CR` writer - Manual write KOSEN at WL mode"]
pub type DckWlKosEnForceICrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_WL_KOS_EN_FORCE_Q_CR` reader - Manual write KOSEN at WL mode"]
pub type DckWlKosEnForceQCrR = crate::BitReader;
#[doc = "Field `DCK_WL_KOS_EN_FORCE_Q_CR` writer - Manual write KOSEN at WL mode"]
pub type DckWlKosEnForceQCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ad_iq_chge` reader - 0:adci to DCK I ;adcq to DCK Q /1:adci to DCK Q;adcq to DCK I"]
pub type RegAdIqChgeR = crate::BitReader;
#[doc = "Field `reg_ad_iq_chge` writer - 0:adci to DCK I ;adcq to DCK Q /1:adci to DCK Q;adcq to DCK I"]
pub type RegAdIqChgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_RESERVE_CR` reader - "]
pub type DckReserveCrR = crate::FieldReader<u16>;
#[doc = "Field `DCK_RESERVE_CR` writer - "]
pub type DckReserveCrW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `DCK_WAVE_OFSN_CR` reader - "]
pub type DckWaveOfsnCrR = crate::FieldReader;
#[doc = "Field `DCK_WAVE_OFSN_CR` writer - "]
pub type DckWaveOfsnCrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DCK_ADC_OFS_Q_CR` reader - Compensate ADC qpath offset: adc2cal_AD\\[9:0\\] + ADC_ofs_CR\\[5:0\\]"]
pub type DckAdcOfsQCrR = crate::FieldReader;
#[doc = "Field `DCK_ADC_OFS_Q_CR` writer - Compensate ADC qpath offset: adc2cal_AD\\[9:0\\] + ADC_ofs_CR\\[5:0\\]"]
pub type DckAdcOfsQCrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DCK_AD_DATA_SW_DEBUG_CR` reader - 0: choose I path vos data 1: choose Q path vos data"]
pub type DckAdDataSwDebugCrR = crate::BitReader;
#[doc = "Field `DCK_AD_DATA_SW_DEBUG_CR` writer - 0: choose I path vos data 1: choose Q path vos data"]
pub type DckAdDataSwDebugCrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Manual write KOSEN at BT mode"]
    #[inline(always)]
    pub fn dck_bt_kos_en_force_q_cr(&self) -> DckBtKosEnForceQCrR {
        DckBtKosEnForceQCrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Manual write DOS data at WL mode"]
    #[inline(always)]
    pub fn dck_wl_dos_force_i_cr(&self) -> DckWlDosForceICrR {
        DckWlDosForceICrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Manual write DOS data at WL mode"]
    #[inline(always)]
    pub fn dck_wl_dos_force_q_cr(&self) -> DckWlDosForceQCrR {
        DckWlDosForceQCrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Manual write KOSEN at WL mode"]
    #[inline(always)]
    pub fn dck_wl_kos_en_force_i_cr(&self) -> DckWlKosEnForceICrR {
        DckWlKosEnForceICrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Manual write KOSEN at WL mode"]
    #[inline(always)]
    pub fn dck_wl_kos_en_force_q_cr(&self) -> DckWlKosEnForceQCrR {
        DckWlKosEnForceQCrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0:adci to DCK I ;adcq to DCK Q /1:adci to DCK Q;adcq to DCK I"]
    #[inline(always)]
    pub fn reg_ad_iq_chge(&self) -> RegAdIqChgeR {
        RegAdIqChgeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:16"]
    #[inline(always)]
    pub fn dck_reserve_cr(&self) -> DckReserveCrR {
        DckReserveCrR::new(((self.bits >> 6) & 0x07ff) as u16)
    }
    #[doc = "Bits 17:24"]
    #[inline(always)]
    pub fn dck_wave_ofsn_cr(&self) -> DckWaveOfsnCrR {
        DckWaveOfsnCrR::new(((self.bits >> 17) & 0xff) as u8)
    }
    #[doc = "Bits 25:30 - Compensate ADC qpath offset: adc2cal_AD\\[9:0\\] + ADC_ofs_CR\\[5:0\\]"]
    #[inline(always)]
    pub fn dck_adc_ofs_q_cr(&self) -> DckAdcOfsQCrR {
        DckAdcOfsQCrR::new(((self.bits >> 25) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - 0: choose I path vos data 1: choose Q path vos data"]
    #[inline(always)]
    pub fn dck_ad_data_sw_debug_cr(&self) -> DckAdDataSwDebugCrR {
        DckAdDataSwDebugCrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Manual write KOSEN at BT mode"]
    #[inline(always)]
    pub fn dck_bt_kos_en_force_q_cr(&mut self) -> DckBtKosEnForceQCrW<'_, RegAnaparDck3Spec> {
        DckBtKosEnForceQCrW::new(self, 0)
    }
    #[doc = "Bit 1 - Manual write DOS data at WL mode"]
    #[inline(always)]
    pub fn dck_wl_dos_force_i_cr(&mut self) -> DckWlDosForceICrW<'_, RegAnaparDck3Spec> {
        DckWlDosForceICrW::new(self, 1)
    }
    #[doc = "Bit 2 - Manual write DOS data at WL mode"]
    #[inline(always)]
    pub fn dck_wl_dos_force_q_cr(&mut self) -> DckWlDosForceQCrW<'_, RegAnaparDck3Spec> {
        DckWlDosForceQCrW::new(self, 2)
    }
    #[doc = "Bit 3 - Manual write KOSEN at WL mode"]
    #[inline(always)]
    pub fn dck_wl_kos_en_force_i_cr(&mut self) -> DckWlKosEnForceICrW<'_, RegAnaparDck3Spec> {
        DckWlKosEnForceICrW::new(self, 3)
    }
    #[doc = "Bit 4 - Manual write KOSEN at WL mode"]
    #[inline(always)]
    pub fn dck_wl_kos_en_force_q_cr(&mut self) -> DckWlKosEnForceQCrW<'_, RegAnaparDck3Spec> {
        DckWlKosEnForceQCrW::new(self, 4)
    }
    #[doc = "Bit 5 - 0:adci to DCK I ;adcq to DCK Q /1:adci to DCK Q;adcq to DCK I"]
    #[inline(always)]
    pub fn reg_ad_iq_chge(&mut self) -> RegAdIqChgeW<'_, RegAnaparDck3Spec> {
        RegAdIqChgeW::new(self, 5)
    }
    #[doc = "Bits 6:16"]
    #[inline(always)]
    pub fn dck_reserve_cr(&mut self) -> DckReserveCrW<'_, RegAnaparDck3Spec> {
        DckReserveCrW::new(self, 6)
    }
    #[doc = "Bits 17:24"]
    #[inline(always)]
    pub fn dck_wave_ofsn_cr(&mut self) -> DckWaveOfsnCrW<'_, RegAnaparDck3Spec> {
        DckWaveOfsnCrW::new(self, 17)
    }
    #[doc = "Bits 25:30 - Compensate ADC qpath offset: adc2cal_AD\\[9:0\\] + ADC_ofs_CR\\[5:0\\]"]
    #[inline(always)]
    pub fn dck_adc_ofs_q_cr(&mut self) -> DckAdcOfsQCrW<'_, RegAnaparDck3Spec> {
        DckAdcOfsQCrW::new(self, 25)
    }
    #[doc = "Bit 31 - 0: choose I path vos data 1: choose Q path vos data"]
    #[inline(always)]
    pub fn dck_ad_data_sw_debug_cr(&mut self) -> DckAdDataSwDebugCrW<'_, RegAnaparDck3Spec> {
        DckAdDataSwDebugCrW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_anapar_dck_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_anapar_dck_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAnaparDck3Spec;
impl crate::RegisterSpec for RegAnaparDck3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_anapar_dck_3::R`](R) reader structure"]
impl crate::Readable for RegAnaparDck3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_anapar_dck_3::W`](W) writer structure"]
impl crate::Writable for RegAnaparDck3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ANAPAR_DCK_3 to value 0x0092_0000"]
impl crate::Resettable for RegAnaparDck3Spec {
    const RESET_VALUE: u32 = 0x0092_0000;
}
