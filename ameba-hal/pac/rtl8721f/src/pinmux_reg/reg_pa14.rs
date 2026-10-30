#[doc = "Register `REG_PA14` reader"]
pub type R = crate::R<RegPa14Spec>;
#[doc = "Register `REG_PA14` writer"]
pub type W = crate::W<RegPa14Spec>;
#[doc = "Field `pA14_sel` reader - PAD pinmux function ID selection"]
pub type PA14SelR = crate::FieldReader;
#[doc = "Field `pA14_sel` writer - PAD pinmux function ID selection"]
pub type PA14SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA14_ie` reader - PAD input enable control"]
pub type PA14IeR = crate::BitReader;
#[doc = "Field `pA14_ie` writer - PAD input enable control"]
pub type PA14IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA14_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA14PupdcR = crate::BitReader;
#[doc = "Field `pA14_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA14PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA14_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA14E2R = crate::BitReader;
#[doc = "Field `pA14_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA14E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA14_smt` reader - PAD Schmit control"]
pub type PA14SmtR = crate::BitReader;
#[doc = "Field `pA14_smt` writer - PAD Schmit control"]
pub type PA14SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA14_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA14SrR = crate::BitReader;
#[doc = "Field `pA14_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA14SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA14_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA14E3R = crate::BitReader;
#[doc = "Field `pA14_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA14E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA14_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA14DisR = crate::BitReader;
#[doc = "Field `pA14_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA14DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a14_sel(&self) -> PA14SelR {
        PA14SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a14_ie(&self) -> PA14IeR {
        PA14IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a14_pupdc(&self) -> PA14PupdcR {
        PA14PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a14_e2(&self) -> PA14E2R {
        PA14E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a14_smt(&self) -> PA14SmtR {
        PA14SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a14_sr(&self) -> PA14SrR {
        PA14SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a14_e3(&self) -> PA14E3R {
        PA14E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a14_dis(&self) -> PA14DisR {
        PA14DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a14_sel(&mut self) -> PA14SelW<'_, RegPa14Spec> {
        PA14SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a14_ie(&mut self) -> PA14IeW<'_, RegPa14Spec> {
        PA14IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a14_pupdc(&mut self) -> PA14PupdcW<'_, RegPa14Spec> {
        PA14PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a14_e2(&mut self) -> PA14E2W<'_, RegPa14Spec> {
        PA14E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a14_smt(&mut self) -> PA14SmtW<'_, RegPa14Spec> {
        PA14SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a14_sr(&mut self) -> PA14SrW<'_, RegPa14Spec> {
        PA14SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a14_e3(&mut self) -> PA14E3W<'_, RegPa14Spec> {
        PA14E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a14_dis(&mut self) -> PA14DisW<'_, RegPa14Spec> {
        PA14DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa14Spec;
impl crate::RegisterSpec for RegPa14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa14::R`](R) reader structure"]
impl crate::Readable for RegPa14Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa14::W`](W) writer structure"]
impl crate::Writable for RegPa14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA14 to value 0x1900"]
impl crate::Resettable for RegPa14Spec {
    const RESET_VALUE: u32 = 0x1900;
}
