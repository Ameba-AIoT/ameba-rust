#[doc = "Register `REG_AX_PWR_BY_RATE_TABLE6` reader"]
pub type R = crate::R<RegAxPwrByRateTable6Spec>;
#[doc = "Register `REG_AX_PWR_BY_RATE_TABLE6` writer"]
pub type W = crate::W<RegAxPwrByRateTable6Spec>;
#[doc = "Field `r_txagc_nonlegacy_dcm_mcs0_1nss` reader - s(7.1)"]
pub type RTxagcNonlegacyDcmMcs0_1nssR = crate::FieldReader;
#[doc = "Field `r_txagc_nonlegacy_dcm_mcs0_1nss` writer - s(7.1)"]
pub type RTxagcNonlegacyDcmMcs0_1nssW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_txagc_nonlegacy_dcm_mcs1_1nss` reader - s(7.1)"]
pub type RTxagcNonlegacyDcmMcs1_1nssR = crate::FieldReader;
#[doc = "Field `r_txagc_nonlegacy_dcm_mcs1_1nss` writer - s(7.1)"]
pub type RTxagcNonlegacyDcmMcs1_1nssW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_txagc_nonlegacy_dcm_mcs3_1nss` reader - s(7.1)"]
pub type RTxagcNonlegacyDcmMcs3_1nssR = crate::FieldReader;
#[doc = "Field `r_txagc_nonlegacy_dcm_mcs3_1nss` writer - s(7.1)"]
pub type RTxagcNonlegacyDcmMcs3_1nssW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_txagc_nonlegacy_dcm_mcs4_1nss` reader - s(7.1)"]
pub type RTxagcNonlegacyDcmMcs4_1nssR = crate::FieldReader;
#[doc = "Field `r_txagc_nonlegacy_dcm_mcs4_1nss` writer - s(7.1)"]
pub type RTxagcNonlegacyDcmMcs4_1nssW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_dcm_mcs0_1nss(&self) -> RTxagcNonlegacyDcmMcs0_1nssR {
        RTxagcNonlegacyDcmMcs0_1nssR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_dcm_mcs1_1nss(&self) -> RTxagcNonlegacyDcmMcs1_1nssR {
        RTxagcNonlegacyDcmMcs1_1nssR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_dcm_mcs3_1nss(&self) -> RTxagcNonlegacyDcmMcs3_1nssR {
        RTxagcNonlegacyDcmMcs3_1nssR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_dcm_mcs4_1nss(&self) -> RTxagcNonlegacyDcmMcs4_1nssR {
        RTxagcNonlegacyDcmMcs4_1nssR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_dcm_mcs0_1nss(
        &mut self,
    ) -> RTxagcNonlegacyDcmMcs0_1nssW<'_, RegAxPwrByRateTable6Spec> {
        RTxagcNonlegacyDcmMcs0_1nssW::new(self, 0)
    }
    #[doc = "Bits 8:14 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_dcm_mcs1_1nss(
        &mut self,
    ) -> RTxagcNonlegacyDcmMcs1_1nssW<'_, RegAxPwrByRateTable6Spec> {
        RTxagcNonlegacyDcmMcs1_1nssW::new(self, 8)
    }
    #[doc = "Bits 16:22 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_dcm_mcs3_1nss(
        &mut self,
    ) -> RTxagcNonlegacyDcmMcs3_1nssW<'_, RegAxPwrByRateTable6Spec> {
        RTxagcNonlegacyDcmMcs3_1nssW::new(self, 16)
    }
    #[doc = "Bits 24:30 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_dcm_mcs4_1nss(
        &mut self,
    ) -> RTxagcNonlegacyDcmMcs4_1nssW<'_, RegAxPwrByRateTable6Spec> {
        RTxagcNonlegacyDcmMcs4_1nssW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_by_rate_table6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_by_rate_table6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrByRateTable6Spec;
impl crate::RegisterSpec for RegAxPwrByRateTable6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_by_rate_table6::R`](R) reader structure"]
impl crate::Readable for RegAxPwrByRateTable6Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_by_rate_table6::W`](W) writer structure"]
impl crate::Writable for RegAxPwrByRateTable6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_BY_RATE_TABLE6 to value 0"]
impl crate::Resettable for RegAxPwrByRateTable6Spec {}
