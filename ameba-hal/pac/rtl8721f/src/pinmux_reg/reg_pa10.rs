#[doc = "Register `REG_PA10` reader"]
pub type R = crate::R<RegPa10Spec>;
#[doc = "Register `REG_PA10` writer"]
pub type W = crate::W<RegPa10Spec>;
#[doc = "Field `pA10_sel` reader - PAD pinmux function ID selection"]
pub type PA10SelR = crate::FieldReader;
#[doc = "Field `pA10_sel` writer - PAD pinmux function ID selection"]
pub type PA10SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA10_ie` reader - PAD input enable control"]
pub type PA10IeR = crate::BitReader;
#[doc = "Field `pA10_ie` writer - PAD input enable control"]
pub type PA10IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA10_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA10PupdcR = crate::BitReader;
#[doc = "Field `pA10_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA10PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA10_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA10E2R = crate::BitReader;
#[doc = "Field `pA10_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA10E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA10_smt` reader - PAD Schmit control"]
pub type PA10SmtR = crate::BitReader;
#[doc = "Field `pA10_smt` writer - PAD Schmit control"]
pub type PA10SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA10_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA10SrR = crate::BitReader;
#[doc = "Field `pA10_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA10SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA10_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA10E3R = crate::BitReader;
#[doc = "Field `pA10_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA10E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA10_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA10DisR = crate::BitReader;
#[doc = "Field `pA10_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA10DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a10_sel(&self) -> PA10SelR {
        PA10SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a10_ie(&self) -> PA10IeR {
        PA10IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a10_pupdc(&self) -> PA10PupdcR {
        PA10PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a10_e2(&self) -> PA10E2R {
        PA10E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a10_smt(&self) -> PA10SmtR {
        PA10SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a10_sr(&self) -> PA10SrR {
        PA10SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a10_e3(&self) -> PA10E3R {
        PA10E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a10_dis(&self) -> PA10DisR {
        PA10DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a10_sel(&mut self) -> PA10SelW<'_, RegPa10Spec> {
        PA10SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a10_ie(&mut self) -> PA10IeW<'_, RegPa10Spec> {
        PA10IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a10_pupdc(&mut self) -> PA10PupdcW<'_, RegPa10Spec> {
        PA10PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a10_e2(&mut self) -> PA10E2W<'_, RegPa10Spec> {
        PA10E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a10_smt(&mut self) -> PA10SmtW<'_, RegPa10Spec> {
        PA10SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a10_sr(&mut self) -> PA10SrW<'_, RegPa10Spec> {
        PA10SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a10_e3(&mut self) -> PA10E3W<'_, RegPa10Spec> {
        PA10E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a10_dis(&mut self) -> PA10DisW<'_, RegPa10Spec> {
        PA10DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa10Spec;
impl crate::RegisterSpec for RegPa10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa10::R`](R) reader structure"]
impl crate::Readable for RegPa10Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa10::W`](W) writer structure"]
impl crate::Writable for RegPa10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA10 to value 0x1900"]
impl crate::Resettable for RegPa10Spec {
    const RESET_VALUE: u32 = 0x1900;
}
