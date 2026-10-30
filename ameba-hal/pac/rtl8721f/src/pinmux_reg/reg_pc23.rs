#[doc = "Register `REG_PC23` reader"]
pub type R = crate::R<RegPc23Spec>;
#[doc = "Register `REG_PC23` writer"]
pub type W = crate::W<RegPc23Spec>;
#[doc = "Field `pC23_sel` reader - PAD pinmux function ID selection"]
pub type PC23SelR = crate::FieldReader;
#[doc = "Field `pC23_sel` writer - PAD pinmux function ID selection"]
pub type PC23SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC23_ie` reader - PAD input enable control"]
pub type PC23IeR = crate::BitReader;
#[doc = "Field `pC23_ie` writer - PAD input enable control"]
pub type PC23IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC23_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC23PupdcR = crate::BitReader;
#[doc = "Field `pC23_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC23PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC23_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC23E2R = crate::BitReader;
#[doc = "Field `pC23_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC23E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC23_smt` reader - PAD Schmit control"]
pub type PC23SmtR = crate::BitReader;
#[doc = "Field `pC23_smt` writer - PAD Schmit control"]
pub type PC23SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC23_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC23SrR = crate::BitReader;
#[doc = "Field `pC23_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC23SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC23_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC23E3R = crate::BitReader;
#[doc = "Field `pC23_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC23E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC23_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC23DisR = crate::BitReader;
#[doc = "Field `pC23_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC23DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c23_sel(&self) -> PC23SelR {
        PC23SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c23_ie(&self) -> PC23IeR {
        PC23IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c23_pupdc(&self) -> PC23PupdcR {
        PC23PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c23_e2(&self) -> PC23E2R {
        PC23E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c23_smt(&self) -> PC23SmtR {
        PC23SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c23_sr(&self) -> PC23SrR {
        PC23SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c23_e3(&self) -> PC23E3R {
        PC23E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c23_dis(&self) -> PC23DisR {
        PC23DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c23_sel(&mut self) -> PC23SelW<'_, RegPc23Spec> {
        PC23SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c23_ie(&mut self) -> PC23IeW<'_, RegPc23Spec> {
        PC23IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c23_pupdc(&mut self) -> PC23PupdcW<'_, RegPc23Spec> {
        PC23PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c23_e2(&mut self) -> PC23E2W<'_, RegPc23Spec> {
        PC23E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c23_smt(&mut self) -> PC23SmtW<'_, RegPc23Spec> {
        PC23SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c23_sr(&mut self) -> PC23SrW<'_, RegPc23Spec> {
        PC23SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c23_e3(&mut self) -> PC23E3W<'_, RegPc23Spec> {
        PC23E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c23_dis(&mut self) -> PC23DisW<'_, RegPc23Spec> {
        PC23DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc23Spec;
impl crate::RegisterSpec for RegPc23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc23::R`](R) reader structure"]
impl crate::Readable for RegPc23Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc23::W`](W) writer structure"]
impl crate::Writable for RegPc23Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC23 to value 0x1900"]
impl crate::Resettable for RegPc23Spec {
    const RESET_VALUE: u32 = 0x1900;
}
