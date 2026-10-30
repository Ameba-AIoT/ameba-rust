#[doc = "Register `REG_PC9` reader"]
pub type R = crate::R<RegPc9Spec>;
#[doc = "Register `REG_PC9` writer"]
pub type W = crate::W<RegPc9Spec>;
#[doc = "Field `pC9_sel` reader - PAD pinmux function ID selection"]
pub type PC9SelR = crate::FieldReader;
#[doc = "Field `pC9_sel` writer - PAD pinmux function ID selection"]
pub type PC9SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC9_ie` reader - PAD input enable control"]
pub type PC9IeR = crate::BitReader;
#[doc = "Field `pC9_ie` writer - PAD input enable control"]
pub type PC9IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC9_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC9PupdcR = crate::BitReader;
#[doc = "Field `pC9_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC9PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC9_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC9E2R = crate::BitReader;
#[doc = "Field `pC9_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC9E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC9_smt` reader - PAD Schmit control"]
pub type PC9SmtR = crate::BitReader;
#[doc = "Field `pC9_smt` writer - PAD Schmit control"]
pub type PC9SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC9_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC9SrR = crate::BitReader;
#[doc = "Field `pC9_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC9SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC9_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC9E3R = crate::BitReader;
#[doc = "Field `pC9_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC9E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC9_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC9DisR = crate::BitReader;
#[doc = "Field `pC9_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC9DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c9_sel(&self) -> PC9SelR {
        PC9SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c9_ie(&self) -> PC9IeR {
        PC9IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c9_pupdc(&self) -> PC9PupdcR {
        PC9PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c9_e2(&self) -> PC9E2R {
        PC9E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c9_smt(&self) -> PC9SmtR {
        PC9SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c9_sr(&self) -> PC9SrR {
        PC9SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c9_e3(&self) -> PC9E3R {
        PC9E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c9_dis(&self) -> PC9DisR {
        PC9DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c9_sel(&mut self) -> PC9SelW<'_, RegPc9Spec> {
        PC9SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c9_ie(&mut self) -> PC9IeW<'_, RegPc9Spec> {
        PC9IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c9_pupdc(&mut self) -> PC9PupdcW<'_, RegPc9Spec> {
        PC9PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c9_e2(&mut self) -> PC9E2W<'_, RegPc9Spec> {
        PC9E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c9_smt(&mut self) -> PC9SmtW<'_, RegPc9Spec> {
        PC9SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c9_sr(&mut self) -> PC9SrW<'_, RegPc9Spec> {
        PC9SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c9_e3(&mut self) -> PC9E3W<'_, RegPc9Spec> {
        PC9E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c9_dis(&mut self) -> PC9DisW<'_, RegPc9Spec> {
        PC9DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc9Spec;
impl crate::RegisterSpec for RegPc9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc9::R`](R) reader structure"]
impl crate::Readable for RegPc9Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc9::W`](W) writer structure"]
impl crate::Writable for RegPc9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC9 to value 0x1900"]
impl crate::Resettable for RegPc9Spec {
    const RESET_VALUE: u32 = 0x1900;
}
