#[doc = "Register `REG_PB8` reader"]
pub type R = crate::R<RegPb8Spec>;
#[doc = "Register `REG_PB8` writer"]
pub type W = crate::W<RegPb8Spec>;
#[doc = "Field `pB8_sel` reader - PAD pinmux function ID selection"]
pub type PB8SelR = crate::FieldReader;
#[doc = "Field `pB8_sel` writer - PAD pinmux function ID selection"]
pub type PB8SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB8_ie` reader - PAD input enable control"]
pub type PB8IeR = crate::BitReader;
#[doc = "Field `pB8_ie` writer - PAD input enable control"]
pub type PB8IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB8_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB8PupdcR = crate::BitReader;
#[doc = "Field `pB8_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB8PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB8_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB8E2R = crate::BitReader;
#[doc = "Field `pB8_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB8E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB8_smt` reader - PAD Schmit control"]
pub type PB8SmtR = crate::BitReader;
#[doc = "Field `pB8_smt` writer - PAD Schmit control"]
pub type PB8SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB8_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB8SrR = crate::BitReader;
#[doc = "Field `pB8_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB8SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB8_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB8E3R = crate::BitReader;
#[doc = "Field `pB8_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB8E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB8_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB8DisR = crate::BitReader;
#[doc = "Field `pB8_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB8DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b8_sel(&self) -> PB8SelR {
        PB8SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b8_ie(&self) -> PB8IeR {
        PB8IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b8_pupdc(&self) -> PB8PupdcR {
        PB8PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b8_e2(&self) -> PB8E2R {
        PB8E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b8_smt(&self) -> PB8SmtR {
        PB8SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b8_sr(&self) -> PB8SrR {
        PB8SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b8_e3(&self) -> PB8E3R {
        PB8E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b8_dis(&self) -> PB8DisR {
        PB8DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b8_sel(&mut self) -> PB8SelW<'_, RegPb8Spec> {
        PB8SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b8_ie(&mut self) -> PB8IeW<'_, RegPb8Spec> {
        PB8IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b8_pupdc(&mut self) -> PB8PupdcW<'_, RegPb8Spec> {
        PB8PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b8_e2(&mut self) -> PB8E2W<'_, RegPb8Spec> {
        PB8E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b8_smt(&mut self) -> PB8SmtW<'_, RegPb8Spec> {
        PB8SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b8_sr(&mut self) -> PB8SrW<'_, RegPb8Spec> {
        PB8SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b8_e3(&mut self) -> PB8E3W<'_, RegPb8Spec> {
        PB8E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b8_dis(&mut self) -> PB8DisW<'_, RegPb8Spec> {
        PB8DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb8Spec;
impl crate::RegisterSpec for RegPb8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb8::R`](R) reader structure"]
impl crate::Readable for RegPb8Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb8::W`](W) writer structure"]
impl crate::Writable for RegPb8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB8 to value 0x1900"]
impl crate::Resettable for RegPb8Spec {
    const RESET_VALUE: u32 = 0x1900;
}
