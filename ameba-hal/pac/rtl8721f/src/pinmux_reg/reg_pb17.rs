#[doc = "Register `REG_PB17` reader"]
pub type R = crate::R<RegPb17Spec>;
#[doc = "Register `REG_PB17` writer"]
pub type W = crate::W<RegPb17Spec>;
#[doc = "Field `pB17_sel` reader - PAD pinmux function ID selection"]
pub type PB17SelR = crate::FieldReader;
#[doc = "Field `pB17_sel` writer - PAD pinmux function ID selection"]
pub type PB17SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB17_ie` reader - PAD input enable control"]
pub type PB17IeR = crate::BitReader;
#[doc = "Field `pB17_ie` writer - PAD input enable control"]
pub type PB17IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB17_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB17PupdcR = crate::BitReader;
#[doc = "Field `pB17_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB17PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB17_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB17E2R = crate::BitReader;
#[doc = "Field `pB17_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB17E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB17_smt` reader - PAD Schmit control"]
pub type PB17SmtR = crate::BitReader;
#[doc = "Field `pB17_smt` writer - PAD Schmit control"]
pub type PB17SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB17_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB17SrR = crate::BitReader;
#[doc = "Field `pB17_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB17SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB17_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB17E3R = crate::BitReader;
#[doc = "Field `pB17_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB17E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB17_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB17DisR = crate::BitReader;
#[doc = "Field `pB17_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB17DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b17_sel(&self) -> PB17SelR {
        PB17SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b17_ie(&self) -> PB17IeR {
        PB17IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b17_pupdc(&self) -> PB17PupdcR {
        PB17PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b17_e2(&self) -> PB17E2R {
        PB17E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b17_smt(&self) -> PB17SmtR {
        PB17SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b17_sr(&self) -> PB17SrR {
        PB17SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b17_e3(&self) -> PB17E3R {
        PB17E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b17_dis(&self) -> PB17DisR {
        PB17DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b17_sel(&mut self) -> PB17SelW<'_, RegPb17Spec> {
        PB17SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b17_ie(&mut self) -> PB17IeW<'_, RegPb17Spec> {
        PB17IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b17_pupdc(&mut self) -> PB17PupdcW<'_, RegPb17Spec> {
        PB17PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b17_e2(&mut self) -> PB17E2W<'_, RegPb17Spec> {
        PB17E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b17_smt(&mut self) -> PB17SmtW<'_, RegPb17Spec> {
        PB17SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b17_sr(&mut self) -> PB17SrW<'_, RegPb17Spec> {
        PB17SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b17_e3(&mut self) -> PB17E3W<'_, RegPb17Spec> {
        PB17E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b17_dis(&mut self) -> PB17DisW<'_, RegPb17Spec> {
        PB17DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb17Spec;
impl crate::RegisterSpec for RegPb17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb17::R`](R) reader structure"]
impl crate::Readable for RegPb17Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb17::W`](W) writer structure"]
impl crate::Writable for RegPb17Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB17 to value 0x1900"]
impl crate::Resettable for RegPb17Spec {
    const RESET_VALUE: u32 = 0x1900;
}
