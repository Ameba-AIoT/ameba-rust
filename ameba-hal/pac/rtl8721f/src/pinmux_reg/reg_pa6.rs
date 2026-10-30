#[doc = "Register `REG_PA6` reader"]
pub type R = crate::R<RegPa6Spec>;
#[doc = "Register `REG_PA6` writer"]
pub type W = crate::W<RegPa6Spec>;
#[doc = "Field `pA6_sel` reader - PAD pinmux function ID selection"]
pub type PA6SelR = crate::FieldReader;
#[doc = "Field `pA6_sel` writer - PAD pinmux function ID selection"]
pub type PA6SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA6_ie` reader - PAD input enable control"]
pub type PA6IeR = crate::BitReader;
#[doc = "Field `pA6_ie` writer - PAD input enable control"]
pub type PA6IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA6_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA6PupdcR = crate::BitReader;
#[doc = "Field `pA6_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA6PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA6_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA6E2R = crate::BitReader;
#[doc = "Field `pA6_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA6E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA6_smt` reader - PAD Schmit control"]
pub type PA6SmtR = crate::BitReader;
#[doc = "Field `pA6_smt` writer - PAD Schmit control"]
pub type PA6SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA6_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA6SrR = crate::BitReader;
#[doc = "Field `pA6_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA6SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA6_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA6E3R = crate::BitReader;
#[doc = "Field `pA6_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA6E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA6_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA6DisR = crate::BitReader;
#[doc = "Field `pA6_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA6DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a6_sel(&self) -> PA6SelR {
        PA6SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a6_ie(&self) -> PA6IeR {
        PA6IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a6_pupdc(&self) -> PA6PupdcR {
        PA6PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a6_e2(&self) -> PA6E2R {
        PA6E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a6_smt(&self) -> PA6SmtR {
        PA6SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a6_sr(&self) -> PA6SrR {
        PA6SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a6_e3(&self) -> PA6E3R {
        PA6E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a6_dis(&self) -> PA6DisR {
        PA6DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a6_sel(&mut self) -> PA6SelW<'_, RegPa6Spec> {
        PA6SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a6_ie(&mut self) -> PA6IeW<'_, RegPa6Spec> {
        PA6IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a6_pupdc(&mut self) -> PA6PupdcW<'_, RegPa6Spec> {
        PA6PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a6_e2(&mut self) -> PA6E2W<'_, RegPa6Spec> {
        PA6E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a6_smt(&mut self) -> PA6SmtW<'_, RegPa6Spec> {
        PA6SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a6_sr(&mut self) -> PA6SrW<'_, RegPa6Spec> {
        PA6SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a6_e3(&mut self) -> PA6E3W<'_, RegPa6Spec> {
        PA6E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a6_dis(&mut self) -> PA6DisW<'_, RegPa6Spec> {
        PA6DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa6Spec;
impl crate::RegisterSpec for RegPa6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa6::R`](R) reader structure"]
impl crate::Readable for RegPa6Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa6::W`](W) writer structure"]
impl crate::Writable for RegPa6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA6 to value 0x1900"]
impl crate::Resettable for RegPa6Spec {
    const RESET_VALUE: u32 = 0x1900;
}
