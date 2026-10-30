#[doc = "Register `REG_PC16` reader"]
pub type R = crate::R<RegPc16Spec>;
#[doc = "Register `REG_PC16` writer"]
pub type W = crate::W<RegPc16Spec>;
#[doc = "Field `pC16_sel` reader - PAD pinmux function ID selection"]
pub type PC16SelR = crate::FieldReader;
#[doc = "Field `pC16_sel` writer - PAD pinmux function ID selection"]
pub type PC16SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC16_ie` reader - PAD input enable control"]
pub type PC16IeR = crate::BitReader;
#[doc = "Field `pC16_ie` writer - PAD input enable control"]
pub type PC16IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC16_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC16PupdcR = crate::BitReader;
#[doc = "Field `pC16_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC16PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC16_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC16E2R = crate::BitReader;
#[doc = "Field `pC16_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC16E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC16_smt` reader - PAD Schmit control"]
pub type PC16SmtR = crate::BitReader;
#[doc = "Field `pC16_smt` writer - PAD Schmit control"]
pub type PC16SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC16_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC16SrR = crate::BitReader;
#[doc = "Field `pC16_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC16SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC16_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC16E3R = crate::BitReader;
#[doc = "Field `pC16_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC16E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC16_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC16DisR = crate::BitReader;
#[doc = "Field `pC16_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC16DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c16_sel(&self) -> PC16SelR {
        PC16SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c16_ie(&self) -> PC16IeR {
        PC16IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c16_pupdc(&self) -> PC16PupdcR {
        PC16PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c16_e2(&self) -> PC16E2R {
        PC16E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c16_smt(&self) -> PC16SmtR {
        PC16SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c16_sr(&self) -> PC16SrR {
        PC16SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c16_e3(&self) -> PC16E3R {
        PC16E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c16_dis(&self) -> PC16DisR {
        PC16DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c16_sel(&mut self) -> PC16SelW<'_, RegPc16Spec> {
        PC16SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c16_ie(&mut self) -> PC16IeW<'_, RegPc16Spec> {
        PC16IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c16_pupdc(&mut self) -> PC16PupdcW<'_, RegPc16Spec> {
        PC16PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c16_e2(&mut self) -> PC16E2W<'_, RegPc16Spec> {
        PC16E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c16_smt(&mut self) -> PC16SmtW<'_, RegPc16Spec> {
        PC16SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c16_sr(&mut self) -> PC16SrW<'_, RegPc16Spec> {
        PC16SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c16_e3(&mut self) -> PC16E3W<'_, RegPc16Spec> {
        PC16E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c16_dis(&mut self) -> PC16DisW<'_, RegPc16Spec> {
        PC16DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc16Spec;
impl crate::RegisterSpec for RegPc16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc16::R`](R) reader structure"]
impl crate::Readable for RegPc16Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc16::W`](W) writer structure"]
impl crate::Writable for RegPc16Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC16 to value 0x1900"]
impl crate::Resettable for RegPc16Spec {
    const RESET_VALUE: u32 = 0x1900;
}
