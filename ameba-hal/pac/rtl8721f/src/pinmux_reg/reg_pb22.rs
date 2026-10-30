#[doc = "Register `REG_PB22` reader"]
pub type R = crate::R<RegPb22Spec>;
#[doc = "Register `REG_PB22` writer"]
pub type W = crate::W<RegPb22Spec>;
#[doc = "Field `pB22_sel` reader - PAD pinmux function ID selection"]
pub type PB22SelR = crate::FieldReader;
#[doc = "Field `pB22_sel` writer - PAD pinmux function ID selection"]
pub type PB22SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB22_ie` reader - PAD input enable control"]
pub type PB22IeR = crate::BitReader;
#[doc = "Field `pB22_ie` writer - PAD input enable control"]
pub type PB22IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB22_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB22PupdcR = crate::BitReader;
#[doc = "Field `pB22_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB22PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB22_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB22E2R = crate::BitReader;
#[doc = "Field `pB22_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB22E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB22_smt` reader - PAD Schmit control"]
pub type PB22SmtR = crate::BitReader;
#[doc = "Field `pB22_smt` writer - PAD Schmit control"]
pub type PB22SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB22_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB22SrR = crate::BitReader;
#[doc = "Field `pB22_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB22SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB22_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB22E3R = crate::BitReader;
#[doc = "Field `pB22_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB22E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB22_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB22DisR = crate::BitReader;
#[doc = "Field `pB22_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB22DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b22_sel(&self) -> PB22SelR {
        PB22SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b22_ie(&self) -> PB22IeR {
        PB22IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b22_pupdc(&self) -> PB22PupdcR {
        PB22PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b22_e2(&self) -> PB22E2R {
        PB22E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b22_smt(&self) -> PB22SmtR {
        PB22SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b22_sr(&self) -> PB22SrR {
        PB22SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b22_e3(&self) -> PB22E3R {
        PB22E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b22_dis(&self) -> PB22DisR {
        PB22DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b22_sel(&mut self) -> PB22SelW<'_, RegPb22Spec> {
        PB22SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b22_ie(&mut self) -> PB22IeW<'_, RegPb22Spec> {
        PB22IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b22_pupdc(&mut self) -> PB22PupdcW<'_, RegPb22Spec> {
        PB22PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b22_e2(&mut self) -> PB22E2W<'_, RegPb22Spec> {
        PB22E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b22_smt(&mut self) -> PB22SmtW<'_, RegPb22Spec> {
        PB22SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b22_sr(&mut self) -> PB22SrW<'_, RegPb22Spec> {
        PB22SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b22_e3(&mut self) -> PB22E3W<'_, RegPb22Spec> {
        PB22E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b22_dis(&mut self) -> PB22DisW<'_, RegPb22Spec> {
        PB22DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb22Spec;
impl crate::RegisterSpec for RegPb22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb22::R`](R) reader structure"]
impl crate::Readable for RegPb22Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb22::W`](W) writer structure"]
impl crate::Writable for RegPb22Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB22 to value 0x1900"]
impl crate::Resettable for RegPb22Spec {
    const RESET_VALUE: u32 = 0x1900;
}
