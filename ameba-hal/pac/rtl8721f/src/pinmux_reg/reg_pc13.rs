#[doc = "Register `REG_PC13` reader"]
pub type R = crate::R<RegPc13Spec>;
#[doc = "Register `REG_PC13` writer"]
pub type W = crate::W<RegPc13Spec>;
#[doc = "Field `pC13_sel` reader - PAD pinmux function ID selection"]
pub type PC13SelR = crate::FieldReader;
#[doc = "Field `pC13_sel` writer - PAD pinmux function ID selection"]
pub type PC13SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC13_ie` reader - PAD input enable control"]
pub type PC13IeR = crate::BitReader;
#[doc = "Field `pC13_ie` writer - PAD input enable control"]
pub type PC13IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC13_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC13PupdcR = crate::BitReader;
#[doc = "Field `pC13_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC13PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC13_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC13E2R = crate::BitReader;
#[doc = "Field `pC13_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC13E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC13_smt` reader - PAD Schmit control"]
pub type PC13SmtR = crate::BitReader;
#[doc = "Field `pC13_smt` writer - PAD Schmit control"]
pub type PC13SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC13_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC13SrR = crate::BitReader;
#[doc = "Field `pC13_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC13SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC13_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC13E3R = crate::BitReader;
#[doc = "Field `pC13_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC13E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC13_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC13DisR = crate::BitReader;
#[doc = "Field `pC13_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC13DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c13_sel(&self) -> PC13SelR {
        PC13SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c13_ie(&self) -> PC13IeR {
        PC13IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c13_pupdc(&self) -> PC13PupdcR {
        PC13PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c13_e2(&self) -> PC13E2R {
        PC13E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c13_smt(&self) -> PC13SmtR {
        PC13SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c13_sr(&self) -> PC13SrR {
        PC13SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c13_e3(&self) -> PC13E3R {
        PC13E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c13_dis(&self) -> PC13DisR {
        PC13DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c13_sel(&mut self) -> PC13SelW<'_, RegPc13Spec> {
        PC13SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c13_ie(&mut self) -> PC13IeW<'_, RegPc13Spec> {
        PC13IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c13_pupdc(&mut self) -> PC13PupdcW<'_, RegPc13Spec> {
        PC13PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c13_e2(&mut self) -> PC13E2W<'_, RegPc13Spec> {
        PC13E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c13_smt(&mut self) -> PC13SmtW<'_, RegPc13Spec> {
        PC13SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c13_sr(&mut self) -> PC13SrW<'_, RegPc13Spec> {
        PC13SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c13_e3(&mut self) -> PC13E3W<'_, RegPc13Spec> {
        PC13E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c13_dis(&mut self) -> PC13DisW<'_, RegPc13Spec> {
        PC13DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc13Spec;
impl crate::RegisterSpec for RegPc13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc13::R`](R) reader structure"]
impl crate::Readable for RegPc13Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc13::W`](W) writer structure"]
impl crate::Writable for RegPc13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC13 to value 0x1900"]
impl crate::Resettable for RegPc13Spec {
    const RESET_VALUE: u32 = 0x1900;
}
