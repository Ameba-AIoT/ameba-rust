#[doc = "Register `REG_PB29` reader"]
pub type R = crate::R<RegPb29Spec>;
#[doc = "Register `REG_PB29` writer"]
pub type W = crate::W<RegPb29Spec>;
#[doc = "Field `pB29_sel` reader - PAD pinmux function ID selection"]
pub type PB29SelR = crate::FieldReader;
#[doc = "Field `pB29_sel` writer - PAD pinmux function ID selection"]
pub type PB29SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB29_ie` reader - PAD input enable control"]
pub type PB29IeR = crate::BitReader;
#[doc = "Field `pB29_ie` writer - PAD input enable control"]
pub type PB29IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB29_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB29PupdcR = crate::BitReader;
#[doc = "Field `pB29_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB29PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB29_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB29E2R = crate::BitReader;
#[doc = "Field `pB29_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB29E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB29_smt` reader - PAD Schmit control"]
pub type PB29SmtR = crate::BitReader;
#[doc = "Field `pB29_smt` writer - PAD Schmit control"]
pub type PB29SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB29_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB29SrR = crate::BitReader;
#[doc = "Field `pB29_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB29SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB29_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB29E3R = crate::BitReader;
#[doc = "Field `pB29_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB29E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB29_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB29DisR = crate::BitReader;
#[doc = "Field `pB29_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB29DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b29_sel(&self) -> PB29SelR {
        PB29SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b29_ie(&self) -> PB29IeR {
        PB29IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b29_pupdc(&self) -> PB29PupdcR {
        PB29PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b29_e2(&self) -> PB29E2R {
        PB29E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b29_smt(&self) -> PB29SmtR {
        PB29SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b29_sr(&self) -> PB29SrR {
        PB29SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b29_e3(&self) -> PB29E3R {
        PB29E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b29_dis(&self) -> PB29DisR {
        PB29DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b29_sel(&mut self) -> PB29SelW<'_, RegPb29Spec> {
        PB29SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b29_ie(&mut self) -> PB29IeW<'_, RegPb29Spec> {
        PB29IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b29_pupdc(&mut self) -> PB29PupdcW<'_, RegPb29Spec> {
        PB29PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b29_e2(&mut self) -> PB29E2W<'_, RegPb29Spec> {
        PB29E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b29_smt(&mut self) -> PB29SmtW<'_, RegPb29Spec> {
        PB29SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b29_sr(&mut self) -> PB29SrW<'_, RegPb29Spec> {
        PB29SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b29_e3(&mut self) -> PB29E3W<'_, RegPb29Spec> {
        PB29E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b29_dis(&mut self) -> PB29DisW<'_, RegPb29Spec> {
        PB29DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb29::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb29::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb29Spec;
impl crate::RegisterSpec for RegPb29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb29::R`](R) reader structure"]
impl crate::Readable for RegPb29Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb29::W`](W) writer structure"]
impl crate::Writable for RegPb29Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB29 to value 0x1900"]
impl crate::Resettable for RegPb29Spec {
    const RESET_VALUE: u32 = 0x1900;
}
