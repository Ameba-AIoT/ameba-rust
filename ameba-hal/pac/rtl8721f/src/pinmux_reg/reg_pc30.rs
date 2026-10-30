#[doc = "Register `REG_PC30` reader"]
pub type R = crate::R<RegPc30Spec>;
#[doc = "Register `REG_PC30` writer"]
pub type W = crate::W<RegPc30Spec>;
#[doc = "Field `pC30_sel` reader - PAD pinmux function ID selection"]
pub type PC30SelR = crate::FieldReader;
#[doc = "Field `pC30_sel` writer - PAD pinmux function ID selection"]
pub type PC30SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC30_ie` reader - PAD input enable control"]
pub type PC30IeR = crate::BitReader;
#[doc = "Field `pC30_ie` writer - PAD input enable control"]
pub type PC30IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC30_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC30PupdcR = crate::BitReader;
#[doc = "Field `pC30_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC30PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC30_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC30E2R = crate::BitReader;
#[doc = "Field `pC30_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC30E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC30_smt` reader - PAD Schmit control"]
pub type PC30SmtR = crate::BitReader;
#[doc = "Field `pC30_smt` writer - PAD Schmit control"]
pub type PC30SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC30_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC30SrR = crate::BitReader;
#[doc = "Field `pC30_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC30SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC30_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC30E3R = crate::BitReader;
#[doc = "Field `pC30_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC30E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC30_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC30DisR = crate::BitReader;
#[doc = "Field `pC30_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC30DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c30_sel(&self) -> PC30SelR {
        PC30SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c30_ie(&self) -> PC30IeR {
        PC30IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c30_pupdc(&self) -> PC30PupdcR {
        PC30PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c30_e2(&self) -> PC30E2R {
        PC30E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c30_smt(&self) -> PC30SmtR {
        PC30SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c30_sr(&self) -> PC30SrR {
        PC30SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c30_e3(&self) -> PC30E3R {
        PC30E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c30_dis(&self) -> PC30DisR {
        PC30DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c30_sel(&mut self) -> PC30SelW<'_, RegPc30Spec> {
        PC30SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c30_ie(&mut self) -> PC30IeW<'_, RegPc30Spec> {
        PC30IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c30_pupdc(&mut self) -> PC30PupdcW<'_, RegPc30Spec> {
        PC30PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c30_e2(&mut self) -> PC30E2W<'_, RegPc30Spec> {
        PC30E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c30_smt(&mut self) -> PC30SmtW<'_, RegPc30Spec> {
        PC30SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c30_sr(&mut self) -> PC30SrW<'_, RegPc30Spec> {
        PC30SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c30_e3(&mut self) -> PC30E3W<'_, RegPc30Spec> {
        PC30E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c30_dis(&mut self) -> PC30DisW<'_, RegPc30Spec> {
        PC30DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc30Spec;
impl crate::RegisterSpec for RegPc30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc30::R`](R) reader structure"]
impl crate::Readable for RegPc30Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc30::W`](W) writer structure"]
impl crate::Writable for RegPc30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC30 to value 0x1900"]
impl crate::Resettable for RegPc30Spec {
    const RESET_VALUE: u32 = 0x1900;
}
