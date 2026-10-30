#[doc = "Register `REG_PB3` reader"]
pub type R = crate::R<RegPb3Spec>;
#[doc = "Register `REG_PB3` writer"]
pub type W = crate::W<RegPb3Spec>;
#[doc = "Field `pB3_sel` reader - PAD pinmux function ID selection"]
pub type PB3SelR = crate::FieldReader;
#[doc = "Field `pB3_sel` writer - PAD pinmux function ID selection"]
pub type PB3SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB3_ie` reader - PAD input enable control"]
pub type PB3IeR = crate::BitReader;
#[doc = "Field `pB3_ie` writer - PAD input enable control"]
pub type PB3IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB3_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB3PupdcR = crate::BitReader;
#[doc = "Field `pB3_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB3PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB3_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB3E2R = crate::BitReader;
#[doc = "Field `pB3_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB3E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB3_smt` reader - PAD Schmit control"]
pub type PB3SmtR = crate::BitReader;
#[doc = "Field `pB3_smt` writer - PAD Schmit control"]
pub type PB3SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB3_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB3SrR = crate::BitReader;
#[doc = "Field `pB3_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB3SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB3_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB3E3R = crate::BitReader;
#[doc = "Field `pB3_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB3E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB3_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB3DisR = crate::BitReader;
#[doc = "Field `pB3_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB3DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b3_sel(&self) -> PB3SelR {
        PB3SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b3_ie(&self) -> PB3IeR {
        PB3IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b3_pupdc(&self) -> PB3PupdcR {
        PB3PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b3_e2(&self) -> PB3E2R {
        PB3E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b3_smt(&self) -> PB3SmtR {
        PB3SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b3_sr(&self) -> PB3SrR {
        PB3SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b3_e3(&self) -> PB3E3R {
        PB3E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b3_dis(&self) -> PB3DisR {
        PB3DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b3_sel(&mut self) -> PB3SelW<'_, RegPb3Spec> {
        PB3SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b3_ie(&mut self) -> PB3IeW<'_, RegPb3Spec> {
        PB3IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b3_pupdc(&mut self) -> PB3PupdcW<'_, RegPb3Spec> {
        PB3PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b3_e2(&mut self) -> PB3E2W<'_, RegPb3Spec> {
        PB3E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b3_smt(&mut self) -> PB3SmtW<'_, RegPb3Spec> {
        PB3SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b3_sr(&mut self) -> PB3SrW<'_, RegPb3Spec> {
        PB3SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b3_e3(&mut self) -> PB3E3W<'_, RegPb3Spec> {
        PB3E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b3_dis(&mut self) -> PB3DisW<'_, RegPb3Spec> {
        PB3DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb3Spec;
impl crate::RegisterSpec for RegPb3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb3::R`](R) reader structure"]
impl crate::Readable for RegPb3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb3::W`](W) writer structure"]
impl crate::Writable for RegPb3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB3 to value 0x1900"]
impl crate::Resettable for RegPb3Spec {
    const RESET_VALUE: u32 = 0x1900;
}
