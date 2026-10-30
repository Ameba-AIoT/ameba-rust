#[doc = "Register `REG_PA19` reader"]
pub type R = crate::R<RegPa19Spec>;
#[doc = "Register `REG_PA19` writer"]
pub type W = crate::W<RegPa19Spec>;
#[doc = "Field `pA19_sel` reader - PAD pinmux function ID selection"]
pub type PA19SelR = crate::FieldReader;
#[doc = "Field `pA19_sel` writer - PAD pinmux function ID selection"]
pub type PA19SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA19_ie` reader - PAD input enable control"]
pub type PA19IeR = crate::BitReader;
#[doc = "Field `pA19_ie` writer - PAD input enable control"]
pub type PA19IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA19_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA19PupdcR = crate::BitReader;
#[doc = "Field `pA19_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA19PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA19_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA19E2R = crate::BitReader;
#[doc = "Field `pA19_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA19E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA19_smt` reader - PAD Schmit control"]
pub type PA19SmtR = crate::BitReader;
#[doc = "Field `pA19_smt` writer - PAD Schmit control"]
pub type PA19SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA19_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA19SrR = crate::BitReader;
#[doc = "Field `pA19_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA19SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA19_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA19E3R = crate::BitReader;
#[doc = "Field `pA19_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA19E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA19_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA19DisR = crate::BitReader;
#[doc = "Field `pA19_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA19DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a19_sel(&self) -> PA19SelR {
        PA19SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a19_ie(&self) -> PA19IeR {
        PA19IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a19_pupdc(&self) -> PA19PupdcR {
        PA19PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a19_e2(&self) -> PA19E2R {
        PA19E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a19_smt(&self) -> PA19SmtR {
        PA19SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a19_sr(&self) -> PA19SrR {
        PA19SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a19_e3(&self) -> PA19E3R {
        PA19E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a19_dis(&self) -> PA19DisR {
        PA19DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a19_sel(&mut self) -> PA19SelW<'_, RegPa19Spec> {
        PA19SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a19_ie(&mut self) -> PA19IeW<'_, RegPa19Spec> {
        PA19IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a19_pupdc(&mut self) -> PA19PupdcW<'_, RegPa19Spec> {
        PA19PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a19_e2(&mut self) -> PA19E2W<'_, RegPa19Spec> {
        PA19E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a19_smt(&mut self) -> PA19SmtW<'_, RegPa19Spec> {
        PA19SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a19_sr(&mut self) -> PA19SrW<'_, RegPa19Spec> {
        PA19SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a19_e3(&mut self) -> PA19E3W<'_, RegPa19Spec> {
        PA19E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a19_dis(&mut self) -> PA19DisW<'_, RegPa19Spec> {
        PA19DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa19Spec;
impl crate::RegisterSpec for RegPa19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa19::R`](R) reader structure"]
impl crate::Readable for RegPa19Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa19::W`](W) writer structure"]
impl crate::Writable for RegPa19Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA19 to value 0x1900"]
impl crate::Resettable for RegPa19Spec {
    const RESET_VALUE: u32 = 0x1900;
}
