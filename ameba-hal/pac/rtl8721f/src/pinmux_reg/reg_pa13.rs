#[doc = "Register `REG_PA13` reader"]
pub type R = crate::R<RegPa13Spec>;
#[doc = "Register `REG_PA13` writer"]
pub type W = crate::W<RegPa13Spec>;
#[doc = "Field `pA13_sel` reader - PAD pinmux function ID selection"]
pub type PA13SelR = crate::FieldReader;
#[doc = "Field `pA13_sel` writer - PAD pinmux function ID selection"]
pub type PA13SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA13_ie` reader - PAD input enable control"]
pub type PA13IeR = crate::BitReader;
#[doc = "Field `pA13_ie` writer - PAD input enable control"]
pub type PA13IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA13_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA13PupdcR = crate::BitReader;
#[doc = "Field `pA13_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA13PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA13_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA13E2R = crate::BitReader;
#[doc = "Field `pA13_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA13E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA13_smt` reader - PAD Schmit control"]
pub type PA13SmtR = crate::BitReader;
#[doc = "Field `pA13_smt` writer - PAD Schmit control"]
pub type PA13SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA13_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA13SrR = crate::BitReader;
#[doc = "Field `pA13_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA13SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA13_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA13E3R = crate::BitReader;
#[doc = "Field `pA13_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA13E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA13_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA13DisR = crate::BitReader;
#[doc = "Field `pA13_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA13DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a13_sel(&self) -> PA13SelR {
        PA13SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a13_ie(&self) -> PA13IeR {
        PA13IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a13_pupdc(&self) -> PA13PupdcR {
        PA13PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a13_e2(&self) -> PA13E2R {
        PA13E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a13_smt(&self) -> PA13SmtR {
        PA13SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a13_sr(&self) -> PA13SrR {
        PA13SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a13_e3(&self) -> PA13E3R {
        PA13E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a13_dis(&self) -> PA13DisR {
        PA13DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a13_sel(&mut self) -> PA13SelW<'_, RegPa13Spec> {
        PA13SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a13_ie(&mut self) -> PA13IeW<'_, RegPa13Spec> {
        PA13IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a13_pupdc(&mut self) -> PA13PupdcW<'_, RegPa13Spec> {
        PA13PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a13_e2(&mut self) -> PA13E2W<'_, RegPa13Spec> {
        PA13E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a13_smt(&mut self) -> PA13SmtW<'_, RegPa13Spec> {
        PA13SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a13_sr(&mut self) -> PA13SrW<'_, RegPa13Spec> {
        PA13SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a13_e3(&mut self) -> PA13E3W<'_, RegPa13Spec> {
        PA13E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a13_dis(&mut self) -> PA13DisW<'_, RegPa13Spec> {
        PA13DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa13Spec;
impl crate::RegisterSpec for RegPa13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa13::R`](R) reader structure"]
impl crate::Readable for RegPa13Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa13::W`](W) writer structure"]
impl crate::Writable for RegPa13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA13 to value 0x1900"]
impl crate::Resettable for RegPa13Spec {
    const RESET_VALUE: u32 = 0x1900;
}
