#[doc = "Register `REG_AX_PWR_UL_CTRL1` reader"]
pub type R = crate::R<RegAxPwrUlCtrl1Spec>;
#[doc = "Register `REG_AX_PWR_UL_CTRL1` writer"]
pub type W = crate::W<RegAxPwrUlCtrl1Spec>;
#[doc = "Field `r_MACID0` reader - STA_UL"]
pub type RMacid0R = crate::FieldReader;
#[doc = "Field `r_MACID0` writer - STA_UL"]
pub type RMacid0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_MACID1` reader - STA_UL"]
pub type RMacid1R = crate::FieldReader;
#[doc = "Field `r_MACID1` writer - STA_UL"]
pub type RMacid1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_MACID2` reader - STA_UL"]
pub type RMacid2R = crate::FieldReader;
#[doc = "Field `r_MACID2` writer - STA_UL"]
pub type RMacid2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_MACID3` reader - STA_UL"]
pub type RMacid3R = crate::FieldReader;
#[doc = "Field `r_MACID3` writer - STA_UL"]
pub type RMacid3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - STA_UL"]
    #[inline(always)]
    pub fn r_macid0(&self) -> RMacid0R {
        RMacid0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - STA_UL"]
    #[inline(always)]
    pub fn r_macid1(&self) -> RMacid1R {
        RMacid1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - STA_UL"]
    #[inline(always)]
    pub fn r_macid2(&self) -> RMacid2R {
        RMacid2R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - STA_UL"]
    #[inline(always)]
    pub fn r_macid3(&self) -> RMacid3R {
        RMacid3R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - STA_UL"]
    #[inline(always)]
    pub fn r_macid0(&mut self) -> RMacid0W<'_, RegAxPwrUlCtrl1Spec> {
        RMacid0W::new(self, 0)
    }
    #[doc = "Bits 8:14 - STA_UL"]
    #[inline(always)]
    pub fn r_macid1(&mut self) -> RMacid1W<'_, RegAxPwrUlCtrl1Spec> {
        RMacid1W::new(self, 8)
    }
    #[doc = "Bits 16:22 - STA_UL"]
    #[inline(always)]
    pub fn r_macid2(&mut self) -> RMacid2W<'_, RegAxPwrUlCtrl1Spec> {
        RMacid2W::new(self, 16)
    }
    #[doc = "Bits 24:30 - STA_UL"]
    #[inline(always)]
    pub fn r_macid3(&mut self) -> RMacid3W<'_, RegAxPwrUlCtrl1Spec> {
        RMacid3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_ul_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_ul_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrUlCtrl1Spec;
impl crate::RegisterSpec for RegAxPwrUlCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_ul_ctrl1::R`](R) reader structure"]
impl crate::Readable for RegAxPwrUlCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_ul_ctrl1::W`](W) writer structure"]
impl crate::Writable for RegAxPwrUlCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_UL_CTRL1 to value 0"]
impl crate::Resettable for RegAxPwrUlCtrl1Spec {}
