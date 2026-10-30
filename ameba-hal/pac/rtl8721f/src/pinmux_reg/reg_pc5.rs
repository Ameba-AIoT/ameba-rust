#[doc = "Register `REG_PC5` reader"]
pub type R = crate::R<RegPc5Spec>;
#[doc = "Register `REG_PC5` writer"]
pub type W = crate::W<RegPc5Spec>;
#[doc = "Field `pC5_sel` reader - PAD pinmux function ID selection"]
pub type PC5SelR = crate::FieldReader;
#[doc = "Field `pC5_sel` writer - PAD pinmux function ID selection"]
pub type PC5SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC5_ie` reader - PAD input enable control"]
pub type PC5IeR = crate::BitReader;
#[doc = "Field `pC5_ie` writer - PAD input enable control"]
pub type PC5IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC5_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC5PupdcR = crate::BitReader;
#[doc = "Field `pC5_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC5PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC5_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC5E2R = crate::BitReader;
#[doc = "Field `pC5_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC5E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC5_smt` reader - PAD Schmit control"]
pub type PC5SmtR = crate::BitReader;
#[doc = "Field `pC5_smt` writer - PAD Schmit control"]
pub type PC5SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC5_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC5SrR = crate::BitReader;
#[doc = "Field `pC5_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC5SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC5_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC5E3R = crate::BitReader;
#[doc = "Field `pC5_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC5E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC5_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC5DisR = crate::BitReader;
#[doc = "Field `pC5_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC5DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c5_sel(&self) -> PC5SelR {
        PC5SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c5_ie(&self) -> PC5IeR {
        PC5IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c5_pupdc(&self) -> PC5PupdcR {
        PC5PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c5_e2(&self) -> PC5E2R {
        PC5E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c5_smt(&self) -> PC5SmtR {
        PC5SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c5_sr(&self) -> PC5SrR {
        PC5SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c5_e3(&self) -> PC5E3R {
        PC5E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c5_dis(&self) -> PC5DisR {
        PC5DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c5_sel(&mut self) -> PC5SelW<'_, RegPc5Spec> {
        PC5SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c5_ie(&mut self) -> PC5IeW<'_, RegPc5Spec> {
        PC5IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c5_pupdc(&mut self) -> PC5PupdcW<'_, RegPc5Spec> {
        PC5PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c5_e2(&mut self) -> PC5E2W<'_, RegPc5Spec> {
        PC5E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c5_smt(&mut self) -> PC5SmtW<'_, RegPc5Spec> {
        PC5SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c5_sr(&mut self) -> PC5SrW<'_, RegPc5Spec> {
        PC5SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c5_e3(&mut self) -> PC5E3W<'_, RegPc5Spec> {
        PC5E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c5_dis(&mut self) -> PC5DisW<'_, RegPc5Spec> {
        PC5DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc5Spec;
impl crate::RegisterSpec for RegPc5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc5::R`](R) reader structure"]
impl crate::Readable for RegPc5Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc5::W`](W) writer structure"]
impl crate::Writable for RegPc5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC5 to value 0x1900"]
impl crate::Resettable for RegPc5Spec {
    const RESET_VALUE: u32 = 0x1900;
}
