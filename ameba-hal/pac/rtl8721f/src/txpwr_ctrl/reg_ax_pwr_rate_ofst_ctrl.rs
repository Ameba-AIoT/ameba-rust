#[doc = "Register `REG_AX_PWR_RATE_OFST_CTRL` reader"]
pub type R = crate::R<RegAxPwrRateOfstCtrlSpec>;
#[doc = "Register `REG_AX_PWR_RATE_OFST_CTRL` writer"]
pub type W = crate::W<RegAxPwrRateOfstCtrlSpec>;
#[doc = "Field `r_txagc_HE_HT_offset` reader - S(4.1)HE-HT pwr offset"]
pub type RTxagcHeHtOffsetR = crate::FieldReader;
#[doc = "Field `r_txagc_HE_HT_offset` writer - S(4.1)HE-HT pwr offset"]
pub type RTxagcHeHtOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txagc_VHT_HT_offset` reader - S(4.1)VHT-HT pwr offset"]
pub type RTxagcVhtHtOffsetR = crate::FieldReader;
#[doc = "Field `r_txagc_VHT_HT_offset` writer - S(4.1)VHT-HT pwr offset"]
pub type RTxagcVhtHtOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txagc_ht_offset` reader - (RTL_only) S(4.1) HT-HT pwr bias = 0"]
pub type RTxagcHtOffsetR = crate::FieldReader;
#[doc = "Field `r_txagc_ht_offset` writer - (RTL_only) S(4.1) HT-HT pwr bias = 0"]
pub type RTxagcHtOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txagc_legacy_HT_offset` reader - S(4.1)LEG-HT pwr offset"]
pub type RTxagcLegacyHtOffsetR = crate::FieldReader;
#[doc = "Field `r_txagc_legacy_HT_offset` writer - S(4.1)LEG-HT pwr offset"]
pub type RTxagcLegacyHtOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txagc_cck_HT_offset` reader - S(4.1)CCK-HT pwr offset"]
pub type RTxagcCckHtOffsetR = crate::FieldReader;
#[doc = "Field `r_txagc_cck_HT_offset` writer - S(4.1)CCK-HT pwr offset"]
pub type RTxagcCckHtOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txagc_tbl_ra` reader - (RTL_only) indirect read of addr of pwr_by_rate"]
pub type RTxagcTblRaR = crate::FieldReader;
#[doc = "Field `r_txagc_tbl_ra` writer - (RTL_only) indirect read of addr of pwr_by_rate"]
pub type RTxagcTblRaW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `r_txagc_tbl_rd` reader - (RTL_only) 1:indirecr read enable"]
pub type RTxagcTblRdR = crate::BitReader;
#[doc = "Field `r_txagc_tbl_rd` writer - (RTL_only) 1:indirecr read enable"]
pub type RTxagcTblRdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - S(4.1)HE-HT pwr offset"]
    #[inline(always)]
    pub fn r_txagc_he_ht_offset(&self) -> RTxagcHeHtOffsetR {
        RTxagcHeHtOffsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - S(4.1)VHT-HT pwr offset"]
    #[inline(always)]
    pub fn r_txagc_vht_ht_offset(&self) -> RTxagcVhtHtOffsetR {
        RTxagcVhtHtOffsetR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - (RTL_only) S(4.1) HT-HT pwr bias = 0"]
    #[inline(always)]
    pub fn r_txagc_ht_offset(&self) -> RTxagcHtOffsetR {
        RTxagcHtOffsetR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - S(4.1)LEG-HT pwr offset"]
    #[inline(always)]
    pub fn r_txagc_legacy_ht_offset(&self) -> RTxagcLegacyHtOffsetR {
        RTxagcLegacyHtOffsetR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - S(4.1)CCK-HT pwr offset"]
    #[inline(always)]
    pub fn r_txagc_cck_ht_offset(&self) -> RTxagcCckHtOffsetR {
        RTxagcCckHtOffsetR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:25 - (RTL_only) indirect read of addr of pwr_by_rate"]
    #[inline(always)]
    pub fn r_txagc_tbl_ra(&self) -> RTxagcTblRaR {
        RTxagcTblRaR::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bit 26 - (RTL_only) 1:indirecr read enable"]
    #[inline(always)]
    pub fn r_txagc_tbl_rd(&self) -> RTxagcTblRdR {
        RTxagcTblRdR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - S(4.1)HE-HT pwr offset"]
    #[inline(always)]
    pub fn r_txagc_he_ht_offset(&mut self) -> RTxagcHeHtOffsetW<'_, RegAxPwrRateOfstCtrlSpec> {
        RTxagcHeHtOffsetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - S(4.1)VHT-HT pwr offset"]
    #[inline(always)]
    pub fn r_txagc_vht_ht_offset(&mut self) -> RTxagcVhtHtOffsetW<'_, RegAxPwrRateOfstCtrlSpec> {
        RTxagcVhtHtOffsetW::new(self, 4)
    }
    #[doc = "Bits 8:11 - (RTL_only) S(4.1) HT-HT pwr bias = 0"]
    #[inline(always)]
    pub fn r_txagc_ht_offset(&mut self) -> RTxagcHtOffsetW<'_, RegAxPwrRateOfstCtrlSpec> {
        RTxagcHtOffsetW::new(self, 8)
    }
    #[doc = "Bits 12:15 - S(4.1)LEG-HT pwr offset"]
    #[inline(always)]
    pub fn r_txagc_legacy_ht_offset(
        &mut self,
    ) -> RTxagcLegacyHtOffsetW<'_, RegAxPwrRateOfstCtrlSpec> {
        RTxagcLegacyHtOffsetW::new(self, 12)
    }
    #[doc = "Bits 16:19 - S(4.1)CCK-HT pwr offset"]
    #[inline(always)]
    pub fn r_txagc_cck_ht_offset(&mut self) -> RTxagcCckHtOffsetW<'_, RegAxPwrRateOfstCtrlSpec> {
        RTxagcCckHtOffsetW::new(self, 16)
    }
    #[doc = "Bits 20:25 - (RTL_only) indirect read of addr of pwr_by_rate"]
    #[inline(always)]
    pub fn r_txagc_tbl_ra(&mut self) -> RTxagcTblRaW<'_, RegAxPwrRateOfstCtrlSpec> {
        RTxagcTblRaW::new(self, 20)
    }
    #[doc = "Bit 26 - (RTL_only) 1:indirecr read enable"]
    #[inline(always)]
    pub fn r_txagc_tbl_rd(&mut self) -> RTxagcTblRdW<'_, RegAxPwrRateOfstCtrlSpec> {
        RTxagcTblRdW::new(self, 26)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_rate_ofst_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_rate_ofst_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrRateOfstCtrlSpec;
impl crate::RegisterSpec for RegAxPwrRateOfstCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_rate_ofst_ctrl::R`](R) reader structure"]
impl crate::Readable for RegAxPwrRateOfstCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_rate_ofst_ctrl::W`](W) writer structure"]
impl crate::Writable for RegAxPwrRateOfstCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_RATE_OFST_CTRL to value 0"]
impl crate::Resettable for RegAxPwrRateOfstCtrlSpec {}
