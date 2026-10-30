#[doc = "Register `REG_PC6` reader"]
pub type R = crate::R<RegPc6Spec>;
#[doc = "Register `REG_PC6` writer"]
pub type W = crate::W<RegPc6Spec>;
#[doc = "Field `pC6_sel` reader - PAD pinmux function ID selection"]
pub type PC6SelR = crate::FieldReader;
#[doc = "Field `pC6_sel` writer - PAD pinmux function ID selection"]
pub type PC6SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC6_ie` reader - PAD input enable control"]
pub type PC6IeR = crate::BitReader;
#[doc = "Field `pC6_ie` writer - PAD input enable control"]
pub type PC6IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC6_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC6PupdcR = crate::BitReader;
#[doc = "Field `pC6_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC6PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC6_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC6E2R = crate::BitReader;
#[doc = "Field `pC6_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC6E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC6_smt` reader - PAD Schmit control"]
pub type PC6SmtR = crate::BitReader;
#[doc = "Field `pC6_smt` writer - PAD Schmit control"]
pub type PC6SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC6_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC6SrR = crate::BitReader;
#[doc = "Field `pC6_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC6SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC6_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC6E3R = crate::BitReader;
#[doc = "Field `pC6_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC6E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC6_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC6DisR = crate::BitReader;
#[doc = "Field `pC6_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC6DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c6_sel(&self) -> PC6SelR {
        PC6SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c6_ie(&self) -> PC6IeR {
        PC6IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c6_pupdc(&self) -> PC6PupdcR {
        PC6PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c6_e2(&self) -> PC6E2R {
        PC6E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c6_smt(&self) -> PC6SmtR {
        PC6SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c6_sr(&self) -> PC6SrR {
        PC6SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c6_e3(&self) -> PC6E3R {
        PC6E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c6_dis(&self) -> PC6DisR {
        PC6DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c6_sel(&mut self) -> PC6SelW<'_, RegPc6Spec> {
        PC6SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c6_ie(&mut self) -> PC6IeW<'_, RegPc6Spec> {
        PC6IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c6_pupdc(&mut self) -> PC6PupdcW<'_, RegPc6Spec> {
        PC6PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c6_e2(&mut self) -> PC6E2W<'_, RegPc6Spec> {
        PC6E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c6_smt(&mut self) -> PC6SmtW<'_, RegPc6Spec> {
        PC6SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c6_sr(&mut self) -> PC6SrW<'_, RegPc6Spec> {
        PC6SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c6_e3(&mut self) -> PC6E3W<'_, RegPc6Spec> {
        PC6E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c6_dis(&mut self) -> PC6DisW<'_, RegPc6Spec> {
        PC6DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc6Spec;
impl crate::RegisterSpec for RegPc6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc6::R`](R) reader structure"]
impl crate::Readable for RegPc6Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc6::W`](W) writer structure"]
impl crate::Writable for RegPc6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC6 to value 0x1900"]
impl crate::Resettable for RegPc6Spec {
    const RESET_VALUE: u32 = 0x1900;
}
