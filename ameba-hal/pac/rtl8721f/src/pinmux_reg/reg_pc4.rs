#[doc = "Register `REG_PC4` reader"]
pub type R = crate::R<RegPc4Spec>;
#[doc = "Register `REG_PC4` writer"]
pub type W = crate::W<RegPc4Spec>;
#[doc = "Field `pC4_sel` reader - PAD pinmux function ID selection"]
pub type PC4SelR = crate::FieldReader;
#[doc = "Field `pC4_sel` writer - PAD pinmux function ID selection"]
pub type PC4SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC4_ie` reader - PAD input enable control"]
pub type PC4IeR = crate::BitReader;
#[doc = "Field `pC4_ie` writer - PAD input enable control"]
pub type PC4IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC4_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC4PupdcR = crate::BitReader;
#[doc = "Field `pC4_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC4PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC4_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC4E2R = crate::BitReader;
#[doc = "Field `pC4_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC4E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC4_smt` reader - PAD Schmit control"]
pub type PC4SmtR = crate::BitReader;
#[doc = "Field `pC4_smt` writer - PAD Schmit control"]
pub type PC4SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC4_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC4SrR = crate::BitReader;
#[doc = "Field `pC4_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC4SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC4_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC4E3R = crate::BitReader;
#[doc = "Field `pC4_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC4E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC4_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC4DisR = crate::BitReader;
#[doc = "Field `pC4_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC4DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c4_sel(&self) -> PC4SelR {
        PC4SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c4_ie(&self) -> PC4IeR {
        PC4IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c4_pupdc(&self) -> PC4PupdcR {
        PC4PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c4_e2(&self) -> PC4E2R {
        PC4E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c4_smt(&self) -> PC4SmtR {
        PC4SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c4_sr(&self) -> PC4SrR {
        PC4SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c4_e3(&self) -> PC4E3R {
        PC4E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c4_dis(&self) -> PC4DisR {
        PC4DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c4_sel(&mut self) -> PC4SelW<'_, RegPc4Spec> {
        PC4SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c4_ie(&mut self) -> PC4IeW<'_, RegPc4Spec> {
        PC4IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c4_pupdc(&mut self) -> PC4PupdcW<'_, RegPc4Spec> {
        PC4PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c4_e2(&mut self) -> PC4E2W<'_, RegPc4Spec> {
        PC4E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c4_smt(&mut self) -> PC4SmtW<'_, RegPc4Spec> {
        PC4SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c4_sr(&mut self) -> PC4SrW<'_, RegPc4Spec> {
        PC4SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c4_e3(&mut self) -> PC4E3W<'_, RegPc4Spec> {
        PC4E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c4_dis(&mut self) -> PC4DisW<'_, RegPc4Spec> {
        PC4DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc4Spec;
impl crate::RegisterSpec for RegPc4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc4::R`](R) reader structure"]
impl crate::Readable for RegPc4Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc4::W`](W) writer structure"]
impl crate::Writable for RegPc4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC4 to value 0x1900"]
impl crate::Resettable for RegPc4Spec {
    const RESET_VALUE: u32 = 0x1900;
}
