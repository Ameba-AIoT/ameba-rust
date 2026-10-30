#[doc = "Register `REG_PA15` reader"]
pub type R = crate::R<RegPa15Spec>;
#[doc = "Register `REG_PA15` writer"]
pub type W = crate::W<RegPa15Spec>;
#[doc = "Field `pA15_sel` reader - PAD pinmux function ID selection"]
pub type PA15SelR = crate::FieldReader;
#[doc = "Field `pA15_sel` writer - PAD pinmux function ID selection"]
pub type PA15SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA15_ie` reader - PAD input enable control"]
pub type PA15IeR = crate::BitReader;
#[doc = "Field `pA15_ie` writer - PAD input enable control"]
pub type PA15IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA15_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA15PupdcR = crate::BitReader;
#[doc = "Field `pA15_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA15PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA15_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA15E2R = crate::BitReader;
#[doc = "Field `pA15_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA15E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA15_smt` reader - PAD Schmit control"]
pub type PA15SmtR = crate::BitReader;
#[doc = "Field `pA15_smt` writer - PAD Schmit control"]
pub type PA15SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA15_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA15SrR = crate::BitReader;
#[doc = "Field `pA15_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA15SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA15_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA15E3R = crate::BitReader;
#[doc = "Field `pA15_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA15E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA15_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA15DisR = crate::BitReader;
#[doc = "Field `pA15_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA15DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a15_sel(&self) -> PA15SelR {
        PA15SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a15_ie(&self) -> PA15IeR {
        PA15IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a15_pupdc(&self) -> PA15PupdcR {
        PA15PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a15_e2(&self) -> PA15E2R {
        PA15E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a15_smt(&self) -> PA15SmtR {
        PA15SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a15_sr(&self) -> PA15SrR {
        PA15SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a15_e3(&self) -> PA15E3R {
        PA15E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a15_dis(&self) -> PA15DisR {
        PA15DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a15_sel(&mut self) -> PA15SelW<'_, RegPa15Spec> {
        PA15SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a15_ie(&mut self) -> PA15IeW<'_, RegPa15Spec> {
        PA15IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a15_pupdc(&mut self) -> PA15PupdcW<'_, RegPa15Spec> {
        PA15PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a15_e2(&mut self) -> PA15E2W<'_, RegPa15Spec> {
        PA15E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a15_smt(&mut self) -> PA15SmtW<'_, RegPa15Spec> {
        PA15SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a15_sr(&mut self) -> PA15SrW<'_, RegPa15Spec> {
        PA15SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a15_e3(&mut self) -> PA15E3W<'_, RegPa15Spec> {
        PA15E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a15_dis(&mut self) -> PA15DisW<'_, RegPa15Spec> {
        PA15DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa15Spec;
impl crate::RegisterSpec for RegPa15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa15::R`](R) reader structure"]
impl crate::Readable for RegPa15Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa15::W`](W) writer structure"]
impl crate::Writable for RegPa15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA15 to value 0x1900"]
impl crate::Resettable for RegPa15Spec {
    const RESET_VALUE: u32 = 0x1900;
}
