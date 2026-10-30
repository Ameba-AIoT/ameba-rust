#[doc = "Register `REG_PC14` reader"]
pub type R = crate::R<RegPc14Spec>;
#[doc = "Register `REG_PC14` writer"]
pub type W = crate::W<RegPc14Spec>;
#[doc = "Field `pC14_sel` reader - PAD pinmux function ID selection"]
pub type PC14SelR = crate::FieldReader;
#[doc = "Field `pC14_sel` writer - PAD pinmux function ID selection"]
pub type PC14SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC14_ie` reader - PAD input enable control"]
pub type PC14IeR = crate::BitReader;
#[doc = "Field `pC14_ie` writer - PAD input enable control"]
pub type PC14IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC14_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC14PupdcR = crate::BitReader;
#[doc = "Field `pC14_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC14PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC14_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC14E2R = crate::BitReader;
#[doc = "Field `pC14_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC14E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC14_smt` reader - PAD Schmit control"]
pub type PC14SmtR = crate::BitReader;
#[doc = "Field `pC14_smt` writer - PAD Schmit control"]
pub type PC14SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC14_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC14SrR = crate::BitReader;
#[doc = "Field `pC14_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC14SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC14_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC14E3R = crate::BitReader;
#[doc = "Field `pC14_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC14E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC14_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC14DisR = crate::BitReader;
#[doc = "Field `pC14_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC14DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c14_sel(&self) -> PC14SelR {
        PC14SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c14_ie(&self) -> PC14IeR {
        PC14IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c14_pupdc(&self) -> PC14PupdcR {
        PC14PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c14_e2(&self) -> PC14E2R {
        PC14E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c14_smt(&self) -> PC14SmtR {
        PC14SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c14_sr(&self) -> PC14SrR {
        PC14SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c14_e3(&self) -> PC14E3R {
        PC14E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c14_dis(&self) -> PC14DisR {
        PC14DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c14_sel(&mut self) -> PC14SelW<'_, RegPc14Spec> {
        PC14SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c14_ie(&mut self) -> PC14IeW<'_, RegPc14Spec> {
        PC14IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c14_pupdc(&mut self) -> PC14PupdcW<'_, RegPc14Spec> {
        PC14PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c14_e2(&mut self) -> PC14E2W<'_, RegPc14Spec> {
        PC14E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c14_smt(&mut self) -> PC14SmtW<'_, RegPc14Spec> {
        PC14SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c14_sr(&mut self) -> PC14SrW<'_, RegPc14Spec> {
        PC14SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c14_e3(&mut self) -> PC14E3W<'_, RegPc14Spec> {
        PC14E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c14_dis(&mut self) -> PC14DisW<'_, RegPc14Spec> {
        PC14DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc14Spec;
impl crate::RegisterSpec for RegPc14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc14::R`](R) reader structure"]
impl crate::Readable for RegPc14Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc14::W`](W) writer structure"]
impl crate::Writable for RegPc14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC14 to value 0x1900"]
impl crate::Resettable for RegPc14Spec {
    const RESET_VALUE: u32 = 0x1900;
}
