#[doc = "Register `REG_PB12` reader"]
pub type R = crate::R<RegPb12Spec>;
#[doc = "Register `REG_PB12` writer"]
pub type W = crate::W<RegPb12Spec>;
#[doc = "Field `pB12_sel` reader - PAD pinmux function ID selection"]
pub type PB12SelR = crate::FieldReader;
#[doc = "Field `pB12_sel` writer - PAD pinmux function ID selection"]
pub type PB12SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB12_ie` reader - PAD input enable control"]
pub type PB12IeR = crate::BitReader;
#[doc = "Field `pB12_ie` writer - PAD input enable control"]
pub type PB12IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB12_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB12PupdcR = crate::BitReader;
#[doc = "Field `pB12_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB12PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB12_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB12E2R = crate::BitReader;
#[doc = "Field `pB12_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB12E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB12_smt` reader - PAD Schmit control"]
pub type PB12SmtR = crate::BitReader;
#[doc = "Field `pB12_smt` writer - PAD Schmit control"]
pub type PB12SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB12_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB12SrR = crate::BitReader;
#[doc = "Field `pB12_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB12SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB12_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB12E3R = crate::BitReader;
#[doc = "Field `pB12_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB12E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB12_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB12DisR = crate::BitReader;
#[doc = "Field `pB12_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB12DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b12_sel(&self) -> PB12SelR {
        PB12SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b12_ie(&self) -> PB12IeR {
        PB12IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b12_pupdc(&self) -> PB12PupdcR {
        PB12PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b12_e2(&self) -> PB12E2R {
        PB12E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b12_smt(&self) -> PB12SmtR {
        PB12SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b12_sr(&self) -> PB12SrR {
        PB12SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b12_e3(&self) -> PB12E3R {
        PB12E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b12_dis(&self) -> PB12DisR {
        PB12DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b12_sel(&mut self) -> PB12SelW<'_, RegPb12Spec> {
        PB12SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b12_ie(&mut self) -> PB12IeW<'_, RegPb12Spec> {
        PB12IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b12_pupdc(&mut self) -> PB12PupdcW<'_, RegPb12Spec> {
        PB12PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b12_e2(&mut self) -> PB12E2W<'_, RegPb12Spec> {
        PB12E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b12_smt(&mut self) -> PB12SmtW<'_, RegPb12Spec> {
        PB12SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b12_sr(&mut self) -> PB12SrW<'_, RegPb12Spec> {
        PB12SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b12_e3(&mut self) -> PB12E3W<'_, RegPb12Spec> {
        PB12E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b12_dis(&mut self) -> PB12DisW<'_, RegPb12Spec> {
        PB12DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb12Spec;
impl crate::RegisterSpec for RegPb12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb12::R`](R) reader structure"]
impl crate::Readable for RegPb12Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb12::W`](W) writer structure"]
impl crate::Writable for RegPb12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB12 to value 0x1900"]
impl crate::Resettable for RegPb12Spec {
    const RESET_VALUE: u32 = 0x1900;
}
