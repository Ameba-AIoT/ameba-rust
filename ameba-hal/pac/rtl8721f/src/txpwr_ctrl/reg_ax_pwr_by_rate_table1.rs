#[doc = "Register `REG_AX_PWR_BY_RATE_TABLE1` reader"]
pub type R = crate::R<RegAxPwrByRateTable1Spec>;
#[doc = "Register `REG_AX_PWR_BY_RATE_TABLE1` writer"]
pub type W = crate::W<RegAxPwrByRateTable1Spec>;
#[doc = "Field `r_txagc_legacy6M` reader - s(7.1)"]
pub type RTxagcLegacy6mR = crate::FieldReader;
#[doc = "Field `r_txagc_legacy6M` writer - s(7.1)"]
pub type RTxagcLegacy6mW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_txagc_legacy9M` reader - s(7.1)"]
pub type RTxagcLegacy9mR = crate::FieldReader;
#[doc = "Field `r_txagc_legacy9M` writer - s(7.1)"]
pub type RTxagcLegacy9mW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_txagc_legacy12M` reader - s(7.1)"]
pub type RTxagcLegacy12mR = crate::FieldReader;
#[doc = "Field `r_txagc_legacy12M` writer - s(7.1)"]
pub type RTxagcLegacy12mW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_txagc_legacy18M` reader - s(7.1)"]
pub type RTxagcLegacy18mR = crate::FieldReader;
#[doc = "Field `r_txagc_legacy18M` writer - s(7.1)"]
pub type RTxagcLegacy18mW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_legacy6m(&self) -> RTxagcLegacy6mR {
        RTxagcLegacy6mR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_legacy9m(&self) -> RTxagcLegacy9mR {
        RTxagcLegacy9mR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_legacy12m(&self) -> RTxagcLegacy12mR {
        RTxagcLegacy12mR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_legacy18m(&self) -> RTxagcLegacy18mR {
        RTxagcLegacy18mR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_legacy6m(&mut self) -> RTxagcLegacy6mW<'_, RegAxPwrByRateTable1Spec> {
        RTxagcLegacy6mW::new(self, 0)
    }
    #[doc = "Bits 8:14 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_legacy9m(&mut self) -> RTxagcLegacy9mW<'_, RegAxPwrByRateTable1Spec> {
        RTxagcLegacy9mW::new(self, 8)
    }
    #[doc = "Bits 16:22 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_legacy12m(&mut self) -> RTxagcLegacy12mW<'_, RegAxPwrByRateTable1Spec> {
        RTxagcLegacy12mW::new(self, 16)
    }
    #[doc = "Bits 24:30 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_legacy18m(&mut self) -> RTxagcLegacy18mW<'_, RegAxPwrByRateTable1Spec> {
        RTxagcLegacy18mW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_by_rate_table1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_by_rate_table1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrByRateTable1Spec;
impl crate::RegisterSpec for RegAxPwrByRateTable1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_by_rate_table1::R`](R) reader structure"]
impl crate::Readable for RegAxPwrByRateTable1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_by_rate_table1::W`](W) writer structure"]
impl crate::Writable for RegAxPwrByRateTable1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_BY_RATE_TABLE1 to value 0"]
impl crate::Resettable for RegAxPwrByRateTable1Spec {}
