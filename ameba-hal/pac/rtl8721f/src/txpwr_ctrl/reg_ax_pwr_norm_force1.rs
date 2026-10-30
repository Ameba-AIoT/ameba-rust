#[doc = "Register `REG_AX_PWR_NORM_FORCE1` reader"]
pub type R = crate::R<RegAxPwrNormForce1Spec>;
#[doc = "Register `REG_AX_PWR_NORM_FORCE1` writer"]
pub type W = crate::W<RegAxPwrNormForce1Spec>;
#[doc = "Field `r_force_pwr_mode_value` reader - (RTL_only) Force input from MAC"]
pub type RForcePwrModeValueR = crate::FieldReader;
#[doc = "Field `r_force_pwr_mode_value` writer - (RTL_only) Force input from MAC"]
pub type RForcePwrModeValueW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_force_pwr_mode_en` reader - (RTL_only) Force input from MAC en"]
pub type RForcePwrModeEnR = crate::BitReader;
#[doc = "Field `r_force_pwr_mode_en` writer - (RTL_only) Force input from MAC en"]
pub type RForcePwrModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_ntx_value` reader - (RTL_only) Force input from MAC"]
pub type RForceNtxValueR = crate::BitReader;
#[doc = "Field `r_force_ntx_value` writer - (RTL_only) Force input from MAC"]
pub type RForceNtxValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_ntx_en` reader - (RTL_only) Force input from MAC en"]
pub type RForceNtxEnR = crate::BitReader;
#[doc = "Field `r_force_ntx_en` writer - (RTL_only) Force input from MAC en"]
pub type RForceNtxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_txsc_value` reader - (RTL_only) Force input from MAC"]
pub type RForceTxscValueR = crate::FieldReader;
#[doc = "Field `r_force_txsc_value` writer - (RTL_only) Force input from MAC"]
pub type RForceTxscValueW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_force_txsc_en` reader - (RTL_only) Force input from MAC en"]
pub type RForceTxscEnR = crate::BitReader;
#[doc = "Field `r_force_txsc_en` writer - (RTL_only) Force input from MAC en"]
pub type RForceTxscEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_TxBF_en_value` reader - (RTL_only) Force input from MAC"]
pub type RForceTxBfEnValueR = crate::BitReader;
#[doc = "Field `r_force_TxBF_en_value` writer - (RTL_only) Force input from MAC"]
pub type RForceTxBfEnValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_TxBF_en_en` reader - (RTL_only) Force input from MAC en"]
pub type RForceTxBfEnEnR = crate::BitReader;
#[doc = "Field `r_force_TxBF_en_en` writer - (RTL_only) Force input from MAC en"]
pub type RForceTxBfEnEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_Rx_LTE_value` reader - (RTL_only) Force input from MAC"]
pub type RForceRxLteValueR = crate::BitReader;
#[doc = "Field `r_force_Rx_LTE_value` writer - (RTL_only) Force input from MAC"]
pub type RForceRxLteValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_Rx_LTE_en` reader - (RTL_only) Force input from MAC en"]
pub type RForceRxLteEnR = crate::BitReader;
#[doc = "Field `r_force_Rx_LTE_en` writer - (RTL_only) Force input from MAC en"]
pub type RForceRxLteEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_BT_grant_value` reader - (RTL_only) Force input from MAC"]
pub type RForceBtGrantValueR = crate::BitReader;
#[doc = "Field `r_force_BT_grant_value` writer - (RTL_only) Force input from MAC"]
pub type RForceBtGrantValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_BT_grant_en` reader - (RTL_only) Force input from MAC en"]
pub type RForceBtGrantEnR = crate::BitReader;
#[doc = "Field `r_force_BT_grant_en` writer - (RTL_only) Force input from MAC en"]
pub type RForceBtGrantEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_macid_CCA_TH_en_value` reader - (RTL_only) Force input from MAC"]
pub type RForceMacidCcaThEnValueR = crate::BitReader;
#[doc = "Field `r_force_macid_CCA_TH_en_value` writer - (RTL_only) Force input from MAC"]
pub type RForceMacidCcaThEnValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_macid_CCA_TH_en_en` reader - (RTL_only) Force input from MAC en"]
pub type RForceMacidCcaThEnEnR = crate::BitReader;
#[doc = "Field `r_force_macid_CCA_TH_en_en` writer - (RTL_only) Force input from MAC en"]
pub type RForceMacidCcaThEnEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_HE_ER_SU_en_value` reader - (RTL_only) Force input from MAC"]
pub type RForceHeErSuEnValueR = crate::BitReader;
#[doc = "Field `r_force_HE_ER_SU_en_value` writer - (RTL_only) Force input from MAC"]
pub type RForceHeErSuEnValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_HE_ER_SU_en_en` reader - (RTL_only) Force input from MAC en"]
pub type RForceHeErSuEnEnR = crate::BitReader;
#[doc = "Field `r_force_HE_ER_SU_en_en` writer - (RTL_only) Force input from MAC en"]
pub type RForceHeErSuEnEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_txagc_BF_pwr_boost_force_val` reader - (RTL_only) Force input from MAC"]
pub type RTxagcBfPwrBoostForceValR = crate::FieldReader;
#[doc = "Field `r_txagc_BF_pwr_boost_force_val` writer - (RTL_only) Force input from MAC"]
pub type RTxagcBfPwrBoostForceValW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `r_txagc_BF_pwr_boost_force_val_en` reader - (RTL_only) Force input from MAC en"]
pub type RTxagcBfPwrBoostForceValEnR = crate::BitReader;
#[doc = "Field `r_txagc_BF_pwr_boost_force_val_en` writer - (RTL_only) Force input from MAC en"]
pub type RTxagcBfPwrBoostForceValEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_pwr_mode_value(&self) -> RForcePwrModeValueR {
        RForcePwrModeValueR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_pwr_mode_en(&self) -> RForcePwrModeEnR {
        RForcePwrModeEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_ntx_value(&self) -> RForceNtxValueR {
        RForceNtxValueR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_ntx_en(&self) -> RForceNtxEnR {
        RForceNtxEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 9:12 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_txsc_value(&self) -> RForceTxscValueR {
        RForceTxscValueR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_txsc_en(&self) -> RForceTxscEnR {
        RForceTxscEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_tx_bf_en_value(&self) -> RForceTxBfEnValueR {
        RForceTxBfEnValueR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_tx_bf_en_en(&self) -> RForceTxBfEnEnR {
        RForceTxBfEnEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_rx_lte_value(&self) -> RForceRxLteValueR {
        RForceRxLteValueR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_rx_lte_en(&self) -> RForceRxLteEnR {
        RForceRxLteEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_bt_grant_value(&self) -> RForceBtGrantValueR {
        RForceBtGrantValueR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_bt_grant_en(&self) -> RForceBtGrantEnR {
        RForceBtGrantEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_macid_cca_th_en_value(&self) -> RForceMacidCcaThEnValueR {
        RForceMacidCcaThEnValueR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_macid_cca_th_en_en(&self) -> RForceMacidCcaThEnEnR {
        RForceMacidCcaThEnEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_he_er_su_en_value(&self) -> RForceHeErSuEnValueR {
        RForceHeErSuEnValueR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_he_er_su_en_en(&self) -> RForceHeErSuEnEnR {
        RForceHeErSuEnEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:28 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_txagc_bf_pwr_boost_force_val(&self) -> RTxagcBfPwrBoostForceValR {
        RTxagcBfPwrBoostForceValR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_txagc_bf_pwr_boost_force_val_en(&self) -> RTxagcBfPwrBoostForceValEnR {
        RTxagcBfPwrBoostForceValEnR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_pwr_mode_value(&mut self) -> RForcePwrModeValueW<'_, RegAxPwrNormForce1Spec> {
        RForcePwrModeValueW::new(self, 0)
    }
    #[doc = "Bit 3 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_pwr_mode_en(&mut self) -> RForcePwrModeEnW<'_, RegAxPwrNormForce1Spec> {
        RForcePwrModeEnW::new(self, 3)
    }
    #[doc = "Bit 5 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_ntx_value(&mut self) -> RForceNtxValueW<'_, RegAxPwrNormForce1Spec> {
        RForceNtxValueW::new(self, 5)
    }
    #[doc = "Bit 6 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_ntx_en(&mut self) -> RForceNtxEnW<'_, RegAxPwrNormForce1Spec> {
        RForceNtxEnW::new(self, 6)
    }
    #[doc = "Bits 9:12 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_txsc_value(&mut self) -> RForceTxscValueW<'_, RegAxPwrNormForce1Spec> {
        RForceTxscValueW::new(self, 9)
    }
    #[doc = "Bit 13 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_txsc_en(&mut self) -> RForceTxscEnW<'_, RegAxPwrNormForce1Spec> {
        RForceTxscEnW::new(self, 13)
    }
    #[doc = "Bit 14 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_tx_bf_en_value(&mut self) -> RForceTxBfEnValueW<'_, RegAxPwrNormForce1Spec> {
        RForceTxBfEnValueW::new(self, 14)
    }
    #[doc = "Bit 15 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_tx_bf_en_en(&mut self) -> RForceTxBfEnEnW<'_, RegAxPwrNormForce1Spec> {
        RForceTxBfEnEnW::new(self, 15)
    }
    #[doc = "Bit 16 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_rx_lte_value(&mut self) -> RForceRxLteValueW<'_, RegAxPwrNormForce1Spec> {
        RForceRxLteValueW::new(self, 16)
    }
    #[doc = "Bit 17 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_rx_lte_en(&mut self) -> RForceRxLteEnW<'_, RegAxPwrNormForce1Spec> {
        RForceRxLteEnW::new(self, 17)
    }
    #[doc = "Bit 18 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_bt_grant_value(&mut self) -> RForceBtGrantValueW<'_, RegAxPwrNormForce1Spec> {
        RForceBtGrantValueW::new(self, 18)
    }
    #[doc = "Bit 19 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_bt_grant_en(&mut self) -> RForceBtGrantEnW<'_, RegAxPwrNormForce1Spec> {
        RForceBtGrantEnW::new(self, 19)
    }
    #[doc = "Bit 20 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_macid_cca_th_en_value(
        &mut self,
    ) -> RForceMacidCcaThEnValueW<'_, RegAxPwrNormForce1Spec> {
        RForceMacidCcaThEnValueW::new(self, 20)
    }
    #[doc = "Bit 21 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_macid_cca_th_en_en(
        &mut self,
    ) -> RForceMacidCcaThEnEnW<'_, RegAxPwrNormForce1Spec> {
        RForceMacidCcaThEnEnW::new(self, 21)
    }
    #[doc = "Bit 22 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_he_er_su_en_value(
        &mut self,
    ) -> RForceHeErSuEnValueW<'_, RegAxPwrNormForce1Spec> {
        RForceHeErSuEnValueW::new(self, 22)
    }
    #[doc = "Bit 23 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_he_er_su_en_en(&mut self) -> RForceHeErSuEnEnW<'_, RegAxPwrNormForce1Spec> {
        RForceHeErSuEnEnW::new(self, 23)
    }
    #[doc = "Bits 24:28 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_txagc_bf_pwr_boost_force_val(
        &mut self,
    ) -> RTxagcBfPwrBoostForceValW<'_, RegAxPwrNormForce1Spec> {
        RTxagcBfPwrBoostForceValW::new(self, 24)
    }
    #[doc = "Bit 29 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_txagc_bf_pwr_boost_force_val_en(
        &mut self,
    ) -> RTxagcBfPwrBoostForceValEnW<'_, RegAxPwrNormForce1Spec> {
        RTxagcBfPwrBoostForceValEnW::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_norm_force1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_norm_force1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrNormForce1Spec;
impl crate::RegisterSpec for RegAxPwrNormForce1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_norm_force1::R`](R) reader structure"]
impl crate::Readable for RegAxPwrNormForce1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_norm_force1::W`](W) writer structure"]
impl crate::Writable for RegAxPwrNormForce1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_NORM_FORCE1 to value 0"]
impl crate::Resettable for RegAxPwrNormForce1Spec {}
