#[doc = "Register `REG_PA18` reader"]
pub type R = crate::R<RegPa18Spec>;
#[doc = "Register `REG_PA18` writer"]
pub type W = crate::W<RegPa18Spec>;
#[doc = "Field `pA18_sel` reader - PAD pinmux function ID selection"]
pub type PA18SelR = crate::FieldReader;
#[doc = "Field `pA18_sel` writer - PAD pinmux function ID selection"]
pub type PA18SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA18_ie` reader - PAD input enable control"]
pub type PA18IeR = crate::BitReader;
#[doc = "Field `pA18_ie` writer - PAD input enable control"]
pub type PA18IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA18_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA18PupdcR = crate::BitReader;
#[doc = "Field `pA18_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA18PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA18_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA18E2R = crate::BitReader;
#[doc = "Field `pA18_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA18E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA18_smt` reader - PAD Schmit control"]
pub type PA18SmtR = crate::BitReader;
#[doc = "Field `pA18_smt` writer - PAD Schmit control"]
pub type PA18SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA18_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA18SrR = crate::BitReader;
#[doc = "Field `pA18_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA18SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA18_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA18E3R = crate::BitReader;
#[doc = "Field `pA18_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA18E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA18_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA18DisR = crate::BitReader;
#[doc = "Field `pA18_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA18DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a18_sel(&self) -> PA18SelR {
        PA18SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a18_ie(&self) -> PA18IeR {
        PA18IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a18_pupdc(&self) -> PA18PupdcR {
        PA18PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a18_e2(&self) -> PA18E2R {
        PA18E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a18_smt(&self) -> PA18SmtR {
        PA18SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a18_sr(&self) -> PA18SrR {
        PA18SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a18_e3(&self) -> PA18E3R {
        PA18E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a18_dis(&self) -> PA18DisR {
        PA18DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a18_sel(&mut self) -> PA18SelW<'_, RegPa18Spec> {
        PA18SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a18_ie(&mut self) -> PA18IeW<'_, RegPa18Spec> {
        PA18IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a18_pupdc(&mut self) -> PA18PupdcW<'_, RegPa18Spec> {
        PA18PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a18_e2(&mut self) -> PA18E2W<'_, RegPa18Spec> {
        PA18E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a18_smt(&mut self) -> PA18SmtW<'_, RegPa18Spec> {
        PA18SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a18_sr(&mut self) -> PA18SrW<'_, RegPa18Spec> {
        PA18SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a18_e3(&mut self) -> PA18E3W<'_, RegPa18Spec> {
        PA18E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a18_dis(&mut self) -> PA18DisW<'_, RegPa18Spec> {
        PA18DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa18Spec;
impl crate::RegisterSpec for RegPa18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa18::R`](R) reader structure"]
impl crate::Readable for RegPa18Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa18::W`](W) writer structure"]
impl crate::Writable for RegPa18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA18 to value 0x1900"]
impl crate::Resettable for RegPa18Spec {
    const RESET_VALUE: u32 = 0x1900;
}
