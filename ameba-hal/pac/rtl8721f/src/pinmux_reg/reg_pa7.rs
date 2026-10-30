#[doc = "Register `REG_PA7` reader"]
pub type R = crate::R<RegPa7Spec>;
#[doc = "Register `REG_PA7` writer"]
pub type W = crate::W<RegPa7Spec>;
#[doc = "Field `pA7_sel` reader - PAD pinmux function ID selection"]
pub type PA7SelR = crate::FieldReader;
#[doc = "Field `pA7_sel` writer - PAD pinmux function ID selection"]
pub type PA7SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA7_ie` reader - PAD input enable control"]
pub type PA7IeR = crate::BitReader;
#[doc = "Field `pA7_ie` writer - PAD input enable control"]
pub type PA7IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA7_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA7PupdcR = crate::BitReader;
#[doc = "Field `pA7_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA7PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA7_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA7E2R = crate::BitReader;
#[doc = "Field `pA7_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA7E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA7_smt` reader - PAD Schmit control"]
pub type PA7SmtR = crate::BitReader;
#[doc = "Field `pA7_smt` writer - PAD Schmit control"]
pub type PA7SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA7_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA7SrR = crate::BitReader;
#[doc = "Field `pA7_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA7SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA7_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA7E3R = crate::BitReader;
#[doc = "Field `pA7_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA7E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA7_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA7DisR = crate::BitReader;
#[doc = "Field `pA7_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA7DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a7_sel(&self) -> PA7SelR {
        PA7SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a7_ie(&self) -> PA7IeR {
        PA7IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a7_pupdc(&self) -> PA7PupdcR {
        PA7PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a7_e2(&self) -> PA7E2R {
        PA7E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a7_smt(&self) -> PA7SmtR {
        PA7SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a7_sr(&self) -> PA7SrR {
        PA7SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a7_e3(&self) -> PA7E3R {
        PA7E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a7_dis(&self) -> PA7DisR {
        PA7DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a7_sel(&mut self) -> PA7SelW<'_, RegPa7Spec> {
        PA7SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a7_ie(&mut self) -> PA7IeW<'_, RegPa7Spec> {
        PA7IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a7_pupdc(&mut self) -> PA7PupdcW<'_, RegPa7Spec> {
        PA7PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a7_e2(&mut self) -> PA7E2W<'_, RegPa7Spec> {
        PA7E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a7_smt(&mut self) -> PA7SmtW<'_, RegPa7Spec> {
        PA7SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a7_sr(&mut self) -> PA7SrW<'_, RegPa7Spec> {
        PA7SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a7_e3(&mut self) -> PA7E3W<'_, RegPa7Spec> {
        PA7E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a7_dis(&mut self) -> PA7DisW<'_, RegPa7Spec> {
        PA7DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa7Spec;
impl crate::RegisterSpec for RegPa7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa7::R`](R) reader structure"]
impl crate::Readable for RegPa7Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa7::W`](W) writer structure"]
impl crate::Writable for RegPa7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA7 to value 0x1900"]
impl crate::Resettable for RegPa7Spec {
    const RESET_VALUE: u32 = 0x1900;
}
