#[doc = "Register `REG_PB5` reader"]
pub type R = crate::R<RegPb5Spec>;
#[doc = "Register `REG_PB5` writer"]
pub type W = crate::W<RegPb5Spec>;
#[doc = "Field `pB5_sel` reader - PAD pinmux function ID selection"]
pub type PB5SelR = crate::FieldReader;
#[doc = "Field `pB5_sel` writer - PAD pinmux function ID selection"]
pub type PB5SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB5_ie` reader - PAD input enable control"]
pub type PB5IeR = crate::BitReader;
#[doc = "Field `pB5_ie` writer - PAD input enable control"]
pub type PB5IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB5_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB5PupdcR = crate::BitReader;
#[doc = "Field `pB5_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB5PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB5_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB5E2R = crate::BitReader;
#[doc = "Field `pB5_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB5E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB5_smt` reader - PAD Schmit control"]
pub type PB5SmtR = crate::BitReader;
#[doc = "Field `pB5_smt` writer - PAD Schmit control"]
pub type PB5SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB5_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB5SrR = crate::BitReader;
#[doc = "Field `pB5_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB5SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB5_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB5E3R = crate::BitReader;
#[doc = "Field `pB5_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB5E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB5_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB5DisR = crate::BitReader;
#[doc = "Field `pB5_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB5DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b5_sel(&self) -> PB5SelR {
        PB5SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b5_ie(&self) -> PB5IeR {
        PB5IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b5_pupdc(&self) -> PB5PupdcR {
        PB5PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b5_e2(&self) -> PB5E2R {
        PB5E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b5_smt(&self) -> PB5SmtR {
        PB5SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b5_sr(&self) -> PB5SrR {
        PB5SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b5_e3(&self) -> PB5E3R {
        PB5E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b5_dis(&self) -> PB5DisR {
        PB5DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b5_sel(&mut self) -> PB5SelW<'_, RegPb5Spec> {
        PB5SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b5_ie(&mut self) -> PB5IeW<'_, RegPb5Spec> {
        PB5IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b5_pupdc(&mut self) -> PB5PupdcW<'_, RegPb5Spec> {
        PB5PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b5_e2(&mut self) -> PB5E2W<'_, RegPb5Spec> {
        PB5E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b5_smt(&mut self) -> PB5SmtW<'_, RegPb5Spec> {
        PB5SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b5_sr(&mut self) -> PB5SrW<'_, RegPb5Spec> {
        PB5SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b5_e3(&mut self) -> PB5E3W<'_, RegPb5Spec> {
        PB5E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b5_dis(&mut self) -> PB5DisW<'_, RegPb5Spec> {
        PB5DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb5Spec;
impl crate::RegisterSpec for RegPb5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb5::R`](R) reader structure"]
impl crate::Readable for RegPb5Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb5::W`](W) writer structure"]
impl crate::Writable for RegPb5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB5 to value 0x1900"]
impl crate::Resettable for RegPb5Spec {
    const RESET_VALUE: u32 = 0x1900;
}
