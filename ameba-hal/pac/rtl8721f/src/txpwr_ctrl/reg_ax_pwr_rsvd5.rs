#[doc = "Register `REG_AX_PWR_RSVD5` reader"]
pub type R = crate::R<RegAxPwrRsvd5Spec>;
#[doc = "Register `REG_AX_PWR_RSVD5` writer"]
pub type W = crate::W<RegAxPwrRsvd5Spec>;
#[doc = "Field `r_txbias_ht_th_2` reader - txbias region"]
pub type RTxbiasHtTh2R = crate::FieldReader;
#[doc = "Field `r_txbias_ht_th_2` writer - txbias region"]
pub type RTxbiasHtTh2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txbias_vht_th_0` reader - txbias region"]
pub type RTxbiasVhtTh0R = crate::FieldReader;
#[doc = "Field `r_txbias_vht_th_0` writer - txbias region"]
pub type RTxbiasVhtTh0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txbias_vht_th_1` reader - txbias region"]
pub type RTxbiasVhtTh1R = crate::FieldReader;
#[doc = "Field `r_txbias_vht_th_1` writer - txbias region"]
pub type RTxbiasVhtTh1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txbias_vht_th_2` reader - txbias region"]
pub type RTxbiasVhtTh2R = crate::FieldReader;
#[doc = "Field `r_txbias_vht_th_2` writer - txbias region"]
pub type RTxbiasVhtTh2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txbias_he_th_0` reader - txbias region"]
pub type RTxbiasHeTh0R = crate::FieldReader;
#[doc = "Field `r_txbias_he_th_0` writer - txbias region"]
pub type RTxbiasHeTh0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txbias_he_th_1` reader - txbias region"]
pub type RTxbiasHeTh1R = crate::FieldReader;
#[doc = "Field `r_txbias_he_th_1` writer - txbias region"]
pub type RTxbiasHeTh1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txbias_he_th_2` reader - txbias region"]
pub type RTxbiasHeTh2R = crate::FieldReader;
#[doc = "Field `r_txbias_he_th_2` writer - txbias region"]
pub type RTxbiasHeTh2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txbias_cck_region_2` reader - txbias region"]
pub type RTxbiasCckRegion2R = crate::FieldReader;
#[doc = "Field `r_txbias_cck_region_2` writer - txbias region"]
pub type RTxbiasCckRegion2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_txbias_cck_region_3` reader - txbias region"]
pub type RTxbiasCckRegion3R = crate::FieldReader;
#[doc = "Field `r_txbias_cck_region_3` writer - txbias region"]
pub type RTxbiasCckRegion3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_ht_th_2(&self) -> RTxbiasHtTh2R {
        RTxbiasHtTh2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_vht_th_0(&self) -> RTxbiasVhtTh0R {
        RTxbiasVhtTh0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_vht_th_1(&self) -> RTxbiasVhtTh1R {
        RTxbiasVhtTh1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_vht_th_2(&self) -> RTxbiasVhtTh2R {
        RTxbiasVhtTh2R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_he_th_0(&self) -> RTxbiasHeTh0R {
        RTxbiasHeTh0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_he_th_1(&self) -> RTxbiasHeTh1R {
        RTxbiasHeTh1R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_he_th_2(&self) -> RTxbiasHeTh2R {
        RTxbiasHeTh2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_cck_region_2(&self) -> RTxbiasCckRegion2R {
        RTxbiasCckRegion2R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_cck_region_3(&self) -> RTxbiasCckRegion3R {
        RTxbiasCckRegion3R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_ht_th_2(&mut self) -> RTxbiasHtTh2W<'_, RegAxPwrRsvd5Spec> {
        RTxbiasHtTh2W::new(self, 0)
    }
    #[doc = "Bits 4:7 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_vht_th_0(&mut self) -> RTxbiasVhtTh0W<'_, RegAxPwrRsvd5Spec> {
        RTxbiasVhtTh0W::new(self, 4)
    }
    #[doc = "Bits 8:11 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_vht_th_1(&mut self) -> RTxbiasVhtTh1W<'_, RegAxPwrRsvd5Spec> {
        RTxbiasVhtTh1W::new(self, 8)
    }
    #[doc = "Bits 12:15 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_vht_th_2(&mut self) -> RTxbiasVhtTh2W<'_, RegAxPwrRsvd5Spec> {
        RTxbiasVhtTh2W::new(self, 12)
    }
    #[doc = "Bits 16:19 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_he_th_0(&mut self) -> RTxbiasHeTh0W<'_, RegAxPwrRsvd5Spec> {
        RTxbiasHeTh0W::new(self, 16)
    }
    #[doc = "Bits 20:23 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_he_th_1(&mut self) -> RTxbiasHeTh1W<'_, RegAxPwrRsvd5Spec> {
        RTxbiasHeTh1W::new(self, 20)
    }
    #[doc = "Bits 24:27 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_he_th_2(&mut self) -> RTxbiasHeTh2W<'_, RegAxPwrRsvd5Spec> {
        RTxbiasHeTh2W::new(self, 24)
    }
    #[doc = "Bits 28:29 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_cck_region_2(&mut self) -> RTxbiasCckRegion2W<'_, RegAxPwrRsvd5Spec> {
        RTxbiasCckRegion2W::new(self, 28)
    }
    #[doc = "Bits 30:31 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_cck_region_3(&mut self) -> RTxbiasCckRegion3W<'_, RegAxPwrRsvd5Spec> {
        RTxbiasCckRegion3W::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_rsvd5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_rsvd5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrRsvd5Spec;
impl crate::RegisterSpec for RegAxPwrRsvd5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_rsvd5::R`](R) reader structure"]
impl crate::Readable for RegAxPwrRsvd5Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_rsvd5::W`](W) writer structure"]
impl crate::Writable for RegAxPwrRsvd5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_RSVD5 to value 0"]
impl crate::Resettable for RegAxPwrRsvd5Spec {}
