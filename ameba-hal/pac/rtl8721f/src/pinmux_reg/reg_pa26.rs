#[doc = "Register `REG_PA26` reader"]
pub type R = crate::R<RegPa26Spec>;
#[doc = "Register `REG_PA26` writer"]
pub type W = crate::W<RegPa26Spec>;
#[doc = "Field `pA26_sel` reader - PAD pinmux function ID selection"]
pub type PA26SelR = crate::FieldReader;
#[doc = "Field `pA26_sel` writer - PAD pinmux function ID selection"]
pub type PA26SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA26_ie` reader - PAD input enable control"]
pub type PA26IeR = crate::BitReader;
#[doc = "Field `pA26_ie` writer - PAD input enable control"]
pub type PA26IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA26_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA26PupdcR = crate::BitReader;
#[doc = "Field `pA26_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA26PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA26_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA26E2R = crate::BitReader;
#[doc = "Field `pA26_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA26E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA26_smt` reader - PAD Schmit control"]
pub type PA26SmtR = crate::BitReader;
#[doc = "Field `pA26_smt` writer - PAD Schmit control"]
pub type PA26SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA26_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA26SrR = crate::BitReader;
#[doc = "Field `pA26_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA26SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA26_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA26E3R = crate::BitReader;
#[doc = "Field `pA26_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA26E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA26_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA26DisR = crate::BitReader;
#[doc = "Field `pA26_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA26DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a26_sel(&self) -> PA26SelR {
        PA26SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a26_ie(&self) -> PA26IeR {
        PA26IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a26_pupdc(&self) -> PA26PupdcR {
        PA26PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a26_e2(&self) -> PA26E2R {
        PA26E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a26_smt(&self) -> PA26SmtR {
        PA26SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a26_sr(&self) -> PA26SrR {
        PA26SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a26_e3(&self) -> PA26E3R {
        PA26E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a26_dis(&self) -> PA26DisR {
        PA26DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a26_sel(&mut self) -> PA26SelW<'_, RegPa26Spec> {
        PA26SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a26_ie(&mut self) -> PA26IeW<'_, RegPa26Spec> {
        PA26IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a26_pupdc(&mut self) -> PA26PupdcW<'_, RegPa26Spec> {
        PA26PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a26_e2(&mut self) -> PA26E2W<'_, RegPa26Spec> {
        PA26E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a26_smt(&mut self) -> PA26SmtW<'_, RegPa26Spec> {
        PA26SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a26_sr(&mut self) -> PA26SrW<'_, RegPa26Spec> {
        PA26SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a26_e3(&mut self) -> PA26E3W<'_, RegPa26Spec> {
        PA26E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a26_dis(&mut self) -> PA26DisW<'_, RegPa26Spec> {
        PA26DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa26::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa26::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa26Spec;
impl crate::RegisterSpec for RegPa26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa26::R`](R) reader structure"]
impl crate::Readable for RegPa26Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa26::W`](W) writer structure"]
impl crate::Writable for RegPa26Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA26 to value 0x1900"]
impl crate::Resettable for RegPa26Spec {
    const RESET_VALUE: u32 = 0x1900;
}
