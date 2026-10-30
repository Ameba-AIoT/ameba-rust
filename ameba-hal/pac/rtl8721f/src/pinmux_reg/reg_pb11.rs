#[doc = "Register `REG_PB11` reader"]
pub type R = crate::R<RegPb11Spec>;
#[doc = "Register `REG_PB11` writer"]
pub type W = crate::W<RegPb11Spec>;
#[doc = "Field `pB11_sel` reader - PAD pinmux function ID selection"]
pub type PB11SelR = crate::FieldReader;
#[doc = "Field `pB11_sel` writer - PAD pinmux function ID selection"]
pub type PB11SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB11_ie` reader - PAD input enable control"]
pub type PB11IeR = crate::BitReader;
#[doc = "Field `pB11_ie` writer - PAD input enable control"]
pub type PB11IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB11_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB11PupdcR = crate::BitReader;
#[doc = "Field `pB11_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB11PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB11_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB11E2R = crate::BitReader;
#[doc = "Field `pB11_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB11E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB11_smt` reader - PAD Schmit control"]
pub type PB11SmtR = crate::BitReader;
#[doc = "Field `pB11_smt` writer - PAD Schmit control"]
pub type PB11SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB11_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB11SrR = crate::BitReader;
#[doc = "Field `pB11_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB11SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB11_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB11E3R = crate::BitReader;
#[doc = "Field `pB11_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB11E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB11_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB11DisR = crate::BitReader;
#[doc = "Field `pB11_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB11DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b11_sel(&self) -> PB11SelR {
        PB11SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b11_ie(&self) -> PB11IeR {
        PB11IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b11_pupdc(&self) -> PB11PupdcR {
        PB11PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b11_e2(&self) -> PB11E2R {
        PB11E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b11_smt(&self) -> PB11SmtR {
        PB11SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b11_sr(&self) -> PB11SrR {
        PB11SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b11_e3(&self) -> PB11E3R {
        PB11E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b11_dis(&self) -> PB11DisR {
        PB11DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b11_sel(&mut self) -> PB11SelW<'_, RegPb11Spec> {
        PB11SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b11_ie(&mut self) -> PB11IeW<'_, RegPb11Spec> {
        PB11IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b11_pupdc(&mut self) -> PB11PupdcW<'_, RegPb11Spec> {
        PB11PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b11_e2(&mut self) -> PB11E2W<'_, RegPb11Spec> {
        PB11E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b11_smt(&mut self) -> PB11SmtW<'_, RegPb11Spec> {
        PB11SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b11_sr(&mut self) -> PB11SrW<'_, RegPb11Spec> {
        PB11SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b11_e3(&mut self) -> PB11E3W<'_, RegPb11Spec> {
        PB11E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b11_dis(&mut self) -> PB11DisW<'_, RegPb11Spec> {
        PB11DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb11Spec;
impl crate::RegisterSpec for RegPb11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb11::R`](R) reader structure"]
impl crate::Readable for RegPb11Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb11::W`](W) writer structure"]
impl crate::Writable for RegPb11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB11 to value 0x1900"]
impl crate::Resettable for RegPb11Spec {
    const RESET_VALUE: u32 = 0x1900;
}
