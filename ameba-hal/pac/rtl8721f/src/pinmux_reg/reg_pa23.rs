#[doc = "Register `REG_PA23` reader"]
pub type R = crate::R<RegPa23Spec>;
#[doc = "Register `REG_PA23` writer"]
pub type W = crate::W<RegPa23Spec>;
#[doc = "Field `pA23_sel` reader - PAD pinmux function ID selection"]
pub type PA23SelR = crate::FieldReader;
#[doc = "Field `pA23_sel` writer - PAD pinmux function ID selection"]
pub type PA23SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA23_ie` reader - PAD input enable control"]
pub type PA23IeR = crate::BitReader;
#[doc = "Field `pA23_ie` writer - PAD input enable control"]
pub type PA23IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA23_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA23PupdcR = crate::BitReader;
#[doc = "Field `pA23_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA23PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA23_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA23E2R = crate::BitReader;
#[doc = "Field `pA23_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA23E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA23_smt` reader - PAD Schmit control"]
pub type PA23SmtR = crate::BitReader;
#[doc = "Field `pA23_smt` writer - PAD Schmit control"]
pub type PA23SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA23_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA23SrR = crate::BitReader;
#[doc = "Field `pA23_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA23SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA23_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA23E3R = crate::BitReader;
#[doc = "Field `pA23_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA23E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA23_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA23DisR = crate::BitReader;
#[doc = "Field `pA23_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA23DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a23_sel(&self) -> PA23SelR {
        PA23SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a23_ie(&self) -> PA23IeR {
        PA23IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a23_pupdc(&self) -> PA23PupdcR {
        PA23PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a23_e2(&self) -> PA23E2R {
        PA23E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a23_smt(&self) -> PA23SmtR {
        PA23SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a23_sr(&self) -> PA23SrR {
        PA23SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a23_e3(&self) -> PA23E3R {
        PA23E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a23_dis(&self) -> PA23DisR {
        PA23DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a23_sel(&mut self) -> PA23SelW<'_, RegPa23Spec> {
        PA23SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a23_ie(&mut self) -> PA23IeW<'_, RegPa23Spec> {
        PA23IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a23_pupdc(&mut self) -> PA23PupdcW<'_, RegPa23Spec> {
        PA23PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a23_e2(&mut self) -> PA23E2W<'_, RegPa23Spec> {
        PA23E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a23_smt(&mut self) -> PA23SmtW<'_, RegPa23Spec> {
        PA23SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a23_sr(&mut self) -> PA23SrW<'_, RegPa23Spec> {
        PA23SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a23_e3(&mut self) -> PA23E3W<'_, RegPa23Spec> {
        PA23E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a23_dis(&mut self) -> PA23DisW<'_, RegPa23Spec> {
        PA23DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa23Spec;
impl crate::RegisterSpec for RegPa23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa23::R`](R) reader structure"]
impl crate::Readable for RegPa23Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa23::W`](W) writer structure"]
impl crate::Writable for RegPa23Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA23 to value 0x1900"]
impl crate::Resettable for RegPa23Spec {
    const RESET_VALUE: u32 = 0x1900;
}
