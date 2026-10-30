#[doc = "Register `ANAPAR_POW_XTAL_0` reader"]
pub type R = crate::R<AnaparPowXtal0Spec>;
#[doc = "Register `ANAPAR_POW_XTAL_0` writer"]
pub type W = crate::W<AnaparPowXtal0Spec>;
#[doc = "Field `POW_BG` reader - Enable BG (BG for xtal and AFE) (has an option to turn it off)"]
pub type PowBgR = crate::BitReader;
#[doc = "Field `POW_XTAL` reader - 0 to 1 Enable xtal"]
pub type PowXtalR = crate::BitReader;
#[doc = "Field `xtal_state` reader - 0 to 1 Enable xtal"]
pub type XtalStateR = crate::FieldReader;
#[doc = "Field `xtal_ckrdy` reader - xtal clock ready status"]
pub type XtalCkrdyR = crate::BitReader;
#[doc = "Field `xmd_done` reader - xmd change ok status"]
pub type XmdDoneR = crate::BitReader;
#[doc = "Field `xtal_bg2hpmod_cyc` reader - XTAL arb bandgap enable to hp mode delay cycle @4M"]
pub type XtalBg2hpmodCycR = crate::FieldReader;
#[doc = "Field `xtal_bg2hpmod_cyc` writer - XTAL arb bandgap enable to hp mode delay cycle @4M"]
pub type XtalBg2hpmodCycW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Enable BG (BG for xtal and AFE) (has an option to turn it off)"]
    #[inline(always)]
    pub fn pow_bg(&self) -> PowBgR {
        PowBgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0 to 1 Enable xtal"]
    #[inline(always)]
    pub fn pow_xtal(&self) -> PowXtalR {
        PowXtalR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - 0 to 1 Enable xtal"]
    #[inline(always)]
    pub fn xtal_state(&self) -> XtalStateR {
        XtalStateR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - xtal clock ready status"]
    #[inline(always)]
    pub fn xtal_ckrdy(&self) -> XtalCkrdyR {
        XtalCkrdyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - xmd change ok status"]
    #[inline(always)]
    pub fn xmd_done(&self) -> XmdDoneR {
        XmdDoneR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:23 - XTAL arb bandgap enable to hp mode delay cycle @4M"]
    #[inline(always)]
    pub fn xtal_bg2hpmod_cyc(&self) -> XtalBg2hpmodCycR {
        XtalBg2hpmodCycR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - XTAL arb bandgap enable to hp mode delay cycle @4M"]
    #[inline(always)]
    pub fn xtal_bg2hpmod_cyc(&mut self) -> XtalBg2hpmodCycW<'_, AnaparPowXtal0Spec> {
        XtalBg2hpmodCycW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`anapar_pow_xtal_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anapar_pow_xtal_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaparPowXtal0Spec;
impl crate::RegisterSpec for AnaparPowXtal0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anapar_pow_xtal_0::R`](R) reader structure"]
impl crate::Readable for AnaparPowXtal0Spec {}
#[doc = "`write(|w| ..)` method takes [`anapar_pow_xtal_0::W`](W) writer structure"]
impl crate::Writable for AnaparPowXtal0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANAPAR_POW_XTAL_0 to value 0x0049_0000"]
impl crate::Resettable for AnaparPowXtal0Spec {
    const RESET_VALUE: u32 = 0x0049_0000;
}
