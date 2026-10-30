#[doc = "Register `REG_CKSL_GRP0` reader"]
pub type R = crate::R<RegCkslGrp0Spec>;
#[doc = "Register `REG_CKSL_GRP0` writer"]
pub type W = crate::W<RegCkslGrp0Spec>;
#[doc = "Field `cksl_hp` reader - HP clock select,used for KM4 and sram ##x0: xtal ##01: usb_pll ##11: sys_pll"]
pub type CkslHpR = crate::FieldReader;
#[doc = "Field `cksl_hp` writer - HP clock select,used for KM4 and sram ##x0: xtal ##01: usb_pll ##11: sys_pll"]
pub type CkslHpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cksl_lperi` reader - LPERI clock select ##0: osc4m ##1: xtal_40m"]
pub type CkslLperiR = crate::BitReader;
#[doc = "Field `cksl_lperi` writer - LPERI clock select ##0: osc4m ##1: xtal_40m"]
pub type CkslLperiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cksl_otpc` reader - otpc clock selection ##0: osc4m ##1: xtal Note: this bit will be set by pmc auto, when SW use otp, it will be set to 1"]
pub type CkslOtpcR = crate::BitReader;
#[doc = "Field `cksl_otpc` writer - otpc clock selection ##0: osc4m ##1: xtal Note: this bit will be set by pmc auto, when SW use otp, it will be set to 1"]
pub type CkslOtpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cksl_hperi` reader - hperi clock seletion ##x0: xtal ##01: usb_pll ##11: sys_pll"]
pub type CkslHperiR = crate::FieldReader;
#[doc = "Field `cksl_hperi` writer - hperi clock seletion ##x0: xtal ##01: usb_pll ##11: sys_pll"]
pub type CkslHperiW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cksl_spic` reader - spic clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
pub type CkslSpicR = crate::FieldReader;
#[doc = "Field `cksl_spic` writer - spic clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
pub type CkslSpicW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cksl_psram` reader - psram clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
pub type CkslPsramR = crate::FieldReader;
#[doc = "Field `cksl_psram` writer - psram clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
pub type CkslPsramW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cksl_shperi` reader - peri_shclk clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
pub type CkslShperiR = crate::FieldReader;
#[doc = "Field `cksl_shperi` writer - peri_shclk clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
pub type CkslShperiW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cksl_can` reader - can clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
pub type CkslCanR = crate::FieldReader;
#[doc = "Field `cksl_can` writer - can clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
pub type CkslCanW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cksl_vo` reader - vo clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
pub type CkslVoR = crate::FieldReader;
#[doc = "Field `cksl_vo` writer - vo clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
pub type CkslVoW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cksl_gmac` reader - gmac clock select ##x0: ext50m ##01: usb_pll ##11: sys_pll"]
pub type CkslGmacR = crate::FieldReader;
#[doc = "Field `cksl_gmac` writer - gmac clock select ##x0: ext50m ##01: usb_pll ##11: sys_pll"]
pub type CkslGmacW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cksl_i2s` reader - i2s clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
pub type CkslI2sR = crate::FieldReader;
#[doc = "Field `cksl_i2s` writer - i2s clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
pub type CkslI2sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cksl_sdh` reader - sdio host input clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
pub type CkslSdhR = crate::FieldReader;
#[doc = "Field `cksl_sdh` writer - sdio host input clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
pub type CkslSdhW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - HP clock select,used for KM4 and sram ##x0: xtal ##01: usb_pll ##11: sys_pll"]
    #[inline(always)]
    pub fn cksl_hp(&self) -> CkslHpR {
        CkslHpR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - LPERI clock select ##0: osc4m ##1: xtal_40m"]
    #[inline(always)]
    pub fn cksl_lperi(&self) -> CkslLperiR {
        CkslLperiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - otpc clock selection ##0: osc4m ##1: xtal Note: this bit will be set by pmc auto, when SW use otp, it will be set to 1"]
    #[inline(always)]
    pub fn cksl_otpc(&self) -> CkslOtpcR {
        CkslOtpcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - hperi clock seletion ##x0: xtal ##01: usb_pll ##11: sys_pll"]
    #[inline(always)]
    pub fn cksl_hperi(&self) -> CkslHperiR {
        CkslHperiR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - spic clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
    #[inline(always)]
    pub fn cksl_spic(&self) -> CkslSpicR {
        CkslSpicR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - psram clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
    #[inline(always)]
    pub fn cksl_psram(&self) -> CkslPsramR {
        CkslPsramR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - peri_shclk clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
    #[inline(always)]
    pub fn cksl_shperi(&self) -> CkslShperiR {
        CkslShperiR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - can clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
    #[inline(always)]
    pub fn cksl_can(&self) -> CkslCanR {
        CkslCanR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - vo clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
    #[inline(always)]
    pub fn cksl_vo(&self) -> CkslVoR {
        CkslVoR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - gmac clock select ##x0: ext50m ##01: usb_pll ##11: sys_pll"]
    #[inline(always)]
    pub fn cksl_gmac(&self) -> CkslGmacR {
        CkslGmacR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - i2s clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
    #[inline(always)]
    pub fn cksl_i2s(&self) -> CkslI2sR {
        CkslI2sR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - sdio host input clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
    #[inline(always)]
    pub fn cksl_sdh(&self) -> CkslSdhR {
        CkslSdhR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - HP clock select,used for KM4 and sram ##x0: xtal ##01: usb_pll ##11: sys_pll"]
    #[inline(always)]
    pub fn cksl_hp(&mut self) -> CkslHpW<'_, RegCkslGrp0Spec> {
        CkslHpW::new(self, 0)
    }
    #[doc = "Bit 2 - LPERI clock select ##0: osc4m ##1: xtal_40m"]
    #[inline(always)]
    pub fn cksl_lperi(&mut self) -> CkslLperiW<'_, RegCkslGrp0Spec> {
        CkslLperiW::new(self, 2)
    }
    #[doc = "Bit 3 - otpc clock selection ##0: osc4m ##1: xtal Note: this bit will be set by pmc auto, when SW use otp, it will be set to 1"]
    #[inline(always)]
    pub fn cksl_otpc(&mut self) -> CkslOtpcW<'_, RegCkslGrp0Spec> {
        CkslOtpcW::new(self, 3)
    }
    #[doc = "Bits 4:5 - hperi clock seletion ##x0: xtal ##01: usb_pll ##11: sys_pll"]
    #[inline(always)]
    pub fn cksl_hperi(&mut self) -> CkslHperiW<'_, RegCkslGrp0Spec> {
        CkslHperiW::new(self, 4)
    }
    #[doc = "Bits 6:7 - spic clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
    #[inline(always)]
    pub fn cksl_spic(&mut self) -> CkslSpicW<'_, RegCkslGrp0Spec> {
        CkslSpicW::new(self, 6)
    }
    #[doc = "Bits 8:9 - psram clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
    #[inline(always)]
    pub fn cksl_psram(&mut self) -> CkslPsramW<'_, RegCkslGrp0Spec> {
        CkslPsramW::new(self, 8)
    }
    #[doc = "Bits 10:11 - peri_shclk clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
    #[inline(always)]
    pub fn cksl_shperi(&mut self) -> CkslShperiW<'_, RegCkslGrp0Spec> {
        CkslShperiW::new(self, 10)
    }
    #[doc = "Bits 12:13 - can clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
    #[inline(always)]
    pub fn cksl_can(&mut self) -> CkslCanW<'_, RegCkslGrp0Spec> {
        CkslCanW::new(self, 12)
    }
    #[doc = "Bits 14:15 - vo clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
    #[inline(always)]
    pub fn cksl_vo(&mut self) -> CkslVoW<'_, RegCkslGrp0Spec> {
        CkslVoW::new(self, 14)
    }
    #[doc = "Bits 16:17 - gmac clock select ##x0: ext50m ##01: usb_pll ##11: sys_pll"]
    #[inline(always)]
    pub fn cksl_gmac(&mut self) -> CkslGmacW<'_, RegCkslGrp0Spec> {
        CkslGmacW::new(self, 16)
    }
    #[doc = "Bits 18:19 - i2s clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
    #[inline(always)]
    pub fn cksl_i2s(&mut self) -> CkslI2sW<'_, RegCkslGrp0Spec> {
        CkslI2sW::new(self, 18)
    }
    #[doc = "Bits 20:21 - sdio host input clock select ##x0: xtal ##01: usb_pll ##11: sys_pll"]
    #[inline(always)]
    pub fn cksl_sdh(&mut self) -> CkslSdhW<'_, RegCkslGrp0Spec> {
        CkslSdhW::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cksl_grp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cksl_grp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCkslGrp0Spec;
impl crate::RegisterSpec for RegCkslGrp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cksl_grp0::R`](R) reader structure"]
impl crate::Readable for RegCkslGrp0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_cksl_grp0::W`](W) writer structure"]
impl crate::Writable for RegCkslGrp0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CKSL_GRP0 to value 0x0015_5500"]
impl crate::Resettable for RegCkslGrp0Spec {
    const RESET_VALUE: u32 = 0x0015_5500;
}
