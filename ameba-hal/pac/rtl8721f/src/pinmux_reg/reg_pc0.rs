#[doc = "Register `REG_PC0` reader"]
pub type R = crate::R<RegPc0Spec>;
#[doc = "Register `REG_PC0` writer"]
pub type W = crate::W<RegPc0Spec>;
#[doc = "Field `pC0_sel` reader - PAD pinmux function ID selection"]
pub type PC0SelR = crate::FieldReader;
#[doc = "Field `pC0_sel` writer - PAD pinmux function ID selection"]
pub type PC0SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC0_ie` reader - PAD input enable control"]
pub type PC0IeR = crate::BitReader;
#[doc = "Field `pC0_ie` writer - PAD input enable control"]
pub type PC0IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC0_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC0PupdcR = crate::BitReader;
#[doc = "Field `pC0_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC0PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC0_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC0E2R = crate::BitReader;
#[doc = "Field `pC0_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC0E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC0_smt` reader - PAD Schmit control"]
pub type PC0SmtR = crate::BitReader;
#[doc = "Field `pC0_smt` writer - PAD Schmit control"]
pub type PC0SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC0_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC0SrR = crate::BitReader;
#[doc = "Field `pC0_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC0SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC0_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC0E3R = crate::BitReader;
#[doc = "Field `pC0_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC0E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC0_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC0DisR = crate::BitReader;
#[doc = "Field `pC0_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC0DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c0_sel(&self) -> PC0SelR {
        PC0SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c0_ie(&self) -> PC0IeR {
        PC0IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c0_pupdc(&self) -> PC0PupdcR {
        PC0PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c0_e2(&self) -> PC0E2R {
        PC0E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c0_smt(&self) -> PC0SmtR {
        PC0SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c0_sr(&self) -> PC0SrR {
        PC0SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c0_e3(&self) -> PC0E3R {
        PC0E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c0_dis(&self) -> PC0DisR {
        PC0DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c0_sel(&mut self) -> PC0SelW<'_, RegPc0Spec> {
        PC0SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c0_ie(&mut self) -> PC0IeW<'_, RegPc0Spec> {
        PC0IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c0_pupdc(&mut self) -> PC0PupdcW<'_, RegPc0Spec> {
        PC0PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c0_e2(&mut self) -> PC0E2W<'_, RegPc0Spec> {
        PC0E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c0_smt(&mut self) -> PC0SmtW<'_, RegPc0Spec> {
        PC0SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c0_sr(&mut self) -> PC0SrW<'_, RegPc0Spec> {
        PC0SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c0_e3(&mut self) -> PC0E3W<'_, RegPc0Spec> {
        PC0E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c0_dis(&mut self) -> PC0DisW<'_, RegPc0Spec> {
        PC0DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc0Spec;
impl crate::RegisterSpec for RegPc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc0::R`](R) reader structure"]
impl crate::Readable for RegPc0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc0::W`](W) writer structure"]
impl crate::Writable for RegPc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC0 to value 0x1900"]
impl crate::Resettable for RegPc0Spec {
    const RESET_VALUE: u32 = 0x1900;
}
