#[doc = "Register `GRP_PWR_MSK0` reader"]
pub type R = crate::R<GrpPwrMsk0Spec>;
#[doc = "Register `GRP_PWR_MSK0` writer"]
pub type W = crate::W<GrpPwrMsk0Spec>;
#[doc = "Field `grp_pmsk_hsys_e0` reader - "]
pub type GrpPmskHsysE0R = crate::FieldReader;
#[doc = "Field `grp_pmsk_hsys_e0` writer - "]
pub type GrpPmskHsysE0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `grp_pmsk_hsys_e1` reader - "]
pub type GrpPmskHsysE1R = crate::FieldReader<u16>;
#[doc = "Field `grp_pmsk_hsys_e1` writer - "]
pub type GrpPmskHsysE1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `grp_pmsk_lret_e0` reader - "]
pub type GrpPmskLretE0R = crate::BitReader;
#[doc = "Field `grp_pmsk_lret_e0` writer - "]
pub type GrpPmskLretE0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_pmsk_lspmc_e0` reader - "]
pub type GrpPmskLspmcE0R = crate::BitReader;
#[doc = "Field `grp_pmsk_lspmc_e0` writer - "]
pub type GrpPmskLspmcE0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_pmsk_aonpmc_e0` reader - "]
pub type GrpPmskAonpmcE0R = crate::FieldReader;
#[doc = "Field `grp_pmsk_aonpmc_e0` writer - "]
pub type GrpPmskAonpmcE0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `grp_pmsk_sdio_e0` reader - "]
pub type GrpPmskSdioE0R = crate::FieldReader;
#[doc = "Field `grp_pmsk_sdio_e0` writer - "]
pub type GrpPmskSdioE0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `grp_pmsk_sdio_e1` reader - "]
pub type GrpPmskSdioE1R = crate::BitReader;
#[doc = "Field `grp_pmsk_sdio_e1` writer - "]
pub type GrpPmskSdioE1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_pmsk_usb_e0` reader - "]
pub type GrpPmskUsbE0R = crate::BitReader;
#[doc = "Field `grp_pmsk_usb_e0` writer - "]
pub type GrpPmskUsbE0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_pmsk_wlk4_e0` reader - "]
pub type GrpPmskWlk4E0R = crate::FieldReader;
#[doc = "Field `grp_pmsk_wlk4_e0` writer - "]
pub type GrpPmskWlk4E0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `grp_pmsk_wpoff_e0` reader - "]
pub type GrpPmskWpoffE0R = crate::BitReader;
#[doc = "Field `grp_pmsk_wpoff_e0` writer - "]
pub type GrpPmskWpoffE0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn grp_pmsk_hsys_e0(&self) -> GrpPmskHsysE0R {
        GrpPmskHsysE0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:16"]
    #[inline(always)]
    pub fn grp_pmsk_hsys_e1(&self) -> GrpPmskHsysE1R {
        GrpPmskHsysE1R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn grp_pmsk_lret_e0(&self) -> GrpPmskLretE0R {
        GrpPmskLretE0R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn grp_pmsk_lspmc_e0(&self) -> GrpPmskLspmcE0R {
        GrpPmskLspmcE0R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn grp_pmsk_aonpmc_e0(&self) -> GrpPmskAonpmcE0R {
        GrpPmskAonpmcE0R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn grp_pmsk_sdio_e0(&self) -> GrpPmskSdioE0R {
        GrpPmskSdioE0R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn grp_pmsk_sdio_e1(&self) -> GrpPmskSdioE1R {
        GrpPmskSdioE1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn grp_pmsk_usb_e0(&self) -> GrpPmskUsbE0R {
        GrpPmskUsbE0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn grp_pmsk_wlk4_e0(&self) -> GrpPmskWlk4E0R {
        GrpPmskWlk4E0R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn grp_pmsk_wpoff_e0(&self) -> GrpPmskWpoffE0R {
        GrpPmskWpoffE0R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn grp_pmsk_hsys_e0(&mut self) -> GrpPmskHsysE0W<'_, GrpPwrMsk0Spec> {
        GrpPmskHsysE0W::new(self, 0)
    }
    #[doc = "Bits 8:16"]
    #[inline(always)]
    pub fn grp_pmsk_hsys_e1(&mut self) -> GrpPmskHsysE1W<'_, GrpPwrMsk0Spec> {
        GrpPmskHsysE1W::new(self, 8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn grp_pmsk_lret_e0(&mut self) -> GrpPmskLretE0W<'_, GrpPwrMsk0Spec> {
        GrpPmskLretE0W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn grp_pmsk_lspmc_e0(&mut self) -> GrpPmskLspmcE0W<'_, GrpPwrMsk0Spec> {
        GrpPmskLspmcE0W::new(self, 18)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn grp_pmsk_aonpmc_e0(&mut self) -> GrpPmskAonpmcE0W<'_, GrpPwrMsk0Spec> {
        GrpPmskAonpmcE0W::new(self, 19)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn grp_pmsk_sdio_e0(&mut self) -> GrpPmskSdioE0W<'_, GrpPwrMsk0Spec> {
        GrpPmskSdioE0W::new(self, 21)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn grp_pmsk_sdio_e1(&mut self) -> GrpPmskSdioE1W<'_, GrpPwrMsk0Spec> {
        GrpPmskSdioE1W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn grp_pmsk_usb_e0(&mut self) -> GrpPmskUsbE0W<'_, GrpPwrMsk0Spec> {
        GrpPmskUsbE0W::new(self, 24)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn grp_pmsk_wlk4_e0(&mut self) -> GrpPmskWlk4E0W<'_, GrpPwrMsk0Spec> {
        GrpPmskWlk4E0W::new(self, 25)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn grp_pmsk_wpoff_e0(&mut self) -> GrpPmskWpoffE0W<'_, GrpPwrMsk0Spec> {
        GrpPmskWpoffE0W::new(self, 27)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_pwr_msk0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_pwr_msk0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpPwrMsk0Spec;
impl crate::RegisterSpec for GrpPwrMsk0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_pwr_msk0::R`](R) reader structure"]
impl crate::Readable for GrpPwrMsk0Spec {}
#[doc = "`write(|w| ..)` method takes [`grp_pwr_msk0::W`](W) writer structure"]
impl crate::Writable for GrpPwrMsk0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_PWR_MSK0 to value 0x0fff_ffff"]
impl crate::Resettable for GrpPwrMsk0Spec {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
