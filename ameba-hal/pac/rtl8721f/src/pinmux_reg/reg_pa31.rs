#[doc = "Register `REG_PA31` reader"]
pub type R = crate::R<RegPa31Spec>;
#[doc = "Register `REG_PA31` writer"]
pub type W = crate::W<RegPa31Spec>;
#[doc = "Field `pA31_sel` reader - PAD pinmux function ID selection"]
pub type PA31SelR = crate::FieldReader;
#[doc = "Field `pA31_sel` writer - PAD pinmux function ID selection"]
pub type PA31SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA31_ie` reader - PAD input enable control"]
pub type PA31IeR = crate::BitReader;
#[doc = "Field `pA31_ie` writer - PAD input enable control"]
pub type PA31IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA31_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA31PupdcR = crate::BitReader;
#[doc = "Field `pA31_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA31PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA31_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA31E2R = crate::BitReader;
#[doc = "Field `pA31_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA31E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA31_smt` reader - PAD Schmit control"]
pub type PA31SmtR = crate::BitReader;
#[doc = "Field `pA31_smt` writer - PAD Schmit control"]
pub type PA31SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA31_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA31SrR = crate::BitReader;
#[doc = "Field `pA31_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA31SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA31_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA31E3R = crate::BitReader;
#[doc = "Field `pA31_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA31E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA31_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA31DisR = crate::BitReader;
#[doc = "Field `pA31_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA31DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a31_sel(&self) -> PA31SelR {
        PA31SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a31_ie(&self) -> PA31IeR {
        PA31IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a31_pupdc(&self) -> PA31PupdcR {
        PA31PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a31_e2(&self) -> PA31E2R {
        PA31E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a31_smt(&self) -> PA31SmtR {
        PA31SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a31_sr(&self) -> PA31SrR {
        PA31SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a31_e3(&self) -> PA31E3R {
        PA31E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a31_dis(&self) -> PA31DisR {
        PA31DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a31_sel(&mut self) -> PA31SelW<'_, RegPa31Spec> {
        PA31SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a31_ie(&mut self) -> PA31IeW<'_, RegPa31Spec> {
        PA31IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a31_pupdc(&mut self) -> PA31PupdcW<'_, RegPa31Spec> {
        PA31PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a31_e2(&mut self) -> PA31E2W<'_, RegPa31Spec> {
        PA31E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a31_smt(&mut self) -> PA31SmtW<'_, RegPa31Spec> {
        PA31SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a31_sr(&mut self) -> PA31SrW<'_, RegPa31Spec> {
        PA31SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a31_e3(&mut self) -> PA31E3W<'_, RegPa31Spec> {
        PA31E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a31_dis(&mut self) -> PA31DisW<'_, RegPa31Spec> {
        PA31DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa31Spec;
impl crate::RegisterSpec for RegPa31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa31::R`](R) reader structure"]
impl crate::Readable for RegPa31Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa31::W`](W) writer structure"]
impl crate::Writable for RegPa31Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA31 to value 0x1900"]
impl crate::Resettable for RegPa31Spec {
    const RESET_VALUE: u32 = 0x1900;
}
