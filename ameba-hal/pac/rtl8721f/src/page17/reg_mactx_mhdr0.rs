#[doc = "Register `REG_MACTX_MHDR0` reader"]
pub type R = crate::R<RegMactxMhdr0Spec>;
#[doc = "Register `REG_MACTX_MHDR0` writer"]
pub type W = crate::W<RegMactxMhdr0Spec>;
#[doc = "Field `r_BSR_BK_TID_SEL` reader - 1: tid 3; 0: tid 0;"]
pub type RBsrBkTidSelR = crate::BitReader;
#[doc = "Field `r_BSR_BK_TID_SEL` writer - 1: tid 3; 0: tid 0;"]
pub type RBsrBkTidSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_BSR_BE_TID_SEL` reader - 1: tid 2; 0: tid 1;"]
pub type RBsrBeTidSelR = crate::BitReader;
#[doc = "Field `r_BSR_BE_TID_SEL` writer - 1: tid 2; 0: tid 1;"]
pub type RBsrBeTidSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_BSR_VI_TID_SEL` reader - 1: tid 5; 0: tid 4;"]
pub type RBsrViTidSelR = crate::BitReader;
#[doc = "Field `r_BSR_VI_TID_SEL` writer - 1: tid 5; 0: tid 4;"]
pub type RBsrViTidSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_BSR_VO_TID_SEL` reader - 1: tid 7; 0: tid 6;"]
pub type RBsrVoTidSelR = crate::BitReader;
#[doc = "Field `r_BSR_VO_TID_SEL` writer - 1: tid 7; 0: tid 6;"]
pub type RBsrVoTidSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_BSR_QOS_SEL` reader - Buffer status report enable"]
pub type RBsrQosSelR = crate::BitReader;
#[doc = "Field `r_BSR_QOS_SEL` writer - Buffer status report enable"]
pub type RBsrQosSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_BQR_TID_VAL` reader - bandwidth query report TID value"]
pub type RBqrTidValR = crate::FieldReader;
#[doc = "Field `r_BQR_TID_VAL` writer - bandwidth query report TID value"]
pub type RBqrTidValW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_BQR_PRIMCH` reader - shift bqr bitmap to the allocatied primary 20M channel"]
pub type RBqrPrimchR = crate::FieldReader;
#[doc = "Field `r_BQR_PRIMCH` writer - shift bqr bitmap to the allocatied primary 20M channel"]
pub type RBqrPrimchW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_WMAC_NON_LEGACY_USTIMER` reader - timer cnt before bb fetch txd for he/vht/ht/cck"]
pub type RWmacNonLegacyUstimerR = crate::FieldReader;
#[doc = "Field `r_WMAC_NON_LEGACY_USTIMER` writer - timer cnt before bb fetch txd for he/vht/ht/cck"]
pub type RWmacNonLegacyUstimerW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `r_WMAC_LEGACY_USTIMER` reader - timer cnt before bb fetch txd for ofdm"]
pub type RWmacLegacyUstimerR = crate::FieldReader;
#[doc = "Field `r_WMAC_LEGACY_USTIMER` writer - timer cnt before bb fetch txd for ofdm"]
pub type RWmacLegacyUstimerW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `r_QOS_NULL_TID_VAL` reader - tid value of qos null"]
pub type RQosNullTidValR = crate::FieldReader;
#[doc = "Field `r_QOS_NULL_TID_VAL` writer - tid value of qos null"]
pub type RQosNullTidValW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_TCR_BA_RSVD_VAL` reader - multi-sta BA bactl rsvd filed value"]
pub type RTcrBaRsvdValR = crate::BitReader;
#[doc = "Field `r_TCR_BA_RSVD_VAL` writer - multi-sta BA bactl rsvd filed value"]
pub type RTcrBaRsvdValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_WMAC_MHDR_UPH_EN` reader - eanble reg wirte HTC field of tb_ppdu, which contain UPH a-control(4)"]
pub type RWmacMhdrUphEnR = crate::BitReader;
#[doc = "Field `r_WMAC_MHDR_UPH_EN` writer - eanble reg wirte HTC field of tb_ppdu, which contain UPH a-control(4)"]
pub type RWmacMhdrUphEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_TB_QOSNULL_PWRMGT_VAL` reader - the power management bit value in tb qos null, used during twt operation"]
pub type RTbQosnullPwrmgtValR = crate::BitReader;
#[doc = "Field `r_TB_QOSNULL_PWRMGT_VAL` writer - the power management bit value in tb qos null, used during twt operation"]
pub type RTbQosnullPwrmgtValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_WMAC_EN_HW_SSN` reader - enable the hw ssn for qos null frame."]
pub type RWmacEnHwSsnR = crate::BitReader;
#[doc = "Field `r_WMAC_EN_HW_SSN` writer - enable the hw ssn for qos null frame."]
pub type RWmacEnHwSsnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_QNULL_ACK_POL_VAL` reader - basic trigger resp tb QOS NULL ack policy bit 0 value"]
pub type RQnullAckPolValR = crate::BitReader;
#[doc = "Field `r_QNULL_ACK_POL_VAL` writer - basic trigger resp tb QOS NULL ack policy bit 0 value"]
pub type RQnullAckPolValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_UPH_CAS_QOS_SEL` reader - UPH and CAS HTC control field enable"]
pub type RUphCasQosSelR = crate::BitReader;
#[doc = "Field `r_UPH_CAS_QOS_SEL` writer - UPH and CAS HTC control field enable"]
pub type RUphCasQosSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: tid 3; 0: tid 0;"]
    #[inline(always)]
    pub fn r_bsr_bk_tid_sel(&self) -> RBsrBkTidSelR {
        RBsrBkTidSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: tid 2; 0: tid 1;"]
    #[inline(always)]
    pub fn r_bsr_be_tid_sel(&self) -> RBsrBeTidSelR {
        RBsrBeTidSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: tid 5; 0: tid 4;"]
    #[inline(always)]
    pub fn r_bsr_vi_tid_sel(&self) -> RBsrViTidSelR {
        RBsrViTidSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: tid 7; 0: tid 6;"]
    #[inline(always)]
    pub fn r_bsr_vo_tid_sel(&self) -> RBsrVoTidSelR {
        RBsrVoTidSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer status report enable"]
    #[inline(always)]
    pub fn r_bsr_qos_sel(&self) -> RBsrQosSelR {
        RBsrQosSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - bandwidth query report TID value"]
    #[inline(always)]
    pub fn r_bqr_tid_val(&self) -> RBqrTidValR {
        RBqrTidValR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:11 - shift bqr bitmap to the allocatied primary 20M channel"]
    #[inline(always)]
    pub fn r_bqr_primch(&self) -> RBqrPrimchR {
        RBqrPrimchR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:16 - timer cnt before bb fetch txd for he/vht/ht/cck"]
    #[inline(always)]
    pub fn r_wmac_non_legacy_ustimer(&self) -> RWmacNonLegacyUstimerR {
        RWmacNonLegacyUstimerR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21 - timer cnt before bb fetch txd for ofdm"]
    #[inline(always)]
    pub fn r_wmac_legacy_ustimer(&self) -> RWmacLegacyUstimerR {
        RWmacLegacyUstimerR::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:25 - tid value of qos null"]
    #[inline(always)]
    pub fn r_qos_null_tid_val(&self) -> RQosNullTidValR {
        RQosNullTidValR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 26 - multi-sta BA bactl rsvd filed value"]
    #[inline(always)]
    pub fn r_tcr_ba_rsvd_val(&self) -> RTcrBaRsvdValR {
        RTcrBaRsvdValR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - eanble reg wirte HTC field of tb_ppdu, which contain UPH a-control(4)"]
    #[inline(always)]
    pub fn r_wmac_mhdr_uph_en(&self) -> RWmacMhdrUphEnR {
        RWmacMhdrUphEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - the power management bit value in tb qos null, used during twt operation"]
    #[inline(always)]
    pub fn r_tb_qosnull_pwrmgt_val(&self) -> RTbQosnullPwrmgtValR {
        RTbQosnullPwrmgtValR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - enable the hw ssn for qos null frame."]
    #[inline(always)]
    pub fn r_wmac_en_hw_ssn(&self) -> RWmacEnHwSsnR {
        RWmacEnHwSsnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - basic trigger resp tb QOS NULL ack policy bit 0 value"]
    #[inline(always)]
    pub fn r_qnull_ack_pol_val(&self) -> RQnullAckPolValR {
        RQnullAckPolValR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UPH and CAS HTC control field enable"]
    #[inline(always)]
    pub fn r_uph_cas_qos_sel(&self) -> RUphCasQosSelR {
        RUphCasQosSelR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: tid 3; 0: tid 0;"]
    #[inline(always)]
    pub fn r_bsr_bk_tid_sel(&mut self) -> RBsrBkTidSelW<'_, RegMactxMhdr0Spec> {
        RBsrBkTidSelW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: tid 2; 0: tid 1;"]
    #[inline(always)]
    pub fn r_bsr_be_tid_sel(&mut self) -> RBsrBeTidSelW<'_, RegMactxMhdr0Spec> {
        RBsrBeTidSelW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: tid 5; 0: tid 4;"]
    #[inline(always)]
    pub fn r_bsr_vi_tid_sel(&mut self) -> RBsrViTidSelW<'_, RegMactxMhdr0Spec> {
        RBsrViTidSelW::new(self, 2)
    }
    #[doc = "Bit 3 - 1: tid 7; 0: tid 6;"]
    #[inline(always)]
    pub fn r_bsr_vo_tid_sel(&mut self) -> RBsrVoTidSelW<'_, RegMactxMhdr0Spec> {
        RBsrVoTidSelW::new(self, 3)
    }
    #[doc = "Bit 4 - Buffer status report enable"]
    #[inline(always)]
    pub fn r_bsr_qos_sel(&mut self) -> RBsrQosSelW<'_, RegMactxMhdr0Spec> {
        RBsrQosSelW::new(self, 4)
    }
    #[doc = "Bits 5:8 - bandwidth query report TID value"]
    #[inline(always)]
    pub fn r_bqr_tid_val(&mut self) -> RBqrTidValW<'_, RegMactxMhdr0Spec> {
        RBqrTidValW::new(self, 5)
    }
    #[doc = "Bits 9:11 - shift bqr bitmap to the allocatied primary 20M channel"]
    #[inline(always)]
    pub fn r_bqr_primch(&mut self) -> RBqrPrimchW<'_, RegMactxMhdr0Spec> {
        RBqrPrimchW::new(self, 9)
    }
    #[doc = "Bits 12:16 - timer cnt before bb fetch txd for he/vht/ht/cck"]
    #[inline(always)]
    pub fn r_wmac_non_legacy_ustimer(&mut self) -> RWmacNonLegacyUstimerW<'_, RegMactxMhdr0Spec> {
        RWmacNonLegacyUstimerW::new(self, 12)
    }
    #[doc = "Bits 17:21 - timer cnt before bb fetch txd for ofdm"]
    #[inline(always)]
    pub fn r_wmac_legacy_ustimer(&mut self) -> RWmacLegacyUstimerW<'_, RegMactxMhdr0Spec> {
        RWmacLegacyUstimerW::new(self, 17)
    }
    #[doc = "Bits 22:25 - tid value of qos null"]
    #[inline(always)]
    pub fn r_qos_null_tid_val(&mut self) -> RQosNullTidValW<'_, RegMactxMhdr0Spec> {
        RQosNullTidValW::new(self, 22)
    }
    #[doc = "Bit 26 - multi-sta BA bactl rsvd filed value"]
    #[inline(always)]
    pub fn r_tcr_ba_rsvd_val(&mut self) -> RTcrBaRsvdValW<'_, RegMactxMhdr0Spec> {
        RTcrBaRsvdValW::new(self, 26)
    }
    #[doc = "Bit 27 - eanble reg wirte HTC field of tb_ppdu, which contain UPH a-control(4)"]
    #[inline(always)]
    pub fn r_wmac_mhdr_uph_en(&mut self) -> RWmacMhdrUphEnW<'_, RegMactxMhdr0Spec> {
        RWmacMhdrUphEnW::new(self, 27)
    }
    #[doc = "Bit 28 - the power management bit value in tb qos null, used during twt operation"]
    #[inline(always)]
    pub fn r_tb_qosnull_pwrmgt_val(&mut self) -> RTbQosnullPwrmgtValW<'_, RegMactxMhdr0Spec> {
        RTbQosnullPwrmgtValW::new(self, 28)
    }
    #[doc = "Bit 29 - enable the hw ssn for qos null frame."]
    #[inline(always)]
    pub fn r_wmac_en_hw_ssn(&mut self) -> RWmacEnHwSsnW<'_, RegMactxMhdr0Spec> {
        RWmacEnHwSsnW::new(self, 29)
    }
    #[doc = "Bit 30 - basic trigger resp tb QOS NULL ack policy bit 0 value"]
    #[inline(always)]
    pub fn r_qnull_ack_pol_val(&mut self) -> RQnullAckPolValW<'_, RegMactxMhdr0Spec> {
        RQnullAckPolValW::new(self, 30)
    }
    #[doc = "Bit 31 - UPH and CAS HTC control field enable"]
    #[inline(always)]
    pub fn r_uph_cas_qos_sel(&mut self) -> RUphCasQosSelW<'_, RegMactxMhdr0Spec> {
        RUphCasQosSelW::new(self, 31)
    }
}
#[doc = "MAC header configure\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mactx_mhdr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mactx_mhdr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMactxMhdr0Spec;
impl crate::RegisterSpec for RegMactxMhdr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mactx_mhdr0::R`](R) reader structure"]
impl crate::Readable for RegMactxMhdr0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_mactx_mhdr0::W`](W) writer structure"]
impl crate::Writable for RegMactxMhdr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MACTX_MHDR0 to value 0x6014_d010"]
impl crate::Resettable for RegMactxMhdr0Spec {
    const RESET_VALUE: u32 = 0x6014_d010;
}
