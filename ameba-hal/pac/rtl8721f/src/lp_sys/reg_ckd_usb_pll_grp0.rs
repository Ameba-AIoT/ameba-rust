#[doc = "Register `REG_CKD_USB_PLL_GRP0` reader"]
pub type R = crate::R<RegCkdUsbPllGrp0Spec>;
#[doc = "Register `REG_CKD_USB_PLL_GRP0` writer"]
pub type W = crate::W<RegCkdUsbPllGrp0Spec>;
#[doc = "Field `ckd_usb_pll_hp` reader - usb_pll divider for hp 0: Not support 1: div2 ... DIV_MIN:1"]
pub type CkdUsbPllHpR = crate::FieldReader;
#[doc = "Field `ckd_usb_pll_hp` writer - usb_pll divider for hp 0: Not support 1: div2 ... DIV_MIN:1"]
pub type CkdUsbPllHpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ckd_usb_pll_hperi` reader - usb_pll divider for hperi 0: Not support 1: div2 ... DIV_MIN:1"]
pub type CkdUsbPllHperiR = crate::FieldReader;
#[doc = "Field `ckd_usb_pll_hperi` writer - usb_pll divider for hperi 0: Not support 1: div2 ... DIV_MIN:1"]
pub type CkdUsbPllHperiW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ckd_usb_pll_spic` reader - usb_pll divider for spic 0: Not support 1: div2 ... DIV_MIN:1"]
pub type CkdUsbPllSpicR = crate::FieldReader;
#[doc = "Field `ckd_usb_pll_spic` writer - usb_pll divider for spic 0: Not support 1: div2 ... DIV_MIN:1"]
pub type CkdUsbPllSpicW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ckd_usb_pll_psram` reader - usb_pll divider for psram 0: Not support 1: div2 ... DIV_MIN:1"]
pub type CkdUsbPllPsramR = crate::FieldReader;
#[doc = "Field `ckd_usb_pll_psram` writer - usb_pll divider for psram 0: Not support 1: div2 ... DIV_MIN:1"]
pub type CkdUsbPllPsramW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ckd_usb_pll_shperi` reader - usb_pll divider for shperi 0: div1 1: div2 ... DIV_MIN:0"]
pub type CkdUsbPllShperiR = crate::FieldReader;
#[doc = "Field `ckd_usb_pll_shperi` writer - usb_pll divider for shperi 0: div1 1: div2 ... DIV_MIN:0"]
pub type CkdUsbPllShperiW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ckd_usb_pll_can` reader - usb_pll divider for can 0: div1 1: div2 ... Note: default 960M/8=120M DIV_MIN:0"]
pub type CkdUsbPllCanR = crate::FieldReader;
#[doc = "Field `ckd_usb_pll_can` writer - usb_pll divider for can 0: div1 1: div2 ... Note: default 960M/8=120M DIV_MIN:0"]
pub type CkdUsbPllCanW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - usb_pll divider for hp 0: Not support 1: div2 ... DIV_MIN:1"]
    #[inline(always)]
    pub fn ckd_usb_pll_hp(&self) -> CkdUsbPllHpR {
        CkdUsbPllHpR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - usb_pll divider for hperi 0: Not support 1: div2 ... DIV_MIN:1"]
    #[inline(always)]
    pub fn ckd_usb_pll_hperi(&self) -> CkdUsbPllHperiR {
        CkdUsbPllHperiR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - usb_pll divider for spic 0: Not support 1: div2 ... DIV_MIN:1"]
    #[inline(always)]
    pub fn ckd_usb_pll_spic(&self) -> CkdUsbPllSpicR {
        CkdUsbPllSpicR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - usb_pll divider for psram 0: Not support 1: div2 ... DIV_MIN:1"]
    #[inline(always)]
    pub fn ckd_usb_pll_psram(&self) -> CkdUsbPllPsramR {
        CkdUsbPllPsramR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - usb_pll divider for shperi 0: div1 1: div2 ... DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_usb_pll_shperi(&self) -> CkdUsbPllShperiR {
        CkdUsbPllShperiR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - usb_pll divider for can 0: div1 1: div2 ... Note: default 960M/8=120M DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_usb_pll_can(&self) -> CkdUsbPllCanR {
        CkdUsbPllCanR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - usb_pll divider for hp 0: Not support 1: div2 ... DIV_MIN:1"]
    #[inline(always)]
    pub fn ckd_usb_pll_hp(&mut self) -> CkdUsbPllHpW<'_, RegCkdUsbPllGrp0Spec> {
        CkdUsbPllHpW::new(self, 0)
    }
    #[doc = "Bits 4:7 - usb_pll divider for hperi 0: Not support 1: div2 ... DIV_MIN:1"]
    #[inline(always)]
    pub fn ckd_usb_pll_hperi(&mut self) -> CkdUsbPllHperiW<'_, RegCkdUsbPllGrp0Spec> {
        CkdUsbPllHperiW::new(self, 4)
    }
    #[doc = "Bits 8:11 - usb_pll divider for spic 0: Not support 1: div2 ... DIV_MIN:1"]
    #[inline(always)]
    pub fn ckd_usb_pll_spic(&mut self) -> CkdUsbPllSpicW<'_, RegCkdUsbPllGrp0Spec> {
        CkdUsbPllSpicW::new(self, 8)
    }
    #[doc = "Bits 12:15 - usb_pll divider for psram 0: Not support 1: div2 ... DIV_MIN:1"]
    #[inline(always)]
    pub fn ckd_usb_pll_psram(&mut self) -> CkdUsbPllPsramW<'_, RegCkdUsbPllGrp0Spec> {
        CkdUsbPllPsramW::new(self, 12)
    }
    #[doc = "Bits 16:19 - usb_pll divider for shperi 0: div1 1: div2 ... DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_usb_pll_shperi(&mut self) -> CkdUsbPllShperiW<'_, RegCkdUsbPllGrp0Spec> {
        CkdUsbPllShperiW::new(self, 16)
    }
    #[doc = "Bits 20:23 - usb_pll divider for can 0: div1 1: div2 ... Note: default 960M/8=120M DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_usb_pll_can(&mut self) -> CkdUsbPllCanW<'_, RegCkdUsbPllGrp0Spec> {
        CkdUsbPllCanW::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ckd_usb_pll_grp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ckd_usb_pll_grp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCkdUsbPllGrp0Spec;
impl crate::RegisterSpec for RegCkdUsbPllGrp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ckd_usb_pll_grp0::R`](R) reader structure"]
impl crate::Readable for RegCkdUsbPllGrp0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ckd_usb_pll_grp0::W`](W) writer structure"]
impl crate::Writable for RegCkdUsbPllGrp0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CKD_USB_PLL_GRP0 to value 0x0074_3493"]
impl crate::Resettable for RegCkdUsbPllGrp0Spec {
    const RESET_VALUE: u32 = 0x0074_3493;
}
