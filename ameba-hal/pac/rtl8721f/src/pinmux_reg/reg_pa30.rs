#[doc = "Register `REG_PA30` reader"]
pub type R = crate::R<RegPa30Spec>;
#[doc = "Register `REG_PA30` writer"]
pub type W = crate::W<RegPa30Spec>;
#[doc = "Field `pA30_sel` reader - PAD pinmux function ID selection"]
pub type PA30SelR = crate::FieldReader;
#[doc = "Field `pA30_sel` writer - PAD pinmux function ID selection"]
pub type PA30SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA30_ie` reader - PAD input enable control"]
pub type PA30IeR = crate::BitReader;
#[doc = "Field `pA30_ie` writer - PAD input enable control"]
pub type PA30IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA30_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA30PupdcR = crate::BitReader;
#[doc = "Field `pA30_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA30PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA30_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA30E2R = crate::BitReader;
#[doc = "Field `pA30_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA30E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA30_smt` reader - PAD Schmit control"]
pub type PA30SmtR = crate::BitReader;
#[doc = "Field `pA30_smt` writer - PAD Schmit control"]
pub type PA30SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA30_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA30SrR = crate::BitReader;
#[doc = "Field `pA30_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA30SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA30_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA30E3R = crate::BitReader;
#[doc = "Field `pA30_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA30E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA30_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA30DisR = crate::BitReader;
#[doc = "Field `pA30_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA30DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a30_sel(&self) -> PA30SelR {
        PA30SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a30_ie(&self) -> PA30IeR {
        PA30IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a30_pupdc(&self) -> PA30PupdcR {
        PA30PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a30_e2(&self) -> PA30E2R {
        PA30E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a30_smt(&self) -> PA30SmtR {
        PA30SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a30_sr(&self) -> PA30SrR {
        PA30SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a30_e3(&self) -> PA30E3R {
        PA30E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a30_dis(&self) -> PA30DisR {
        PA30DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a30_sel(&mut self) -> PA30SelW<'_, RegPa30Spec> {
        PA30SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a30_ie(&mut self) -> PA30IeW<'_, RegPa30Spec> {
        PA30IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a30_pupdc(&mut self) -> PA30PupdcW<'_, RegPa30Spec> {
        PA30PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a30_e2(&mut self) -> PA30E2W<'_, RegPa30Spec> {
        PA30E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a30_smt(&mut self) -> PA30SmtW<'_, RegPa30Spec> {
        PA30SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a30_sr(&mut self) -> PA30SrW<'_, RegPa30Spec> {
        PA30SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a30_e3(&mut self) -> PA30E3W<'_, RegPa30Spec> {
        PA30E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a30_dis(&mut self) -> PA30DisW<'_, RegPa30Spec> {
        PA30DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa30Spec;
impl crate::RegisterSpec for RegPa30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa30::R`](R) reader structure"]
impl crate::Readable for RegPa30Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa30::W`](W) writer structure"]
impl crate::Writable for RegPa30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA30 to value 0x1900"]
impl crate::Resettable for RegPa30Spec {
    const RESET_VALUE: u32 = 0x1900;
}
