#[doc = "Register `REG_AX_PWR_TSSI_TABLE9` reader"]
pub type R = crate::R<RegAxPwrTssiTable9Spec>;
#[doc = "Register `REG_AX_PWR_TSSI_TABLE9` writer"]
pub type W = crate::W<RegAxPwrTssiTable9Spec>;
#[doc = "Field `r_cbw_sub20_4` reader - "]
pub type RCbwSub20_4R = crate::FieldReader;
#[doc = "Field `r_cbw_sub20_4` writer - "]
pub type RCbwSub20_4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_cbw_sub20_5` reader - "]
pub type RCbwSub20_5R = crate::FieldReader;
#[doc = "Field `r_cbw_sub20_5` writer - "]
pub type RCbwSub20_5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_cbw_sub20_6` reader - "]
pub type RCbwSub20_6R = crate::FieldReader;
#[doc = "Field `r_cbw_sub20_6` writer - "]
pub type RCbwSub20_6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_cbw_sub20_7` reader - "]
pub type RCbwSub20_7R = crate::FieldReader;
#[doc = "Field `r_cbw_sub20_7` writer - "]
pub type RCbwSub20_7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_cbw_sub20_4(&self) -> RCbwSub20_4R {
        RCbwSub20_4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_cbw_sub20_5(&self) -> RCbwSub20_5R {
        RCbwSub20_5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_cbw_sub20_6(&self) -> RCbwSub20_6R {
        RCbwSub20_6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_cbw_sub20_7(&self) -> RCbwSub20_7R {
        RCbwSub20_7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_cbw_sub20_4(&mut self) -> RCbwSub20_4W<'_, RegAxPwrTssiTable9Spec> {
        RCbwSub20_4W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_cbw_sub20_5(&mut self) -> RCbwSub20_5W<'_, RegAxPwrTssiTable9Spec> {
        RCbwSub20_5W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_cbw_sub20_6(&mut self) -> RCbwSub20_6W<'_, RegAxPwrTssiTable9Spec> {
        RCbwSub20_6W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_cbw_sub20_7(&mut self) -> RCbwSub20_7W<'_, RegAxPwrTssiTable9Spec> {
        RCbwSub20_7W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tssi_table9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tssi_table9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTssiTable9Spec;
impl crate::RegisterSpec for RegAxPwrTssiTable9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tssi_table9::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTssiTable9Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tssi_table9::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTssiTable9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TSSI_TABLE9 to value 0"]
impl crate::Resettable for RegAxPwrTssiTable9Spec {}
