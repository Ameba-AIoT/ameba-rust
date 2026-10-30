#[doc = "Register `REG_PB27` reader"]
pub type R = crate::R<RegPb27Spec>;
#[doc = "Register `REG_PB27` writer"]
pub type W = crate::W<RegPb27Spec>;
#[doc = "Field `pB27_sel` reader - PAD pinmux function ID selection"]
pub type PB27SelR = crate::FieldReader;
#[doc = "Field `pB27_sel` writer - PAD pinmux function ID selection"]
pub type PB27SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB27_ie` reader - PAD input enable control"]
pub type PB27IeR = crate::BitReader;
#[doc = "Field `pB27_ie` writer - PAD input enable control"]
pub type PB27IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB27_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB27PupdcR = crate::BitReader;
#[doc = "Field `pB27_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB27PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB27_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB27E2R = crate::BitReader;
#[doc = "Field `pB27_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB27E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB27_smt` reader - PAD Schmit control"]
pub type PB27SmtR = crate::BitReader;
#[doc = "Field `pB27_smt` writer - PAD Schmit control"]
pub type PB27SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB27_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB27SrR = crate::BitReader;
#[doc = "Field `pB27_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB27SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB27_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB27E3R = crate::BitReader;
#[doc = "Field `pB27_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB27E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB27_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB27DisR = crate::BitReader;
#[doc = "Field `pB27_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB27DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b27_sel(&self) -> PB27SelR {
        PB27SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b27_ie(&self) -> PB27IeR {
        PB27IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b27_pupdc(&self) -> PB27PupdcR {
        PB27PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b27_e2(&self) -> PB27E2R {
        PB27E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b27_smt(&self) -> PB27SmtR {
        PB27SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b27_sr(&self) -> PB27SrR {
        PB27SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b27_e3(&self) -> PB27E3R {
        PB27E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b27_dis(&self) -> PB27DisR {
        PB27DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b27_sel(&mut self) -> PB27SelW<'_, RegPb27Spec> {
        PB27SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b27_ie(&mut self) -> PB27IeW<'_, RegPb27Spec> {
        PB27IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b27_pupdc(&mut self) -> PB27PupdcW<'_, RegPb27Spec> {
        PB27PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b27_e2(&mut self) -> PB27E2W<'_, RegPb27Spec> {
        PB27E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b27_smt(&mut self) -> PB27SmtW<'_, RegPb27Spec> {
        PB27SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b27_sr(&mut self) -> PB27SrW<'_, RegPb27Spec> {
        PB27SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b27_e3(&mut self) -> PB27E3W<'_, RegPb27Spec> {
        PB27E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b27_dis(&mut self) -> PB27DisW<'_, RegPb27Spec> {
        PB27DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb27::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb27::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb27Spec;
impl crate::RegisterSpec for RegPb27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb27::R`](R) reader structure"]
impl crate::Readable for RegPb27Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb27::W`](W) writer structure"]
impl crate::Writable for RegPb27Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB27 to value 0x1900"]
impl crate::Resettable for RegPb27Spec {
    const RESET_VALUE: u32 = 0x1900;
}
