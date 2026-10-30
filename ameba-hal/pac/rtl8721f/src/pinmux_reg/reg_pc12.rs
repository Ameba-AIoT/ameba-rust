#[doc = "Register `REG_PC12` reader"]
pub type R = crate::R<RegPc12Spec>;
#[doc = "Register `REG_PC12` writer"]
pub type W = crate::W<RegPc12Spec>;
#[doc = "Field `pC12_sel` reader - PAD pinmux function ID selection"]
pub type PC12SelR = crate::FieldReader;
#[doc = "Field `pC12_sel` writer - PAD pinmux function ID selection"]
pub type PC12SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC12_ie` reader - PAD input enable control"]
pub type PC12IeR = crate::BitReader;
#[doc = "Field `pC12_ie` writer - PAD input enable control"]
pub type PC12IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC12_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC12PupdcR = crate::BitReader;
#[doc = "Field `pC12_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC12PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC12_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC12E2R = crate::BitReader;
#[doc = "Field `pC12_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC12E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC12_smt` reader - PAD Schmit control"]
pub type PC12SmtR = crate::BitReader;
#[doc = "Field `pC12_smt` writer - PAD Schmit control"]
pub type PC12SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC12_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC12SrR = crate::BitReader;
#[doc = "Field `pC12_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC12SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC12_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC12E3R = crate::BitReader;
#[doc = "Field `pC12_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC12E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC12_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC12DisR = crate::BitReader;
#[doc = "Field `pC12_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC12DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c12_sel(&self) -> PC12SelR {
        PC12SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c12_ie(&self) -> PC12IeR {
        PC12IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c12_pupdc(&self) -> PC12PupdcR {
        PC12PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c12_e2(&self) -> PC12E2R {
        PC12E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c12_smt(&self) -> PC12SmtR {
        PC12SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c12_sr(&self) -> PC12SrR {
        PC12SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c12_e3(&self) -> PC12E3R {
        PC12E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c12_dis(&self) -> PC12DisR {
        PC12DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c12_sel(&mut self) -> PC12SelW<'_, RegPc12Spec> {
        PC12SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c12_ie(&mut self) -> PC12IeW<'_, RegPc12Spec> {
        PC12IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c12_pupdc(&mut self) -> PC12PupdcW<'_, RegPc12Spec> {
        PC12PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c12_e2(&mut self) -> PC12E2W<'_, RegPc12Spec> {
        PC12E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c12_smt(&mut self) -> PC12SmtW<'_, RegPc12Spec> {
        PC12SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c12_sr(&mut self) -> PC12SrW<'_, RegPc12Spec> {
        PC12SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c12_e3(&mut self) -> PC12E3W<'_, RegPc12Spec> {
        PC12E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c12_dis(&mut self) -> PC12DisW<'_, RegPc12Spec> {
        PC12DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc12Spec;
impl crate::RegisterSpec for RegPc12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc12::R`](R) reader structure"]
impl crate::Readable for RegPc12Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc12::W`](W) writer structure"]
impl crate::Writable for RegPc12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC12 to value 0x1900"]
impl crate::Resettable for RegPc12Spec {
    const RESET_VALUE: u32 = 0x1900;
}
