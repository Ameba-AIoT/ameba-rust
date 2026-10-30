#[doc = "Register `REG_PA29` reader"]
pub type R = crate::R<RegPa29Spec>;
#[doc = "Register `REG_PA29` writer"]
pub type W = crate::W<RegPa29Spec>;
#[doc = "Field `pA29_sel` reader - PAD pinmux function ID selection"]
pub type PA29SelR = crate::FieldReader;
#[doc = "Field `pA29_sel` writer - PAD pinmux function ID selection"]
pub type PA29SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA29_ie` reader - PAD input enable control"]
pub type PA29IeR = crate::BitReader;
#[doc = "Field `pA29_ie` writer - PAD input enable control"]
pub type PA29IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA29_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA29PupdcR = crate::BitReader;
#[doc = "Field `pA29_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA29PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA29_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA29E2R = crate::BitReader;
#[doc = "Field `pA29_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA29E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA29_smt` reader - PAD Schmit control"]
pub type PA29SmtR = crate::BitReader;
#[doc = "Field `pA29_smt` writer - PAD Schmit control"]
pub type PA29SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA29_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA29SrR = crate::BitReader;
#[doc = "Field `pA29_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA29SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA29_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA29E3R = crate::BitReader;
#[doc = "Field `pA29_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA29E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA29_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA29DisR = crate::BitReader;
#[doc = "Field `pA29_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA29DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a29_sel(&self) -> PA29SelR {
        PA29SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a29_ie(&self) -> PA29IeR {
        PA29IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a29_pupdc(&self) -> PA29PupdcR {
        PA29PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a29_e2(&self) -> PA29E2R {
        PA29E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a29_smt(&self) -> PA29SmtR {
        PA29SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a29_sr(&self) -> PA29SrR {
        PA29SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a29_e3(&self) -> PA29E3R {
        PA29E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a29_dis(&self) -> PA29DisR {
        PA29DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a29_sel(&mut self) -> PA29SelW<'_, RegPa29Spec> {
        PA29SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a29_ie(&mut self) -> PA29IeW<'_, RegPa29Spec> {
        PA29IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a29_pupdc(&mut self) -> PA29PupdcW<'_, RegPa29Spec> {
        PA29PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a29_e2(&mut self) -> PA29E2W<'_, RegPa29Spec> {
        PA29E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a29_smt(&mut self) -> PA29SmtW<'_, RegPa29Spec> {
        PA29SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a29_sr(&mut self) -> PA29SrW<'_, RegPa29Spec> {
        PA29SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a29_e3(&mut self) -> PA29E3W<'_, RegPa29Spec> {
        PA29E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a29_dis(&mut self) -> PA29DisW<'_, RegPa29Spec> {
        PA29DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa29::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa29::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa29Spec;
impl crate::RegisterSpec for RegPa29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa29::R`](R) reader structure"]
impl crate::Readable for RegPa29Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa29::W`](W) writer structure"]
impl crate::Writable for RegPa29Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA29 to value 0x1900"]
impl crate::Resettable for RegPa29Spec {
    const RESET_VALUE: u32 = 0x1900;
}
