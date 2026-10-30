#[doc = "Register `REG_PB16` reader"]
pub type R = crate::R<RegPb16Spec>;
#[doc = "Register `REG_PB16` writer"]
pub type W = crate::W<RegPb16Spec>;
#[doc = "Field `pB16_sel` reader - PAD pinmux function ID selection"]
pub type PB16SelR = crate::FieldReader;
#[doc = "Field `pB16_sel` writer - PAD pinmux function ID selection"]
pub type PB16SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB16_ie` reader - PAD input enable control"]
pub type PB16IeR = crate::BitReader;
#[doc = "Field `pB16_ie` writer - PAD input enable control"]
pub type PB16IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB16_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB16PupdcR = crate::BitReader;
#[doc = "Field `pB16_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB16PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB16_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB16E2R = crate::BitReader;
#[doc = "Field `pB16_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB16E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB16_smt` reader - PAD Schmit control"]
pub type PB16SmtR = crate::BitReader;
#[doc = "Field `pB16_smt` writer - PAD Schmit control"]
pub type PB16SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB16_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB16SrR = crate::BitReader;
#[doc = "Field `pB16_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB16SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB16_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB16E3R = crate::BitReader;
#[doc = "Field `pB16_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB16E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB16_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB16DisR = crate::BitReader;
#[doc = "Field `pB16_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB16DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b16_sel(&self) -> PB16SelR {
        PB16SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b16_ie(&self) -> PB16IeR {
        PB16IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b16_pupdc(&self) -> PB16PupdcR {
        PB16PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b16_e2(&self) -> PB16E2R {
        PB16E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b16_smt(&self) -> PB16SmtR {
        PB16SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b16_sr(&self) -> PB16SrR {
        PB16SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b16_e3(&self) -> PB16E3R {
        PB16E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b16_dis(&self) -> PB16DisR {
        PB16DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b16_sel(&mut self) -> PB16SelW<'_, RegPb16Spec> {
        PB16SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b16_ie(&mut self) -> PB16IeW<'_, RegPb16Spec> {
        PB16IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b16_pupdc(&mut self) -> PB16PupdcW<'_, RegPb16Spec> {
        PB16PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b16_e2(&mut self) -> PB16E2W<'_, RegPb16Spec> {
        PB16E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b16_smt(&mut self) -> PB16SmtW<'_, RegPb16Spec> {
        PB16SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b16_sr(&mut self) -> PB16SrW<'_, RegPb16Spec> {
        PB16SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b16_e3(&mut self) -> PB16E3W<'_, RegPb16Spec> {
        PB16E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b16_dis(&mut self) -> PB16DisW<'_, RegPb16Spec> {
        PB16DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb16Spec;
impl crate::RegisterSpec for RegPb16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb16::R`](R) reader structure"]
impl crate::Readable for RegPb16Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb16::W`](W) writer structure"]
impl crate::Writable for RegPb16Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB16 to value 0x1900"]
impl crate::Resettable for RegPb16Spec {
    const RESET_VALUE: u32 = 0x1900;
}
