#[doc = "Register `REG_PA2` reader"]
pub type R = crate::R<RegPa2Spec>;
#[doc = "Register `REG_PA2` writer"]
pub type W = crate::W<RegPa2Spec>;
#[doc = "Field `pA2_sel` reader - PAD pinmux function ID selection"]
pub type PA2SelR = crate::FieldReader;
#[doc = "Field `pA2_sel` writer - PAD pinmux function ID selection"]
pub type PA2SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA2_ie` reader - PAD input enable control"]
pub type PA2IeR = crate::BitReader;
#[doc = "Field `pA2_ie` writer - PAD input enable control"]
pub type PA2IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA2_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA2PupdcR = crate::BitReader;
#[doc = "Field `pA2_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA2PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA2_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA2E2R = crate::BitReader;
#[doc = "Field `pA2_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA2E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA2_smt` reader - PAD Schmit control"]
pub type PA2SmtR = crate::BitReader;
#[doc = "Field `pA2_smt` writer - PAD Schmit control"]
pub type PA2SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA2_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA2SrR = crate::BitReader;
#[doc = "Field `pA2_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA2SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA2_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA2E3R = crate::BitReader;
#[doc = "Field `pA2_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA2E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA2_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA2DisR = crate::BitReader;
#[doc = "Field `pA2_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA2DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a2_sel(&self) -> PA2SelR {
        PA2SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a2_ie(&self) -> PA2IeR {
        PA2IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a2_pupdc(&self) -> PA2PupdcR {
        PA2PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a2_e2(&self) -> PA2E2R {
        PA2E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a2_smt(&self) -> PA2SmtR {
        PA2SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a2_sr(&self) -> PA2SrR {
        PA2SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a2_e3(&self) -> PA2E3R {
        PA2E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a2_dis(&self) -> PA2DisR {
        PA2DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a2_sel(&mut self) -> PA2SelW<'_, RegPa2Spec> {
        PA2SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a2_ie(&mut self) -> PA2IeW<'_, RegPa2Spec> {
        PA2IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a2_pupdc(&mut self) -> PA2PupdcW<'_, RegPa2Spec> {
        PA2PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a2_e2(&mut self) -> PA2E2W<'_, RegPa2Spec> {
        PA2E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a2_smt(&mut self) -> PA2SmtW<'_, RegPa2Spec> {
        PA2SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a2_sr(&mut self) -> PA2SrW<'_, RegPa2Spec> {
        PA2SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a2_e3(&mut self) -> PA2E3W<'_, RegPa2Spec> {
        PA2E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a2_dis(&mut self) -> PA2DisW<'_, RegPa2Spec> {
        PA2DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa2Spec;
impl crate::RegisterSpec for RegPa2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa2::R`](R) reader structure"]
impl crate::Readable for RegPa2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa2::W`](W) writer structure"]
impl crate::Writable for RegPa2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA2 to value 0x1900"]
impl crate::Resettable for RegPa2Spec {
    const RESET_VALUE: u32 = 0x1900;
}
