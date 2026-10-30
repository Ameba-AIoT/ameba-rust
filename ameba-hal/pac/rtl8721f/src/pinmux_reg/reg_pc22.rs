#[doc = "Register `REG_PC22` reader"]
pub type R = crate::R<RegPc22Spec>;
#[doc = "Register `REG_PC22` writer"]
pub type W = crate::W<RegPc22Spec>;
#[doc = "Field `pC22_sel` reader - PAD pinmux function ID selection"]
pub type PC22SelR = crate::FieldReader;
#[doc = "Field `pC22_sel` writer - PAD pinmux function ID selection"]
pub type PC22SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC22_ie` reader - PAD input enable control"]
pub type PC22IeR = crate::BitReader;
#[doc = "Field `pC22_ie` writer - PAD input enable control"]
pub type PC22IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC22_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC22PupdcR = crate::BitReader;
#[doc = "Field `pC22_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC22PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC22_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC22E2R = crate::BitReader;
#[doc = "Field `pC22_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC22E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC22_smt` reader - PAD Schmit control"]
pub type PC22SmtR = crate::BitReader;
#[doc = "Field `pC22_smt` writer - PAD Schmit control"]
pub type PC22SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC22_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC22SrR = crate::BitReader;
#[doc = "Field `pC22_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC22SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC22_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC22E3R = crate::BitReader;
#[doc = "Field `pC22_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC22E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC22_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC22DisR = crate::BitReader;
#[doc = "Field `pC22_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC22DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c22_sel(&self) -> PC22SelR {
        PC22SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c22_ie(&self) -> PC22IeR {
        PC22IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c22_pupdc(&self) -> PC22PupdcR {
        PC22PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c22_e2(&self) -> PC22E2R {
        PC22E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c22_smt(&self) -> PC22SmtR {
        PC22SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c22_sr(&self) -> PC22SrR {
        PC22SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c22_e3(&self) -> PC22E3R {
        PC22E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c22_dis(&self) -> PC22DisR {
        PC22DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c22_sel(&mut self) -> PC22SelW<'_, RegPc22Spec> {
        PC22SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c22_ie(&mut self) -> PC22IeW<'_, RegPc22Spec> {
        PC22IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c22_pupdc(&mut self) -> PC22PupdcW<'_, RegPc22Spec> {
        PC22PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c22_e2(&mut self) -> PC22E2W<'_, RegPc22Spec> {
        PC22E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c22_smt(&mut self) -> PC22SmtW<'_, RegPc22Spec> {
        PC22SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c22_sr(&mut self) -> PC22SrW<'_, RegPc22Spec> {
        PC22SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c22_e3(&mut self) -> PC22E3W<'_, RegPc22Spec> {
        PC22E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c22_dis(&mut self) -> PC22DisW<'_, RegPc22Spec> {
        PC22DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc22Spec;
impl crate::RegisterSpec for RegPc22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc22::R`](R) reader structure"]
impl crate::Readable for RegPc22Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc22::W`](W) writer structure"]
impl crate::Writable for RegPc22Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC22 to value 0x1900"]
impl crate::Resettable for RegPc22Spec {
    const RESET_VALUE: u32 = 0x1900;
}
