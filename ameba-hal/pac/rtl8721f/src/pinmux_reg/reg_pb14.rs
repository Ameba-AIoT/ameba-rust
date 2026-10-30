#[doc = "Register `REG_PB14` reader"]
pub type R = crate::R<RegPb14Spec>;
#[doc = "Register `REG_PB14` writer"]
pub type W = crate::W<RegPb14Spec>;
#[doc = "Field `pB14_sel` reader - PAD pinmux function ID selection"]
pub type PB14SelR = crate::FieldReader;
#[doc = "Field `pB14_sel` writer - PAD pinmux function ID selection"]
pub type PB14SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB14_ie` reader - PAD input enable control"]
pub type PB14IeR = crate::BitReader;
#[doc = "Field `pB14_ie` writer - PAD input enable control"]
pub type PB14IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB14_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB14PupdcR = crate::BitReader;
#[doc = "Field `pB14_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB14PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB14_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB14E2R = crate::BitReader;
#[doc = "Field `pB14_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB14E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB14_smt` reader - PAD Schmit control"]
pub type PB14SmtR = crate::BitReader;
#[doc = "Field `pB14_smt` writer - PAD Schmit control"]
pub type PB14SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB14_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB14SrR = crate::BitReader;
#[doc = "Field `pB14_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB14SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB14_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB14E3R = crate::BitReader;
#[doc = "Field `pB14_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB14E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB14_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB14DisR = crate::BitReader;
#[doc = "Field `pB14_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB14DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b14_sel(&self) -> PB14SelR {
        PB14SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b14_ie(&self) -> PB14IeR {
        PB14IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b14_pupdc(&self) -> PB14PupdcR {
        PB14PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b14_e2(&self) -> PB14E2R {
        PB14E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b14_smt(&self) -> PB14SmtR {
        PB14SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b14_sr(&self) -> PB14SrR {
        PB14SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b14_e3(&self) -> PB14E3R {
        PB14E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b14_dis(&self) -> PB14DisR {
        PB14DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b14_sel(&mut self) -> PB14SelW<'_, RegPb14Spec> {
        PB14SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b14_ie(&mut self) -> PB14IeW<'_, RegPb14Spec> {
        PB14IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b14_pupdc(&mut self) -> PB14PupdcW<'_, RegPb14Spec> {
        PB14PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b14_e2(&mut self) -> PB14E2W<'_, RegPb14Spec> {
        PB14E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b14_smt(&mut self) -> PB14SmtW<'_, RegPb14Spec> {
        PB14SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b14_sr(&mut self) -> PB14SrW<'_, RegPb14Spec> {
        PB14SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b14_e3(&mut self) -> PB14E3W<'_, RegPb14Spec> {
        PB14E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b14_dis(&mut self) -> PB14DisW<'_, RegPb14Spec> {
        PB14DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb14Spec;
impl crate::RegisterSpec for RegPb14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb14::R`](R) reader structure"]
impl crate::Readable for RegPb14Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb14::W`](W) writer structure"]
impl crate::Writable for RegPb14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB14 to value 0x1900"]
impl crate::Resettable for RegPb14Spec {
    const RESET_VALUE: u32 = 0x1900;
}
