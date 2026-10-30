#[doc = "Register `REG_ANAPAR_DCK_1` reader"]
pub type R = crate::R<RegAnaparDck1Spec>;
#[doc = "Register `REG_ANAPAR_DCK_1` writer"]
pub type W = crate::W<RegAnaparDck1Spec>;
#[doc = "Field `DCK_WAIT_CYC_SET_CR` reader - DC-offset calibration digital waiting clock cycle for setting time"]
pub type DckWaitCycSetCrR = crate::FieldReader;
#[doc = "Field `DCK_WAIT_CYC_SET_CR` writer - DC-offset calibration digital waiting clock cycle for setting time"]
pub type DckWaitCycSetCrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DCK_AVG_CYC_CR` reader - DC-offset calibration fetch ADC data numbers to average"]
pub type DckAvgCycCrR = crate::FieldReader;
#[doc = "Field `DCK_AVG_CYC_CR` writer - DC-offset calibration fetch ADC data numbers to average"]
pub type DckAvgCycCrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DCK_SWITCH_WAIT_IQ_CR` reader - Switch I2Q waiting time"]
pub type DckSwitchWaitIqCrR = crate::FieldReader;
#[doc = "Field `DCK_SWITCH_WAIT_IQ_CR` writer - Switch I2Q waiting time"]
pub type DckSwitchWaitIqCrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DCK_VOSK_SEL_I_CR` reader - 0: fetch the middle VOS_SR=VOS1-VOS2 0: first / 1: final"]
pub type DckVoskSelICrR = crate::BitReader;
#[doc = "Field `DCK_VOSK_SEL_I_CR` writer - 0: fetch the middle VOS_SR=VOS1-VOS2 0: first / 1: final"]
pub type DckVoskSelICrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_VOSK_SEL_Q_CR` reader - 0: fetch the middle VOS_SR=VOS1-VOS2 0: first / 1: final"]
pub type DckVoskSelQCrR = crate::BitReader;
#[doc = "Field `DCK_VOSK_SEL_Q_CR` writer - 0: fetch the middle VOS_SR=VOS1-VOS2 0: first / 1: final"]
pub type DckVoskSelQCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_DEBUG_PATH_CR` reader - 1: DCK ready for debug"]
pub type DckDebugPathCrR = crate::BitReader;
#[doc = "Field `DCK_DEBUG_PATH_CR` writer - 1: DCK ready for debug"]
pub type DckDebugPathCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_DOS_FORCE_CR` reader - Force the DOS\\[5:0\\] from DOS_Manu_CR\\[5:0\\]"]
pub type DckDosForceCrR = crate::BitReader;
#[doc = "Field `DCK_DOS_FORCE_CR` writer - Force the DOS\\[5:0\\] from DOS_Manu_CR\\[5:0\\]"]
pub type DckDosForceCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_DOS_FORCE_I_CR` reader - Force the DOS_I\\[5:0\\] from DOS_Manu_I_CR\\[5:0\\]"]
pub type DckDosForceICrR = crate::BitReader;
#[doc = "Field `DCK_DOS_FORCE_I_CR` writer - Force the DOS_I\\[5:0\\] from DOS_Manu_I_CR\\[5:0\\]"]
pub type DckDosForceICrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_DOS_FORCE_Q_CR` reader - Force the DOS_Q\\[5:0\\] from DOS_Manu_Q_CR\\[5:0\\]"]
pub type DckDosForceQCrR = crate::BitReader;
#[doc = "Field `DCK_DOS_FORCE_Q_CR` writer - Force the DOS_Q\\[5:0\\] from DOS_Manu_Q_CR\\[5:0\\]"]
pub type DckDosForceQCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_KOS_EN_FORCE_CR` reader - Force the KOS_EN\\[5:0\\] from KOS_EN_Manu_CR\\[5:0\\]"]
pub type DckKosEnForceCrR = crate::BitReader;
#[doc = "Field `DCK_KOS_EN_FORCE_CR` writer - Force the KOS_EN\\[5:0\\] from KOS_EN_Manu_CR\\[5:0\\]"]
pub type DckKosEnForceCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_DOS_MANU_CR` reader - DOS\\[5:0\\] by manual"]
pub type DckDosManuCrR = crate::FieldReader;
#[doc = "Field `DCK_DOS_MANU_CR` writer - DOS\\[5:0\\] by manual"]
pub type DckDosManuCrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DCK_DOS_MANU_I_CR` reader - DOS_I\\[5:0\\] by manual"]
pub type DckDosManuICrR = crate::FieldReader;
#[doc = "Field `DCK_DOS_MANU_I_CR` writer - DOS_I\\[5:0\\] by manual"]
pub type DckDosManuICrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DCK_KOS_EN_FORCE_I_CR` reader - Force the KOS_EN_I\\[5:0\\] from KOS_EN_Manu_I_CR\\[5:0\\]"]
pub type DckKosEnForceICrR = crate::BitReader;
#[doc = "Field `DCK_KOS_EN_FORCE_I_CR` writer - Force the KOS_EN_I\\[5:0\\] from KOS_EN_Manu_I_CR\\[5:0\\]"]
pub type DckKosEnForceICrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_KOS_EN_FORCE_Q_CR` reader - Force the KOS_EN_Q\\[5:0\\] from KOS_EN_Manu_Q_CR\\[5:0\\]"]
pub type DckKosEnForceQCrR = crate::BitReader;
#[doc = "Field `DCK_KOS_EN_FORCE_Q_CR` writer - Force the KOS_EN_Q\\[5:0\\] from KOS_EN_Manu_Q_CR\\[5:0\\]"]
pub type DckKosEnForceQCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_FETCH_ADDR_CR` reader - Fetch the _AD-DATA 0: first 1: half-odd 2: half-even 3: final"]
pub type DckFetchAddrCrR = crate::FieldReader;
#[doc = "Field `DCK_FETCH_ADDR_CR` writer - Fetch the _AD-DATA 0: first 1: half-odd 2: half-even 3: final"]
pub type DckFetchAddrCrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - DC-offset calibration digital waiting clock cycle for setting time"]
    #[inline(always)]
    pub fn dck_wait_cyc_set_cr(&self) -> DckWaitCycSetCrR {
        DckWaitCycSetCrR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - DC-offset calibration fetch ADC data numbers to average"]
    #[inline(always)]
    pub fn dck_avg_cyc_cr(&self) -> DckAvgCycCrR {
        DckAvgCycCrR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Switch I2Q waiting time"]
    #[inline(always)]
    pub fn dck_switch_wait_iq_cr(&self) -> DckSwitchWaitIqCrR {
        DckSwitchWaitIqCrR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9 - 0: fetch the middle VOS_SR=VOS1-VOS2 0: first / 1: final"]
    #[inline(always)]
    pub fn dck_vosk_sel_i_cr(&self) -> DckVoskSelICrR {
        DckVoskSelICrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 0: fetch the middle VOS_SR=VOS1-VOS2 0: first / 1: final"]
    #[inline(always)]
    pub fn dck_vosk_sel_q_cr(&self) -> DckVoskSelQCrR {
        DckVoskSelQCrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: DCK ready for debug"]
    #[inline(always)]
    pub fn dck_debug_path_cr(&self) -> DckDebugPathCrR {
        DckDebugPathCrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Force the DOS\\[5:0\\] from DOS_Manu_CR\\[5:0\\]"]
    #[inline(always)]
    pub fn dck_dos_force_cr(&self) -> DckDosForceCrR {
        DckDosForceCrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Force the DOS_I\\[5:0\\] from DOS_Manu_I_CR\\[5:0\\]"]
    #[inline(always)]
    pub fn dck_dos_force_i_cr(&self) -> DckDosForceICrR {
        DckDosForceICrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Force the DOS_Q\\[5:0\\] from DOS_Manu_Q_CR\\[5:0\\]"]
    #[inline(always)]
    pub fn dck_dos_force_q_cr(&self) -> DckDosForceQCrR {
        DckDosForceQCrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Force the KOS_EN\\[5:0\\] from KOS_EN_Manu_CR\\[5:0\\]"]
    #[inline(always)]
    pub fn dck_kos_en_force_cr(&self) -> DckKosEnForceCrR {
        DckKosEnForceCrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - DOS\\[5:0\\] by manual"]
    #[inline(always)]
    pub fn dck_dos_manu_cr(&self) -> DckDosManuCrR {
        DckDosManuCrR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:27 - DOS_I\\[5:0\\] by manual"]
    #[inline(always)]
    pub fn dck_dos_manu_i_cr(&self) -> DckDosManuICrR {
        DckDosManuICrR::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bit 28 - Force the KOS_EN_I\\[5:0\\] from KOS_EN_Manu_I_CR\\[5:0\\]"]
    #[inline(always)]
    pub fn dck_kos_en_force_i_cr(&self) -> DckKosEnForceICrR {
        DckKosEnForceICrR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Force the KOS_EN_Q\\[5:0\\] from KOS_EN_Manu_Q_CR\\[5:0\\]"]
    #[inline(always)]
    pub fn dck_kos_en_force_q_cr(&self) -> DckKosEnForceQCrR {
        DckKosEnForceQCrR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Fetch the _AD-DATA 0: first 1: half-odd 2: half-even 3: final"]
    #[inline(always)]
    pub fn dck_fetch_addr_cr(&self) -> DckFetchAddrCrR {
        DckFetchAddrCrR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - DC-offset calibration digital waiting clock cycle for setting time"]
    #[inline(always)]
    pub fn dck_wait_cyc_set_cr(&mut self) -> DckWaitCycSetCrW<'_, RegAnaparDck1Spec> {
        DckWaitCycSetCrW::new(self, 0)
    }
    #[doc = "Bits 3:5 - DC-offset calibration fetch ADC data numbers to average"]
    #[inline(always)]
    pub fn dck_avg_cyc_cr(&mut self) -> DckAvgCycCrW<'_, RegAnaparDck1Spec> {
        DckAvgCycCrW::new(self, 3)
    }
    #[doc = "Bits 6:8 - Switch I2Q waiting time"]
    #[inline(always)]
    pub fn dck_switch_wait_iq_cr(&mut self) -> DckSwitchWaitIqCrW<'_, RegAnaparDck1Spec> {
        DckSwitchWaitIqCrW::new(self, 6)
    }
    #[doc = "Bit 9 - 0: fetch the middle VOS_SR=VOS1-VOS2 0: first / 1: final"]
    #[inline(always)]
    pub fn dck_vosk_sel_i_cr(&mut self) -> DckVoskSelICrW<'_, RegAnaparDck1Spec> {
        DckVoskSelICrW::new(self, 9)
    }
    #[doc = "Bit 10 - 0: fetch the middle VOS_SR=VOS1-VOS2 0: first / 1: final"]
    #[inline(always)]
    pub fn dck_vosk_sel_q_cr(&mut self) -> DckVoskSelQCrW<'_, RegAnaparDck1Spec> {
        DckVoskSelQCrW::new(self, 10)
    }
    #[doc = "Bit 11 - 1: DCK ready for debug"]
    #[inline(always)]
    pub fn dck_debug_path_cr(&mut self) -> DckDebugPathCrW<'_, RegAnaparDck1Spec> {
        DckDebugPathCrW::new(self, 11)
    }
    #[doc = "Bit 12 - Force the DOS\\[5:0\\] from DOS_Manu_CR\\[5:0\\]"]
    #[inline(always)]
    pub fn dck_dos_force_cr(&mut self) -> DckDosForceCrW<'_, RegAnaparDck1Spec> {
        DckDosForceCrW::new(self, 12)
    }
    #[doc = "Bit 13 - Force the DOS_I\\[5:0\\] from DOS_Manu_I_CR\\[5:0\\]"]
    #[inline(always)]
    pub fn dck_dos_force_i_cr(&mut self) -> DckDosForceICrW<'_, RegAnaparDck1Spec> {
        DckDosForceICrW::new(self, 13)
    }
    #[doc = "Bit 14 - Force the DOS_Q\\[5:0\\] from DOS_Manu_Q_CR\\[5:0\\]"]
    #[inline(always)]
    pub fn dck_dos_force_q_cr(&mut self) -> DckDosForceQCrW<'_, RegAnaparDck1Spec> {
        DckDosForceQCrW::new(self, 14)
    }
    #[doc = "Bit 15 - Force the KOS_EN\\[5:0\\] from KOS_EN_Manu_CR\\[5:0\\]"]
    #[inline(always)]
    pub fn dck_kos_en_force_cr(&mut self) -> DckKosEnForceCrW<'_, RegAnaparDck1Spec> {
        DckKosEnForceCrW::new(self, 15)
    }
    #[doc = "Bits 16:21 - DOS\\[5:0\\] by manual"]
    #[inline(always)]
    pub fn dck_dos_manu_cr(&mut self) -> DckDosManuCrW<'_, RegAnaparDck1Spec> {
        DckDosManuCrW::new(self, 16)
    }
    #[doc = "Bits 22:27 - DOS_I\\[5:0\\] by manual"]
    #[inline(always)]
    pub fn dck_dos_manu_i_cr(&mut self) -> DckDosManuICrW<'_, RegAnaparDck1Spec> {
        DckDosManuICrW::new(self, 22)
    }
    #[doc = "Bit 28 - Force the KOS_EN_I\\[5:0\\] from KOS_EN_Manu_I_CR\\[5:0\\]"]
    #[inline(always)]
    pub fn dck_kos_en_force_i_cr(&mut self) -> DckKosEnForceICrW<'_, RegAnaparDck1Spec> {
        DckKosEnForceICrW::new(self, 28)
    }
    #[doc = "Bit 29 - Force the KOS_EN_Q\\[5:0\\] from KOS_EN_Manu_Q_CR\\[5:0\\]"]
    #[inline(always)]
    pub fn dck_kos_en_force_q_cr(&mut self) -> DckKosEnForceQCrW<'_, RegAnaparDck1Spec> {
        DckKosEnForceQCrW::new(self, 29)
    }
    #[doc = "Bits 30:31 - Fetch the _AD-DATA 0: first 1: half-odd 2: half-even 3: final"]
    #[inline(always)]
    pub fn dck_fetch_addr_cr(&mut self) -> DckFetchAddrCrW<'_, RegAnaparDck1Spec> {
        DckFetchAddrCrW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_anapar_dck_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_anapar_dck_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAnaparDck1Spec;
impl crate::RegisterSpec for RegAnaparDck1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_anapar_dck_1::R`](R) reader structure"]
impl crate::Readable for RegAnaparDck1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_anapar_dck_1::W`](W) writer structure"]
impl crate::Writable for RegAnaparDck1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ANAPAR_DCK_1 to value 0x3f"]
impl crate::Resettable for RegAnaparDck1Spec {
    const RESET_VALUE: u32 = 0x3f;
}
