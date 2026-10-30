#[doc = "Register `REG_PA24` reader"]
pub type R = crate::R<RegPa24Spec>;
#[doc = "Register `REG_PA24` writer"]
pub type W = crate::W<RegPa24Spec>;
#[doc = "Field `pA24_sel` reader - PAD pinmux function ID selection"]
pub type PA24SelR = crate::FieldReader;
#[doc = "Field `pA24_sel` writer - PAD pinmux function ID selection"]
pub type PA24SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA24_ie` reader - PAD input enable control"]
pub type PA24IeR = crate::BitReader;
#[doc = "Field `pA24_ie` writer - PAD input enable control"]
pub type PA24IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA24_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA24PupdcR = crate::BitReader;
#[doc = "Field `pA24_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA24PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA24_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA24E2R = crate::BitReader;
#[doc = "Field `pA24_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA24E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA24_smt` reader - PAD Schmit control"]
pub type PA24SmtR = crate::BitReader;
#[doc = "Field `pA24_smt` writer - PAD Schmit control"]
pub type PA24SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA24_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA24SrR = crate::BitReader;
#[doc = "Field `pA24_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA24SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA24_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA24E3R = crate::BitReader;
#[doc = "Field `pA24_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA24E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA24_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA24DisR = crate::BitReader;
#[doc = "Field `pA24_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA24DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a24_sel(&self) -> PA24SelR {
        PA24SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a24_ie(&self) -> PA24IeR {
        PA24IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a24_pupdc(&self) -> PA24PupdcR {
        PA24PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a24_e2(&self) -> PA24E2R {
        PA24E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a24_smt(&self) -> PA24SmtR {
        PA24SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a24_sr(&self) -> PA24SrR {
        PA24SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a24_e3(&self) -> PA24E3R {
        PA24E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a24_dis(&self) -> PA24DisR {
        PA24DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a24_sel(&mut self) -> PA24SelW<'_, RegPa24Spec> {
        PA24SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a24_ie(&mut self) -> PA24IeW<'_, RegPa24Spec> {
        PA24IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a24_pupdc(&mut self) -> PA24PupdcW<'_, RegPa24Spec> {
        PA24PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a24_e2(&mut self) -> PA24E2W<'_, RegPa24Spec> {
        PA24E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a24_smt(&mut self) -> PA24SmtW<'_, RegPa24Spec> {
        PA24SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a24_sr(&mut self) -> PA24SrW<'_, RegPa24Spec> {
        PA24SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a24_e3(&mut self) -> PA24E3W<'_, RegPa24Spec> {
        PA24E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a24_dis(&mut self) -> PA24DisW<'_, RegPa24Spec> {
        PA24DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa24Spec;
impl crate::RegisterSpec for RegPa24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa24::R`](R) reader structure"]
impl crate::Readable for RegPa24Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa24::W`](W) writer structure"]
impl crate::Writable for RegPa24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA24 to value 0x1900"]
impl crate::Resettable for RegPa24Spec {
    const RESET_VALUE: u32 = 0x1900;
}
