#[doc = "Register `REG_PA9` reader"]
pub type R = crate::R<RegPa9Spec>;
#[doc = "Register `REG_PA9` writer"]
pub type W = crate::W<RegPa9Spec>;
#[doc = "Field `pA9_sel` reader - PAD pinmux function ID selection"]
pub type PA9SelR = crate::FieldReader;
#[doc = "Field `pA9_sel` writer - PAD pinmux function ID selection"]
pub type PA9SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA9_ie` reader - PAD input enable control"]
pub type PA9IeR = crate::BitReader;
#[doc = "Field `pA9_ie` writer - PAD input enable control"]
pub type PA9IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA9_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA9PupdcR = crate::BitReader;
#[doc = "Field `pA9_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA9PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA9_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA9E2R = crate::BitReader;
#[doc = "Field `pA9_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA9E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA9_smt` reader - PAD Schmit control"]
pub type PA9SmtR = crate::BitReader;
#[doc = "Field `pA9_smt` writer - PAD Schmit control"]
pub type PA9SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA9_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA9SrR = crate::BitReader;
#[doc = "Field `pA9_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA9SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA9_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA9E3R = crate::BitReader;
#[doc = "Field `pA9_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA9E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA9_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA9DisR = crate::BitReader;
#[doc = "Field `pA9_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA9DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a9_sel(&self) -> PA9SelR {
        PA9SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a9_ie(&self) -> PA9IeR {
        PA9IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a9_pupdc(&self) -> PA9PupdcR {
        PA9PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a9_e2(&self) -> PA9E2R {
        PA9E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a9_smt(&self) -> PA9SmtR {
        PA9SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a9_sr(&self) -> PA9SrR {
        PA9SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a9_e3(&self) -> PA9E3R {
        PA9E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a9_dis(&self) -> PA9DisR {
        PA9DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a9_sel(&mut self) -> PA9SelW<'_, RegPa9Spec> {
        PA9SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a9_ie(&mut self) -> PA9IeW<'_, RegPa9Spec> {
        PA9IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a9_pupdc(&mut self) -> PA9PupdcW<'_, RegPa9Spec> {
        PA9PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a9_e2(&mut self) -> PA9E2W<'_, RegPa9Spec> {
        PA9E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a9_smt(&mut self) -> PA9SmtW<'_, RegPa9Spec> {
        PA9SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a9_sr(&mut self) -> PA9SrW<'_, RegPa9Spec> {
        PA9SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a9_e3(&mut self) -> PA9E3W<'_, RegPa9Spec> {
        PA9E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a9_dis(&mut self) -> PA9DisW<'_, RegPa9Spec> {
        PA9DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa9Spec;
impl crate::RegisterSpec for RegPa9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa9::R`](R) reader structure"]
impl crate::Readable for RegPa9Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa9::W`](W) writer structure"]
impl crate::Writable for RegPa9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA9 to value 0x1900"]
impl crate::Resettable for RegPa9Spec {
    const RESET_VALUE: u32 = 0x1900;
}
