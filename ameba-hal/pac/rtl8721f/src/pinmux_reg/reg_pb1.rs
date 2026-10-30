#[doc = "Register `REG_PB1` reader"]
pub type R = crate::R<RegPb1Spec>;
#[doc = "Register `REG_PB1` writer"]
pub type W = crate::W<RegPb1Spec>;
#[doc = "Field `pB1_sel` reader - PAD pinmux function ID selection"]
pub type PB1SelR = crate::FieldReader;
#[doc = "Field `pB1_sel` writer - PAD pinmux function ID selection"]
pub type PB1SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB1_ie` reader - PAD input enable control"]
pub type PB1IeR = crate::BitReader;
#[doc = "Field `pB1_ie` writer - PAD input enable control"]
pub type PB1IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB1_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB1PupdcR = crate::BitReader;
#[doc = "Field `pB1_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB1PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB1_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB1E2R = crate::BitReader;
#[doc = "Field `pB1_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB1E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB1_smt` reader - PAD Schmit control"]
pub type PB1SmtR = crate::BitReader;
#[doc = "Field `pB1_smt` writer - PAD Schmit control"]
pub type PB1SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB1_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB1SrR = crate::BitReader;
#[doc = "Field `pB1_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB1SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB1_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB1E3R = crate::BitReader;
#[doc = "Field `pB1_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB1E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB1_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB1DisR = crate::BitReader;
#[doc = "Field `pB1_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB1DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b1_sel(&self) -> PB1SelR {
        PB1SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b1_ie(&self) -> PB1IeR {
        PB1IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b1_pupdc(&self) -> PB1PupdcR {
        PB1PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b1_e2(&self) -> PB1E2R {
        PB1E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b1_smt(&self) -> PB1SmtR {
        PB1SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b1_sr(&self) -> PB1SrR {
        PB1SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b1_e3(&self) -> PB1E3R {
        PB1E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b1_dis(&self) -> PB1DisR {
        PB1DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b1_sel(&mut self) -> PB1SelW<'_, RegPb1Spec> {
        PB1SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b1_ie(&mut self) -> PB1IeW<'_, RegPb1Spec> {
        PB1IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b1_pupdc(&mut self) -> PB1PupdcW<'_, RegPb1Spec> {
        PB1PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b1_e2(&mut self) -> PB1E2W<'_, RegPb1Spec> {
        PB1E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b1_smt(&mut self) -> PB1SmtW<'_, RegPb1Spec> {
        PB1SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b1_sr(&mut self) -> PB1SrW<'_, RegPb1Spec> {
        PB1SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b1_e3(&mut self) -> PB1E3W<'_, RegPb1Spec> {
        PB1E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b1_dis(&mut self) -> PB1DisW<'_, RegPb1Spec> {
        PB1DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb1Spec;
impl crate::RegisterSpec for RegPb1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb1::R`](R) reader structure"]
impl crate::Readable for RegPb1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb1::W`](W) writer structure"]
impl crate::Writable for RegPb1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB1 to value 0x1900"]
impl crate::Resettable for RegPb1Spec {
    const RESET_VALUE: u32 = 0x1900;
}
