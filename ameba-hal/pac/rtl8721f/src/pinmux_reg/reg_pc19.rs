#[doc = "Register `REG_PC19` reader"]
pub type R = crate::R<RegPc19Spec>;
#[doc = "Register `REG_PC19` writer"]
pub type W = crate::W<RegPc19Spec>;
#[doc = "Field `pC19_sel` reader - PAD pinmux function ID selection"]
pub type PC19SelR = crate::FieldReader;
#[doc = "Field `pC19_sel` writer - PAD pinmux function ID selection"]
pub type PC19SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC19_ie` reader - PAD input enable control"]
pub type PC19IeR = crate::BitReader;
#[doc = "Field `pC19_ie` writer - PAD input enable control"]
pub type PC19IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC19_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC19PupdcR = crate::BitReader;
#[doc = "Field `pC19_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC19PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC19_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC19E2R = crate::BitReader;
#[doc = "Field `pC19_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC19E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC19_smt` reader - PAD Schmit control"]
pub type PC19SmtR = crate::BitReader;
#[doc = "Field `pC19_smt` writer - PAD Schmit control"]
pub type PC19SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC19_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC19SrR = crate::BitReader;
#[doc = "Field `pC19_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC19SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC19_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC19E3R = crate::BitReader;
#[doc = "Field `pC19_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC19E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC19_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC19DisR = crate::BitReader;
#[doc = "Field `pC19_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC19DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c19_sel(&self) -> PC19SelR {
        PC19SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c19_ie(&self) -> PC19IeR {
        PC19IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c19_pupdc(&self) -> PC19PupdcR {
        PC19PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c19_e2(&self) -> PC19E2R {
        PC19E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c19_smt(&self) -> PC19SmtR {
        PC19SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c19_sr(&self) -> PC19SrR {
        PC19SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c19_e3(&self) -> PC19E3R {
        PC19E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c19_dis(&self) -> PC19DisR {
        PC19DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c19_sel(&mut self) -> PC19SelW<'_, RegPc19Spec> {
        PC19SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c19_ie(&mut self) -> PC19IeW<'_, RegPc19Spec> {
        PC19IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c19_pupdc(&mut self) -> PC19PupdcW<'_, RegPc19Spec> {
        PC19PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c19_e2(&mut self) -> PC19E2W<'_, RegPc19Spec> {
        PC19E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c19_smt(&mut self) -> PC19SmtW<'_, RegPc19Spec> {
        PC19SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c19_sr(&mut self) -> PC19SrW<'_, RegPc19Spec> {
        PC19SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c19_e3(&mut self) -> PC19E3W<'_, RegPc19Spec> {
        PC19E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c19_dis(&mut self) -> PC19DisW<'_, RegPc19Spec> {
        PC19DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc19Spec;
impl crate::RegisterSpec for RegPc19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc19::R`](R) reader structure"]
impl crate::Readable for RegPc19Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc19::W`](W) writer structure"]
impl crate::Writable for RegPc19Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC19 to value 0x1900"]
impl crate::Resettable for RegPc19Spec {
    const RESET_VALUE: u32 = 0x1900;
}
