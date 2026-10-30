#[doc = "Register `REG_AX_PWR_RSVD8` reader"]
pub type R = crate::R<RegAxPwrRsvd8Spec>;
#[doc = "Register `REG_AX_PWR_RSVD8` writer"]
pub type W = crate::W<RegAxPwrRsvd8Spec>;
#[doc = "Field `r_ap_tx_pwr_bias_norm_resp` reader - "]
pub type RApTxPwrBiasNormRespR = crate::FieldReader;
#[doc = "Field `r_ap_tx_pwr_bias_norm_resp` writer - "]
pub type RApTxPwrBiasNormRespW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_ap_tx_pwr_bias_sta_ul` reader - "]
pub type RApTxPwrBiasStaUlR = crate::FieldReader;
#[doc = "Field `r_ap_tx_pwr_bias_sta_ul` writer - "]
pub type RApTxPwrBiasStaUlW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_DBW20_RU26_alloc_lv0` reader - txpwr by ru limit RU_id level 0 when DBW20M and RU26"]
pub type RDbw20Ru26AllocLv0R = crate::FieldReader;
#[doc = "Field `r_DBW20_RU26_alloc_lv0` writer - txpwr by ru limit RU_id level 0 when DBW20M and RU26"]
pub type RDbw20Ru26AllocLv0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_DBW20_RU26_alloc_lv1` reader - txpwr by ru limit RU_id level 1 when DBW20M and RU26"]
pub type RDbw20Ru26AllocLv1R = crate::FieldReader;
#[doc = "Field `r_DBW20_RU26_alloc_lv1` writer - txpwr by ru limit RU_id level 1 when DBW20M and RU26"]
pub type RDbw20Ru26AllocLv1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_DBW20_RU52_alloc_lv0` reader - txpwr by ru limit RU_id level 0 when DBW20M and RU52"]
pub type RDbw20Ru52AllocLv0R = crate::FieldReader;
#[doc = "Field `r_DBW20_RU52_alloc_lv0` writer - txpwr by ru limit RU_id level 0 when DBW20M and RU52"]
pub type RDbw20Ru52AllocLv0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_DBW20_RU52_alloc_lv1` reader - txpwr by ru limit RU_id level 1 when DBW20M and RU52"]
pub type RDbw20Ru52AllocLv1R = crate::FieldReader;
#[doc = "Field `r_DBW20_RU52_alloc_lv1` writer - txpwr by ru limit RU_id level 1 when DBW20M and RU52"]
pub type RDbw20Ru52AllocLv1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn r_ap_tx_pwr_bias_norm_resp(&self) -> RApTxPwrBiasNormRespR {
        RApTxPwrBiasNormRespR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn r_ap_tx_pwr_bias_sta_ul(&self) -> RApTxPwrBiasStaUlR {
        RApTxPwrBiasStaUlR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:19 - txpwr by ru limit RU_id level 0 when DBW20M and RU26"]
    #[inline(always)]
    pub fn r_dbw20_ru26_alloc_lv0(&self) -> RDbw20Ru26AllocLv0R {
        RDbw20Ru26AllocLv0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - txpwr by ru limit RU_id level 1 when DBW20M and RU26"]
    #[inline(always)]
    pub fn r_dbw20_ru26_alloc_lv1(&self) -> RDbw20Ru26AllocLv1R {
        RDbw20Ru26AllocLv1R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - txpwr by ru limit RU_id level 0 when DBW20M and RU52"]
    #[inline(always)]
    pub fn r_dbw20_ru52_alloc_lv0(&self) -> RDbw20Ru52AllocLv0R {
        RDbw20Ru52AllocLv0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - txpwr by ru limit RU_id level 1 when DBW20M and RU52"]
    #[inline(always)]
    pub fn r_dbw20_ru52_alloc_lv1(&self) -> RDbw20Ru52AllocLv1R {
        RDbw20Ru52AllocLv1R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn r_ap_tx_pwr_bias_norm_resp(&mut self) -> RApTxPwrBiasNormRespW<'_, RegAxPwrRsvd8Spec> {
        RApTxPwrBiasNormRespW::new(self, 0)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn r_ap_tx_pwr_bias_sta_ul(&mut self) -> RApTxPwrBiasStaUlW<'_, RegAxPwrRsvd8Spec> {
        RApTxPwrBiasStaUlW::new(self, 8)
    }
    #[doc = "Bits 16:19 - txpwr by ru limit RU_id level 0 when DBW20M and RU26"]
    #[inline(always)]
    pub fn r_dbw20_ru26_alloc_lv0(&mut self) -> RDbw20Ru26AllocLv0W<'_, RegAxPwrRsvd8Spec> {
        RDbw20Ru26AllocLv0W::new(self, 16)
    }
    #[doc = "Bits 20:23 - txpwr by ru limit RU_id level 1 when DBW20M and RU26"]
    #[inline(always)]
    pub fn r_dbw20_ru26_alloc_lv1(&mut self) -> RDbw20Ru26AllocLv1W<'_, RegAxPwrRsvd8Spec> {
        RDbw20Ru26AllocLv1W::new(self, 20)
    }
    #[doc = "Bits 24:27 - txpwr by ru limit RU_id level 0 when DBW20M and RU52"]
    #[inline(always)]
    pub fn r_dbw20_ru52_alloc_lv0(&mut self) -> RDbw20Ru52AllocLv0W<'_, RegAxPwrRsvd8Spec> {
        RDbw20Ru52AllocLv0W::new(self, 24)
    }
    #[doc = "Bits 28:31 - txpwr by ru limit RU_id level 1 when DBW20M and RU52"]
    #[inline(always)]
    pub fn r_dbw20_ru52_alloc_lv1(&mut self) -> RDbw20Ru52AllocLv1W<'_, RegAxPwrRsvd8Spec> {
        RDbw20Ru52AllocLv1W::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_rsvd8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_rsvd8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrRsvd8Spec;
impl crate::RegisterSpec for RegAxPwrRsvd8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_rsvd8::R`](R) reader structure"]
impl crate::Readable for RegAxPwrRsvd8Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_rsvd8::W`](W) writer structure"]
impl crate::Writable for RegAxPwrRsvd8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_RSVD8 to value 0x4292_0000"]
impl crate::Resettable for RegAxPwrRsvd8Spec {
    const RESET_VALUE: u32 = 0x4292_0000;
}
