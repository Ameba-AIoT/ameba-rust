#[doc = "Register `REG_PA17` reader"]
pub type R = crate::R<RegPa17Spec>;
#[doc = "Register `REG_PA17` writer"]
pub type W = crate::W<RegPa17Spec>;
#[doc = "Field `pA17_sel` reader - PAD pinmux function ID selection"]
pub type PA17SelR = crate::FieldReader;
#[doc = "Field `pA17_sel` writer - PAD pinmux function ID selection"]
pub type PA17SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA17_ie` reader - PAD input enable control"]
pub type PA17IeR = crate::BitReader;
#[doc = "Field `pA17_ie` writer - PAD input enable control"]
pub type PA17IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA17_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA17PupdcR = crate::BitReader;
#[doc = "Field `pA17_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA17PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA17_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA17E2R = crate::BitReader;
#[doc = "Field `pA17_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA17E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA17_smt` reader - PAD Schmit control"]
pub type PA17SmtR = crate::BitReader;
#[doc = "Field `pA17_smt` writer - PAD Schmit control"]
pub type PA17SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA17_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA17SrR = crate::BitReader;
#[doc = "Field `pA17_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA17SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA17_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA17E3R = crate::BitReader;
#[doc = "Field `pA17_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA17E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA17_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA17DisR = crate::BitReader;
#[doc = "Field `pA17_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA17DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a17_sel(&self) -> PA17SelR {
        PA17SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a17_ie(&self) -> PA17IeR {
        PA17IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a17_pupdc(&self) -> PA17PupdcR {
        PA17PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a17_e2(&self) -> PA17E2R {
        PA17E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a17_smt(&self) -> PA17SmtR {
        PA17SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a17_sr(&self) -> PA17SrR {
        PA17SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a17_e3(&self) -> PA17E3R {
        PA17E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a17_dis(&self) -> PA17DisR {
        PA17DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a17_sel(&mut self) -> PA17SelW<'_, RegPa17Spec> {
        PA17SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a17_ie(&mut self) -> PA17IeW<'_, RegPa17Spec> {
        PA17IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a17_pupdc(&mut self) -> PA17PupdcW<'_, RegPa17Spec> {
        PA17PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a17_e2(&mut self) -> PA17E2W<'_, RegPa17Spec> {
        PA17E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a17_smt(&mut self) -> PA17SmtW<'_, RegPa17Spec> {
        PA17SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a17_sr(&mut self) -> PA17SrW<'_, RegPa17Spec> {
        PA17SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a17_e3(&mut self) -> PA17E3W<'_, RegPa17Spec> {
        PA17E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a17_dis(&mut self) -> PA17DisW<'_, RegPa17Spec> {
        PA17DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa17Spec;
impl crate::RegisterSpec for RegPa17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa17::R`](R) reader structure"]
impl crate::Readable for RegPa17Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa17::W`](W) writer structure"]
impl crate::Writable for RegPa17Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA17 to value 0x1900"]
impl crate::Resettable for RegPa17Spec {
    const RESET_VALUE: u32 = 0x1900;
}
