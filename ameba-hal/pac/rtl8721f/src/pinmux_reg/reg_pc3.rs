#[doc = "Register `REG_PC3` reader"]
pub type R = crate::R<RegPc3Spec>;
#[doc = "Register `REG_PC3` writer"]
pub type W = crate::W<RegPc3Spec>;
#[doc = "Field `pC3_sel` reader - PAD pinmux function ID selection"]
pub type PC3SelR = crate::FieldReader;
#[doc = "Field `pC3_sel` writer - PAD pinmux function ID selection"]
pub type PC3SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC3_ie` reader - PAD input enable control"]
pub type PC3IeR = crate::BitReader;
#[doc = "Field `pC3_ie` writer - PAD input enable control"]
pub type PC3IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC3_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC3PupdcR = crate::BitReader;
#[doc = "Field `pC3_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC3PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC3_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC3E2R = crate::BitReader;
#[doc = "Field `pC3_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC3E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC3_smt` reader - PAD Schmit control"]
pub type PC3SmtR = crate::BitReader;
#[doc = "Field `pC3_smt` writer - PAD Schmit control"]
pub type PC3SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC3_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC3SrR = crate::BitReader;
#[doc = "Field `pC3_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC3SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC3_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC3E3R = crate::BitReader;
#[doc = "Field `pC3_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC3E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC3_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC3DisR = crate::BitReader;
#[doc = "Field `pC3_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC3DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c3_sel(&self) -> PC3SelR {
        PC3SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c3_ie(&self) -> PC3IeR {
        PC3IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c3_pupdc(&self) -> PC3PupdcR {
        PC3PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c3_e2(&self) -> PC3E2R {
        PC3E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c3_smt(&self) -> PC3SmtR {
        PC3SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c3_sr(&self) -> PC3SrR {
        PC3SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c3_e3(&self) -> PC3E3R {
        PC3E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c3_dis(&self) -> PC3DisR {
        PC3DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c3_sel(&mut self) -> PC3SelW<'_, RegPc3Spec> {
        PC3SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c3_ie(&mut self) -> PC3IeW<'_, RegPc3Spec> {
        PC3IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c3_pupdc(&mut self) -> PC3PupdcW<'_, RegPc3Spec> {
        PC3PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c3_e2(&mut self) -> PC3E2W<'_, RegPc3Spec> {
        PC3E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c3_smt(&mut self) -> PC3SmtW<'_, RegPc3Spec> {
        PC3SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c3_sr(&mut self) -> PC3SrW<'_, RegPc3Spec> {
        PC3SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c3_e3(&mut self) -> PC3E3W<'_, RegPc3Spec> {
        PC3E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c3_dis(&mut self) -> PC3DisW<'_, RegPc3Spec> {
        PC3DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc3Spec;
impl crate::RegisterSpec for RegPc3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc3::R`](R) reader structure"]
impl crate::Readable for RegPc3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc3::W`](W) writer structure"]
impl crate::Writable for RegPc3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC3 to value 0x1900"]
impl crate::Resettable for RegPc3Spec {
    const RESET_VALUE: u32 = 0x1900;
}
