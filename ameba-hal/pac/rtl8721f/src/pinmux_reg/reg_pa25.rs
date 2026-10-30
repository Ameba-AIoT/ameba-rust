#[doc = "Register `REG_PA25` reader"]
pub type R = crate::R<RegPa25Spec>;
#[doc = "Register `REG_PA25` writer"]
pub type W = crate::W<RegPa25Spec>;
#[doc = "Field `pA25_sel` reader - PAD pinmux function ID selection"]
pub type PA25SelR = crate::FieldReader;
#[doc = "Field `pA25_sel` writer - PAD pinmux function ID selection"]
pub type PA25SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA25_ie` reader - PAD input enable control"]
pub type PA25IeR = crate::BitReader;
#[doc = "Field `pA25_ie` writer - PAD input enable control"]
pub type PA25IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA25_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA25PupdcR = crate::BitReader;
#[doc = "Field `pA25_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA25PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA25_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA25E2R = crate::BitReader;
#[doc = "Field `pA25_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA25E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA25_smt` reader - PAD Schmit control"]
pub type PA25SmtR = crate::BitReader;
#[doc = "Field `pA25_smt` writer - PAD Schmit control"]
pub type PA25SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA25_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA25SrR = crate::BitReader;
#[doc = "Field `pA25_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA25SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA25_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA25E3R = crate::BitReader;
#[doc = "Field `pA25_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA25E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA25_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA25DisR = crate::BitReader;
#[doc = "Field `pA25_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA25DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a25_sel(&self) -> PA25SelR {
        PA25SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a25_ie(&self) -> PA25IeR {
        PA25IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a25_pupdc(&self) -> PA25PupdcR {
        PA25PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a25_e2(&self) -> PA25E2R {
        PA25E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a25_smt(&self) -> PA25SmtR {
        PA25SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a25_sr(&self) -> PA25SrR {
        PA25SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a25_e3(&self) -> PA25E3R {
        PA25E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a25_dis(&self) -> PA25DisR {
        PA25DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a25_sel(&mut self) -> PA25SelW<'_, RegPa25Spec> {
        PA25SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a25_ie(&mut self) -> PA25IeW<'_, RegPa25Spec> {
        PA25IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a25_pupdc(&mut self) -> PA25PupdcW<'_, RegPa25Spec> {
        PA25PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a25_e2(&mut self) -> PA25E2W<'_, RegPa25Spec> {
        PA25E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a25_smt(&mut self) -> PA25SmtW<'_, RegPa25Spec> {
        PA25SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a25_sr(&mut self) -> PA25SrW<'_, RegPa25Spec> {
        PA25SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a25_e3(&mut self) -> PA25E3W<'_, RegPa25Spec> {
        PA25E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a25_dis(&mut self) -> PA25DisW<'_, RegPa25Spec> {
        PA25DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa25Spec;
impl crate::RegisterSpec for RegPa25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa25::R`](R) reader structure"]
impl crate::Readable for RegPa25Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa25::W`](W) writer structure"]
impl crate::Writable for RegPa25Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA25 to value 0x1900"]
impl crate::Resettable for RegPa25Spec {
    const RESET_VALUE: u32 = 0x1900;
}
