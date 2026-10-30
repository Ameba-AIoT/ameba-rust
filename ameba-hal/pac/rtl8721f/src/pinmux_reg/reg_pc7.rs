#[doc = "Register `REG_PC7` reader"]
pub type R = crate::R<RegPc7Spec>;
#[doc = "Register `REG_PC7` writer"]
pub type W = crate::W<RegPc7Spec>;
#[doc = "Field `pC7_sel` reader - PAD pinmux function ID selection"]
pub type PC7SelR = crate::FieldReader;
#[doc = "Field `pC7_sel` writer - PAD pinmux function ID selection"]
pub type PC7SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC7_ie` reader - PAD input enable control"]
pub type PC7IeR = crate::BitReader;
#[doc = "Field `pC7_ie` writer - PAD input enable control"]
pub type PC7IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC7_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC7PupdcR = crate::BitReader;
#[doc = "Field `pC7_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC7PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC7_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC7E2R = crate::BitReader;
#[doc = "Field `pC7_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC7E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC7_smt` reader - PAD Schmit control"]
pub type PC7SmtR = crate::BitReader;
#[doc = "Field `pC7_smt` writer - PAD Schmit control"]
pub type PC7SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC7_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC7SrR = crate::BitReader;
#[doc = "Field `pC7_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC7SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC7_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC7E3R = crate::BitReader;
#[doc = "Field `pC7_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC7E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC7_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC7DisR = crate::BitReader;
#[doc = "Field `pC7_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC7DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c7_sel(&self) -> PC7SelR {
        PC7SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c7_ie(&self) -> PC7IeR {
        PC7IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c7_pupdc(&self) -> PC7PupdcR {
        PC7PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c7_e2(&self) -> PC7E2R {
        PC7E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c7_smt(&self) -> PC7SmtR {
        PC7SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c7_sr(&self) -> PC7SrR {
        PC7SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c7_e3(&self) -> PC7E3R {
        PC7E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c7_dis(&self) -> PC7DisR {
        PC7DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c7_sel(&mut self) -> PC7SelW<'_, RegPc7Spec> {
        PC7SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c7_ie(&mut self) -> PC7IeW<'_, RegPc7Spec> {
        PC7IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c7_pupdc(&mut self) -> PC7PupdcW<'_, RegPc7Spec> {
        PC7PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c7_e2(&mut self) -> PC7E2W<'_, RegPc7Spec> {
        PC7E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c7_smt(&mut self) -> PC7SmtW<'_, RegPc7Spec> {
        PC7SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c7_sr(&mut self) -> PC7SrW<'_, RegPc7Spec> {
        PC7SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c7_e3(&mut self) -> PC7E3W<'_, RegPc7Spec> {
        PC7E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c7_dis(&mut self) -> PC7DisW<'_, RegPc7Spec> {
        PC7DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc7Spec;
impl crate::RegisterSpec for RegPc7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc7::R`](R) reader structure"]
impl crate::Readable for RegPc7Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc7::W`](W) writer structure"]
impl crate::Writable for RegPc7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC7 to value 0x1900"]
impl crate::Resettable for RegPc7Spec {
    const RESET_VALUE: u32 = 0x1900;
}
