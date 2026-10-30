#[doc = "Register `REG_PB18` reader"]
pub type R = crate::R<RegPb18Spec>;
#[doc = "Register `REG_PB18` writer"]
pub type W = crate::W<RegPb18Spec>;
#[doc = "Field `pB18_sel` reader - PAD pinmux function ID selection"]
pub type PB18SelR = crate::FieldReader;
#[doc = "Field `pB18_sel` writer - PAD pinmux function ID selection"]
pub type PB18SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB18_ie` reader - PAD input enable control"]
pub type PB18IeR = crate::BitReader;
#[doc = "Field `pB18_ie` writer - PAD input enable control"]
pub type PB18IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB18_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB18PupdcR = crate::BitReader;
#[doc = "Field `pB18_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB18PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB18_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB18E2R = crate::BitReader;
#[doc = "Field `pB18_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB18E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB18_smt` reader - PAD Schmit control"]
pub type PB18SmtR = crate::BitReader;
#[doc = "Field `pB18_smt` writer - PAD Schmit control"]
pub type PB18SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB18_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB18SrR = crate::BitReader;
#[doc = "Field `pB18_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB18SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB18_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB18E3R = crate::BitReader;
#[doc = "Field `pB18_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB18E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB18_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB18DisR = crate::BitReader;
#[doc = "Field `pB18_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB18DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b18_sel(&self) -> PB18SelR {
        PB18SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b18_ie(&self) -> PB18IeR {
        PB18IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b18_pupdc(&self) -> PB18PupdcR {
        PB18PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b18_e2(&self) -> PB18E2R {
        PB18E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b18_smt(&self) -> PB18SmtR {
        PB18SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b18_sr(&self) -> PB18SrR {
        PB18SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b18_e3(&self) -> PB18E3R {
        PB18E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b18_dis(&self) -> PB18DisR {
        PB18DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b18_sel(&mut self) -> PB18SelW<'_, RegPb18Spec> {
        PB18SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b18_ie(&mut self) -> PB18IeW<'_, RegPb18Spec> {
        PB18IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b18_pupdc(&mut self) -> PB18PupdcW<'_, RegPb18Spec> {
        PB18PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b18_e2(&mut self) -> PB18E2W<'_, RegPb18Spec> {
        PB18E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b18_smt(&mut self) -> PB18SmtW<'_, RegPb18Spec> {
        PB18SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b18_sr(&mut self) -> PB18SrW<'_, RegPb18Spec> {
        PB18SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b18_e3(&mut self) -> PB18E3W<'_, RegPb18Spec> {
        PB18E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b18_dis(&mut self) -> PB18DisW<'_, RegPb18Spec> {
        PB18DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb18Spec;
impl crate::RegisterSpec for RegPb18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb18::R`](R) reader structure"]
impl crate::Readable for RegPb18Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb18::W`](W) writer structure"]
impl crate::Writable for RegPb18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB18 to value 0x1900"]
impl crate::Resettable for RegPb18Spec {
    const RESET_VALUE: u32 = 0x1900;
}
