#[doc = "Register `REG_PB23` reader"]
pub type R = crate::R<RegPb23Spec>;
#[doc = "Register `REG_PB23` writer"]
pub type W = crate::W<RegPb23Spec>;
#[doc = "Field `pB23_sel` reader - PAD pinmux function ID selection"]
pub type PB23SelR = crate::FieldReader;
#[doc = "Field `pB23_sel` writer - PAD pinmux function ID selection"]
pub type PB23SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB23_ie` reader - PAD input enable control"]
pub type PB23IeR = crate::BitReader;
#[doc = "Field `pB23_ie` writer - PAD input enable control"]
pub type PB23IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB23_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB23PupdcR = crate::BitReader;
#[doc = "Field `pB23_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB23PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB23_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB23E2R = crate::BitReader;
#[doc = "Field `pB23_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB23E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB23_smt` reader - PAD Schmit control"]
pub type PB23SmtR = crate::BitReader;
#[doc = "Field `pB23_smt` writer - PAD Schmit control"]
pub type PB23SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB23_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB23SrR = crate::BitReader;
#[doc = "Field `pB23_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB23SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB23_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB23E3R = crate::BitReader;
#[doc = "Field `pB23_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB23E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB23_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB23DisR = crate::BitReader;
#[doc = "Field `pB23_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB23DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b23_sel(&self) -> PB23SelR {
        PB23SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b23_ie(&self) -> PB23IeR {
        PB23IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b23_pupdc(&self) -> PB23PupdcR {
        PB23PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b23_e2(&self) -> PB23E2R {
        PB23E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b23_smt(&self) -> PB23SmtR {
        PB23SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b23_sr(&self) -> PB23SrR {
        PB23SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b23_e3(&self) -> PB23E3R {
        PB23E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b23_dis(&self) -> PB23DisR {
        PB23DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b23_sel(&mut self) -> PB23SelW<'_, RegPb23Spec> {
        PB23SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b23_ie(&mut self) -> PB23IeW<'_, RegPb23Spec> {
        PB23IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b23_pupdc(&mut self) -> PB23PupdcW<'_, RegPb23Spec> {
        PB23PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b23_e2(&mut self) -> PB23E2W<'_, RegPb23Spec> {
        PB23E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b23_smt(&mut self) -> PB23SmtW<'_, RegPb23Spec> {
        PB23SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b23_sr(&mut self) -> PB23SrW<'_, RegPb23Spec> {
        PB23SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b23_e3(&mut self) -> PB23E3W<'_, RegPb23Spec> {
        PB23E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b23_dis(&mut self) -> PB23DisW<'_, RegPb23Spec> {
        PB23DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb23Spec;
impl crate::RegisterSpec for RegPb23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb23::R`](R) reader structure"]
impl crate::Readable for RegPb23Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb23::W`](W) writer structure"]
impl crate::Writable for RegPb23Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB23 to value 0x1900"]
impl crate::Resettable for RegPb23Spec {
    const RESET_VALUE: u32 = 0x1900;
}
