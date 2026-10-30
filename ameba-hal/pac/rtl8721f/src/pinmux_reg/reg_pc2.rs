#[doc = "Register `REG_PC2` reader"]
pub type R = crate::R<RegPc2Spec>;
#[doc = "Register `REG_PC2` writer"]
pub type W = crate::W<RegPc2Spec>;
#[doc = "Field `pC2_sel` reader - PAD pinmux function ID selection"]
pub type PC2SelR = crate::FieldReader;
#[doc = "Field `pC2_sel` writer - PAD pinmux function ID selection"]
pub type PC2SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC2_ie` reader - PAD input enable control"]
pub type PC2IeR = crate::BitReader;
#[doc = "Field `pC2_ie` writer - PAD input enable control"]
pub type PC2IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC2_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC2PupdcR = crate::BitReader;
#[doc = "Field `pC2_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC2PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC2_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC2E2R = crate::BitReader;
#[doc = "Field `pC2_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC2E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC2_smt` reader - PAD Schmit control"]
pub type PC2SmtR = crate::BitReader;
#[doc = "Field `pC2_smt` writer - PAD Schmit control"]
pub type PC2SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC2_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC2SrR = crate::BitReader;
#[doc = "Field `pC2_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC2SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC2_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC2E3R = crate::BitReader;
#[doc = "Field `pC2_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC2E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC2_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC2DisR = crate::BitReader;
#[doc = "Field `pC2_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC2DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c2_sel(&self) -> PC2SelR {
        PC2SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c2_ie(&self) -> PC2IeR {
        PC2IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c2_pupdc(&self) -> PC2PupdcR {
        PC2PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c2_e2(&self) -> PC2E2R {
        PC2E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c2_smt(&self) -> PC2SmtR {
        PC2SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c2_sr(&self) -> PC2SrR {
        PC2SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c2_e3(&self) -> PC2E3R {
        PC2E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c2_dis(&self) -> PC2DisR {
        PC2DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c2_sel(&mut self) -> PC2SelW<'_, RegPc2Spec> {
        PC2SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c2_ie(&mut self) -> PC2IeW<'_, RegPc2Spec> {
        PC2IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c2_pupdc(&mut self) -> PC2PupdcW<'_, RegPc2Spec> {
        PC2PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c2_e2(&mut self) -> PC2E2W<'_, RegPc2Spec> {
        PC2E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c2_smt(&mut self) -> PC2SmtW<'_, RegPc2Spec> {
        PC2SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c2_sr(&mut self) -> PC2SrW<'_, RegPc2Spec> {
        PC2SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c2_e3(&mut self) -> PC2E3W<'_, RegPc2Spec> {
        PC2E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c2_dis(&mut self) -> PC2DisW<'_, RegPc2Spec> {
        PC2DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc2Spec;
impl crate::RegisterSpec for RegPc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc2::R`](R) reader structure"]
impl crate::Readable for RegPc2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc2::W`](W) writer structure"]
impl crate::Writable for RegPc2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC2 to value 0x1900"]
impl crate::Resettable for RegPc2Spec {
    const RESET_VALUE: u32 = 0x1900;
}
