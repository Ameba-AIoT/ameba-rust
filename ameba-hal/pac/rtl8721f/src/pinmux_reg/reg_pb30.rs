#[doc = "Register `REG_PB30` reader"]
pub type R = crate::R<RegPb30Spec>;
#[doc = "Register `REG_PB30` writer"]
pub type W = crate::W<RegPb30Spec>;
#[doc = "Field `pB30_sel` reader - PAD pinmux function ID selection"]
pub type PB30SelR = crate::FieldReader;
#[doc = "Field `pB30_sel` writer - PAD pinmux function ID selection"]
pub type PB30SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB30_ie` reader - PAD input enable control"]
pub type PB30IeR = crate::BitReader;
#[doc = "Field `pB30_ie` writer - PAD input enable control"]
pub type PB30IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB30_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB30PupdcR = crate::BitReader;
#[doc = "Field `pB30_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB30PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB30_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB30E2R = crate::BitReader;
#[doc = "Field `pB30_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB30E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB30_smt` reader - PAD Schmit control"]
pub type PB30SmtR = crate::BitReader;
#[doc = "Field `pB30_smt` writer - PAD Schmit control"]
pub type PB30SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB30_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB30SrR = crate::BitReader;
#[doc = "Field `pB30_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB30SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB30_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB30E3R = crate::BitReader;
#[doc = "Field `pB30_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB30E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB30_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB30DisR = crate::BitReader;
#[doc = "Field `pB30_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB30DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b30_sel(&self) -> PB30SelR {
        PB30SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b30_ie(&self) -> PB30IeR {
        PB30IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b30_pupdc(&self) -> PB30PupdcR {
        PB30PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b30_e2(&self) -> PB30E2R {
        PB30E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b30_smt(&self) -> PB30SmtR {
        PB30SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b30_sr(&self) -> PB30SrR {
        PB30SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b30_e3(&self) -> PB30E3R {
        PB30E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b30_dis(&self) -> PB30DisR {
        PB30DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b30_sel(&mut self) -> PB30SelW<'_, RegPb30Spec> {
        PB30SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b30_ie(&mut self) -> PB30IeW<'_, RegPb30Spec> {
        PB30IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b30_pupdc(&mut self) -> PB30PupdcW<'_, RegPb30Spec> {
        PB30PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b30_e2(&mut self) -> PB30E2W<'_, RegPb30Spec> {
        PB30E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b30_smt(&mut self) -> PB30SmtW<'_, RegPb30Spec> {
        PB30SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b30_sr(&mut self) -> PB30SrW<'_, RegPb30Spec> {
        PB30SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b30_e3(&mut self) -> PB30E3W<'_, RegPb30Spec> {
        PB30E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b30_dis(&mut self) -> PB30DisW<'_, RegPb30Spec> {
        PB30DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb30Spec;
impl crate::RegisterSpec for RegPb30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb30::R`](R) reader structure"]
impl crate::Readable for RegPb30Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb30::W`](W) writer structure"]
impl crate::Writable for RegPb30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB30 to value 0x1900"]
impl crate::Resettable for RegPb30Spec {
    const RESET_VALUE: u32 = 0x1900;
}
