#[doc = "Register `REG_PC10` reader"]
pub type R = crate::R<RegPc10Spec>;
#[doc = "Register `REG_PC10` writer"]
pub type W = crate::W<RegPc10Spec>;
#[doc = "Field `pC10_sel` reader - PAD pinmux function ID selection"]
pub type PC10SelR = crate::FieldReader;
#[doc = "Field `pC10_sel` writer - PAD pinmux function ID selection"]
pub type PC10SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC10_ie` reader - PAD input enable control"]
pub type PC10IeR = crate::BitReader;
#[doc = "Field `pC10_ie` writer - PAD input enable control"]
pub type PC10IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC10_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC10PupdcR = crate::BitReader;
#[doc = "Field `pC10_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC10PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC10_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC10E2R = crate::BitReader;
#[doc = "Field `pC10_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC10E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC10_smt` reader - PAD Schmit control"]
pub type PC10SmtR = crate::BitReader;
#[doc = "Field `pC10_smt` writer - PAD Schmit control"]
pub type PC10SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC10_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC10SrR = crate::BitReader;
#[doc = "Field `pC10_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC10SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC10_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC10E3R = crate::BitReader;
#[doc = "Field `pC10_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC10E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC10_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC10DisR = crate::BitReader;
#[doc = "Field `pC10_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC10DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c10_sel(&self) -> PC10SelR {
        PC10SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c10_ie(&self) -> PC10IeR {
        PC10IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c10_pupdc(&self) -> PC10PupdcR {
        PC10PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c10_e2(&self) -> PC10E2R {
        PC10E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c10_smt(&self) -> PC10SmtR {
        PC10SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c10_sr(&self) -> PC10SrR {
        PC10SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c10_e3(&self) -> PC10E3R {
        PC10E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c10_dis(&self) -> PC10DisR {
        PC10DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c10_sel(&mut self) -> PC10SelW<'_, RegPc10Spec> {
        PC10SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c10_ie(&mut self) -> PC10IeW<'_, RegPc10Spec> {
        PC10IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c10_pupdc(&mut self) -> PC10PupdcW<'_, RegPc10Spec> {
        PC10PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c10_e2(&mut self) -> PC10E2W<'_, RegPc10Spec> {
        PC10E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c10_smt(&mut self) -> PC10SmtW<'_, RegPc10Spec> {
        PC10SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c10_sr(&mut self) -> PC10SrW<'_, RegPc10Spec> {
        PC10SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c10_e3(&mut self) -> PC10E3W<'_, RegPc10Spec> {
        PC10E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c10_dis(&mut self) -> PC10DisW<'_, RegPc10Spec> {
        PC10DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc10Spec;
impl crate::RegisterSpec for RegPc10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc10::R`](R) reader structure"]
impl crate::Readable for RegPc10Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc10::W`](W) writer structure"]
impl crate::Writable for RegPc10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC10 to value 0x1900"]
impl crate::Resettable for RegPc10Spec {
    const RESET_VALUE: u32 = 0x1900;
}
