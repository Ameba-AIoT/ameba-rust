#[doc = "Register `REG_PA3` reader"]
pub type R = crate::R<RegPa3Spec>;
#[doc = "Register `REG_PA3` writer"]
pub type W = crate::W<RegPa3Spec>;
#[doc = "Field `pA3_sel` reader - PAD pinmux function ID selection"]
pub type PA3SelR = crate::FieldReader;
#[doc = "Field `pA3_sel` writer - PAD pinmux function ID selection"]
pub type PA3SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA3_ie` reader - PAD input enable control"]
pub type PA3IeR = crate::BitReader;
#[doc = "Field `pA3_ie` writer - PAD input enable control"]
pub type PA3IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA3_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA3PupdcR = crate::BitReader;
#[doc = "Field `pA3_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA3PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA3_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA3E2R = crate::BitReader;
#[doc = "Field `pA3_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA3E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA3_smt` reader - PAD Schmit control"]
pub type PA3SmtR = crate::BitReader;
#[doc = "Field `pA3_smt` writer - PAD Schmit control"]
pub type PA3SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA3_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA3SrR = crate::BitReader;
#[doc = "Field `pA3_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA3SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA3_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA3E3R = crate::BitReader;
#[doc = "Field `pA3_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA3E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA3_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA3DisR = crate::BitReader;
#[doc = "Field `pA3_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA3DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a3_sel(&self) -> PA3SelR {
        PA3SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a3_ie(&self) -> PA3IeR {
        PA3IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a3_pupdc(&self) -> PA3PupdcR {
        PA3PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a3_e2(&self) -> PA3E2R {
        PA3E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a3_smt(&self) -> PA3SmtR {
        PA3SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a3_sr(&self) -> PA3SrR {
        PA3SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a3_e3(&self) -> PA3E3R {
        PA3E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a3_dis(&self) -> PA3DisR {
        PA3DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a3_sel(&mut self) -> PA3SelW<'_, RegPa3Spec> {
        PA3SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a3_ie(&mut self) -> PA3IeW<'_, RegPa3Spec> {
        PA3IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a3_pupdc(&mut self) -> PA3PupdcW<'_, RegPa3Spec> {
        PA3PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a3_e2(&mut self) -> PA3E2W<'_, RegPa3Spec> {
        PA3E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a3_smt(&mut self) -> PA3SmtW<'_, RegPa3Spec> {
        PA3SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a3_sr(&mut self) -> PA3SrW<'_, RegPa3Spec> {
        PA3SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a3_e3(&mut self) -> PA3E3W<'_, RegPa3Spec> {
        PA3E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a3_dis(&mut self) -> PA3DisW<'_, RegPa3Spec> {
        PA3DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa3Spec;
impl crate::RegisterSpec for RegPa3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa3::R`](R) reader structure"]
impl crate::Readable for RegPa3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa3::W`](W) writer structure"]
impl crate::Writable for RegPa3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA3 to value 0x1900"]
impl crate::Resettable for RegPa3Spec {
    const RESET_VALUE: u32 = 0x1900;
}
