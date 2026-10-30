#[doc = "Register `REG_AX_PWR_TSSI_TABLE11` reader"]
pub type R = crate::R<RegAxPwrTssiTable11Spec>;
#[doc = "Register `REG_AX_PWR_TSSI_TABLE11` writer"]
pub type W = crate::W<RegAxPwrTssiTable11Spec>;
#[doc = "Field `r_ch_comb_ofst_bandedge_nonzero_bw80` reader - "]
pub type RChCombOfstBandedgeNonzeroBw80R = crate::FieldReader;
#[doc = "Field `r_ch_comb_ofst_bandedge_nonzero_bw80` writer - "]
pub type RChCombOfstBandedgeNonzeroBw80W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_ch_comb_ofst_bandedge_zero_bw160` reader - "]
pub type RChCombOfstBandedgeZeroBw160R = crate::FieldReader;
#[doc = "Field `r_ch_comb_ofst_bandedge_zero_bw160` writer - "]
pub type RChCombOfstBandedgeZeroBw160W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_ch_comb_ofst_bandedge_zero_bw20` reader - "]
pub type RChCombOfstBandedgeZeroBw20R = crate::FieldReader;
#[doc = "Field `r_ch_comb_ofst_bandedge_zero_bw20` writer - "]
pub type RChCombOfstBandedgeZeroBw20W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_ch_comb_ofst_bandedge_zero_bw40` reader - "]
pub type RChCombOfstBandedgeZeroBw40R = crate::FieldReader;
#[doc = "Field `r_ch_comb_ofst_bandedge_zero_bw40` writer - "]
pub type RChCombOfstBandedgeZeroBw40W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_ch_comb_ofst_bandedge_nonzero_bw80(&self) -> RChCombOfstBandedgeNonzeroBw80R {
        RChCombOfstBandedgeNonzeroBw80R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_ch_comb_ofst_bandedge_zero_bw160(&self) -> RChCombOfstBandedgeZeroBw160R {
        RChCombOfstBandedgeZeroBw160R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_ch_comb_ofst_bandedge_zero_bw20(&self) -> RChCombOfstBandedgeZeroBw20R {
        RChCombOfstBandedgeZeroBw20R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_ch_comb_ofst_bandedge_zero_bw40(&self) -> RChCombOfstBandedgeZeroBw40R {
        RChCombOfstBandedgeZeroBw40R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_ch_comb_ofst_bandedge_nonzero_bw80(
        &mut self,
    ) -> RChCombOfstBandedgeNonzeroBw80W<'_, RegAxPwrTssiTable11Spec> {
        RChCombOfstBandedgeNonzeroBw80W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_ch_comb_ofst_bandedge_zero_bw160(
        &mut self,
    ) -> RChCombOfstBandedgeZeroBw160W<'_, RegAxPwrTssiTable11Spec> {
        RChCombOfstBandedgeZeroBw160W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_ch_comb_ofst_bandedge_zero_bw20(
        &mut self,
    ) -> RChCombOfstBandedgeZeroBw20W<'_, RegAxPwrTssiTable11Spec> {
        RChCombOfstBandedgeZeroBw20W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_ch_comb_ofst_bandedge_zero_bw40(
        &mut self,
    ) -> RChCombOfstBandedgeZeroBw40W<'_, RegAxPwrTssiTable11Spec> {
        RChCombOfstBandedgeZeroBw40W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tssi_table11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tssi_table11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTssiTable11Spec;
impl crate::RegisterSpec for RegAxPwrTssiTable11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tssi_table11::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTssiTable11Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tssi_table11::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTssiTable11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TSSI_TABLE11 to value 0"]
impl crate::Resettable for RegAxPwrTssiTable11Spec {}
