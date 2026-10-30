#[doc = "Register `REG_AX_PWR_BY_RATE_TABLE4` reader"]
pub type R = crate::R<RegAxPwrByRateTable4Spec>;
#[doc = "Register `REG_AX_PWR_BY_RATE_TABLE4` writer"]
pub type W = crate::W<RegAxPwrByRateTable4Spec>;
#[doc = "Field `r_txagc_nonlegacy_mcs4_1nss` reader - s(7.1)"]
pub type RTxagcNonlegacyMcs4_1nssR = crate::FieldReader;
#[doc = "Field `r_txagc_nonlegacy_mcs4_1nss` writer - s(7.1)"]
pub type RTxagcNonlegacyMcs4_1nssW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_txagc_nonlegacy_mcs5_1nss` reader - s(7.1)"]
pub type RTxagcNonlegacyMcs5_1nssR = crate::FieldReader;
#[doc = "Field `r_txagc_nonlegacy_mcs5_1nss` writer - s(7.1)"]
pub type RTxagcNonlegacyMcs5_1nssW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_txagc_nonlegacy_mcs6_1nss` reader - s(7.1)"]
pub type RTxagcNonlegacyMcs6_1nssR = crate::FieldReader;
#[doc = "Field `r_txagc_nonlegacy_mcs6_1nss` writer - s(7.1)"]
pub type RTxagcNonlegacyMcs6_1nssW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_txagc_nonlegacy_mcs7_1nss` reader - s(7.1)"]
pub type RTxagcNonlegacyMcs7_1nssR = crate::FieldReader;
#[doc = "Field `r_txagc_nonlegacy_mcs7_1nss` writer - s(7.1)"]
pub type RTxagcNonlegacyMcs7_1nssW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_mcs4_1nss(&self) -> RTxagcNonlegacyMcs4_1nssR {
        RTxagcNonlegacyMcs4_1nssR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_mcs5_1nss(&self) -> RTxagcNonlegacyMcs5_1nssR {
        RTxagcNonlegacyMcs5_1nssR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_mcs6_1nss(&self) -> RTxagcNonlegacyMcs6_1nssR {
        RTxagcNonlegacyMcs6_1nssR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_mcs7_1nss(&self) -> RTxagcNonlegacyMcs7_1nssR {
        RTxagcNonlegacyMcs7_1nssR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_mcs4_1nss(
        &mut self,
    ) -> RTxagcNonlegacyMcs4_1nssW<'_, RegAxPwrByRateTable4Spec> {
        RTxagcNonlegacyMcs4_1nssW::new(self, 0)
    }
    #[doc = "Bits 8:14 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_mcs5_1nss(
        &mut self,
    ) -> RTxagcNonlegacyMcs5_1nssW<'_, RegAxPwrByRateTable4Spec> {
        RTxagcNonlegacyMcs5_1nssW::new(self, 8)
    }
    #[doc = "Bits 16:22 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_mcs6_1nss(
        &mut self,
    ) -> RTxagcNonlegacyMcs6_1nssW<'_, RegAxPwrByRateTable4Spec> {
        RTxagcNonlegacyMcs6_1nssW::new(self, 16)
    }
    #[doc = "Bits 24:30 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_mcs7_1nss(
        &mut self,
    ) -> RTxagcNonlegacyMcs7_1nssW<'_, RegAxPwrByRateTable4Spec> {
        RTxagcNonlegacyMcs7_1nssW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_by_rate_table4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_by_rate_table4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrByRateTable4Spec;
impl crate::RegisterSpec for RegAxPwrByRateTable4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_by_rate_table4::R`](R) reader structure"]
impl crate::Readable for RegAxPwrByRateTable4Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_by_rate_table4::W`](W) writer structure"]
impl crate::Writable for RegAxPwrByRateTable4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_BY_RATE_TABLE4 to value 0"]
impl crate::Resettable for RegAxPwrByRateTable4Spec {}
