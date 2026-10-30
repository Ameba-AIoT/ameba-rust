#[doc = "Register `REG_AX_PWR_RSVD4` reader"]
pub type R = crate::R<RegAxPwrRsvd4Spec>;
#[doc = "Register `REG_AX_PWR_RSVD4` writer"]
pub type W = crate::W<RegAxPwrRsvd4Spec>;
#[doc = "Field `r_txbias_cck_th_0` reader - txbias region"]
pub type RTxbiasCckTh0R = crate::FieldReader;
#[doc = "Field `r_txbias_cck_th_0` writer - txbias region"]
pub type RTxbiasCckTh0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_txbias_cck_th_1` reader - txbias region"]
pub type RTxbiasCckTh1R = crate::FieldReader;
#[doc = "Field `r_txbias_cck_th_1` writer - txbias region"]
pub type RTxbiasCckTh1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_txbias_cck_th_2` reader - txbias region"]
pub type RTxbiasCckTh2R = crate::FieldReader;
#[doc = "Field `r_txbias_cck_th_2` writer - txbias region"]
pub type RTxbiasCckTh2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_txbias_legacy_th_0` reader - txbias region"]
pub type RTxbiasLegacyTh0R = crate::FieldReader;
#[doc = "Field `r_txbias_legacy_th_0` writer - txbias region"]
pub type RTxbiasLegacyTh0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txbias_legacy_th_1` reader - txbias region"]
pub type RTxbiasLegacyTh1R = crate::FieldReader;
#[doc = "Field `r_txbias_legacy_th_1` writer - txbias region"]
pub type RTxbiasLegacyTh1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txbias_legacy_th_2` reader - txbias region"]
pub type RTxbiasLegacyTh2R = crate::FieldReader;
#[doc = "Field `r_txbias_legacy_th_2` writer - txbias region"]
pub type RTxbiasLegacyTh2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txbias_ht_th_0` reader - txbias region"]
pub type RTxbiasHtTh0R = crate::FieldReader;
#[doc = "Field `r_txbias_ht_th_0` writer - txbias region"]
pub type RTxbiasHtTh0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txbias_ht_th_1` reader - txbias region"]
pub type RTxbiasHtTh1R = crate::FieldReader;
#[doc = "Field `r_txbias_ht_th_1` writer - txbias region"]
pub type RTxbiasHtTh1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_cck_th_0(&self) -> RTxbiasCckTh0R {
        RTxbiasCckTh0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_cck_th_1(&self) -> RTxbiasCckTh1R {
        RTxbiasCckTh1R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_cck_th_2(&self) -> RTxbiasCckTh2R {
        RTxbiasCckTh2R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:12 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_legacy_th_0(&self) -> RTxbiasLegacyTh0R {
        RTxbiasLegacyTh0R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:16 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_legacy_th_1(&self) -> RTxbiasLegacyTh1R {
        RTxbiasLegacyTh1R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:20 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_legacy_th_2(&self) -> RTxbiasLegacyTh2R {
        RTxbiasLegacyTh2R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 23:26 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_ht_th_0(&self) -> RTxbiasHtTh0R {
        RTxbiasHtTh0R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 27:30 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_ht_th_1(&self) -> RTxbiasHtTh1R {
        RTxbiasHtTh1R::new(((self.bits >> 27) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_cck_th_0(&mut self) -> RTxbiasCckTh0W<'_, RegAxPwrRsvd4Spec> {
        RTxbiasCckTh0W::new(self, 0)
    }
    #[doc = "Bits 3:5 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_cck_th_1(&mut self) -> RTxbiasCckTh1W<'_, RegAxPwrRsvd4Spec> {
        RTxbiasCckTh1W::new(self, 3)
    }
    #[doc = "Bits 6:8 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_cck_th_2(&mut self) -> RTxbiasCckTh2W<'_, RegAxPwrRsvd4Spec> {
        RTxbiasCckTh2W::new(self, 6)
    }
    #[doc = "Bits 9:12 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_legacy_th_0(&mut self) -> RTxbiasLegacyTh0W<'_, RegAxPwrRsvd4Spec> {
        RTxbiasLegacyTh0W::new(self, 9)
    }
    #[doc = "Bits 13:16 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_legacy_th_1(&mut self) -> RTxbiasLegacyTh1W<'_, RegAxPwrRsvd4Spec> {
        RTxbiasLegacyTh1W::new(self, 13)
    }
    #[doc = "Bits 17:20 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_legacy_th_2(&mut self) -> RTxbiasLegacyTh2W<'_, RegAxPwrRsvd4Spec> {
        RTxbiasLegacyTh2W::new(self, 17)
    }
    #[doc = "Bits 23:26 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_ht_th_0(&mut self) -> RTxbiasHtTh0W<'_, RegAxPwrRsvd4Spec> {
        RTxbiasHtTh0W::new(self, 23)
    }
    #[doc = "Bits 27:30 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_ht_th_1(&mut self) -> RTxbiasHtTh1W<'_, RegAxPwrRsvd4Spec> {
        RTxbiasHtTh1W::new(self, 27)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_rsvd4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_rsvd4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrRsvd4Spec;
impl crate::RegisterSpec for RegAxPwrRsvd4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_rsvd4::R`](R) reader structure"]
impl crate::Readable for RegAxPwrRsvd4Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_rsvd4::W`](W) writer structure"]
impl crate::Writable for RegAxPwrRsvd4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_RSVD4 to value 0"]
impl crate::Resettable for RegAxPwrRsvd4Spec {}
