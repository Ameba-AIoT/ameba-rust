#[doc = "Register `REG_PC25` reader"]
pub type R = crate::R<RegPc25Spec>;
#[doc = "Register `REG_PC25` writer"]
pub type W = crate::W<RegPc25Spec>;
#[doc = "Field `pC25_sel` reader - PAD pinmux function ID selection"]
pub type PC25SelR = crate::FieldReader;
#[doc = "Field `pC25_sel` writer - PAD pinmux function ID selection"]
pub type PC25SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC25_ie` reader - PAD input enable control"]
pub type PC25IeR = crate::BitReader;
#[doc = "Field `pC25_ie` writer - PAD input enable control"]
pub type PC25IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC25_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC25PupdcR = crate::BitReader;
#[doc = "Field `pC25_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC25PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC25_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC25E2R = crate::BitReader;
#[doc = "Field `pC25_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC25E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC25_smt` reader - PAD Schmit control"]
pub type PC25SmtR = crate::BitReader;
#[doc = "Field `pC25_smt` writer - PAD Schmit control"]
pub type PC25SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC25_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC25SrR = crate::BitReader;
#[doc = "Field `pC25_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC25SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC25_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC25E3R = crate::BitReader;
#[doc = "Field `pC25_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC25E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC25_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC25DisR = crate::BitReader;
#[doc = "Field `pC25_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC25DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c25_sel(&self) -> PC25SelR {
        PC25SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c25_ie(&self) -> PC25IeR {
        PC25IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c25_pupdc(&self) -> PC25PupdcR {
        PC25PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c25_e2(&self) -> PC25E2R {
        PC25E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c25_smt(&self) -> PC25SmtR {
        PC25SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c25_sr(&self) -> PC25SrR {
        PC25SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c25_e3(&self) -> PC25E3R {
        PC25E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c25_dis(&self) -> PC25DisR {
        PC25DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c25_sel(&mut self) -> PC25SelW<'_, RegPc25Spec> {
        PC25SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c25_ie(&mut self) -> PC25IeW<'_, RegPc25Spec> {
        PC25IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c25_pupdc(&mut self) -> PC25PupdcW<'_, RegPc25Spec> {
        PC25PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c25_e2(&mut self) -> PC25E2W<'_, RegPc25Spec> {
        PC25E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c25_smt(&mut self) -> PC25SmtW<'_, RegPc25Spec> {
        PC25SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c25_sr(&mut self) -> PC25SrW<'_, RegPc25Spec> {
        PC25SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c25_e3(&mut self) -> PC25E3W<'_, RegPc25Spec> {
        PC25E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c25_dis(&mut self) -> PC25DisW<'_, RegPc25Spec> {
        PC25DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc25Spec;
impl crate::RegisterSpec for RegPc25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc25::R`](R) reader structure"]
impl crate::Readable for RegPc25Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc25::W`](W) writer structure"]
impl crate::Writable for RegPc25Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC25 to value 0x1900"]
impl crate::Resettable for RegPc25Spec {
    const RESET_VALUE: u32 = 0x1900;
}
