#[doc = "Register `REG_PC21` reader"]
pub type R = crate::R<RegPc21Spec>;
#[doc = "Register `REG_PC21` writer"]
pub type W = crate::W<RegPc21Spec>;
#[doc = "Field `pC21_sel` reader - PAD pinmux function ID selection"]
pub type PC21SelR = crate::FieldReader;
#[doc = "Field `pC21_sel` writer - PAD pinmux function ID selection"]
pub type PC21SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC21_ie` reader - PAD input enable control"]
pub type PC21IeR = crate::BitReader;
#[doc = "Field `pC21_ie` writer - PAD input enable control"]
pub type PC21IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC21_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC21PupdcR = crate::BitReader;
#[doc = "Field `pC21_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC21PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC21_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC21E2R = crate::BitReader;
#[doc = "Field `pC21_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC21E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC21_smt` reader - PAD Schmit control"]
pub type PC21SmtR = crate::BitReader;
#[doc = "Field `pC21_smt` writer - PAD Schmit control"]
pub type PC21SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC21_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC21SrR = crate::BitReader;
#[doc = "Field `pC21_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC21SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC21_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC21E3R = crate::BitReader;
#[doc = "Field `pC21_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC21E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC21_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC21DisR = crate::BitReader;
#[doc = "Field `pC21_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC21DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c21_sel(&self) -> PC21SelR {
        PC21SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c21_ie(&self) -> PC21IeR {
        PC21IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c21_pupdc(&self) -> PC21PupdcR {
        PC21PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c21_e2(&self) -> PC21E2R {
        PC21E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c21_smt(&self) -> PC21SmtR {
        PC21SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c21_sr(&self) -> PC21SrR {
        PC21SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c21_e3(&self) -> PC21E3R {
        PC21E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c21_dis(&self) -> PC21DisR {
        PC21DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c21_sel(&mut self) -> PC21SelW<'_, RegPc21Spec> {
        PC21SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c21_ie(&mut self) -> PC21IeW<'_, RegPc21Spec> {
        PC21IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c21_pupdc(&mut self) -> PC21PupdcW<'_, RegPc21Spec> {
        PC21PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c21_e2(&mut self) -> PC21E2W<'_, RegPc21Spec> {
        PC21E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c21_smt(&mut self) -> PC21SmtW<'_, RegPc21Spec> {
        PC21SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c21_sr(&mut self) -> PC21SrW<'_, RegPc21Spec> {
        PC21SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c21_e3(&mut self) -> PC21E3W<'_, RegPc21Spec> {
        PC21E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c21_dis(&mut self) -> PC21DisW<'_, RegPc21Spec> {
        PC21DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc21Spec;
impl crate::RegisterSpec for RegPc21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc21::R`](R) reader structure"]
impl crate::Readable for RegPc21Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc21::W`](W) writer structure"]
impl crate::Writable for RegPc21Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC21 to value 0x1900"]
impl crate::Resettable for RegPc21Spec {
    const RESET_VALUE: u32 = 0x1900;
}
