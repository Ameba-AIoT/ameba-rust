#[doc = "Register `REG_PC17` reader"]
pub type R = crate::R<RegPc17Spec>;
#[doc = "Register `REG_PC17` writer"]
pub type W = crate::W<RegPc17Spec>;
#[doc = "Field `pC17_sel` reader - PAD pinmux function ID selection"]
pub type PC17SelR = crate::FieldReader;
#[doc = "Field `pC17_sel` writer - PAD pinmux function ID selection"]
pub type PC17SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC17_ie` reader - PAD input enable control"]
pub type PC17IeR = crate::BitReader;
#[doc = "Field `pC17_ie` writer - PAD input enable control"]
pub type PC17IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC17_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC17PupdcR = crate::BitReader;
#[doc = "Field `pC17_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC17PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC17_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC17E2R = crate::BitReader;
#[doc = "Field `pC17_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC17E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC17_smt` reader - PAD Schmit control"]
pub type PC17SmtR = crate::BitReader;
#[doc = "Field `pC17_smt` writer - PAD Schmit control"]
pub type PC17SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC17_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC17SrR = crate::BitReader;
#[doc = "Field `pC17_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC17SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC17_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC17E3R = crate::BitReader;
#[doc = "Field `pC17_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC17E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC17_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC17DisR = crate::BitReader;
#[doc = "Field `pC17_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC17DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c17_sel(&self) -> PC17SelR {
        PC17SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c17_ie(&self) -> PC17IeR {
        PC17IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c17_pupdc(&self) -> PC17PupdcR {
        PC17PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c17_e2(&self) -> PC17E2R {
        PC17E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c17_smt(&self) -> PC17SmtR {
        PC17SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c17_sr(&self) -> PC17SrR {
        PC17SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c17_e3(&self) -> PC17E3R {
        PC17E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c17_dis(&self) -> PC17DisR {
        PC17DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c17_sel(&mut self) -> PC17SelW<'_, RegPc17Spec> {
        PC17SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c17_ie(&mut self) -> PC17IeW<'_, RegPc17Spec> {
        PC17IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c17_pupdc(&mut self) -> PC17PupdcW<'_, RegPc17Spec> {
        PC17PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c17_e2(&mut self) -> PC17E2W<'_, RegPc17Spec> {
        PC17E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c17_smt(&mut self) -> PC17SmtW<'_, RegPc17Spec> {
        PC17SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c17_sr(&mut self) -> PC17SrW<'_, RegPc17Spec> {
        PC17SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c17_e3(&mut self) -> PC17E3W<'_, RegPc17Spec> {
        PC17E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c17_dis(&mut self) -> PC17DisW<'_, RegPc17Spec> {
        PC17DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc17Spec;
impl crate::RegisterSpec for RegPc17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc17::R`](R) reader structure"]
impl crate::Readable for RegPc17Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc17::W`](W) writer structure"]
impl crate::Writable for RegPc17Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC17 to value 0x1900"]
impl crate::Resettable for RegPc17Spec {
    const RESET_VALUE: u32 = 0x1900;
}
