#[doc = "Register `REG_PA11` reader"]
pub type R = crate::R<RegPa11Spec>;
#[doc = "Register `REG_PA11` writer"]
pub type W = crate::W<RegPa11Spec>;
#[doc = "Field `pA11_sel` reader - PAD pinmux function ID selection"]
pub type PA11SelR = crate::FieldReader;
#[doc = "Field `pA11_sel` writer - PAD pinmux function ID selection"]
pub type PA11SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA11_ie` reader - PAD input enable control"]
pub type PA11IeR = crate::BitReader;
#[doc = "Field `pA11_ie` writer - PAD input enable control"]
pub type PA11IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA11_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA11PupdcR = crate::BitReader;
#[doc = "Field `pA11_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA11PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA11_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA11E2R = crate::BitReader;
#[doc = "Field `pA11_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA11E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA11_smt` reader - PAD Schmit control"]
pub type PA11SmtR = crate::BitReader;
#[doc = "Field `pA11_smt` writer - PAD Schmit control"]
pub type PA11SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA11_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA11SrR = crate::BitReader;
#[doc = "Field `pA11_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA11SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA11_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA11E3R = crate::BitReader;
#[doc = "Field `pA11_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA11E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA11_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA11DisR = crate::BitReader;
#[doc = "Field `pA11_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA11DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a11_sel(&self) -> PA11SelR {
        PA11SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a11_ie(&self) -> PA11IeR {
        PA11IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a11_pupdc(&self) -> PA11PupdcR {
        PA11PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a11_e2(&self) -> PA11E2R {
        PA11E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a11_smt(&self) -> PA11SmtR {
        PA11SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a11_sr(&self) -> PA11SrR {
        PA11SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a11_e3(&self) -> PA11E3R {
        PA11E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a11_dis(&self) -> PA11DisR {
        PA11DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a11_sel(&mut self) -> PA11SelW<'_, RegPa11Spec> {
        PA11SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a11_ie(&mut self) -> PA11IeW<'_, RegPa11Spec> {
        PA11IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a11_pupdc(&mut self) -> PA11PupdcW<'_, RegPa11Spec> {
        PA11PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a11_e2(&mut self) -> PA11E2W<'_, RegPa11Spec> {
        PA11E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a11_smt(&mut self) -> PA11SmtW<'_, RegPa11Spec> {
        PA11SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a11_sr(&mut self) -> PA11SrW<'_, RegPa11Spec> {
        PA11SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a11_e3(&mut self) -> PA11E3W<'_, RegPa11Spec> {
        PA11E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a11_dis(&mut self) -> PA11DisW<'_, RegPa11Spec> {
        PA11DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa11Spec;
impl crate::RegisterSpec for RegPa11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa11::R`](R) reader structure"]
impl crate::Readable for RegPa11Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa11::W`](W) writer structure"]
impl crate::Writable for RegPa11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA11 to value 0x1900"]
impl crate::Resettable for RegPa11Spec {
    const RESET_VALUE: u32 = 0x1900;
}
