#[doc = "Register `REG_PA4` reader"]
pub type R = crate::R<RegPa4Spec>;
#[doc = "Register `REG_PA4` writer"]
pub type W = crate::W<RegPa4Spec>;
#[doc = "Field `pA4_sel` reader - PAD pinmux function ID selection"]
pub type PA4SelR = crate::FieldReader;
#[doc = "Field `pA4_sel` writer - PAD pinmux function ID selection"]
pub type PA4SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA4_ie` reader - PAD input enable control"]
pub type PA4IeR = crate::BitReader;
#[doc = "Field `pA4_ie` writer - PAD input enable control"]
pub type PA4IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA4_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA4PupdcR = crate::BitReader;
#[doc = "Field `pA4_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA4PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA4_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA4E2R = crate::BitReader;
#[doc = "Field `pA4_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA4E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA4_smt` reader - PAD Schmit control"]
pub type PA4SmtR = crate::BitReader;
#[doc = "Field `pA4_smt` writer - PAD Schmit control"]
pub type PA4SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA4_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA4SrR = crate::BitReader;
#[doc = "Field `pA4_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA4SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA4_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA4E3R = crate::BitReader;
#[doc = "Field `pA4_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA4E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA4_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA4DisR = crate::BitReader;
#[doc = "Field `pA4_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA4DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a4_sel(&self) -> PA4SelR {
        PA4SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a4_ie(&self) -> PA4IeR {
        PA4IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a4_pupdc(&self) -> PA4PupdcR {
        PA4PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a4_e2(&self) -> PA4E2R {
        PA4E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a4_smt(&self) -> PA4SmtR {
        PA4SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a4_sr(&self) -> PA4SrR {
        PA4SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a4_e3(&self) -> PA4E3R {
        PA4E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a4_dis(&self) -> PA4DisR {
        PA4DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a4_sel(&mut self) -> PA4SelW<'_, RegPa4Spec> {
        PA4SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a4_ie(&mut self) -> PA4IeW<'_, RegPa4Spec> {
        PA4IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a4_pupdc(&mut self) -> PA4PupdcW<'_, RegPa4Spec> {
        PA4PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a4_e2(&mut self) -> PA4E2W<'_, RegPa4Spec> {
        PA4E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a4_smt(&mut self) -> PA4SmtW<'_, RegPa4Spec> {
        PA4SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a4_sr(&mut self) -> PA4SrW<'_, RegPa4Spec> {
        PA4SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a4_e3(&mut self) -> PA4E3W<'_, RegPa4Spec> {
        PA4E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a4_dis(&mut self) -> PA4DisW<'_, RegPa4Spec> {
        PA4DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa4Spec;
impl crate::RegisterSpec for RegPa4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa4::R`](R) reader structure"]
impl crate::Readable for RegPa4Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa4::W`](W) writer structure"]
impl crate::Writable for RegPa4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA4 to value 0x1900"]
impl crate::Resettable for RegPa4Spec {
    const RESET_VALUE: u32 = 0x1900;
}
