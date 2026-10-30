#[doc = "Register `REG_PB25` reader"]
pub type R = crate::R<RegPb25Spec>;
#[doc = "Register `REG_PB25` writer"]
pub type W = crate::W<RegPb25Spec>;
#[doc = "Field `pB25_sel` reader - PAD pinmux function ID selection"]
pub type PB25SelR = crate::FieldReader;
#[doc = "Field `pB25_sel` writer - PAD pinmux function ID selection"]
pub type PB25SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB25_ie` reader - PAD input enable control"]
pub type PB25IeR = crate::BitReader;
#[doc = "Field `pB25_ie` writer - PAD input enable control"]
pub type PB25IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB25_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB25PupdcR = crate::BitReader;
#[doc = "Field `pB25_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB25PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB25_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB25E2R = crate::BitReader;
#[doc = "Field `pB25_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB25E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB25_smt` reader - PAD Schmit control"]
pub type PB25SmtR = crate::BitReader;
#[doc = "Field `pB25_smt` writer - PAD Schmit control"]
pub type PB25SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB25_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB25SrR = crate::BitReader;
#[doc = "Field `pB25_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB25SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB25_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB25E3R = crate::BitReader;
#[doc = "Field `pB25_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB25E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB25_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB25DisR = crate::BitReader;
#[doc = "Field `pB25_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB25DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b25_sel(&self) -> PB25SelR {
        PB25SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b25_ie(&self) -> PB25IeR {
        PB25IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b25_pupdc(&self) -> PB25PupdcR {
        PB25PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b25_e2(&self) -> PB25E2R {
        PB25E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b25_smt(&self) -> PB25SmtR {
        PB25SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b25_sr(&self) -> PB25SrR {
        PB25SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b25_e3(&self) -> PB25E3R {
        PB25E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b25_dis(&self) -> PB25DisR {
        PB25DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b25_sel(&mut self) -> PB25SelW<'_, RegPb25Spec> {
        PB25SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b25_ie(&mut self) -> PB25IeW<'_, RegPb25Spec> {
        PB25IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b25_pupdc(&mut self) -> PB25PupdcW<'_, RegPb25Spec> {
        PB25PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b25_e2(&mut self) -> PB25E2W<'_, RegPb25Spec> {
        PB25E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b25_smt(&mut self) -> PB25SmtW<'_, RegPb25Spec> {
        PB25SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b25_sr(&mut self) -> PB25SrW<'_, RegPb25Spec> {
        PB25SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b25_e3(&mut self) -> PB25E3W<'_, RegPb25Spec> {
        PB25E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b25_dis(&mut self) -> PB25DisW<'_, RegPb25Spec> {
        PB25DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb25Spec;
impl crate::RegisterSpec for RegPb25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb25::R`](R) reader structure"]
impl crate::Readable for RegPb25Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb25::W`](W) writer structure"]
impl crate::Writable for RegPb25Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB25 to value 0x1900"]
impl crate::Resettable for RegPb25Spec {
    const RESET_VALUE: u32 = 0x1900;
}
