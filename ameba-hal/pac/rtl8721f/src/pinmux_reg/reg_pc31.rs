#[doc = "Register `REG_PC31` reader"]
pub type R = crate::R<RegPc31Spec>;
#[doc = "Register `REG_PC31` writer"]
pub type W = crate::W<RegPc31Spec>;
#[doc = "Field `pC31_sel` reader - PAD pinmux function ID selection"]
pub type PC31SelR = crate::FieldReader;
#[doc = "Field `pC31_sel` writer - PAD pinmux function ID selection"]
pub type PC31SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC31_ie` reader - PAD input enable control"]
pub type PC31IeR = crate::BitReader;
#[doc = "Field `pC31_ie` writer - PAD input enable control"]
pub type PC31IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC31_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC31PupdcR = crate::BitReader;
#[doc = "Field `pC31_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC31PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC31_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC31E2R = crate::BitReader;
#[doc = "Field `pC31_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC31E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC31_smt` reader - PAD Schmit control"]
pub type PC31SmtR = crate::BitReader;
#[doc = "Field `pC31_smt` writer - PAD Schmit control"]
pub type PC31SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC31_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC31SrR = crate::BitReader;
#[doc = "Field `pC31_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC31SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC31_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC31E3R = crate::BitReader;
#[doc = "Field `pC31_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC31E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC31_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC31DisR = crate::BitReader;
#[doc = "Field `pC31_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC31DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c31_sel(&self) -> PC31SelR {
        PC31SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c31_ie(&self) -> PC31IeR {
        PC31IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c31_pupdc(&self) -> PC31PupdcR {
        PC31PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c31_e2(&self) -> PC31E2R {
        PC31E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c31_smt(&self) -> PC31SmtR {
        PC31SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c31_sr(&self) -> PC31SrR {
        PC31SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c31_e3(&self) -> PC31E3R {
        PC31E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c31_dis(&self) -> PC31DisR {
        PC31DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c31_sel(&mut self) -> PC31SelW<'_, RegPc31Spec> {
        PC31SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c31_ie(&mut self) -> PC31IeW<'_, RegPc31Spec> {
        PC31IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c31_pupdc(&mut self) -> PC31PupdcW<'_, RegPc31Spec> {
        PC31PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c31_e2(&mut self) -> PC31E2W<'_, RegPc31Spec> {
        PC31E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c31_smt(&mut self) -> PC31SmtW<'_, RegPc31Spec> {
        PC31SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c31_sr(&mut self) -> PC31SrW<'_, RegPc31Spec> {
        PC31SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c31_e3(&mut self) -> PC31E3W<'_, RegPc31Spec> {
        PC31E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c31_dis(&mut self) -> PC31DisW<'_, RegPc31Spec> {
        PC31DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc31Spec;
impl crate::RegisterSpec for RegPc31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc31::R`](R) reader structure"]
impl crate::Readable for RegPc31Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc31::W`](W) writer structure"]
impl crate::Writable for RegPc31Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC31 to value 0x1900"]
impl crate::Resettable for RegPc31Spec {
    const RESET_VALUE: u32 = 0x1900;
}
