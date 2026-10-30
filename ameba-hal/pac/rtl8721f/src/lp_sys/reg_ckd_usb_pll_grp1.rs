#[doc = "Register `REG_CKD_USB_PLL_GRP1` reader"]
pub type R = crate::R<RegCkdUsbPllGrp1Spec>;
#[doc = "Register `REG_CKD_USB_PLL_GRP1` writer"]
pub type W = crate::W<RegCkdUsbPllGrp1Spec>;
#[doc = "Field `ckd_usb_pll_vo` reader - usb_pll divider for vo/panel clk 0: div1 1: div2 ... Note: max 200M DIV_MIN:0"]
pub type CkdUsbPllVoR = crate::FieldReader;
#[doc = "Field `ckd_usb_pll_vo` writer - usb_pll divider for vo/panel clk 0: div1 1: div2 ... Note: max 200M DIV_MIN:0"]
pub type CkdUsbPllVoW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ckd_usb_pll_gmac` reader - usb_pll divider for vo/panel clk 0: div1 1: div2 ... Note: fix to 50M DIV_MIN:0"]
pub type CkdUsbPllGmacR = crate::FieldReader;
#[doc = "Field `ckd_usb_pll_gmac` writer - usb_pll divider for vo/panel clk 0: div1 1: div2 ... Note: fix to 50M DIV_MIN:0"]
pub type CkdUsbPllGmacW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ckd_usb_pll_i2s` reader - usb_pll divider for i2s clk 0: div1 1: div2 ... DIV_MIN:0"]
pub type CkdUsbPllI2sR = crate::FieldReader;
#[doc = "Field `ckd_usb_pll_i2s` writer - usb_pll divider for i2s clk 0: div1 1: div2 ... DIV_MIN:0"]
pub type CkdUsbPllI2sW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ckd_usb_pll_sdh` reader - usb_pll divider for sdh clk 0: div1 1: div2 ... DIV_MIN:0"]
pub type CkdUsbPllSdhR = crate::FieldReader;
#[doc = "Field `ckd_usb_pll_sdh` writer - usb_pll divider for sdh clk 0: div1 1: div2 ... DIV_MIN:0"]
pub type CkdUsbPllSdhW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:5 - usb_pll divider for vo/panel clk 0: div1 1: div2 ... Note: max 200M DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_usb_pll_vo(&self) -> CkdUsbPllVoR {
        CkdUsbPllVoR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - usb_pll divider for vo/panel clk 0: div1 1: div2 ... Note: fix to 50M DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_usb_pll_gmac(&self) -> CkdUsbPllGmacR {
        CkdUsbPllGmacR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:21 - usb_pll divider for i2s clk 0: div1 1: div2 ... DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_usb_pll_i2s(&self) -> CkdUsbPllI2sR {
        CkdUsbPllI2sR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - usb_pll divider for sdh clk 0: div1 1: div2 ... DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_usb_pll_sdh(&self) -> CkdUsbPllSdhR {
        CkdUsbPllSdhR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - usb_pll divider for vo/panel clk 0: div1 1: div2 ... Note: max 200M DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_usb_pll_vo(&mut self) -> CkdUsbPllVoW<'_, RegCkdUsbPllGrp1Spec> {
        CkdUsbPllVoW::new(self, 0)
    }
    #[doc = "Bits 8:12 - usb_pll divider for vo/panel clk 0: div1 1: div2 ... Note: fix to 50M DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_usb_pll_gmac(&mut self) -> CkdUsbPllGmacW<'_, RegCkdUsbPllGrp1Spec> {
        CkdUsbPllGmacW::new(self, 8)
    }
    #[doc = "Bits 16:21 - usb_pll divider for i2s clk 0: div1 1: div2 ... DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_usb_pll_i2s(&mut self) -> CkdUsbPllI2sW<'_, RegCkdUsbPllGrp1Spec> {
        CkdUsbPllI2sW::new(self, 16)
    }
    #[doc = "Bits 24:27 - usb_pll divider for sdh clk 0: div1 1: div2 ... DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_usb_pll_sdh(&mut self) -> CkdUsbPllSdhW<'_, RegCkdUsbPllGrp1Spec> {
        CkdUsbPllSdhW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ckd_usb_pll_grp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ckd_usb_pll_grp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCkdUsbPllGrp1Spec;
impl crate::RegisterSpec for RegCkdUsbPllGrp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ckd_usb_pll_grp1::R`](R) reader structure"]
impl crate::Readable for RegCkdUsbPllGrp1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ckd_usb_pll_grp1::W`](W) writer structure"]
impl crate::Writable for RegCkdUsbPllGrp1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CKD_USB_PLL_GRP1 to value 0x0909_120f"]
impl crate::Resettable for RegCkdUsbPllGrp1Spec {
    const RESET_VALUE: u32 = 0x0909_120f;
}
