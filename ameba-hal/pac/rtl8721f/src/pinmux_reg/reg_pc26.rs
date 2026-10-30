#[doc = "Register `REG_PC26` reader"]
pub type R = crate::R<RegPc26Spec>;
#[doc = "Register `REG_PC26` writer"]
pub type W = crate::W<RegPc26Spec>;
#[doc = "Field `pC26_sel` reader - PAD pinmux function ID selection"]
pub type PC26SelR = crate::FieldReader;
#[doc = "Field `pC26_sel` writer - PAD pinmux function ID selection"]
pub type PC26SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC26_ie` reader - PAD input enable control"]
pub type PC26IeR = crate::BitReader;
#[doc = "Field `pC26_ie` writer - PAD input enable control"]
pub type PC26IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC26_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC26PupdcR = crate::BitReader;
#[doc = "Field `pC26_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC26PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC26_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC26E2R = crate::BitReader;
#[doc = "Field `pC26_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC26E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC26_smt` reader - PAD Schmit control"]
pub type PC26SmtR = crate::BitReader;
#[doc = "Field `pC26_smt` writer - PAD Schmit control"]
pub type PC26SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC26_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC26SrR = crate::BitReader;
#[doc = "Field `pC26_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC26SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC26_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC26E3R = crate::BitReader;
#[doc = "Field `pC26_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC26E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC26_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC26DisR = crate::BitReader;
#[doc = "Field `pC26_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC26DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c26_sel(&self) -> PC26SelR {
        PC26SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c26_ie(&self) -> PC26IeR {
        PC26IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c26_pupdc(&self) -> PC26PupdcR {
        PC26PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c26_e2(&self) -> PC26E2R {
        PC26E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c26_smt(&self) -> PC26SmtR {
        PC26SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c26_sr(&self) -> PC26SrR {
        PC26SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c26_e3(&self) -> PC26E3R {
        PC26E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c26_dis(&self) -> PC26DisR {
        PC26DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c26_sel(&mut self) -> PC26SelW<'_, RegPc26Spec> {
        PC26SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c26_ie(&mut self) -> PC26IeW<'_, RegPc26Spec> {
        PC26IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c26_pupdc(&mut self) -> PC26PupdcW<'_, RegPc26Spec> {
        PC26PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c26_e2(&mut self) -> PC26E2W<'_, RegPc26Spec> {
        PC26E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c26_smt(&mut self) -> PC26SmtW<'_, RegPc26Spec> {
        PC26SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c26_sr(&mut self) -> PC26SrW<'_, RegPc26Spec> {
        PC26SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c26_e3(&mut self) -> PC26E3W<'_, RegPc26Spec> {
        PC26E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c26_dis(&mut self) -> PC26DisW<'_, RegPc26Spec> {
        PC26DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc26::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc26::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc26Spec;
impl crate::RegisterSpec for RegPc26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc26::R`](R) reader structure"]
impl crate::Readable for RegPc26Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc26::W`](W) writer structure"]
impl crate::Writable for RegPc26Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC26 to value 0x1900"]
impl crate::Resettable for RegPc26Spec {
    const RESET_VALUE: u32 = 0x1900;
}
