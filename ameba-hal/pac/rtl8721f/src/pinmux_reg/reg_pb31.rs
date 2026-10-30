#[doc = "Register `REG_PB31` reader"]
pub type R = crate::R<RegPb31Spec>;
#[doc = "Register `REG_PB31` writer"]
pub type W = crate::W<RegPb31Spec>;
#[doc = "Field `pB31_sel` reader - PAD pinmux function ID selection"]
pub type PB31SelR = crate::FieldReader;
#[doc = "Field `pB31_sel` writer - PAD pinmux function ID selection"]
pub type PB31SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB31_ie` reader - PAD input enable control"]
pub type PB31IeR = crate::BitReader;
#[doc = "Field `pB31_ie` writer - PAD input enable control"]
pub type PB31IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB31_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB31PupdcR = crate::BitReader;
#[doc = "Field `pB31_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB31PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB31_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB31E2R = crate::BitReader;
#[doc = "Field `pB31_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB31E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB31_smt` reader - PAD Schmit control"]
pub type PB31SmtR = crate::BitReader;
#[doc = "Field `pB31_smt` writer - PAD Schmit control"]
pub type PB31SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB31_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB31SrR = crate::BitReader;
#[doc = "Field `pB31_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB31SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB31_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB31E3R = crate::BitReader;
#[doc = "Field `pB31_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB31E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB31_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB31DisR = crate::BitReader;
#[doc = "Field `pB31_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB31DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b31_sel(&self) -> PB31SelR {
        PB31SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b31_ie(&self) -> PB31IeR {
        PB31IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b31_pupdc(&self) -> PB31PupdcR {
        PB31PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b31_e2(&self) -> PB31E2R {
        PB31E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b31_smt(&self) -> PB31SmtR {
        PB31SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b31_sr(&self) -> PB31SrR {
        PB31SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b31_e3(&self) -> PB31E3R {
        PB31E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b31_dis(&self) -> PB31DisR {
        PB31DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b31_sel(&mut self) -> PB31SelW<'_, RegPb31Spec> {
        PB31SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b31_ie(&mut self) -> PB31IeW<'_, RegPb31Spec> {
        PB31IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b31_pupdc(&mut self) -> PB31PupdcW<'_, RegPb31Spec> {
        PB31PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b31_e2(&mut self) -> PB31E2W<'_, RegPb31Spec> {
        PB31E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b31_smt(&mut self) -> PB31SmtW<'_, RegPb31Spec> {
        PB31SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b31_sr(&mut self) -> PB31SrW<'_, RegPb31Spec> {
        PB31SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b31_e3(&mut self) -> PB31E3W<'_, RegPb31Spec> {
        PB31E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b31_dis(&mut self) -> PB31DisW<'_, RegPb31Spec> {
        PB31DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb31Spec;
impl crate::RegisterSpec for RegPb31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb31::R`](R) reader structure"]
impl crate::Readable for RegPb31Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb31::W`](W) writer structure"]
impl crate::Writable for RegPb31Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB31 to value 0x1900"]
impl crate::Resettable for RegPb31Spec {
    const RESET_VALUE: u32 = 0x1900;
}
