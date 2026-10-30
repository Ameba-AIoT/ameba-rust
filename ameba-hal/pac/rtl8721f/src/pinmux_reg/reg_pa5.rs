#[doc = "Register `REG_PA5` reader"]
pub type R = crate::R<RegPa5Spec>;
#[doc = "Register `REG_PA5` writer"]
pub type W = crate::W<RegPa5Spec>;
#[doc = "Field `pA5_sel` reader - PAD pinmux function ID selection"]
pub type PA5SelR = crate::FieldReader;
#[doc = "Field `pA5_sel` writer - PAD pinmux function ID selection"]
pub type PA5SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA5_ie` reader - PAD input enable control"]
pub type PA5IeR = crate::BitReader;
#[doc = "Field `pA5_ie` writer - PAD input enable control"]
pub type PA5IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA5_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA5PupdcR = crate::BitReader;
#[doc = "Field `pA5_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA5PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA5_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA5E2R = crate::BitReader;
#[doc = "Field `pA5_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA5E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA5_smt` reader - PAD Schmit control"]
pub type PA5SmtR = crate::BitReader;
#[doc = "Field `pA5_smt` writer - PAD Schmit control"]
pub type PA5SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA5_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA5SrR = crate::BitReader;
#[doc = "Field `pA5_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA5SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA5_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA5E3R = crate::BitReader;
#[doc = "Field `pA5_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA5E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA5_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA5DisR = crate::BitReader;
#[doc = "Field `pA5_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA5DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a5_sel(&self) -> PA5SelR {
        PA5SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a5_ie(&self) -> PA5IeR {
        PA5IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a5_pupdc(&self) -> PA5PupdcR {
        PA5PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a5_e2(&self) -> PA5E2R {
        PA5E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a5_smt(&self) -> PA5SmtR {
        PA5SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a5_sr(&self) -> PA5SrR {
        PA5SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a5_e3(&self) -> PA5E3R {
        PA5E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a5_dis(&self) -> PA5DisR {
        PA5DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a5_sel(&mut self) -> PA5SelW<'_, RegPa5Spec> {
        PA5SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a5_ie(&mut self) -> PA5IeW<'_, RegPa5Spec> {
        PA5IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a5_pupdc(&mut self) -> PA5PupdcW<'_, RegPa5Spec> {
        PA5PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a5_e2(&mut self) -> PA5E2W<'_, RegPa5Spec> {
        PA5E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a5_smt(&mut self) -> PA5SmtW<'_, RegPa5Spec> {
        PA5SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a5_sr(&mut self) -> PA5SrW<'_, RegPa5Spec> {
        PA5SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a5_e3(&mut self) -> PA5E3W<'_, RegPa5Spec> {
        PA5E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a5_dis(&mut self) -> PA5DisW<'_, RegPa5Spec> {
        PA5DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa5Spec;
impl crate::RegisterSpec for RegPa5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa5::R`](R) reader structure"]
impl crate::Readable for RegPa5Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa5::W`](W) writer structure"]
impl crate::Writable for RegPa5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA5 to value 0x1900"]
impl crate::Resettable for RegPa5Spec {
    const RESET_VALUE: u32 = 0x1900;
}
