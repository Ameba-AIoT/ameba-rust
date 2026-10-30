#[doc = "Register `REG_PA0` reader"]
pub type R = crate::R<RegPa0Spec>;
#[doc = "Register `REG_PA0` writer"]
pub type W = crate::W<RegPa0Spec>;
#[doc = "Field `pA0_sel` reader - PAD pinmux function ID selection"]
pub type PA0SelR = crate::FieldReader;
#[doc = "Field `pA0_sel` writer - PAD pinmux function ID selection"]
pub type PA0SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA0_ie` reader - PAD input enable control"]
pub type PA0IeR = crate::BitReader;
#[doc = "Field `pA0_ie` writer - PAD input enable control"]
pub type PA0IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA0_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA0PupdcR = crate::BitReader;
#[doc = "Field `pA0_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA0PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA0_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA0E2R = crate::BitReader;
#[doc = "Field `pA0_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA0E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA0_smt` reader - PAD Schmit control"]
pub type PA0SmtR = crate::BitReader;
#[doc = "Field `pA0_smt` writer - PAD Schmit control"]
pub type PA0SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA0_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA0SrR = crate::BitReader;
#[doc = "Field `pA0_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA0SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA0_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA0E3R = crate::BitReader;
#[doc = "Field `pA0_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA0E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA0_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA0DisR = crate::BitReader;
#[doc = "Field `pA0_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA0DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a0_sel(&self) -> PA0SelR {
        PA0SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a0_ie(&self) -> PA0IeR {
        PA0IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a0_pupdc(&self) -> PA0PupdcR {
        PA0PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a0_e2(&self) -> PA0E2R {
        PA0E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a0_smt(&self) -> PA0SmtR {
        PA0SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a0_sr(&self) -> PA0SrR {
        PA0SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a0_e3(&self) -> PA0E3R {
        PA0E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a0_dis(&self) -> PA0DisR {
        PA0DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a0_sel(&mut self) -> PA0SelW<'_, RegPa0Spec> {
        PA0SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a0_ie(&mut self) -> PA0IeW<'_, RegPa0Spec> {
        PA0IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a0_pupdc(&mut self) -> PA0PupdcW<'_, RegPa0Spec> {
        PA0PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a0_e2(&mut self) -> PA0E2W<'_, RegPa0Spec> {
        PA0E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a0_smt(&mut self) -> PA0SmtW<'_, RegPa0Spec> {
        PA0SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a0_sr(&mut self) -> PA0SrW<'_, RegPa0Spec> {
        PA0SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a0_e3(&mut self) -> PA0E3W<'_, RegPa0Spec> {
        PA0E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a0_dis(&mut self) -> PA0DisW<'_, RegPa0Spec> {
        PA0DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa0Spec;
impl crate::RegisterSpec for RegPa0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa0::R`](R) reader structure"]
impl crate::Readable for RegPa0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa0::W`](W) writer structure"]
impl crate::Writable for RegPa0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA0 to value 0x1900"]
impl crate::Resettable for RegPa0Spec {
    const RESET_VALUE: u32 = 0x1900;
}
