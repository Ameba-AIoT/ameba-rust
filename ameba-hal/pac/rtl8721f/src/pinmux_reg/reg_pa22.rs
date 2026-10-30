#[doc = "Register `REG_PA22` reader"]
pub type R = crate::R<RegPa22Spec>;
#[doc = "Register `REG_PA22` writer"]
pub type W = crate::W<RegPa22Spec>;
#[doc = "Field `pA22_sel` reader - PAD pinmux function ID selection"]
pub type PA22SelR = crate::FieldReader;
#[doc = "Field `pA22_sel` writer - PAD pinmux function ID selection"]
pub type PA22SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA22_ie` reader - PAD input enable control"]
pub type PA22IeR = crate::BitReader;
#[doc = "Field `pA22_ie` writer - PAD input enable control"]
pub type PA22IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA22_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA22PupdcR = crate::BitReader;
#[doc = "Field `pA22_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA22PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA22_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA22E2R = crate::BitReader;
#[doc = "Field `pA22_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA22E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA22_smt` reader - PAD Schmit control"]
pub type PA22SmtR = crate::BitReader;
#[doc = "Field `pA22_smt` writer - PAD Schmit control"]
pub type PA22SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA22_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA22SrR = crate::BitReader;
#[doc = "Field `pA22_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA22SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA22_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA22E3R = crate::BitReader;
#[doc = "Field `pA22_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA22E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA22_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA22DisR = crate::BitReader;
#[doc = "Field `pA22_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA22DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a22_sel(&self) -> PA22SelR {
        PA22SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a22_ie(&self) -> PA22IeR {
        PA22IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a22_pupdc(&self) -> PA22PupdcR {
        PA22PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a22_e2(&self) -> PA22E2R {
        PA22E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a22_smt(&self) -> PA22SmtR {
        PA22SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a22_sr(&self) -> PA22SrR {
        PA22SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a22_e3(&self) -> PA22E3R {
        PA22E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a22_dis(&self) -> PA22DisR {
        PA22DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a22_sel(&mut self) -> PA22SelW<'_, RegPa22Spec> {
        PA22SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a22_ie(&mut self) -> PA22IeW<'_, RegPa22Spec> {
        PA22IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a22_pupdc(&mut self) -> PA22PupdcW<'_, RegPa22Spec> {
        PA22PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a22_e2(&mut self) -> PA22E2W<'_, RegPa22Spec> {
        PA22E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a22_smt(&mut self) -> PA22SmtW<'_, RegPa22Spec> {
        PA22SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a22_sr(&mut self) -> PA22SrW<'_, RegPa22Spec> {
        PA22SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a22_e3(&mut self) -> PA22E3W<'_, RegPa22Spec> {
        PA22E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a22_dis(&mut self) -> PA22DisW<'_, RegPa22Spec> {
        PA22DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa22Spec;
impl crate::RegisterSpec for RegPa22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa22::R`](R) reader structure"]
impl crate::Readable for RegPa22Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa22::W`](W) writer structure"]
impl crate::Writable for RegPa22Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA22 to value 0x1900"]
impl crate::Resettable for RegPa22Spec {
    const RESET_VALUE: u32 = 0x1900;
}
