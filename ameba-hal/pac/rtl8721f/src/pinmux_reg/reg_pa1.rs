#[doc = "Register `REG_PA1` reader"]
pub type R = crate::R<RegPa1Spec>;
#[doc = "Register `REG_PA1` writer"]
pub type W = crate::W<RegPa1Spec>;
#[doc = "Field `pA1_sel` reader - PAD pinmux function ID selection"]
pub type PA1SelR = crate::FieldReader;
#[doc = "Field `pA1_sel` writer - PAD pinmux function ID selection"]
pub type PA1SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA1_ie` reader - PAD input enable control"]
pub type PA1IeR = crate::BitReader;
#[doc = "Field `pA1_ie` writer - PAD input enable control"]
pub type PA1IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA1_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA1PupdcR = crate::BitReader;
#[doc = "Field `pA1_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA1PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA1_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA1E2R = crate::BitReader;
#[doc = "Field `pA1_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA1E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA1_smt` reader - PAD Schmit control"]
pub type PA1SmtR = crate::BitReader;
#[doc = "Field `pA1_smt` writer - PAD Schmit control"]
pub type PA1SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA1_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA1SrR = crate::BitReader;
#[doc = "Field `pA1_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA1SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA1_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA1E3R = crate::BitReader;
#[doc = "Field `pA1_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA1E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA1_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA1DisR = crate::BitReader;
#[doc = "Field `pA1_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA1DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a1_sel(&self) -> PA1SelR {
        PA1SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a1_ie(&self) -> PA1IeR {
        PA1IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a1_pupdc(&self) -> PA1PupdcR {
        PA1PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a1_e2(&self) -> PA1E2R {
        PA1E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a1_smt(&self) -> PA1SmtR {
        PA1SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a1_sr(&self) -> PA1SrR {
        PA1SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a1_e3(&self) -> PA1E3R {
        PA1E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a1_dis(&self) -> PA1DisR {
        PA1DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a1_sel(&mut self) -> PA1SelW<'_, RegPa1Spec> {
        PA1SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a1_ie(&mut self) -> PA1IeW<'_, RegPa1Spec> {
        PA1IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a1_pupdc(&mut self) -> PA1PupdcW<'_, RegPa1Spec> {
        PA1PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a1_e2(&mut self) -> PA1E2W<'_, RegPa1Spec> {
        PA1E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a1_smt(&mut self) -> PA1SmtW<'_, RegPa1Spec> {
        PA1SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a1_sr(&mut self) -> PA1SrW<'_, RegPa1Spec> {
        PA1SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a1_e3(&mut self) -> PA1E3W<'_, RegPa1Spec> {
        PA1E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a1_dis(&mut self) -> PA1DisW<'_, RegPa1Spec> {
        PA1DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa1Spec;
impl crate::RegisterSpec for RegPa1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa1::R`](R) reader structure"]
impl crate::Readable for RegPa1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa1::W`](W) writer structure"]
impl crate::Writable for RegPa1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA1 to value 0x1900"]
impl crate::Resettable for RegPa1Spec {
    const RESET_VALUE: u32 = 0x1900;
}
