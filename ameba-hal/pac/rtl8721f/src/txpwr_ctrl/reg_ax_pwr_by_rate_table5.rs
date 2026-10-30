#[doc = "Register `REG_AX_PWR_BY_RATE_TABLE5` reader"]
pub type R = crate::R<RegAxPwrByRateTable5Spec>;
#[doc = "Register `REG_AX_PWR_BY_RATE_TABLE5` writer"]
pub type W = crate::W<RegAxPwrByRateTable5Spec>;
#[doc = "Field `r_txagc_nonlegacy_mcs8_1nss` reader - s(7.1)"]
pub type RTxagcNonlegacyMcs8_1nssR = crate::FieldReader;
#[doc = "Field `r_txagc_nonlegacy_mcs8_1nss` writer - s(7.1)"]
pub type RTxagcNonlegacyMcs8_1nssW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_txagc_nonlegacy_mcs9_1nss` reader - s(7.1)"]
pub type RTxagcNonlegacyMcs9_1nssR = crate::FieldReader;
#[doc = "Field `r_txagc_nonlegacy_mcs9_1nss` writer - s(7.1)"]
pub type RTxagcNonlegacyMcs9_1nssW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_txagc_nonlegacy_mcs10_1nss` reader - s(7.1)"]
pub type RTxagcNonlegacyMcs10_1nssR = crate::FieldReader;
#[doc = "Field `r_txagc_nonlegacy_mcs10_1nss` writer - s(7.1)"]
pub type RTxagcNonlegacyMcs10_1nssW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_txagc_nonlegacy_mcs11_1nss` reader - s(7.1)"]
pub type RTxagcNonlegacyMcs11_1nssR = crate::FieldReader;
#[doc = "Field `r_txagc_nonlegacy_mcs11_1nss` writer - s(7.1)"]
pub type RTxagcNonlegacyMcs11_1nssW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_mcs8_1nss(&self) -> RTxagcNonlegacyMcs8_1nssR {
        RTxagcNonlegacyMcs8_1nssR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_mcs9_1nss(&self) -> RTxagcNonlegacyMcs9_1nssR {
        RTxagcNonlegacyMcs9_1nssR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_mcs10_1nss(&self) -> RTxagcNonlegacyMcs10_1nssR {
        RTxagcNonlegacyMcs10_1nssR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_mcs11_1nss(&self) -> RTxagcNonlegacyMcs11_1nssR {
        RTxagcNonlegacyMcs11_1nssR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_mcs8_1nss(
        &mut self,
    ) -> RTxagcNonlegacyMcs8_1nssW<'_, RegAxPwrByRateTable5Spec> {
        RTxagcNonlegacyMcs8_1nssW::new(self, 0)
    }
    #[doc = "Bits 8:14 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_mcs9_1nss(
        &mut self,
    ) -> RTxagcNonlegacyMcs9_1nssW<'_, RegAxPwrByRateTable5Spec> {
        RTxagcNonlegacyMcs9_1nssW::new(self, 8)
    }
    #[doc = "Bits 16:22 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_mcs10_1nss(
        &mut self,
    ) -> RTxagcNonlegacyMcs10_1nssW<'_, RegAxPwrByRateTable5Spec> {
        RTxagcNonlegacyMcs10_1nssW::new(self, 16)
    }
    #[doc = "Bits 24:30 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_mcs11_1nss(
        &mut self,
    ) -> RTxagcNonlegacyMcs11_1nssW<'_, RegAxPwrByRateTable5Spec> {
        RTxagcNonlegacyMcs11_1nssW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_by_rate_table5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_by_rate_table5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrByRateTable5Spec;
impl crate::RegisterSpec for RegAxPwrByRateTable5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_by_rate_table5::R`](R) reader structure"]
impl crate::Readable for RegAxPwrByRateTable5Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_by_rate_table5::W`](W) writer structure"]
impl crate::Writable for RegAxPwrByRateTable5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_BY_RATE_TABLE5 to value 0"]
impl crate::Resettable for RegAxPwrByRateTable5Spec {}
