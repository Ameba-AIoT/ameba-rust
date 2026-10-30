#[doc = "Register `REG_PB10` reader"]
pub type R = crate::R<RegPb10Spec>;
#[doc = "Register `REG_PB10` writer"]
pub type W = crate::W<RegPb10Spec>;
#[doc = "Field `pB10_sel` reader - PAD pinmux function ID selection"]
pub type PB10SelR = crate::FieldReader;
#[doc = "Field `pB10_sel` writer - PAD pinmux function ID selection"]
pub type PB10SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB10_ie` reader - PAD input enable control"]
pub type PB10IeR = crate::BitReader;
#[doc = "Field `pB10_ie` writer - PAD input enable control"]
pub type PB10IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB10_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB10PupdcR = crate::BitReader;
#[doc = "Field `pB10_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB10PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB10_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB10E2R = crate::BitReader;
#[doc = "Field `pB10_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB10E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB10_smt` reader - PAD Schmit control"]
pub type PB10SmtR = crate::BitReader;
#[doc = "Field `pB10_smt` writer - PAD Schmit control"]
pub type PB10SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB10_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB10SrR = crate::BitReader;
#[doc = "Field `pB10_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB10SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB10_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB10E3R = crate::BitReader;
#[doc = "Field `pB10_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB10E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB10_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB10DisR = crate::BitReader;
#[doc = "Field `pB10_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB10DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b10_sel(&self) -> PB10SelR {
        PB10SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b10_ie(&self) -> PB10IeR {
        PB10IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b10_pupdc(&self) -> PB10PupdcR {
        PB10PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b10_e2(&self) -> PB10E2R {
        PB10E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b10_smt(&self) -> PB10SmtR {
        PB10SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b10_sr(&self) -> PB10SrR {
        PB10SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b10_e3(&self) -> PB10E3R {
        PB10E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b10_dis(&self) -> PB10DisR {
        PB10DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b10_sel(&mut self) -> PB10SelW<'_, RegPb10Spec> {
        PB10SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b10_ie(&mut self) -> PB10IeW<'_, RegPb10Spec> {
        PB10IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b10_pupdc(&mut self) -> PB10PupdcW<'_, RegPb10Spec> {
        PB10PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b10_e2(&mut self) -> PB10E2W<'_, RegPb10Spec> {
        PB10E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b10_smt(&mut self) -> PB10SmtW<'_, RegPb10Spec> {
        PB10SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b10_sr(&mut self) -> PB10SrW<'_, RegPb10Spec> {
        PB10SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b10_e3(&mut self) -> PB10E3W<'_, RegPb10Spec> {
        PB10E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b10_dis(&mut self) -> PB10DisW<'_, RegPb10Spec> {
        PB10DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb10Spec;
impl crate::RegisterSpec for RegPb10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb10::R`](R) reader structure"]
impl crate::Readable for RegPb10Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb10::W`](W) writer structure"]
impl crate::Writable for RegPb10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB10 to value 0x1900"]
impl crate::Resettable for RegPb10Spec {
    const RESET_VALUE: u32 = 0x1900;
}
