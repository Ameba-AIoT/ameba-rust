#[doc = "Register `REG_PA12` reader"]
pub type R = crate::R<RegPa12Spec>;
#[doc = "Register `REG_PA12` writer"]
pub type W = crate::W<RegPa12Spec>;
#[doc = "Field `pA12_sel` reader - PAD pinmux function ID selection"]
pub type PA12SelR = crate::FieldReader;
#[doc = "Field `pA12_sel` writer - PAD pinmux function ID selection"]
pub type PA12SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA12_ie` reader - PAD input enable control"]
pub type PA12IeR = crate::BitReader;
#[doc = "Field `pA12_ie` writer - PAD input enable control"]
pub type PA12IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA12_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA12PupdcR = crate::BitReader;
#[doc = "Field `pA12_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA12PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA12_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA12E2R = crate::BitReader;
#[doc = "Field `pA12_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA12E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA12_smt` reader - PAD Schmit control"]
pub type PA12SmtR = crate::BitReader;
#[doc = "Field `pA12_smt` writer - PAD Schmit control"]
pub type PA12SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA12_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA12SrR = crate::BitReader;
#[doc = "Field `pA12_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA12SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA12_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA12E3R = crate::BitReader;
#[doc = "Field `pA12_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA12E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA12_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA12DisR = crate::BitReader;
#[doc = "Field `pA12_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA12DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a12_sel(&self) -> PA12SelR {
        PA12SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a12_ie(&self) -> PA12IeR {
        PA12IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a12_pupdc(&self) -> PA12PupdcR {
        PA12PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a12_e2(&self) -> PA12E2R {
        PA12E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a12_smt(&self) -> PA12SmtR {
        PA12SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a12_sr(&self) -> PA12SrR {
        PA12SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a12_e3(&self) -> PA12E3R {
        PA12E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a12_dis(&self) -> PA12DisR {
        PA12DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a12_sel(&mut self) -> PA12SelW<'_, RegPa12Spec> {
        PA12SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a12_ie(&mut self) -> PA12IeW<'_, RegPa12Spec> {
        PA12IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a12_pupdc(&mut self) -> PA12PupdcW<'_, RegPa12Spec> {
        PA12PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a12_e2(&mut self) -> PA12E2W<'_, RegPa12Spec> {
        PA12E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a12_smt(&mut self) -> PA12SmtW<'_, RegPa12Spec> {
        PA12SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a12_sr(&mut self) -> PA12SrW<'_, RegPa12Spec> {
        PA12SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a12_e3(&mut self) -> PA12E3W<'_, RegPa12Spec> {
        PA12E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a12_dis(&mut self) -> PA12DisW<'_, RegPa12Spec> {
        PA12DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa12Spec;
impl crate::RegisterSpec for RegPa12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa12::R`](R) reader structure"]
impl crate::Readable for RegPa12Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa12::W`](W) writer structure"]
impl crate::Writable for RegPa12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA12 to value 0x1900"]
impl crate::Resettable for RegPa12Spec {
    const RESET_VALUE: u32 = 0x1900;
}
