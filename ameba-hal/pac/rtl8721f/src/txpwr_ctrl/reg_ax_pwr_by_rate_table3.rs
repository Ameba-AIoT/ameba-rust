#[doc = "Register `REG_AX_PWR_BY_RATE_TABLE3` reader"]
pub type R = crate::R<RegAxPwrByRateTable3Spec>;
#[doc = "Register `REG_AX_PWR_BY_RATE_TABLE3` writer"]
pub type W = crate::W<RegAxPwrByRateTable3Spec>;
#[doc = "Field `r_txagc_nonlegacy_mcs0_1nss` reader - s(7.1)"]
pub type RTxagcNonlegacyMcs0_1nssR = crate::FieldReader;
#[doc = "Field `r_txagc_nonlegacy_mcs0_1nss` writer - s(7.1)"]
pub type RTxagcNonlegacyMcs0_1nssW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_txagc_nonlegacy_mcs1_1nss` reader - s(7.1)"]
pub type RTxagcNonlegacyMcs1_1nssR = crate::FieldReader;
#[doc = "Field `r_txagc_nonlegacy_mcs1_1nss` writer - s(7.1)"]
pub type RTxagcNonlegacyMcs1_1nssW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_txagc_nonlegacy_mcs2_1nss` reader - s(7.1)"]
pub type RTxagcNonlegacyMcs2_1nssR = crate::FieldReader;
#[doc = "Field `r_txagc_nonlegacy_mcs2_1nss` writer - s(7.1)"]
pub type RTxagcNonlegacyMcs2_1nssW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_txagc_nonlegacy_mcs3_1nss` reader - s(7.1)"]
pub type RTxagcNonlegacyMcs3_1nssR = crate::FieldReader;
#[doc = "Field `r_txagc_nonlegacy_mcs3_1nss` writer - s(7.1)"]
pub type RTxagcNonlegacyMcs3_1nssW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_mcs0_1nss(&self) -> RTxagcNonlegacyMcs0_1nssR {
        RTxagcNonlegacyMcs0_1nssR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_mcs1_1nss(&self) -> RTxagcNonlegacyMcs1_1nssR {
        RTxagcNonlegacyMcs1_1nssR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_mcs2_1nss(&self) -> RTxagcNonlegacyMcs2_1nssR {
        RTxagcNonlegacyMcs2_1nssR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_mcs3_1nss(&self) -> RTxagcNonlegacyMcs3_1nssR {
        RTxagcNonlegacyMcs3_1nssR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_mcs0_1nss(
        &mut self,
    ) -> RTxagcNonlegacyMcs0_1nssW<'_, RegAxPwrByRateTable3Spec> {
        RTxagcNonlegacyMcs0_1nssW::new(self, 0)
    }
    #[doc = "Bits 8:14 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_mcs1_1nss(
        &mut self,
    ) -> RTxagcNonlegacyMcs1_1nssW<'_, RegAxPwrByRateTable3Spec> {
        RTxagcNonlegacyMcs1_1nssW::new(self, 8)
    }
    #[doc = "Bits 16:22 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_mcs2_1nss(
        &mut self,
    ) -> RTxagcNonlegacyMcs2_1nssW<'_, RegAxPwrByRateTable3Spec> {
        RTxagcNonlegacyMcs2_1nssW::new(self, 16)
    }
    #[doc = "Bits 24:30 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_nonlegacy_mcs3_1nss(
        &mut self,
    ) -> RTxagcNonlegacyMcs3_1nssW<'_, RegAxPwrByRateTable3Spec> {
        RTxagcNonlegacyMcs3_1nssW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_by_rate_table3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_by_rate_table3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrByRateTable3Spec;
impl crate::RegisterSpec for RegAxPwrByRateTable3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_by_rate_table3::R`](R) reader structure"]
impl crate::Readable for RegAxPwrByRateTable3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_by_rate_table3::W`](W) writer structure"]
impl crate::Writable for RegAxPwrByRateTable3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_BY_RATE_TABLE3 to value 0"]
impl crate::Resettable for RegAxPwrByRateTable3Spec {}
