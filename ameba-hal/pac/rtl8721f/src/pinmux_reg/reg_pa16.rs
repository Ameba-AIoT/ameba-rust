#[doc = "Register `REG_PA16` reader"]
pub type R = crate::R<RegPa16Spec>;
#[doc = "Register `REG_PA16` writer"]
pub type W = crate::W<RegPa16Spec>;
#[doc = "Field `pA16_sel` reader - PAD pinmux function ID selection"]
pub type PA16SelR = crate::FieldReader;
#[doc = "Field `pA16_sel` writer - PAD pinmux function ID selection"]
pub type PA16SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA16_ie` reader - PAD input enable control"]
pub type PA16IeR = crate::BitReader;
#[doc = "Field `pA16_ie` writer - PAD input enable control"]
pub type PA16IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA16_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA16PupdcR = crate::BitReader;
#[doc = "Field `pA16_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA16PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA16_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA16E2R = crate::BitReader;
#[doc = "Field `pA16_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA16E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA16_smt` reader - PAD Schmit control"]
pub type PA16SmtR = crate::BitReader;
#[doc = "Field `pA16_smt` writer - PAD Schmit control"]
pub type PA16SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA16_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA16SrR = crate::BitReader;
#[doc = "Field `pA16_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA16SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA16_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA16E3R = crate::BitReader;
#[doc = "Field `pA16_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA16E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA16_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA16DisR = crate::BitReader;
#[doc = "Field `pA16_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA16DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a16_sel(&self) -> PA16SelR {
        PA16SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a16_ie(&self) -> PA16IeR {
        PA16IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a16_pupdc(&self) -> PA16PupdcR {
        PA16PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a16_e2(&self) -> PA16E2R {
        PA16E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a16_smt(&self) -> PA16SmtR {
        PA16SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a16_sr(&self) -> PA16SrR {
        PA16SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a16_e3(&self) -> PA16E3R {
        PA16E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a16_dis(&self) -> PA16DisR {
        PA16DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a16_sel(&mut self) -> PA16SelW<'_, RegPa16Spec> {
        PA16SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a16_ie(&mut self) -> PA16IeW<'_, RegPa16Spec> {
        PA16IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a16_pupdc(&mut self) -> PA16PupdcW<'_, RegPa16Spec> {
        PA16PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a16_e2(&mut self) -> PA16E2W<'_, RegPa16Spec> {
        PA16E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a16_smt(&mut self) -> PA16SmtW<'_, RegPa16Spec> {
        PA16SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a16_sr(&mut self) -> PA16SrW<'_, RegPa16Spec> {
        PA16SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a16_e3(&mut self) -> PA16E3W<'_, RegPa16Spec> {
        PA16E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a16_dis(&mut self) -> PA16DisW<'_, RegPa16Spec> {
        PA16DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa16Spec;
impl crate::RegisterSpec for RegPa16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa16::R`](R) reader structure"]
impl crate::Readable for RegPa16Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa16::W`](W) writer structure"]
impl crate::Writable for RegPa16Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA16 to value 0x1900"]
impl crate::Resettable for RegPa16Spec {
    const RESET_VALUE: u32 = 0x1900;
}
