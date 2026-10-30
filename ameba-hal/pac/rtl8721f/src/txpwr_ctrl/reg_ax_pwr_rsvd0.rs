#[doc = "Register `REG_AX_PWR_RSVD0` reader"]
pub type R = crate::R<RegAxPwrRsvd0Spec>;
#[doc = "Register `REG_AX_PWR_RSVD0` writer"]
pub type W = crate::W<RegAxPwrRsvd0Spec>;
#[doc = "Field `r_RU_norm_RU26` reader - (RTL_only) S(10.4) RU_lim tbl -RU norm ,0x9b=9.6875dB"]
pub type RRuNormRu26R = crate::FieldReader<u16>;
#[doc = "Field `r_RU_norm_RU26` writer - (RTL_only) S(10.4) RU_lim tbl -RU norm ,0x9b=9.6875dB"]
pub type RRuNormRu26W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `r_RU_norm_RU52` reader - (RTL_only) S(10.4) RU_lim tbl -RU norm,0x6b=6.6875dB"]
pub type RRuNormRu52R = crate::FieldReader<u16>;
#[doc = "Field `r_RU_norm_RU52` writer - (RTL_only) S(10.4) RU_lim tbl -RU norm,0x6b=6.6875dB"]
pub type RRuNormRu52W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `r_RU_norm_RU106` reader - (RTL_only) S(10.4) RU_lim tbl -RU norm,0x39=3.5625dB"]
pub type RRuNormRu106R = crate::FieldReader<u16>;
#[doc = "Field `r_RU_norm_RU106` writer - (RTL_only) S(10.4) RU_lim tbl -RU norm,0x39=3.5625dB"]
pub type RRuNormRu106W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `r_RU_norm_en` reader - (RTL_only) RU_norm_enable (RTL_ONLY TEST )"]
pub type RRuNormEnR = crate::BitReader;
#[doc = "Field `r_RU_norm_en` writer - (RTL_only) RU_norm_enable (RTL_ONLY TEST )"]
pub type RRuNormEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_ru_pwr_lim_mux_en` reader - 1:by_rate->macid_lim->RU_lim 0:by_rate->macid->BF0_pwr_lim->RU_lim"]
pub type RRuPwrLimMuxEnR = crate::BitReader;
#[doc = "Field `r_ru_pwr_lim_mux_en` writer - 1:by_rate->macid_lim->RU_lim 0:by_rate->macid->BF0_pwr_lim->RU_lim"]
pub type RRuPwrLimMuxEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - (RTL_only) S(10.4) RU_lim tbl -RU norm ,0x9b=9.6875dB"]
    #[inline(always)]
    pub fn r_ru_norm_ru26(&self) -> RRuNormRu26R {
        RRuNormRu26R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - (RTL_only) S(10.4) RU_lim tbl -RU norm,0x6b=6.6875dB"]
    #[inline(always)]
    pub fn r_ru_norm_ru52(&self) -> RRuNormRu52R {
        RRuNormRu52R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - (RTL_only) S(10.4) RU_lim tbl -RU norm,0x39=3.5625dB"]
    #[inline(always)]
    pub fn r_ru_norm_ru106(&self) -> RRuNormRu106R {
        RRuNormRu106R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - (RTL_only) RU_norm_enable (RTL_ONLY TEST )"]
    #[inline(always)]
    pub fn r_ru_norm_en(&self) -> RRuNormEnR {
        RRuNormEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1:by_rate->macid_lim->RU_lim 0:by_rate->macid->BF0_pwr_lim->RU_lim"]
    #[inline(always)]
    pub fn r_ru_pwr_lim_mux_en(&self) -> RRuPwrLimMuxEnR {
        RRuPwrLimMuxEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - (RTL_only) S(10.4) RU_lim tbl -RU norm ,0x9b=9.6875dB"]
    #[inline(always)]
    pub fn r_ru_norm_ru26(&mut self) -> RRuNormRu26W<'_, RegAxPwrRsvd0Spec> {
        RRuNormRu26W::new(self, 0)
    }
    #[doc = "Bits 10:19 - (RTL_only) S(10.4) RU_lim tbl -RU norm,0x6b=6.6875dB"]
    #[inline(always)]
    pub fn r_ru_norm_ru52(&mut self) -> RRuNormRu52W<'_, RegAxPwrRsvd0Spec> {
        RRuNormRu52W::new(self, 10)
    }
    #[doc = "Bits 20:29 - (RTL_only) S(10.4) RU_lim tbl -RU norm,0x39=3.5625dB"]
    #[inline(always)]
    pub fn r_ru_norm_ru106(&mut self) -> RRuNormRu106W<'_, RegAxPwrRsvd0Spec> {
        RRuNormRu106W::new(self, 20)
    }
    #[doc = "Bit 30 - (RTL_only) RU_norm_enable (RTL_ONLY TEST )"]
    #[inline(always)]
    pub fn r_ru_norm_en(&mut self) -> RRuNormEnW<'_, RegAxPwrRsvd0Spec> {
        RRuNormEnW::new(self, 30)
    }
    #[doc = "Bit 31 - 1:by_rate->macid_lim->RU_lim 0:by_rate->macid->BF0_pwr_lim->RU_lim"]
    #[inline(always)]
    pub fn r_ru_pwr_lim_mux_en(&mut self) -> RRuPwrLimMuxEnW<'_, RegAxPwrRsvd0Spec> {
        RRuPwrLimMuxEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_rsvd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_rsvd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrRsvd0Spec;
impl crate::RegisterSpec for RegAxPwrRsvd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_rsvd0::R`](R) reader structure"]
impl crate::Readable for RegAxPwrRsvd0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_rsvd0::W`](W) writer structure"]
impl crate::Writable for RegAxPwrRsvd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_RSVD0 to value 0x0391_ac9b"]
impl crate::Resettable for RegAxPwrRsvd0Spec {
    const RESET_VALUE: u32 = 0x0391_ac9b;
}
