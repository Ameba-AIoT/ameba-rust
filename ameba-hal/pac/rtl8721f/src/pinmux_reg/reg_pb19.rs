#[doc = "Register `REG_PB19` reader"]
pub type R = crate::R<RegPb19Spec>;
#[doc = "Register `REG_PB19` writer"]
pub type W = crate::W<RegPb19Spec>;
#[doc = "Field `pB19_sel` reader - PAD pinmux function ID selection"]
pub type PB19SelR = crate::FieldReader;
#[doc = "Field `pB19_sel` writer - PAD pinmux function ID selection"]
pub type PB19SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB19_ie` reader - PAD input enable control"]
pub type PB19IeR = crate::BitReader;
#[doc = "Field `pB19_ie` writer - PAD input enable control"]
pub type PB19IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB19_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB19PupdcR = crate::BitReader;
#[doc = "Field `pB19_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB19PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB19_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB19E2R = crate::BitReader;
#[doc = "Field `pB19_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB19E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB19_smt` reader - PAD Schmit control"]
pub type PB19SmtR = crate::BitReader;
#[doc = "Field `pB19_smt` writer - PAD Schmit control"]
pub type PB19SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB19_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB19SrR = crate::BitReader;
#[doc = "Field `pB19_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB19SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB19_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB19E3R = crate::BitReader;
#[doc = "Field `pB19_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB19E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB19_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB19DisR = crate::BitReader;
#[doc = "Field `pB19_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB19DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b19_sel(&self) -> PB19SelR {
        PB19SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b19_ie(&self) -> PB19IeR {
        PB19IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b19_pupdc(&self) -> PB19PupdcR {
        PB19PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b19_e2(&self) -> PB19E2R {
        PB19E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b19_smt(&self) -> PB19SmtR {
        PB19SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b19_sr(&self) -> PB19SrR {
        PB19SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b19_e3(&self) -> PB19E3R {
        PB19E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b19_dis(&self) -> PB19DisR {
        PB19DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b19_sel(&mut self) -> PB19SelW<'_, RegPb19Spec> {
        PB19SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b19_ie(&mut self) -> PB19IeW<'_, RegPb19Spec> {
        PB19IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b19_pupdc(&mut self) -> PB19PupdcW<'_, RegPb19Spec> {
        PB19PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b19_e2(&mut self) -> PB19E2W<'_, RegPb19Spec> {
        PB19E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b19_smt(&mut self) -> PB19SmtW<'_, RegPb19Spec> {
        PB19SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b19_sr(&mut self) -> PB19SrW<'_, RegPb19Spec> {
        PB19SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b19_e3(&mut self) -> PB19E3W<'_, RegPb19Spec> {
        PB19E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b19_dis(&mut self) -> PB19DisW<'_, RegPb19Spec> {
        PB19DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb19Spec;
impl crate::RegisterSpec for RegPb19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb19::R`](R) reader structure"]
impl crate::Readable for RegPb19Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb19::W`](W) writer structure"]
impl crate::Writable for RegPb19Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB19 to value 0x1900"]
impl crate::Resettable for RegPb19Spec {
    const RESET_VALUE: u32 = 0x1900;
}
