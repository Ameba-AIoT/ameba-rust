#[doc = "Register `REG_PB7` reader"]
pub type R = crate::R<RegPb7Spec>;
#[doc = "Register `REG_PB7` writer"]
pub type W = crate::W<RegPb7Spec>;
#[doc = "Field `pB7_sel` reader - PAD pinmux function ID selection"]
pub type PB7SelR = crate::FieldReader;
#[doc = "Field `pB7_sel` writer - PAD pinmux function ID selection"]
pub type PB7SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB7_ie` reader - PAD input enable control"]
pub type PB7IeR = crate::BitReader;
#[doc = "Field `pB7_ie` writer - PAD input enable control"]
pub type PB7IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB7_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB7PupdcR = crate::BitReader;
#[doc = "Field `pB7_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB7PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB7_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB7E2R = crate::BitReader;
#[doc = "Field `pB7_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB7E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB7_smt` reader - PAD Schmit control"]
pub type PB7SmtR = crate::BitReader;
#[doc = "Field `pB7_smt` writer - PAD Schmit control"]
pub type PB7SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB7_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB7SrR = crate::BitReader;
#[doc = "Field `pB7_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB7SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB7_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB7E3R = crate::BitReader;
#[doc = "Field `pB7_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB7E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB7_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB7DisR = crate::BitReader;
#[doc = "Field `pB7_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB7DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b7_sel(&self) -> PB7SelR {
        PB7SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b7_ie(&self) -> PB7IeR {
        PB7IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b7_pupdc(&self) -> PB7PupdcR {
        PB7PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b7_e2(&self) -> PB7E2R {
        PB7E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b7_smt(&self) -> PB7SmtR {
        PB7SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b7_sr(&self) -> PB7SrR {
        PB7SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b7_e3(&self) -> PB7E3R {
        PB7E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b7_dis(&self) -> PB7DisR {
        PB7DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b7_sel(&mut self) -> PB7SelW<'_, RegPb7Spec> {
        PB7SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b7_ie(&mut self) -> PB7IeW<'_, RegPb7Spec> {
        PB7IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b7_pupdc(&mut self) -> PB7PupdcW<'_, RegPb7Spec> {
        PB7PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b7_e2(&mut self) -> PB7E2W<'_, RegPb7Spec> {
        PB7E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b7_smt(&mut self) -> PB7SmtW<'_, RegPb7Spec> {
        PB7SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b7_sr(&mut self) -> PB7SrW<'_, RegPb7Spec> {
        PB7SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b7_e3(&mut self) -> PB7E3W<'_, RegPb7Spec> {
        PB7E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b7_dis(&mut self) -> PB7DisW<'_, RegPb7Spec> {
        PB7DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb7Spec;
impl crate::RegisterSpec for RegPb7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb7::R`](R) reader structure"]
impl crate::Readable for RegPb7Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb7::W`](W) writer structure"]
impl crate::Writable for RegPb7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB7 to value 0x1900"]
impl crate::Resettable for RegPb7Spec {
    const RESET_VALUE: u32 = 0x1900;
}
