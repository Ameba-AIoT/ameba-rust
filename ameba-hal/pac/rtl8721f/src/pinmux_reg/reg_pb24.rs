#[doc = "Register `REG_PB24` reader"]
pub type R = crate::R<RegPb24Spec>;
#[doc = "Register `REG_PB24` writer"]
pub type W = crate::W<RegPb24Spec>;
#[doc = "Field `pB24_sel` reader - PAD pinmux function ID selection"]
pub type PB24SelR = crate::FieldReader;
#[doc = "Field `pB24_sel` writer - PAD pinmux function ID selection"]
pub type PB24SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB24_ie` reader - PAD input enable control"]
pub type PB24IeR = crate::BitReader;
#[doc = "Field `pB24_ie` writer - PAD input enable control"]
pub type PB24IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB24_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB24PupdcR = crate::BitReader;
#[doc = "Field `pB24_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB24PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB24_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB24E2R = crate::BitReader;
#[doc = "Field `pB24_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB24E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB24_smt` reader - PAD Schmit control"]
pub type PB24SmtR = crate::BitReader;
#[doc = "Field `pB24_smt` writer - PAD Schmit control"]
pub type PB24SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB24_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB24SrR = crate::BitReader;
#[doc = "Field `pB24_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB24SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB24_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB24E3R = crate::BitReader;
#[doc = "Field `pB24_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB24E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB24_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB24DisR = crate::BitReader;
#[doc = "Field `pB24_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB24DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b24_sel(&self) -> PB24SelR {
        PB24SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b24_ie(&self) -> PB24IeR {
        PB24IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b24_pupdc(&self) -> PB24PupdcR {
        PB24PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b24_e2(&self) -> PB24E2R {
        PB24E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b24_smt(&self) -> PB24SmtR {
        PB24SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b24_sr(&self) -> PB24SrR {
        PB24SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b24_e3(&self) -> PB24E3R {
        PB24E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b24_dis(&self) -> PB24DisR {
        PB24DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b24_sel(&mut self) -> PB24SelW<'_, RegPb24Spec> {
        PB24SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b24_ie(&mut self) -> PB24IeW<'_, RegPb24Spec> {
        PB24IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b24_pupdc(&mut self) -> PB24PupdcW<'_, RegPb24Spec> {
        PB24PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b24_e2(&mut self) -> PB24E2W<'_, RegPb24Spec> {
        PB24E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b24_smt(&mut self) -> PB24SmtW<'_, RegPb24Spec> {
        PB24SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b24_sr(&mut self) -> PB24SrW<'_, RegPb24Spec> {
        PB24SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b24_e3(&mut self) -> PB24E3W<'_, RegPb24Spec> {
        PB24E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b24_dis(&mut self) -> PB24DisW<'_, RegPb24Spec> {
        PB24DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb24Spec;
impl crate::RegisterSpec for RegPb24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb24::R`](R) reader structure"]
impl crate::Readable for RegPb24Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb24::W`](W) writer structure"]
impl crate::Writable for RegPb24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB24 to value 0x1900"]
impl crate::Resettable for RegPb24Spec {
    const RESET_VALUE: u32 = 0x1900;
}
