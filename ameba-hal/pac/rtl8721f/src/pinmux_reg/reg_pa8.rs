#[doc = "Register `REG_PA8` reader"]
pub type R = crate::R<RegPa8Spec>;
#[doc = "Register `REG_PA8` writer"]
pub type W = crate::W<RegPa8Spec>;
#[doc = "Field `pA8_sel` reader - PAD pinmux function ID selection"]
pub type PA8SelR = crate::FieldReader;
#[doc = "Field `pA8_sel` writer - PAD pinmux function ID selection"]
pub type PA8SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA8_ie` reader - PAD input enable control"]
pub type PA8IeR = crate::BitReader;
#[doc = "Field `pA8_ie` writer - PAD input enable control"]
pub type PA8IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA8_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA8PupdcR = crate::BitReader;
#[doc = "Field `pA8_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA8PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA8_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA8E2R = crate::BitReader;
#[doc = "Field `pA8_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA8E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA8_smt` reader - PAD Schmit control"]
pub type PA8SmtR = crate::BitReader;
#[doc = "Field `pA8_smt` writer - PAD Schmit control"]
pub type PA8SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA8_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA8SrR = crate::BitReader;
#[doc = "Field `pA8_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA8SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA8_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA8E3R = crate::BitReader;
#[doc = "Field `pA8_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA8E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA8_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA8DisR = crate::BitReader;
#[doc = "Field `pA8_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA8DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a8_sel(&self) -> PA8SelR {
        PA8SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a8_ie(&self) -> PA8IeR {
        PA8IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a8_pupdc(&self) -> PA8PupdcR {
        PA8PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a8_e2(&self) -> PA8E2R {
        PA8E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a8_smt(&self) -> PA8SmtR {
        PA8SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a8_sr(&self) -> PA8SrR {
        PA8SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a8_e3(&self) -> PA8E3R {
        PA8E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a8_dis(&self) -> PA8DisR {
        PA8DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a8_sel(&mut self) -> PA8SelW<'_, RegPa8Spec> {
        PA8SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a8_ie(&mut self) -> PA8IeW<'_, RegPa8Spec> {
        PA8IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a8_pupdc(&mut self) -> PA8PupdcW<'_, RegPa8Spec> {
        PA8PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a8_e2(&mut self) -> PA8E2W<'_, RegPa8Spec> {
        PA8E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a8_smt(&mut self) -> PA8SmtW<'_, RegPa8Spec> {
        PA8SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a8_sr(&mut self) -> PA8SrW<'_, RegPa8Spec> {
        PA8SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a8_e3(&mut self) -> PA8E3W<'_, RegPa8Spec> {
        PA8E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a8_dis(&mut self) -> PA8DisW<'_, RegPa8Spec> {
        PA8DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa8Spec;
impl crate::RegisterSpec for RegPa8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa8::R`](R) reader structure"]
impl crate::Readable for RegPa8Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa8::W`](W) writer structure"]
impl crate::Writable for RegPa8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA8 to value 0x1900"]
impl crate::Resettable for RegPa8Spec {
    const RESET_VALUE: u32 = 0x1900;
}
