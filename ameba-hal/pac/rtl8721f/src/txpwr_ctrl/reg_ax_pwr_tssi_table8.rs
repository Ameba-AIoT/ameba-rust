#[doc = "Register `REG_AX_PWR_TSSI_TABLE8` reader"]
pub type R = crate::R<RegAxPwrTssiTable8Spec>;
#[doc = "Register `REG_AX_PWR_TSSI_TABLE8` writer"]
pub type W = crate::W<RegAxPwrTssiTable8Spec>;
#[doc = "Field `r_cbw_sub20_0` reader - "]
pub type RCbwSub20_0R = crate::FieldReader;
#[doc = "Field `r_cbw_sub20_0` writer - "]
pub type RCbwSub20_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_cbw_sub20_1` reader - "]
pub type RCbwSub20_1R = crate::FieldReader;
#[doc = "Field `r_cbw_sub20_1` writer - "]
pub type RCbwSub20_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_cbw_sub20_2` reader - "]
pub type RCbwSub20_2R = crate::FieldReader;
#[doc = "Field `r_cbw_sub20_2` writer - "]
pub type RCbwSub20_2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_cbw_sub20_3` reader - "]
pub type RCbwSub20_3R = crate::FieldReader;
#[doc = "Field `r_cbw_sub20_3` writer - "]
pub type RCbwSub20_3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_cbw_sub20_0(&self) -> RCbwSub20_0R {
        RCbwSub20_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_cbw_sub20_1(&self) -> RCbwSub20_1R {
        RCbwSub20_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_cbw_sub20_2(&self) -> RCbwSub20_2R {
        RCbwSub20_2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_cbw_sub20_3(&self) -> RCbwSub20_3R {
        RCbwSub20_3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_cbw_sub20_0(&mut self) -> RCbwSub20_0W<'_, RegAxPwrTssiTable8Spec> {
        RCbwSub20_0W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_cbw_sub20_1(&mut self) -> RCbwSub20_1W<'_, RegAxPwrTssiTable8Spec> {
        RCbwSub20_1W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_cbw_sub20_2(&mut self) -> RCbwSub20_2W<'_, RegAxPwrTssiTable8Spec> {
        RCbwSub20_2W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_cbw_sub20_3(&mut self) -> RCbwSub20_3W<'_, RegAxPwrTssiTable8Spec> {
        RCbwSub20_3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tssi_table8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tssi_table8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTssiTable8Spec;
impl crate::RegisterSpec for RegAxPwrTssiTable8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tssi_table8::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTssiTable8Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tssi_table8::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTssiTable8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TSSI_TABLE8 to value 0"]
impl crate::Resettable for RegAxPwrTssiTable8Spec {}
