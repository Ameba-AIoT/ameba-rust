#[doc = "Register `REG_PC24` reader"]
pub type R = crate::R<RegPc24Spec>;
#[doc = "Register `REG_PC24` writer"]
pub type W = crate::W<RegPc24Spec>;
#[doc = "Field `pC24_sel` reader - PAD pinmux function ID selection"]
pub type PC24SelR = crate::FieldReader;
#[doc = "Field `pC24_sel` writer - PAD pinmux function ID selection"]
pub type PC24SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC24_ie` reader - PAD input enable control"]
pub type PC24IeR = crate::BitReader;
#[doc = "Field `pC24_ie` writer - PAD input enable control"]
pub type PC24IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC24_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC24PupdcR = crate::BitReader;
#[doc = "Field `pC24_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC24PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC24_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC24E2R = crate::BitReader;
#[doc = "Field `pC24_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC24E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC24_smt` reader - PAD Schmit control"]
pub type PC24SmtR = crate::BitReader;
#[doc = "Field `pC24_smt` writer - PAD Schmit control"]
pub type PC24SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC24_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC24SrR = crate::BitReader;
#[doc = "Field `pC24_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC24SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC24_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC24E3R = crate::BitReader;
#[doc = "Field `pC24_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC24E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC24_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC24DisR = crate::BitReader;
#[doc = "Field `pC24_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC24DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c24_sel(&self) -> PC24SelR {
        PC24SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c24_ie(&self) -> PC24IeR {
        PC24IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c24_pupdc(&self) -> PC24PupdcR {
        PC24PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c24_e2(&self) -> PC24E2R {
        PC24E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c24_smt(&self) -> PC24SmtR {
        PC24SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c24_sr(&self) -> PC24SrR {
        PC24SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c24_e3(&self) -> PC24E3R {
        PC24E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c24_dis(&self) -> PC24DisR {
        PC24DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c24_sel(&mut self) -> PC24SelW<'_, RegPc24Spec> {
        PC24SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c24_ie(&mut self) -> PC24IeW<'_, RegPc24Spec> {
        PC24IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c24_pupdc(&mut self) -> PC24PupdcW<'_, RegPc24Spec> {
        PC24PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c24_e2(&mut self) -> PC24E2W<'_, RegPc24Spec> {
        PC24E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c24_smt(&mut self) -> PC24SmtW<'_, RegPc24Spec> {
        PC24SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c24_sr(&mut self) -> PC24SrW<'_, RegPc24Spec> {
        PC24SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c24_e3(&mut self) -> PC24E3W<'_, RegPc24Spec> {
        PC24E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c24_dis(&mut self) -> PC24DisW<'_, RegPc24Spec> {
        PC24DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc24Spec;
impl crate::RegisterSpec for RegPc24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc24::R`](R) reader structure"]
impl crate::Readable for RegPc24Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc24::W`](W) writer structure"]
impl crate::Writable for RegPc24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC24 to value 0x1900"]
impl crate::Resettable for RegPc24Spec {
    const RESET_VALUE: u32 = 0x1900;
}
