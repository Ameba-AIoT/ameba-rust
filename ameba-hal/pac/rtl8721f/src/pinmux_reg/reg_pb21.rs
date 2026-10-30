#[doc = "Register `REG_PB21` reader"]
pub type R = crate::R<RegPb21Spec>;
#[doc = "Register `REG_PB21` writer"]
pub type W = crate::W<RegPb21Spec>;
#[doc = "Field `pB21_sel` reader - PAD pinmux function ID selection"]
pub type PB21SelR = crate::FieldReader;
#[doc = "Field `pB21_sel` writer - PAD pinmux function ID selection"]
pub type PB21SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB21_ie` reader - PAD input enable control"]
pub type PB21IeR = crate::BitReader;
#[doc = "Field `pB21_ie` writer - PAD input enable control"]
pub type PB21IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB21_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB21PupdcR = crate::BitReader;
#[doc = "Field `pB21_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB21PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB21_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB21E2R = crate::BitReader;
#[doc = "Field `pB21_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB21E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB21_smt` reader - PAD Schmit control"]
pub type PB21SmtR = crate::BitReader;
#[doc = "Field `pB21_smt` writer - PAD Schmit control"]
pub type PB21SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB21_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB21SrR = crate::BitReader;
#[doc = "Field `pB21_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB21SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB21_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB21E3R = crate::BitReader;
#[doc = "Field `pB21_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB21E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB21_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB21DisR = crate::BitReader;
#[doc = "Field `pB21_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB21DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b21_sel(&self) -> PB21SelR {
        PB21SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b21_ie(&self) -> PB21IeR {
        PB21IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b21_pupdc(&self) -> PB21PupdcR {
        PB21PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b21_e2(&self) -> PB21E2R {
        PB21E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b21_smt(&self) -> PB21SmtR {
        PB21SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b21_sr(&self) -> PB21SrR {
        PB21SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b21_e3(&self) -> PB21E3R {
        PB21E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b21_dis(&self) -> PB21DisR {
        PB21DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b21_sel(&mut self) -> PB21SelW<'_, RegPb21Spec> {
        PB21SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b21_ie(&mut self) -> PB21IeW<'_, RegPb21Spec> {
        PB21IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b21_pupdc(&mut self) -> PB21PupdcW<'_, RegPb21Spec> {
        PB21PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b21_e2(&mut self) -> PB21E2W<'_, RegPb21Spec> {
        PB21E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b21_smt(&mut self) -> PB21SmtW<'_, RegPb21Spec> {
        PB21SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b21_sr(&mut self) -> PB21SrW<'_, RegPb21Spec> {
        PB21SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b21_e3(&mut self) -> PB21E3W<'_, RegPb21Spec> {
        PB21E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b21_dis(&mut self) -> PB21DisW<'_, RegPb21Spec> {
        PB21DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb21Spec;
impl crate::RegisterSpec for RegPb21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb21::R`](R) reader structure"]
impl crate::Readable for RegPb21Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb21::W`](W) writer structure"]
impl crate::Writable for RegPb21Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB21 to value 0x1900"]
impl crate::Resettable for RegPb21Spec {
    const RESET_VALUE: u32 = 0x1900;
}
