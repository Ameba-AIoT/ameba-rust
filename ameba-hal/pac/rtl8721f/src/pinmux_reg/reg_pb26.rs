#[doc = "Register `REG_PB26` reader"]
pub type R = crate::R<RegPb26Spec>;
#[doc = "Register `REG_PB26` writer"]
pub type W = crate::W<RegPb26Spec>;
#[doc = "Field `pB26_sel` reader - PAD pinmux function ID selection"]
pub type PB26SelR = crate::FieldReader;
#[doc = "Field `pB26_sel` writer - PAD pinmux function ID selection"]
pub type PB26SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB26_ie` reader - PAD input enable control"]
pub type PB26IeR = crate::BitReader;
#[doc = "Field `pB26_ie` writer - PAD input enable control"]
pub type PB26IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB26_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB26PupdcR = crate::BitReader;
#[doc = "Field `pB26_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB26PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB26_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB26E2R = crate::BitReader;
#[doc = "Field `pB26_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB26E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB26_smt` reader - PAD Schmit control"]
pub type PB26SmtR = crate::BitReader;
#[doc = "Field `pB26_smt` writer - PAD Schmit control"]
pub type PB26SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB26_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB26SrR = crate::BitReader;
#[doc = "Field `pB26_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB26SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB26_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB26E3R = crate::BitReader;
#[doc = "Field `pB26_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB26E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB26_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB26DisR = crate::BitReader;
#[doc = "Field `pB26_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB26DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b26_sel(&self) -> PB26SelR {
        PB26SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b26_ie(&self) -> PB26IeR {
        PB26IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b26_pupdc(&self) -> PB26PupdcR {
        PB26PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b26_e2(&self) -> PB26E2R {
        PB26E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b26_smt(&self) -> PB26SmtR {
        PB26SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b26_sr(&self) -> PB26SrR {
        PB26SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b26_e3(&self) -> PB26E3R {
        PB26E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b26_dis(&self) -> PB26DisR {
        PB26DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b26_sel(&mut self) -> PB26SelW<'_, RegPb26Spec> {
        PB26SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b26_ie(&mut self) -> PB26IeW<'_, RegPb26Spec> {
        PB26IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b26_pupdc(&mut self) -> PB26PupdcW<'_, RegPb26Spec> {
        PB26PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b26_e2(&mut self) -> PB26E2W<'_, RegPb26Spec> {
        PB26E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b26_smt(&mut self) -> PB26SmtW<'_, RegPb26Spec> {
        PB26SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b26_sr(&mut self) -> PB26SrW<'_, RegPb26Spec> {
        PB26SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b26_e3(&mut self) -> PB26E3W<'_, RegPb26Spec> {
        PB26E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b26_dis(&mut self) -> PB26DisW<'_, RegPb26Spec> {
        PB26DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb26::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb26::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb26Spec;
impl crate::RegisterSpec for RegPb26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb26::R`](R) reader structure"]
impl crate::Readable for RegPb26Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb26::W`](W) writer structure"]
impl crate::Writable for RegPb26Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB26 to value 0x1900"]
impl crate::Resettable for RegPb26Spec {
    const RESET_VALUE: u32 = 0x1900;
}
