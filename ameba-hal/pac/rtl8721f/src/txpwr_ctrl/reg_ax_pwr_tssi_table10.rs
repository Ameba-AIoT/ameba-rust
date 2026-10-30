#[doc = "Register `REG_AX_PWR_TSSI_TABLE10` reader"]
pub type R = crate::R<RegAxPwrTssiTable10Spec>;
#[doc = "Register `REG_AX_PWR_TSSI_TABLE10` writer"]
pub type W = crate::W<RegAxPwrTssiTable10Spec>;
#[doc = "Field `r_ch_comb_common_diff_db` reader - "]
pub type RChCombCommonDiffDbR = crate::FieldReader;
#[doc = "Field `r_ch_comb_common_diff_db` writer - "]
pub type RChCombCommonDiffDbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_ch_comb_ofst_bandedge_nonzero_bw160` reader - "]
pub type RChCombOfstBandedgeNonzeroBw160R = crate::FieldReader;
#[doc = "Field `r_ch_comb_ofst_bandedge_nonzero_bw160` writer - "]
pub type RChCombOfstBandedgeNonzeroBw160W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_ch_comb_ofst_bandedge_nonzero_bw20` reader - "]
pub type RChCombOfstBandedgeNonzeroBw20R = crate::FieldReader;
#[doc = "Field `r_ch_comb_ofst_bandedge_nonzero_bw20` writer - "]
pub type RChCombOfstBandedgeNonzeroBw20W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_ch_comb_ofst_bandedge_nonzero_bw40` reader - "]
pub type RChCombOfstBandedgeNonzeroBw40R = crate::FieldReader;
#[doc = "Field `r_ch_comb_ofst_bandedge_nonzero_bw40` writer - "]
pub type RChCombOfstBandedgeNonzeroBw40W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_ch_comb_common_diff_db(&self) -> RChCombCommonDiffDbR {
        RChCombCommonDiffDbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_ch_comb_ofst_bandedge_nonzero_bw160(&self) -> RChCombOfstBandedgeNonzeroBw160R {
        RChCombOfstBandedgeNonzeroBw160R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_ch_comb_ofst_bandedge_nonzero_bw20(&self) -> RChCombOfstBandedgeNonzeroBw20R {
        RChCombOfstBandedgeNonzeroBw20R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_ch_comb_ofst_bandedge_nonzero_bw40(&self) -> RChCombOfstBandedgeNonzeroBw40R {
        RChCombOfstBandedgeNonzeroBw40R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_ch_comb_common_diff_db(
        &mut self,
    ) -> RChCombCommonDiffDbW<'_, RegAxPwrTssiTable10Spec> {
        RChCombCommonDiffDbW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_ch_comb_ofst_bandedge_nonzero_bw160(
        &mut self,
    ) -> RChCombOfstBandedgeNonzeroBw160W<'_, RegAxPwrTssiTable10Spec> {
        RChCombOfstBandedgeNonzeroBw160W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_ch_comb_ofst_bandedge_nonzero_bw20(
        &mut self,
    ) -> RChCombOfstBandedgeNonzeroBw20W<'_, RegAxPwrTssiTable10Spec> {
        RChCombOfstBandedgeNonzeroBw20W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_ch_comb_ofst_bandedge_nonzero_bw40(
        &mut self,
    ) -> RChCombOfstBandedgeNonzeroBw40W<'_, RegAxPwrTssiTable10Spec> {
        RChCombOfstBandedgeNonzeroBw40W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tssi_table10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tssi_table10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTssiTable10Spec;
impl crate::RegisterSpec for RegAxPwrTssiTable10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tssi_table10::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTssiTable10Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tssi_table10::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTssiTable10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TSSI_TABLE10 to value 0"]
impl crate::Resettable for RegAxPwrTssiTable10Spec {}
