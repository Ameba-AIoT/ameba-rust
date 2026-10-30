#[doc = "Register `REG_PB4` reader"]
pub type R = crate::R<RegPb4Spec>;
#[doc = "Register `REG_PB4` writer"]
pub type W = crate::W<RegPb4Spec>;
#[doc = "Field `pB4_sel` reader - PAD pinmux function ID selection"]
pub type PB4SelR = crate::FieldReader;
#[doc = "Field `pB4_sel` writer - PAD pinmux function ID selection"]
pub type PB4SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB4_ie` reader - PAD input enable control"]
pub type PB4IeR = crate::BitReader;
#[doc = "Field `pB4_ie` writer - PAD input enable control"]
pub type PB4IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB4_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB4PupdcR = crate::BitReader;
#[doc = "Field `pB4_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB4PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB4_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB4E2R = crate::BitReader;
#[doc = "Field `pB4_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB4E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB4_smt` reader - PAD Schmit control"]
pub type PB4SmtR = crate::BitReader;
#[doc = "Field `pB4_smt` writer - PAD Schmit control"]
pub type PB4SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB4_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB4SrR = crate::BitReader;
#[doc = "Field `pB4_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB4SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB4_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB4E3R = crate::BitReader;
#[doc = "Field `pB4_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB4E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB4_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB4DisR = crate::BitReader;
#[doc = "Field `pB4_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB4DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b4_sel(&self) -> PB4SelR {
        PB4SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b4_ie(&self) -> PB4IeR {
        PB4IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b4_pupdc(&self) -> PB4PupdcR {
        PB4PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b4_e2(&self) -> PB4E2R {
        PB4E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b4_smt(&self) -> PB4SmtR {
        PB4SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b4_sr(&self) -> PB4SrR {
        PB4SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b4_e3(&self) -> PB4E3R {
        PB4E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b4_dis(&self) -> PB4DisR {
        PB4DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b4_sel(&mut self) -> PB4SelW<'_, RegPb4Spec> {
        PB4SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b4_ie(&mut self) -> PB4IeW<'_, RegPb4Spec> {
        PB4IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b4_pupdc(&mut self) -> PB4PupdcW<'_, RegPb4Spec> {
        PB4PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b4_e2(&mut self) -> PB4E2W<'_, RegPb4Spec> {
        PB4E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b4_smt(&mut self) -> PB4SmtW<'_, RegPb4Spec> {
        PB4SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b4_sr(&mut self) -> PB4SrW<'_, RegPb4Spec> {
        PB4SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b4_e3(&mut self) -> PB4E3W<'_, RegPb4Spec> {
        PB4E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b4_dis(&mut self) -> PB4DisW<'_, RegPb4Spec> {
        PB4DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb4Spec;
impl crate::RegisterSpec for RegPb4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb4::R`](R) reader structure"]
impl crate::Readable for RegPb4Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb4::W`](W) writer structure"]
impl crate::Writable for RegPb4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB4 to value 0x1900"]
impl crate::Resettable for RegPb4Spec {
    const RESET_VALUE: u32 = 0x1900;
}
