#[doc = "Register `REG_AX_PWR_BY_RATE_TABLE0` reader"]
pub type R = crate::R<RegAxPwrByRateTable0Spec>;
#[doc = "Register `REG_AX_PWR_BY_RATE_TABLE0` writer"]
pub type W = crate::W<RegAxPwrByRateTable0Spec>;
#[doc = "Field `r_txagc_cck1M` reader - s(7.1)"]
pub type RTxagcCck1mR = crate::FieldReader;
#[doc = "Field `r_txagc_cck1M` writer - s(7.1)"]
pub type RTxagcCck1mW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_txagc_cck2M` reader - s(7.1)"]
pub type RTxagcCck2mR = crate::FieldReader;
#[doc = "Field `r_txagc_cck2M` writer - s(7.1)"]
pub type RTxagcCck2mW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_txagc_cck5p5M` reader - s(7.1)"]
pub type RTxagcCck5p5mR = crate::FieldReader;
#[doc = "Field `r_txagc_cck5p5M` writer - s(7.1)"]
pub type RTxagcCck5p5mW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_txagc_cck11M` reader - s(7.1)"]
pub type RTxagcCck11mR = crate::FieldReader;
#[doc = "Field `r_txagc_cck11M` writer - s(7.1)"]
pub type RTxagcCck11mW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_cck1m(&self) -> RTxagcCck1mR {
        RTxagcCck1mR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_cck2m(&self) -> RTxagcCck2mR {
        RTxagcCck2mR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_cck5p5m(&self) -> RTxagcCck5p5mR {
        RTxagcCck5p5mR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_cck11m(&self) -> RTxagcCck11mR {
        RTxagcCck11mR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_cck1m(&mut self) -> RTxagcCck1mW<'_, RegAxPwrByRateTable0Spec> {
        RTxagcCck1mW::new(self, 0)
    }
    #[doc = "Bits 8:14 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_cck2m(&mut self) -> RTxagcCck2mW<'_, RegAxPwrByRateTable0Spec> {
        RTxagcCck2mW::new(self, 8)
    }
    #[doc = "Bits 16:22 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_cck5p5m(&mut self) -> RTxagcCck5p5mW<'_, RegAxPwrByRateTable0Spec> {
        RTxagcCck5p5mW::new(self, 16)
    }
    #[doc = "Bits 24:30 - s(7.1)"]
    #[inline(always)]
    pub fn r_txagc_cck11m(&mut self) -> RTxagcCck11mW<'_, RegAxPwrByRateTable0Spec> {
        RTxagcCck11mW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_by_rate_table0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_by_rate_table0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrByRateTable0Spec;
impl crate::RegisterSpec for RegAxPwrByRateTable0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_by_rate_table0::R`](R) reader structure"]
impl crate::Readable for RegAxPwrByRateTable0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_by_rate_table0::W`](W) writer structure"]
impl crate::Writable for RegAxPwrByRateTable0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_BY_RATE_TABLE0 to value 0"]
impl crate::Resettable for RegAxPwrByRateTable0Spec {}
