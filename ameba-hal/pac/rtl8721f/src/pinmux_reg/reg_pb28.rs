#[doc = "Register `REG_PB28` reader"]
pub type R = crate::R<RegPb28Spec>;
#[doc = "Register `REG_PB28` writer"]
pub type W = crate::W<RegPb28Spec>;
#[doc = "Field `pB28_sel` reader - PAD pinmux function ID selection"]
pub type PB28SelR = crate::FieldReader;
#[doc = "Field `pB28_sel` writer - PAD pinmux function ID selection"]
pub type PB28SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB28_ie` reader - PAD input enable control"]
pub type PB28IeR = crate::BitReader;
#[doc = "Field `pB28_ie` writer - PAD input enable control"]
pub type PB28IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB28_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB28PupdcR = crate::BitReader;
#[doc = "Field `pB28_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB28PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB28_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB28E2R = crate::BitReader;
#[doc = "Field `pB28_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB28E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB28_smt` reader - PAD Schmit control"]
pub type PB28SmtR = crate::BitReader;
#[doc = "Field `pB28_smt` writer - PAD Schmit control"]
pub type PB28SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB28_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB28SrR = crate::BitReader;
#[doc = "Field `pB28_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB28SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB28_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB28E3R = crate::BitReader;
#[doc = "Field `pB28_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB28E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB28_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB28DisR = crate::BitReader;
#[doc = "Field `pB28_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB28DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b28_sel(&self) -> PB28SelR {
        PB28SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b28_ie(&self) -> PB28IeR {
        PB28IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b28_pupdc(&self) -> PB28PupdcR {
        PB28PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b28_e2(&self) -> PB28E2R {
        PB28E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b28_smt(&self) -> PB28SmtR {
        PB28SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b28_sr(&self) -> PB28SrR {
        PB28SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b28_e3(&self) -> PB28E3R {
        PB28E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b28_dis(&self) -> PB28DisR {
        PB28DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b28_sel(&mut self) -> PB28SelW<'_, RegPb28Spec> {
        PB28SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b28_ie(&mut self) -> PB28IeW<'_, RegPb28Spec> {
        PB28IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b28_pupdc(&mut self) -> PB28PupdcW<'_, RegPb28Spec> {
        PB28PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b28_e2(&mut self) -> PB28E2W<'_, RegPb28Spec> {
        PB28E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b28_smt(&mut self) -> PB28SmtW<'_, RegPb28Spec> {
        PB28SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b28_sr(&mut self) -> PB28SrW<'_, RegPb28Spec> {
        PB28SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b28_e3(&mut self) -> PB28E3W<'_, RegPb28Spec> {
        PB28E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b28_dis(&mut self) -> PB28DisW<'_, RegPb28Spec> {
        PB28DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb28Spec;
impl crate::RegisterSpec for RegPb28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb28::R`](R) reader structure"]
impl crate::Readable for RegPb28Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb28::W`](W) writer structure"]
impl crate::Writable for RegPb28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB28 to value 0x1900"]
impl crate::Resettable for RegPb28Spec {
    const RESET_VALUE: u32 = 0x1900;
}
