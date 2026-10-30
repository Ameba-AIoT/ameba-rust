#[doc = "Register `REG_PB15` reader"]
pub type R = crate::R<RegPb15Spec>;
#[doc = "Register `REG_PB15` writer"]
pub type W = crate::W<RegPb15Spec>;
#[doc = "Field `pB15_sel` reader - PAD pinmux function ID selection"]
pub type PB15SelR = crate::FieldReader;
#[doc = "Field `pB15_sel` writer - PAD pinmux function ID selection"]
pub type PB15SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB15_ie` reader - PAD input enable control"]
pub type PB15IeR = crate::BitReader;
#[doc = "Field `pB15_ie` writer - PAD input enable control"]
pub type PB15IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB15_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB15PupdcR = crate::BitReader;
#[doc = "Field `pB15_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB15PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB15_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB15E2R = crate::BitReader;
#[doc = "Field `pB15_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB15E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB15_smt` reader - PAD Schmit control"]
pub type PB15SmtR = crate::BitReader;
#[doc = "Field `pB15_smt` writer - PAD Schmit control"]
pub type PB15SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB15_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB15SrR = crate::BitReader;
#[doc = "Field `pB15_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB15SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB15_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB15E3R = crate::BitReader;
#[doc = "Field `pB15_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB15E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB15_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB15DisR = crate::BitReader;
#[doc = "Field `pB15_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB15DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b15_sel(&self) -> PB15SelR {
        PB15SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b15_ie(&self) -> PB15IeR {
        PB15IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b15_pupdc(&self) -> PB15PupdcR {
        PB15PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b15_e2(&self) -> PB15E2R {
        PB15E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b15_smt(&self) -> PB15SmtR {
        PB15SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b15_sr(&self) -> PB15SrR {
        PB15SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b15_e3(&self) -> PB15E3R {
        PB15E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b15_dis(&self) -> PB15DisR {
        PB15DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b15_sel(&mut self) -> PB15SelW<'_, RegPb15Spec> {
        PB15SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b15_ie(&mut self) -> PB15IeW<'_, RegPb15Spec> {
        PB15IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b15_pupdc(&mut self) -> PB15PupdcW<'_, RegPb15Spec> {
        PB15PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b15_e2(&mut self) -> PB15E2W<'_, RegPb15Spec> {
        PB15E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b15_smt(&mut self) -> PB15SmtW<'_, RegPb15Spec> {
        PB15SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b15_sr(&mut self) -> PB15SrW<'_, RegPb15Spec> {
        PB15SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b15_e3(&mut self) -> PB15E3W<'_, RegPb15Spec> {
        PB15E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b15_dis(&mut self) -> PB15DisW<'_, RegPb15Spec> {
        PB15DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb15Spec;
impl crate::RegisterSpec for RegPb15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb15::R`](R) reader structure"]
impl crate::Readable for RegPb15Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb15::W`](W) writer structure"]
impl crate::Writable for RegPb15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB15 to value 0x1900"]
impl crate::Resettable for RegPb15Spec {
    const RESET_VALUE: u32 = 0x1900;
}
