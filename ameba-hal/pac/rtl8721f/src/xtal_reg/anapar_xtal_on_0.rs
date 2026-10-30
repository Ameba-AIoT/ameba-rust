#[doc = "Register `ANAPAR_XTAL_ON_0` reader"]
pub type R = crate::R<AnaparXtalOn0Spec>;
#[doc = "Register `ANAPAR_XTAL_ON_0` writer"]
pub type W = crate::W<AnaparXtalOn0Spec>;
#[doc = "Field `XTAL_GMP` reader - GMP: 0~31"]
pub type XtalGmpR = crate::FieldReader;
#[doc = "Field `XTAL_GMP` writer - GMP: 0~31"]
pub type XtalGmpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `XTAL_GMN` reader - GMN: 0~31"]
pub type XtalGmnR = crate::FieldReader;
#[doc = "Field `XTAL_GMN` writer - GMN: 0~31"]
pub type XtalGmnW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `XTAL_SC_XI` reader - 1: reduce cap, 0: increase cap. (220fF per unit) \\[efuse for BT only \\]"]
pub type XtalScXiR = crate::FieldReader;
#[doc = "Field `XTAL_SC_XI` writer - 1: reduce cap, 0: increase cap. (220fF per unit) \\[efuse for BT only \\]"]
pub type XtalScXiW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `XTAL_SC_XO` reader - 1: reduce cap, 0: increase cap. (220fF per unit) \\[efuse for BT only \\]"]
pub type XtalScXoR = crate::FieldReader;
#[doc = "Field `XTAL_SC_XO` writer - 1: reduce cap, 0: increase cap. (220fF per unit) \\[efuse for BT only \\]"]
pub type XtalScXoW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `XTAL_SC_INIT` reader - 0: reduce cap, 1: increase cap. (220fF per unit) \\[efuse for BT only \\] set all-0 when an external capacitor is populated"]
pub type XtalScInitR = crate::FieldReader;
#[doc = "Field `XTAL_SC_INIT` writer - 0: reduce cap, 1: increase cap. (220fF per unit) \\[efuse for BT only \\] set all-0 when an external capacitor is populated"]
pub type XtalScInitW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:4 - GMP: 0~31"]
    #[inline(always)]
    pub fn xtal_gmp(&self) -> XtalGmpR {
        XtalGmpR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - GMN: 0~31"]
    #[inline(always)]
    pub fn xtal_gmn(&self) -> XtalGmnR {
        XtalGmnR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:16 - 1: reduce cap, 0: increase cap. (220fF per unit) \\[efuse for BT only \\]"]
    #[inline(always)]
    pub fn xtal_sc_xi(&self) -> XtalScXiR {
        XtalScXiR::new(((self.bits >> 10) & 0x7f) as u8)
    }
    #[doc = "Bits 17:23 - 1: reduce cap, 0: increase cap. (220fF per unit) \\[efuse for BT only \\]"]
    #[inline(always)]
    pub fn xtal_sc_xo(&self) -> XtalScXoR {
        XtalScXoR::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - 0: reduce cap, 1: increase cap. (220fF per unit) \\[efuse for BT only \\] set all-0 when an external capacitor is populated"]
    #[inline(always)]
    pub fn xtal_sc_init(&self) -> XtalScInitR {
        XtalScInitR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - GMP: 0~31"]
    #[inline(always)]
    pub fn xtal_gmp(&mut self) -> XtalGmpW<'_, AnaparXtalOn0Spec> {
        XtalGmpW::new(self, 0)
    }
    #[doc = "Bits 5:9 - GMN: 0~31"]
    #[inline(always)]
    pub fn xtal_gmn(&mut self) -> XtalGmnW<'_, AnaparXtalOn0Spec> {
        XtalGmnW::new(self, 5)
    }
    #[doc = "Bits 10:16 - 1: reduce cap, 0: increase cap. (220fF per unit) \\[efuse for BT only \\]"]
    #[inline(always)]
    pub fn xtal_sc_xi(&mut self) -> XtalScXiW<'_, AnaparXtalOn0Spec> {
        XtalScXiW::new(self, 10)
    }
    #[doc = "Bits 17:23 - 1: reduce cap, 0: increase cap. (220fF per unit) \\[efuse for BT only \\]"]
    #[inline(always)]
    pub fn xtal_sc_xo(&mut self) -> XtalScXoW<'_, AnaparXtalOn0Spec> {
        XtalScXoW::new(self, 17)
    }
    #[doc = "Bits 24:30 - 0: reduce cap, 1: increase cap. (220fF per unit) \\[efuse for BT only \\] set all-0 when an external capacitor is populated"]
    #[inline(always)]
    pub fn xtal_sc_init(&mut self) -> XtalScInitW<'_, AnaparXtalOn0Spec> {
        XtalScInitW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`anapar_xtal_on_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anapar_xtal_on_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaparXtalOn0Spec;
impl crate::RegisterSpec for AnaparXtalOn0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anapar_xtal_on_0::R`](R) reader structure"]
impl crate::Readable for AnaparXtalOn0Spec {}
#[doc = "`write(|w| ..)` method takes [`anapar_xtal_on_0::W`](W) writer structure"]
impl crate::Writable for AnaparXtalOn0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANAPAR_XTAL_ON_0 to value 0x0081_00e7"]
impl crate::Resettable for AnaparXtalOn0Spec {
    const RESET_VALUE: u32 = 0x0081_00e7;
}
