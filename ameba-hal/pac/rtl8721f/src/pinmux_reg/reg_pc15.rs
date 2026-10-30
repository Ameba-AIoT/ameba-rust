#[doc = "Register `REG_PC15` reader"]
pub type R = crate::R<RegPc15Spec>;
#[doc = "Register `REG_PC15` writer"]
pub type W = crate::W<RegPc15Spec>;
#[doc = "Field `pC15_sel` reader - PAD pinmux function ID selection"]
pub type PC15SelR = crate::FieldReader;
#[doc = "Field `pC15_sel` writer - PAD pinmux function ID selection"]
pub type PC15SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC15_ie` reader - PAD input enable control"]
pub type PC15IeR = crate::BitReader;
#[doc = "Field `pC15_ie` writer - PAD input enable control"]
pub type PC15IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC15_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC15PupdcR = crate::BitReader;
#[doc = "Field `pC15_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC15PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC15_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC15E2R = crate::BitReader;
#[doc = "Field `pC15_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC15E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC15_smt` reader - PAD Schmit control"]
pub type PC15SmtR = crate::BitReader;
#[doc = "Field `pC15_smt` writer - PAD Schmit control"]
pub type PC15SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC15_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC15SrR = crate::BitReader;
#[doc = "Field `pC15_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC15SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC15_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC15E3R = crate::BitReader;
#[doc = "Field `pC15_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC15E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC15_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC15DisR = crate::BitReader;
#[doc = "Field `pC15_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC15DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c15_sel(&self) -> PC15SelR {
        PC15SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c15_ie(&self) -> PC15IeR {
        PC15IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c15_pupdc(&self) -> PC15PupdcR {
        PC15PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c15_e2(&self) -> PC15E2R {
        PC15E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c15_smt(&self) -> PC15SmtR {
        PC15SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c15_sr(&self) -> PC15SrR {
        PC15SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c15_e3(&self) -> PC15E3R {
        PC15E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c15_dis(&self) -> PC15DisR {
        PC15DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c15_sel(&mut self) -> PC15SelW<'_, RegPc15Spec> {
        PC15SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c15_ie(&mut self) -> PC15IeW<'_, RegPc15Spec> {
        PC15IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c15_pupdc(&mut self) -> PC15PupdcW<'_, RegPc15Spec> {
        PC15PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c15_e2(&mut self) -> PC15E2W<'_, RegPc15Spec> {
        PC15E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c15_smt(&mut self) -> PC15SmtW<'_, RegPc15Spec> {
        PC15SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c15_sr(&mut self) -> PC15SrW<'_, RegPc15Spec> {
        PC15SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c15_e3(&mut self) -> PC15E3W<'_, RegPc15Spec> {
        PC15E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c15_dis(&mut self) -> PC15DisW<'_, RegPc15Spec> {
        PC15DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc15Spec;
impl crate::RegisterSpec for RegPc15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc15::R`](R) reader structure"]
impl crate::Readable for RegPc15Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc15::W`](W) writer structure"]
impl crate::Writable for RegPc15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC15 to value 0x1900"]
impl crate::Resettable for RegPc15Spec {
    const RESET_VALUE: u32 = 0x1900;
}
