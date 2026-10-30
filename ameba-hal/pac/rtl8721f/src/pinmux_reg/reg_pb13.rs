#[doc = "Register `REG_PB13` reader"]
pub type R = crate::R<RegPb13Spec>;
#[doc = "Register `REG_PB13` writer"]
pub type W = crate::W<RegPb13Spec>;
#[doc = "Field `pB13_sel` reader - PAD pinmux function ID selection"]
pub type PB13SelR = crate::FieldReader;
#[doc = "Field `pB13_sel` writer - PAD pinmux function ID selection"]
pub type PB13SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB13_ie` reader - PAD input enable control"]
pub type PB13IeR = crate::BitReader;
#[doc = "Field `pB13_ie` writer - PAD input enable control"]
pub type PB13IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB13_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB13PupdcR = crate::BitReader;
#[doc = "Field `pB13_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB13PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB13_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB13E2R = crate::BitReader;
#[doc = "Field `pB13_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB13E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB13_smt` reader - PAD Schmit control"]
pub type PB13SmtR = crate::BitReader;
#[doc = "Field `pB13_smt` writer - PAD Schmit control"]
pub type PB13SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB13_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB13SrR = crate::BitReader;
#[doc = "Field `pB13_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB13SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB13_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB13E3R = crate::BitReader;
#[doc = "Field `pB13_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB13E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB13_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB13DisR = crate::BitReader;
#[doc = "Field `pB13_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB13DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b13_sel(&self) -> PB13SelR {
        PB13SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b13_ie(&self) -> PB13IeR {
        PB13IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b13_pupdc(&self) -> PB13PupdcR {
        PB13PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b13_e2(&self) -> PB13E2R {
        PB13E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b13_smt(&self) -> PB13SmtR {
        PB13SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b13_sr(&self) -> PB13SrR {
        PB13SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b13_e3(&self) -> PB13E3R {
        PB13E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b13_dis(&self) -> PB13DisR {
        PB13DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b13_sel(&mut self) -> PB13SelW<'_, RegPb13Spec> {
        PB13SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b13_ie(&mut self) -> PB13IeW<'_, RegPb13Spec> {
        PB13IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b13_pupdc(&mut self) -> PB13PupdcW<'_, RegPb13Spec> {
        PB13PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b13_e2(&mut self) -> PB13E2W<'_, RegPb13Spec> {
        PB13E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b13_smt(&mut self) -> PB13SmtW<'_, RegPb13Spec> {
        PB13SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b13_sr(&mut self) -> PB13SrW<'_, RegPb13Spec> {
        PB13SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b13_e3(&mut self) -> PB13E3W<'_, RegPb13Spec> {
        PB13E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b13_dis(&mut self) -> PB13DisW<'_, RegPb13Spec> {
        PB13DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb13Spec;
impl crate::RegisterSpec for RegPb13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb13::R`](R) reader structure"]
impl crate::Readable for RegPb13Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb13::W`](W) writer structure"]
impl crate::Writable for RegPb13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB13 to value 0x1900"]
impl crate::Resettable for RegPb13Spec {
    const RESET_VALUE: u32 = 0x1900;
}
