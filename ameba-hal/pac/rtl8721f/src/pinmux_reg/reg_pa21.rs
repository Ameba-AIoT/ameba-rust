#[doc = "Register `REG_PA21` reader"]
pub type R = crate::R<RegPa21Spec>;
#[doc = "Register `REG_PA21` writer"]
pub type W = crate::W<RegPa21Spec>;
#[doc = "Field `pA21_sel` reader - PAD pinmux function ID selection"]
pub type PA21SelR = crate::FieldReader;
#[doc = "Field `pA21_sel` writer - PAD pinmux function ID selection"]
pub type PA21SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA21_ie` reader - PAD input enable control"]
pub type PA21IeR = crate::BitReader;
#[doc = "Field `pA21_ie` writer - PAD input enable control"]
pub type PA21IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA21_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA21PupdcR = crate::BitReader;
#[doc = "Field `pA21_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA21PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA21_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA21E2R = crate::BitReader;
#[doc = "Field `pA21_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA21E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA21_smt` reader - PAD Schmit control"]
pub type PA21SmtR = crate::BitReader;
#[doc = "Field `pA21_smt` writer - PAD Schmit control"]
pub type PA21SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA21_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA21SrR = crate::BitReader;
#[doc = "Field `pA21_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA21SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA21_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA21E3R = crate::BitReader;
#[doc = "Field `pA21_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA21E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA21_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA21DisR = crate::BitReader;
#[doc = "Field `pA21_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA21DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a21_sel(&self) -> PA21SelR {
        PA21SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a21_ie(&self) -> PA21IeR {
        PA21IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a21_pupdc(&self) -> PA21PupdcR {
        PA21PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a21_e2(&self) -> PA21E2R {
        PA21E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a21_smt(&self) -> PA21SmtR {
        PA21SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a21_sr(&self) -> PA21SrR {
        PA21SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a21_e3(&self) -> PA21E3R {
        PA21E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a21_dis(&self) -> PA21DisR {
        PA21DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a21_sel(&mut self) -> PA21SelW<'_, RegPa21Spec> {
        PA21SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a21_ie(&mut self) -> PA21IeW<'_, RegPa21Spec> {
        PA21IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a21_pupdc(&mut self) -> PA21PupdcW<'_, RegPa21Spec> {
        PA21PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a21_e2(&mut self) -> PA21E2W<'_, RegPa21Spec> {
        PA21E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a21_smt(&mut self) -> PA21SmtW<'_, RegPa21Spec> {
        PA21SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a21_sr(&mut self) -> PA21SrW<'_, RegPa21Spec> {
        PA21SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a21_e3(&mut self) -> PA21E3W<'_, RegPa21Spec> {
        PA21E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a21_dis(&mut self) -> PA21DisW<'_, RegPa21Spec> {
        PA21DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa21Spec;
impl crate::RegisterSpec for RegPa21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa21::R`](R) reader structure"]
impl crate::Readable for RegPa21Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa21::W`](W) writer structure"]
impl crate::Writable for RegPa21Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA21 to value 0x1900"]
impl crate::Resettable for RegPa21Spec {
    const RESET_VALUE: u32 = 0x1900;
}
