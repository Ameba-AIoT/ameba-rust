#[doc = "Register `REG_PB0` reader"]
pub type R = crate::R<RegPb0Spec>;
#[doc = "Register `REG_PB0` writer"]
pub type W = crate::W<RegPb0Spec>;
#[doc = "Field `pB0_sel` reader - PAD pinmux function ID selection"]
pub type PB0SelR = crate::FieldReader;
#[doc = "Field `pB0_sel` writer - PAD pinmux function ID selection"]
pub type PB0SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB0_ie` reader - PAD input enable control"]
pub type PB0IeR = crate::BitReader;
#[doc = "Field `pB0_ie` writer - PAD input enable control"]
pub type PB0IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB0_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB0PupdcR = crate::BitReader;
#[doc = "Field `pB0_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB0PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB0_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB0E2R = crate::BitReader;
#[doc = "Field `pB0_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB0E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB0_smt` reader - PAD Schmit control"]
pub type PB0SmtR = crate::BitReader;
#[doc = "Field `pB0_smt` writer - PAD Schmit control"]
pub type PB0SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB0_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB0SrR = crate::BitReader;
#[doc = "Field `pB0_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB0SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB0_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB0E3R = crate::BitReader;
#[doc = "Field `pB0_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB0E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB0_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB0DisR = crate::BitReader;
#[doc = "Field `pB0_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB0DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b0_sel(&self) -> PB0SelR {
        PB0SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b0_ie(&self) -> PB0IeR {
        PB0IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b0_pupdc(&self) -> PB0PupdcR {
        PB0PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b0_e2(&self) -> PB0E2R {
        PB0E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b0_smt(&self) -> PB0SmtR {
        PB0SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b0_sr(&self) -> PB0SrR {
        PB0SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b0_e3(&self) -> PB0E3R {
        PB0E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b0_dis(&self) -> PB0DisR {
        PB0DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b0_sel(&mut self) -> PB0SelW<'_, RegPb0Spec> {
        PB0SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b0_ie(&mut self) -> PB0IeW<'_, RegPb0Spec> {
        PB0IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b0_pupdc(&mut self) -> PB0PupdcW<'_, RegPb0Spec> {
        PB0PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b0_e2(&mut self) -> PB0E2W<'_, RegPb0Spec> {
        PB0E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b0_smt(&mut self) -> PB0SmtW<'_, RegPb0Spec> {
        PB0SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b0_sr(&mut self) -> PB0SrW<'_, RegPb0Spec> {
        PB0SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b0_e3(&mut self) -> PB0E3W<'_, RegPb0Spec> {
        PB0E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b0_dis(&mut self) -> PB0DisW<'_, RegPb0Spec> {
        PB0DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb0Spec;
impl crate::RegisterSpec for RegPb0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb0::R`](R) reader structure"]
impl crate::Readable for RegPb0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb0::W`](W) writer structure"]
impl crate::Writable for RegPb0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB0 to value 0x1900"]
impl crate::Resettable for RegPb0Spec {
    const RESET_VALUE: u32 = 0x1900;
}
