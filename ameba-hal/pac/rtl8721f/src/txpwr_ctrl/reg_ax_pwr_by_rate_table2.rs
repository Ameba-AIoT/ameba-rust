#[doc = "Register `REG_AX_PWR_BY_RATE_TABLE2` reader"]
pub type R = crate::R<RegAxPwrByRateTable2Spec>;
#[doc = "Register `REG_AX_PWR_BY_RATE_TABLE2` writer"]
pub type W = crate::W<RegAxPwrByRateTable2Spec>;
#[doc = "Field `r_txagc_legacy24M` reader - s(7.1)"]
pub type RTxagcLegacy24mR = crate::FieldReader;
#[doc = "Field `r_txagc_legacy24M` writer - s(7.1)"]
pub type RTxagcLegacy24mW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_txagc_legacy36M` reader - s(7.1)"]
pub type RTxagcLegacy36mR = crate::FieldReader;
#[doc = "Field `r_txagc_legacy36M` writer - s(7.1)"]
pub type RTxagcLegacy36mW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_txagc_legacy48M` reader - s(7.1)"]
pub type RTxagcLegacy48mR = crate::FieldReader;
#[doc = "Field `r_txagc_legacy48M` writer - s(7.1)"]
pub type RTxagcLegacy48mW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_txagc_legacy54M` reader - s(7.1)"]
pub type RTxagcLegacy54mR = crate::FieldReader;
#[doc = "Field `r_txagc_legacy54M` writer - s(7.1)"]
pub type RTxagcLegacy54mW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_legacy24m(&self) -> RTxagcLegacy24mR {
        RTxagcLegacy24mR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_legacy36m(&self) -> RTxagcLegacy36mR {
        RTxagcLegacy36mR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_legacy48m(&self) -> RTxagcLegacy48mR {
        RTxagcLegacy48mR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_legacy54m(&self) -> RTxagcLegacy54mR {
        RTxagcLegacy54mR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_legacy24m(&mut self) -> RTxagcLegacy24mW<'_, RegAxPwrByRateTable2Spec> {
        RTxagcLegacy24mW::new(self, 0)
    }
    #[doc = "Bits 8:14 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_legacy36m(&mut self) -> RTxagcLegacy36mW<'_, RegAxPwrByRateTable2Spec> {
        RTxagcLegacy36mW::new(self, 8)
    }
    #[doc = "Bits 16:22 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_legacy48m(&mut self) -> RTxagcLegacy48mW<'_, RegAxPwrByRateTable2Spec> {
        RTxagcLegacy48mW::new(self, 16)
    }
    #[doc = "Bits 24:30 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_legacy54m(&mut self) -> RTxagcLegacy54mW<'_, RegAxPwrByRateTable2Spec> {
        RTxagcLegacy54mW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_by_rate_table2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_by_rate_table2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrByRateTable2Spec;
impl crate::RegisterSpec for RegAxPwrByRateTable2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_by_rate_table2::R`](R) reader structure"]
impl crate::Readable for RegAxPwrByRateTable2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_by_rate_table2::W`](W) writer structure"]
impl crate::Writable for RegAxPwrByRateTable2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_BY_RATE_TABLE2 to value 0"]
impl crate::Resettable for RegAxPwrByRateTable2Spec {}
