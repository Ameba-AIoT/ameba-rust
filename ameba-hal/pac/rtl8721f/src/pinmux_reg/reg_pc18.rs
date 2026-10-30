#[doc = "Register `REG_PC18` reader"]
pub type R = crate::R<RegPc18Spec>;
#[doc = "Register `REG_PC18` writer"]
pub type W = crate::W<RegPc18Spec>;
#[doc = "Field `pC18_sel` reader - PAD pinmux function ID selection"]
pub type PC18SelR = crate::FieldReader;
#[doc = "Field `pC18_sel` writer - PAD pinmux function ID selection"]
pub type PC18SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC18_ie` reader - PAD input enable control"]
pub type PC18IeR = crate::BitReader;
#[doc = "Field `pC18_ie` writer - PAD input enable control"]
pub type PC18IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC18_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC18PupdcR = crate::BitReader;
#[doc = "Field `pC18_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC18PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC18_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC18E2R = crate::BitReader;
#[doc = "Field `pC18_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC18E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC18_smt` reader - PAD Schmit control"]
pub type PC18SmtR = crate::BitReader;
#[doc = "Field `pC18_smt` writer - PAD Schmit control"]
pub type PC18SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC18_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC18SrR = crate::BitReader;
#[doc = "Field `pC18_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC18SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC18_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC18E3R = crate::BitReader;
#[doc = "Field `pC18_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC18E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC18_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC18DisR = crate::BitReader;
#[doc = "Field `pC18_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC18DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c18_sel(&self) -> PC18SelR {
        PC18SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c18_ie(&self) -> PC18IeR {
        PC18IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c18_pupdc(&self) -> PC18PupdcR {
        PC18PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c18_e2(&self) -> PC18E2R {
        PC18E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c18_smt(&self) -> PC18SmtR {
        PC18SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c18_sr(&self) -> PC18SrR {
        PC18SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c18_e3(&self) -> PC18E3R {
        PC18E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c18_dis(&self) -> PC18DisR {
        PC18DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c18_sel(&mut self) -> PC18SelW<'_, RegPc18Spec> {
        PC18SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c18_ie(&mut self) -> PC18IeW<'_, RegPc18Spec> {
        PC18IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c18_pupdc(&mut self) -> PC18PupdcW<'_, RegPc18Spec> {
        PC18PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c18_e2(&mut self) -> PC18E2W<'_, RegPc18Spec> {
        PC18E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c18_smt(&mut self) -> PC18SmtW<'_, RegPc18Spec> {
        PC18SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c18_sr(&mut self) -> PC18SrW<'_, RegPc18Spec> {
        PC18SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c18_e3(&mut self) -> PC18E3W<'_, RegPc18Spec> {
        PC18E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c18_dis(&mut self) -> PC18DisW<'_, RegPc18Spec> {
        PC18DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc18Spec;
impl crate::RegisterSpec for RegPc18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc18::R`](R) reader structure"]
impl crate::Readable for RegPc18Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc18::W`](W) writer structure"]
impl crate::Writable for RegPc18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC18 to value 0x1900"]
impl crate::Resettable for RegPc18Spec {
    const RESET_VALUE: u32 = 0x1900;
}
