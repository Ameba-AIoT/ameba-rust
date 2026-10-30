#[doc = "Register `REG_PC8` reader"]
pub type R = crate::R<RegPc8Spec>;
#[doc = "Register `REG_PC8` writer"]
pub type W = crate::W<RegPc8Spec>;
#[doc = "Field `pC8_sel` reader - PAD pinmux function ID selection"]
pub type PC8SelR = crate::FieldReader;
#[doc = "Field `pC8_sel` writer - PAD pinmux function ID selection"]
pub type PC8SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC8_ie` reader - PAD input enable control"]
pub type PC8IeR = crate::BitReader;
#[doc = "Field `pC8_ie` writer - PAD input enable control"]
pub type PC8IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC8_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC8PupdcR = crate::BitReader;
#[doc = "Field `pC8_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC8PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC8_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC8E2R = crate::BitReader;
#[doc = "Field `pC8_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC8E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC8_smt` reader - PAD Schmit control"]
pub type PC8SmtR = crate::BitReader;
#[doc = "Field `pC8_smt` writer - PAD Schmit control"]
pub type PC8SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC8_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC8SrR = crate::BitReader;
#[doc = "Field `pC8_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC8SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC8_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC8E3R = crate::BitReader;
#[doc = "Field `pC8_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC8E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC8_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC8DisR = crate::BitReader;
#[doc = "Field `pC8_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC8DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c8_sel(&self) -> PC8SelR {
        PC8SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c8_ie(&self) -> PC8IeR {
        PC8IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c8_pupdc(&self) -> PC8PupdcR {
        PC8PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c8_e2(&self) -> PC8E2R {
        PC8E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c8_smt(&self) -> PC8SmtR {
        PC8SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c8_sr(&self) -> PC8SrR {
        PC8SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c8_e3(&self) -> PC8E3R {
        PC8E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c8_dis(&self) -> PC8DisR {
        PC8DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c8_sel(&mut self) -> PC8SelW<'_, RegPc8Spec> {
        PC8SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c8_ie(&mut self) -> PC8IeW<'_, RegPc8Spec> {
        PC8IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c8_pupdc(&mut self) -> PC8PupdcW<'_, RegPc8Spec> {
        PC8PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c8_e2(&mut self) -> PC8E2W<'_, RegPc8Spec> {
        PC8E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c8_smt(&mut self) -> PC8SmtW<'_, RegPc8Spec> {
        PC8SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c8_sr(&mut self) -> PC8SrW<'_, RegPc8Spec> {
        PC8SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c8_e3(&mut self) -> PC8E3W<'_, RegPc8Spec> {
        PC8E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c8_dis(&mut self) -> PC8DisW<'_, RegPc8Spec> {
        PC8DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc8Spec;
impl crate::RegisterSpec for RegPc8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc8::R`](R) reader structure"]
impl crate::Readable for RegPc8Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc8::W`](W) writer structure"]
impl crate::Writable for RegPc8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC8 to value 0x1900"]
impl crate::Resettable for RegPc8Spec {
    const RESET_VALUE: u32 = 0x1900;
}
