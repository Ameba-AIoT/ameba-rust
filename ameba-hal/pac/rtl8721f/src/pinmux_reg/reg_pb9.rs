#[doc = "Register `REG_PB9` reader"]
pub type R = crate::R<RegPb9Spec>;
#[doc = "Register `REG_PB9` writer"]
pub type W = crate::W<RegPb9Spec>;
#[doc = "Field `pB9_sel` reader - PAD pinmux function ID selection"]
pub type PB9SelR = crate::FieldReader;
#[doc = "Field `pB9_sel` writer - PAD pinmux function ID selection"]
pub type PB9SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB9_ie` reader - PAD input enable control"]
pub type PB9IeR = crate::BitReader;
#[doc = "Field `pB9_ie` writer - PAD input enable control"]
pub type PB9IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB9_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB9PupdcR = crate::BitReader;
#[doc = "Field `pB9_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB9PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB9_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB9E2R = crate::BitReader;
#[doc = "Field `pB9_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB9E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB9_smt` reader - PAD Schmit control"]
pub type PB9SmtR = crate::BitReader;
#[doc = "Field `pB9_smt` writer - PAD Schmit control"]
pub type PB9SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB9_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB9SrR = crate::BitReader;
#[doc = "Field `pB9_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB9SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB9_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB9E3R = crate::BitReader;
#[doc = "Field `pB9_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB9E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB9_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB9DisR = crate::BitReader;
#[doc = "Field `pB9_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB9DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b9_sel(&self) -> PB9SelR {
        PB9SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b9_ie(&self) -> PB9IeR {
        PB9IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b9_pupdc(&self) -> PB9PupdcR {
        PB9PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b9_e2(&self) -> PB9E2R {
        PB9E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b9_smt(&self) -> PB9SmtR {
        PB9SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b9_sr(&self) -> PB9SrR {
        PB9SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b9_e3(&self) -> PB9E3R {
        PB9E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b9_dis(&self) -> PB9DisR {
        PB9DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b9_sel(&mut self) -> PB9SelW<'_, RegPb9Spec> {
        PB9SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b9_ie(&mut self) -> PB9IeW<'_, RegPb9Spec> {
        PB9IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b9_pupdc(&mut self) -> PB9PupdcW<'_, RegPb9Spec> {
        PB9PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b9_e2(&mut self) -> PB9E2W<'_, RegPb9Spec> {
        PB9E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b9_smt(&mut self) -> PB9SmtW<'_, RegPb9Spec> {
        PB9SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b9_sr(&mut self) -> PB9SrW<'_, RegPb9Spec> {
        PB9SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b9_e3(&mut self) -> PB9E3W<'_, RegPb9Spec> {
        PB9E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b9_dis(&mut self) -> PB9DisW<'_, RegPb9Spec> {
        PB9DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb9Spec;
impl crate::RegisterSpec for RegPb9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb9::R`](R) reader structure"]
impl crate::Readable for RegPb9Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb9::W`](W) writer structure"]
impl crate::Writable for RegPb9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB9 to value 0x1900"]
impl crate::Resettable for RegPb9Spec {
    const RESET_VALUE: u32 = 0x1900;
}
