#[doc = "Register `REG_PC1` reader"]
pub type R = crate::R<RegPc1Spec>;
#[doc = "Register `REG_PC1` writer"]
pub type W = crate::W<RegPc1Spec>;
#[doc = "Field `pC1_sel` reader - PAD pinmux function ID selection"]
pub type PC1SelR = crate::FieldReader;
#[doc = "Field `pC1_sel` writer - PAD pinmux function ID selection"]
pub type PC1SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC1_ie` reader - PAD input enable control"]
pub type PC1IeR = crate::BitReader;
#[doc = "Field `pC1_ie` writer - PAD input enable control"]
pub type PC1IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC1_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC1PupdcR = crate::BitReader;
#[doc = "Field `pC1_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC1PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC1_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC1E2R = crate::BitReader;
#[doc = "Field `pC1_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC1E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC1_smt` reader - PAD Schmit control"]
pub type PC1SmtR = crate::BitReader;
#[doc = "Field `pC1_smt` writer - PAD Schmit control"]
pub type PC1SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC1_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC1SrR = crate::BitReader;
#[doc = "Field `pC1_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC1SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC1_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC1E3R = crate::BitReader;
#[doc = "Field `pC1_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC1E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC1_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC1DisR = crate::BitReader;
#[doc = "Field `pC1_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC1DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c1_sel(&self) -> PC1SelR {
        PC1SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c1_ie(&self) -> PC1IeR {
        PC1IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c1_pupdc(&self) -> PC1PupdcR {
        PC1PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c1_e2(&self) -> PC1E2R {
        PC1E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c1_smt(&self) -> PC1SmtR {
        PC1SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c1_sr(&self) -> PC1SrR {
        PC1SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c1_e3(&self) -> PC1E3R {
        PC1E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c1_dis(&self) -> PC1DisR {
        PC1DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c1_sel(&mut self) -> PC1SelW<'_, RegPc1Spec> {
        PC1SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c1_ie(&mut self) -> PC1IeW<'_, RegPc1Spec> {
        PC1IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c1_pupdc(&mut self) -> PC1PupdcW<'_, RegPc1Spec> {
        PC1PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c1_e2(&mut self) -> PC1E2W<'_, RegPc1Spec> {
        PC1E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c1_smt(&mut self) -> PC1SmtW<'_, RegPc1Spec> {
        PC1SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c1_sr(&mut self) -> PC1SrW<'_, RegPc1Spec> {
        PC1SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c1_e3(&mut self) -> PC1E3W<'_, RegPc1Spec> {
        PC1E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c1_dis(&mut self) -> PC1DisW<'_, RegPc1Spec> {
        PC1DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc1Spec;
impl crate::RegisterSpec for RegPc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc1::R`](R) reader structure"]
impl crate::Readable for RegPc1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc1::W`](W) writer structure"]
impl crate::Writable for RegPc1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC1 to value 0x1900"]
impl crate::Resettable for RegPc1Spec {
    const RESET_VALUE: u32 = 0x1900;
}
