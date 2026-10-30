#[doc = "Register `REG_PB6` reader"]
pub type R = crate::R<RegPb6Spec>;
#[doc = "Register `REG_PB6` writer"]
pub type W = crate::W<RegPb6Spec>;
#[doc = "Field `pB6_sel` reader - PAD pinmux function ID selection"]
pub type PB6SelR = crate::FieldReader;
#[doc = "Field `pB6_sel` writer - PAD pinmux function ID selection"]
pub type PB6SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB6_ie` reader - PAD input enable control"]
pub type PB6IeR = crate::BitReader;
#[doc = "Field `pB6_ie` writer - PAD input enable control"]
pub type PB6IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB6_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB6PupdcR = crate::BitReader;
#[doc = "Field `pB6_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB6PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB6_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB6E2R = crate::BitReader;
#[doc = "Field `pB6_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB6E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB6_smt` reader - PAD Schmit control"]
pub type PB6SmtR = crate::BitReader;
#[doc = "Field `pB6_smt` writer - PAD Schmit control"]
pub type PB6SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB6_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB6SrR = crate::BitReader;
#[doc = "Field `pB6_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB6SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB6_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB6E3R = crate::BitReader;
#[doc = "Field `pB6_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB6E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB6_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB6DisR = crate::BitReader;
#[doc = "Field `pB6_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB6DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b6_sel(&self) -> PB6SelR {
        PB6SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b6_ie(&self) -> PB6IeR {
        PB6IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b6_pupdc(&self) -> PB6PupdcR {
        PB6PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b6_e2(&self) -> PB6E2R {
        PB6E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b6_smt(&self) -> PB6SmtR {
        PB6SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b6_sr(&self) -> PB6SrR {
        PB6SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b6_e3(&self) -> PB6E3R {
        PB6E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b6_dis(&self) -> PB6DisR {
        PB6DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b6_sel(&mut self) -> PB6SelW<'_, RegPb6Spec> {
        PB6SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b6_ie(&mut self) -> PB6IeW<'_, RegPb6Spec> {
        PB6IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b6_pupdc(&mut self) -> PB6PupdcW<'_, RegPb6Spec> {
        PB6PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b6_e2(&mut self) -> PB6E2W<'_, RegPb6Spec> {
        PB6E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b6_smt(&mut self) -> PB6SmtW<'_, RegPb6Spec> {
        PB6SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b6_sr(&mut self) -> PB6SrW<'_, RegPb6Spec> {
        PB6SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b6_e3(&mut self) -> PB6E3W<'_, RegPb6Spec> {
        PB6E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b6_dis(&mut self) -> PB6DisW<'_, RegPb6Spec> {
        PB6DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb6Spec;
impl crate::RegisterSpec for RegPb6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb6::R`](R) reader structure"]
impl crate::Readable for RegPb6Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb6::W`](W) writer structure"]
impl crate::Writable for RegPb6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB6 to value 0x1900"]
impl crate::Resettable for RegPb6Spec {
    const RESET_VALUE: u32 = 0x1900;
}
