#[doc = "Register `REG_AX_PWR_BF_CTRL` reader"]
pub type R = crate::R<RegAxPwrBfCtrlSpec>;
#[doc = "Register `REG_AX_PWR_BF_CTRL` writer"]
pub type W = crate::W<RegAxPwrBfCtrlSpec>;
#[doc = "Field `r_force_pwr_by_limit_bfon_value` reader - S(9.2) r_force_pwr_by_limit_BF1_en=1 is valid"]
pub type RForcePwrByLimitBfonValueR = crate::FieldReader<u16>;
#[doc = "Field `r_force_pwr_by_limit_bfon_value` writer - S(9.2) r_force_pwr_by_limit_BF1_en=1 is valid"]
pub type RForcePwrByLimitBfonValueW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `r_force_pwr_by_limit_bfoff_value` reader - S(9.2) r_force_pwr_by_limit_BF0_en=1 is valid"]
pub type RForcePwrByLimitBfoffValueR = crate::FieldReader<u16>;
#[doc = "Field `r_force_pwr_by_limit_bfoff_value` writer - S(9.2) r_force_pwr_by_limit_BF0_en=1 is valid"]
pub type RForcePwrByLimitBfoffValueW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `r_HE_ER_SU_pwr_reduce_en` reader - 1:HE_ER_SU pwr lim output will minus 3dB"]
pub type RHeErSuPwrReduceEnR = crate::BitReader;
#[doc = "Field `r_HE_ER_SU_pwr_reduce_en` writer - 1:HE_ER_SU pwr lim output will minus 3dB"]
pub type RHeErSuPwrReduceEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_HE_ER_SU_pwr_reduce_val` reader - (RTL_only) S(6.2) 3dB=0xC"]
pub type RHeErSuPwrReduceValR = crate::FieldReader;
#[doc = "Field `r_HE_ER_SU_pwr_reduce_val` writer - (RTL_only) S(6.2) 3dB=0xC"]
pub type RHeErSuPwrReduceValW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `r_txagc_BF_pwr_boost_en` reader - 1:TxBF will cal pwr boost to BB 0:TxBF pwr_boost =0 & Pwr_lim use BF1_pwr_lim"]
pub type RTxagcBfPwrBoostEnR = crate::BitReader;
#[doc = "Field `r_txagc_BF_pwr_boost_en` writer - 1:TxBF will cal pwr boost to BB 0:TxBF pwr_boost =0 & Pwr_lim use BF1_pwr_lim"]
pub type RTxagcBfPwrBoostEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - S(9.2) r_force_pwr_by_limit_BF1_en=1 is valid"]
    #[inline(always)]
    pub fn r_force_pwr_by_limit_bfon_value(&self) -> RForcePwrByLimitBfonValueR {
        RForcePwrByLimitBfonValueR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - S(9.2) r_force_pwr_by_limit_BF0_en=1 is valid"]
    #[inline(always)]
    pub fn r_force_pwr_by_limit_bfoff_value(&self) -> RForcePwrByLimitBfoffValueR {
        RForcePwrByLimitBfoffValueR::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bit 18 - 1:HE_ER_SU pwr lim output will minus 3dB"]
    #[inline(always)]
    pub fn r_he_er_su_pwr_reduce_en(&self) -> RHeErSuPwrReduceEnR {
        RHeErSuPwrReduceEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:24 - (RTL_only) S(6.2) 3dB=0xC"]
    #[inline(always)]
    pub fn r_he_er_su_pwr_reduce_val(&self) -> RHeErSuPwrReduceValR {
        RHeErSuPwrReduceValR::new(((self.bits >> 19) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - 1:TxBF will cal pwr boost to BB 0:TxBF pwr_boost =0 & Pwr_lim use BF1_pwr_lim"]
    #[inline(always)]
    pub fn r_txagc_bf_pwr_boost_en(&self) -> RTxagcBfPwrBoostEnR {
        RTxagcBfPwrBoostEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - S(9.2) r_force_pwr_by_limit_BF1_en=1 is valid"]
    #[inline(always)]
    pub fn r_force_pwr_by_limit_bfon_value(
        &mut self,
    ) -> RForcePwrByLimitBfonValueW<'_, RegAxPwrBfCtrlSpec> {
        RForcePwrByLimitBfonValueW::new(self, 0)
    }
    #[doc = "Bits 9:17 - S(9.2) r_force_pwr_by_limit_BF0_en=1 is valid"]
    #[inline(always)]
    pub fn r_force_pwr_by_limit_bfoff_value(
        &mut self,
    ) -> RForcePwrByLimitBfoffValueW<'_, RegAxPwrBfCtrlSpec> {
        RForcePwrByLimitBfoffValueW::new(self, 9)
    }
    #[doc = "Bit 18 - 1:HE_ER_SU pwr lim output will minus 3dB"]
    #[inline(always)]
    pub fn r_he_er_su_pwr_reduce_en(&mut self) -> RHeErSuPwrReduceEnW<'_, RegAxPwrBfCtrlSpec> {
        RHeErSuPwrReduceEnW::new(self, 18)
    }
    #[doc = "Bits 19:24 - (RTL_only) S(6.2) 3dB=0xC"]
    #[inline(always)]
    pub fn r_he_er_su_pwr_reduce_val(&mut self) -> RHeErSuPwrReduceValW<'_, RegAxPwrBfCtrlSpec> {
        RHeErSuPwrReduceValW::new(self, 19)
    }
    #[doc = "Bit 31 - 1:TxBF will cal pwr boost to BB 0:TxBF pwr_boost =0 & Pwr_lim use BF1_pwr_lim"]
    #[inline(always)]
    pub fn r_txagc_bf_pwr_boost_en(&mut self) -> RTxagcBfPwrBoostEnW<'_, RegAxPwrBfCtrlSpec> {
        RTxagcBfPwrBoostEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_bf_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_bf_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrBfCtrlSpec;
impl crate::RegisterSpec for RegAxPwrBfCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_bf_ctrl::R`](R) reader structure"]
impl crate::Readable for RegAxPwrBfCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_bf_ctrl::W`](W) writer structure"]
impl crate::Writable for RegAxPwrBfCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_BF_CTRL to value 0x8060_0000"]
impl crate::Resettable for RegAxPwrBfCtrlSpec {
    const RESET_VALUE: u32 = 0x8060_0000;
}
