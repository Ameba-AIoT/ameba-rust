#[doc = "Register `REG_PC11` reader"]
pub type R = crate::R<RegPc11Spec>;
#[doc = "Register `REG_PC11` writer"]
pub type W = crate::W<RegPc11Spec>;
#[doc = "Field `pC11_sel` reader - PAD pinmux function ID selection"]
pub type PC11SelR = crate::FieldReader;
#[doc = "Field `pC11_sel` writer - PAD pinmux function ID selection"]
pub type PC11SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC11_ie` reader - PAD input enable control"]
pub type PC11IeR = crate::BitReader;
#[doc = "Field `pC11_ie` writer - PAD input enable control"]
pub type PC11IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC11_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC11PupdcR = crate::BitReader;
#[doc = "Field `pC11_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC11PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC11_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC11E2R = crate::BitReader;
#[doc = "Field `pC11_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC11E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC11_smt` reader - PAD Schmit control"]
pub type PC11SmtR = crate::BitReader;
#[doc = "Field `pC11_smt` writer - PAD Schmit control"]
pub type PC11SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC11_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC11SrR = crate::BitReader;
#[doc = "Field `pC11_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC11SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC11_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC11E3R = crate::BitReader;
#[doc = "Field `pC11_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC11E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC11_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC11DisR = crate::BitReader;
#[doc = "Field `pC11_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC11DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c11_sel(&self) -> PC11SelR {
        PC11SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c11_ie(&self) -> PC11IeR {
        PC11IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c11_pupdc(&self) -> PC11PupdcR {
        PC11PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c11_e2(&self) -> PC11E2R {
        PC11E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c11_smt(&self) -> PC11SmtR {
        PC11SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c11_sr(&self) -> PC11SrR {
        PC11SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c11_e3(&self) -> PC11E3R {
        PC11E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c11_dis(&self) -> PC11DisR {
        PC11DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c11_sel(&mut self) -> PC11SelW<'_, RegPc11Spec> {
        PC11SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c11_ie(&mut self) -> PC11IeW<'_, RegPc11Spec> {
        PC11IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c11_pupdc(&mut self) -> PC11PupdcW<'_, RegPc11Spec> {
        PC11PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c11_e2(&mut self) -> PC11E2W<'_, RegPc11Spec> {
        PC11E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c11_smt(&mut self) -> PC11SmtW<'_, RegPc11Spec> {
        PC11SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c11_sr(&mut self) -> PC11SrW<'_, RegPc11Spec> {
        PC11SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c11_e3(&mut self) -> PC11E3W<'_, RegPc11Spec> {
        PC11E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c11_dis(&mut self) -> PC11DisW<'_, RegPc11Spec> {
        PC11DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc11Spec;
impl crate::RegisterSpec for RegPc11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc11::R`](R) reader structure"]
impl crate::Readable for RegPc11Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc11::W`](W) writer structure"]
impl crate::Writable for RegPc11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC11 to value 0x1900"]
impl crate::Resettable for RegPc11Spec {
    const RESET_VALUE: u32 = 0x1900;
}
