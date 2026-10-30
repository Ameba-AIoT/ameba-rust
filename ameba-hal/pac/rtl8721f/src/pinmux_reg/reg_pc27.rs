#[doc = "Register `REG_PC27` reader"]
pub type R = crate::R<RegPc27Spec>;
#[doc = "Register `REG_PC27` writer"]
pub type W = crate::W<RegPc27Spec>;
#[doc = "Field `pC27_sel` reader - PAD pinmux function ID selection"]
pub type PC27SelR = crate::FieldReader;
#[doc = "Field `pC27_sel` writer - PAD pinmux function ID selection"]
pub type PC27SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC27_ie` reader - PAD input enable control"]
pub type PC27IeR = crate::BitReader;
#[doc = "Field `pC27_ie` writer - PAD input enable control"]
pub type PC27IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC27_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC27PupdcR = crate::BitReader;
#[doc = "Field `pC27_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC27PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC27_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC27E2R = crate::BitReader;
#[doc = "Field `pC27_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC27E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC27_smt` reader - PAD Schmit control"]
pub type PC27SmtR = crate::BitReader;
#[doc = "Field `pC27_smt` writer - PAD Schmit control"]
pub type PC27SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC27_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC27SrR = crate::BitReader;
#[doc = "Field `pC27_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC27SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC27_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC27E3R = crate::BitReader;
#[doc = "Field `pC27_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC27E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC27_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC27DisR = crate::BitReader;
#[doc = "Field `pC27_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC27DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c27_sel(&self) -> PC27SelR {
        PC27SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c27_ie(&self) -> PC27IeR {
        PC27IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c27_pupdc(&self) -> PC27PupdcR {
        PC27PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c27_e2(&self) -> PC27E2R {
        PC27E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c27_smt(&self) -> PC27SmtR {
        PC27SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c27_sr(&self) -> PC27SrR {
        PC27SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c27_e3(&self) -> PC27E3R {
        PC27E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c27_dis(&self) -> PC27DisR {
        PC27DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c27_sel(&mut self) -> PC27SelW<'_, RegPc27Spec> {
        PC27SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c27_ie(&mut self) -> PC27IeW<'_, RegPc27Spec> {
        PC27IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c27_pupdc(&mut self) -> PC27PupdcW<'_, RegPc27Spec> {
        PC27PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c27_e2(&mut self) -> PC27E2W<'_, RegPc27Spec> {
        PC27E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c27_smt(&mut self) -> PC27SmtW<'_, RegPc27Spec> {
        PC27SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c27_sr(&mut self) -> PC27SrW<'_, RegPc27Spec> {
        PC27SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c27_e3(&mut self) -> PC27E3W<'_, RegPc27Spec> {
        PC27E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c27_dis(&mut self) -> PC27DisW<'_, RegPc27Spec> {
        PC27DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc27::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc27::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc27Spec;
impl crate::RegisterSpec for RegPc27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc27::R`](R) reader structure"]
impl crate::Readable for RegPc27Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc27::W`](W) writer structure"]
impl crate::Writable for RegPc27Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC27 to value 0x1900"]
impl crate::Resettable for RegPc27Spec {
    const RESET_VALUE: u32 = 0x1900;
}
