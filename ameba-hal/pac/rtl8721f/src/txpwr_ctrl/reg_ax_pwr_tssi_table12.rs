#[doc = "Register `REG_AX_PWR_TSSI_TABLE12` reader"]
pub type R = crate::R<RegAxPwrTssiTable12Spec>;
#[doc = "Register `REG_AX_PWR_TSSI_TABLE12` writer"]
pub type W = crate::W<RegAxPwrTssiTable12Spec>;
#[doc = "Field `r_ch_comb_ofst_bandedge_zero_bw80` reader - "]
pub type RChCombOfstBandedgeZeroBw80R = crate::FieldReader;
#[doc = "Field `r_ch_comb_ofst_bandedge_zero_bw80` writer - "]
pub type RChCombOfstBandedgeZeroBw80W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_f_ch20_with_data_legacy_val` reader - "]
pub type RFCh20WithDataLegacyValR = crate::FieldReader;
#[doc = "Field `r_f_ch20_with_data_legacy_val` writer - "]
pub type RFCh20WithDataLegacyValW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_f_ch20_with_data_nonlegacy_val` reader - "]
pub type RFCh20WithDataNonlegacyValR = crate::FieldReader;
#[doc = "Field `r_f_ch20_with_data_nonlegacy_val` writer - "]
pub type RFCh20WithDataNonlegacyValW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_f_pwr_ofst_out_seg0_db` reader - "]
pub type RFPwrOfstOutSeg0DbR = crate::FieldReader;
#[doc = "Field `r_f_pwr_ofst_out_seg0_db` writer - "]
pub type RFPwrOfstOutSeg0DbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_ch_comb_ofst_bandedge_zero_bw80(&self) -> RChCombOfstBandedgeZeroBw80R {
        RChCombOfstBandedgeZeroBw80R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_f_ch20_with_data_legacy_val(&self) -> RFCh20WithDataLegacyValR {
        RFCh20WithDataLegacyValR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_f_ch20_with_data_nonlegacy_val(&self) -> RFCh20WithDataNonlegacyValR {
        RFCh20WithDataNonlegacyValR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_f_pwr_ofst_out_seg0_db(&self) -> RFPwrOfstOutSeg0DbR {
        RFPwrOfstOutSeg0DbR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_ch_comb_ofst_bandedge_zero_bw80(
        &mut self,
    ) -> RChCombOfstBandedgeZeroBw80W<'_, RegAxPwrTssiTable12Spec> {
        RChCombOfstBandedgeZeroBw80W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_f_ch20_with_data_legacy_val(
        &mut self,
    ) -> RFCh20WithDataLegacyValW<'_, RegAxPwrTssiTable12Spec> {
        RFCh20WithDataLegacyValW::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_f_ch20_with_data_nonlegacy_val(
        &mut self,
    ) -> RFCh20WithDataNonlegacyValW<'_, RegAxPwrTssiTable12Spec> {
        RFCh20WithDataNonlegacyValW::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_f_pwr_ofst_out_seg0_db(&mut self) -> RFPwrOfstOutSeg0DbW<'_, RegAxPwrTssiTable12Spec> {
        RFPwrOfstOutSeg0DbW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tssi_table12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tssi_table12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTssiTable12Spec;
impl crate::RegisterSpec for RegAxPwrTssiTable12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tssi_table12::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTssiTable12Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tssi_table12::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTssiTable12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TSSI_TABLE12 to value 0"]
impl crate::Resettable for RegAxPwrTssiTable12Spec {}
