#[doc = "Register `REG_PA27` reader"]
pub type R = crate::R<RegPa27Spec>;
#[doc = "Register `REG_PA27` writer"]
pub type W = crate::W<RegPa27Spec>;
#[doc = "Field `pA27_sel` reader - PAD pinmux function ID selection"]
pub type PA27SelR = crate::FieldReader;
#[doc = "Field `pA27_sel` writer - PAD pinmux function ID selection"]
pub type PA27SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA27_ie` reader - PAD input enable control"]
pub type PA27IeR = crate::BitReader;
#[doc = "Field `pA27_ie` writer - PAD input enable control"]
pub type PA27IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA27_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA27PupdcR = crate::BitReader;
#[doc = "Field `pA27_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA27PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA27_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA27E2R = crate::BitReader;
#[doc = "Field `pA27_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA27E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA27_smt` reader - PAD Schmit control"]
pub type PA27SmtR = crate::BitReader;
#[doc = "Field `pA27_smt` writer - PAD Schmit control"]
pub type PA27SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA27_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA27SrR = crate::BitReader;
#[doc = "Field `pA27_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA27SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA27_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA27E3R = crate::BitReader;
#[doc = "Field `pA27_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA27E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA27_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA27DisR = crate::BitReader;
#[doc = "Field `pA27_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA27DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a27_sel(&self) -> PA27SelR {
        PA27SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a27_ie(&self) -> PA27IeR {
        PA27IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a27_pupdc(&self) -> PA27PupdcR {
        PA27PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a27_e2(&self) -> PA27E2R {
        PA27E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a27_smt(&self) -> PA27SmtR {
        PA27SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a27_sr(&self) -> PA27SrR {
        PA27SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a27_e3(&self) -> PA27E3R {
        PA27E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a27_dis(&self) -> PA27DisR {
        PA27DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a27_sel(&mut self) -> PA27SelW<'_, RegPa27Spec> {
        PA27SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a27_ie(&mut self) -> PA27IeW<'_, RegPa27Spec> {
        PA27IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a27_pupdc(&mut self) -> PA27PupdcW<'_, RegPa27Spec> {
        PA27PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a27_e2(&mut self) -> PA27E2W<'_, RegPa27Spec> {
        PA27E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a27_smt(&mut self) -> PA27SmtW<'_, RegPa27Spec> {
        PA27SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a27_sr(&mut self) -> PA27SrW<'_, RegPa27Spec> {
        PA27SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a27_e3(&mut self) -> PA27E3W<'_, RegPa27Spec> {
        PA27E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a27_dis(&mut self) -> PA27DisW<'_, RegPa27Spec> {
        PA27DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa27::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa27::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa27Spec;
impl crate::RegisterSpec for RegPa27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa27::R`](R) reader structure"]
impl crate::Readable for RegPa27Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa27::W`](W) writer structure"]
impl crate::Writable for RegPa27Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA27 to value 0x1900"]
impl crate::Resettable for RegPa27Spec {
    const RESET_VALUE: u32 = 0x1900;
}
