#[doc = "Register `REG_PA20` reader"]
pub type R = crate::R<RegPa20Spec>;
#[doc = "Register `REG_PA20` writer"]
pub type W = crate::W<RegPa20Spec>;
#[doc = "Field `pA20_sel` reader - PAD pinmux function ID selection"]
pub type PA20SelR = crate::FieldReader;
#[doc = "Field `pA20_sel` writer - PAD pinmux function ID selection"]
pub type PA20SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA20_ie` reader - PAD input enable control"]
pub type PA20IeR = crate::BitReader;
#[doc = "Field `pA20_ie` writer - PAD input enable control"]
pub type PA20IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA20_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA20PupdcR = crate::BitReader;
#[doc = "Field `pA20_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA20PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA20_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA20E2R = crate::BitReader;
#[doc = "Field `pA20_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA20E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA20_smt` reader - PAD Schmit control"]
pub type PA20SmtR = crate::BitReader;
#[doc = "Field `pA20_smt` writer - PAD Schmit control"]
pub type PA20SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA20_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA20SrR = crate::BitReader;
#[doc = "Field `pA20_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA20SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA20_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA20E3R = crate::BitReader;
#[doc = "Field `pA20_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA20E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA20_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA20DisR = crate::BitReader;
#[doc = "Field `pA20_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA20DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a20_sel(&self) -> PA20SelR {
        PA20SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a20_ie(&self) -> PA20IeR {
        PA20IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a20_pupdc(&self) -> PA20PupdcR {
        PA20PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a20_e2(&self) -> PA20E2R {
        PA20E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a20_smt(&self) -> PA20SmtR {
        PA20SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a20_sr(&self) -> PA20SrR {
        PA20SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a20_e3(&self) -> PA20E3R {
        PA20E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a20_dis(&self) -> PA20DisR {
        PA20DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a20_sel(&mut self) -> PA20SelW<'_, RegPa20Spec> {
        PA20SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a20_ie(&mut self) -> PA20IeW<'_, RegPa20Spec> {
        PA20IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a20_pupdc(&mut self) -> PA20PupdcW<'_, RegPa20Spec> {
        PA20PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a20_e2(&mut self) -> PA20E2W<'_, RegPa20Spec> {
        PA20E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a20_smt(&mut self) -> PA20SmtW<'_, RegPa20Spec> {
        PA20SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a20_sr(&mut self) -> PA20SrW<'_, RegPa20Spec> {
        PA20SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a20_e3(&mut self) -> PA20E3W<'_, RegPa20Spec> {
        PA20E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a20_dis(&mut self) -> PA20DisW<'_, RegPa20Spec> {
        PA20DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa20Spec;
impl crate::RegisterSpec for RegPa20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa20::R`](R) reader structure"]
impl crate::Readable for RegPa20Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa20::W`](W) writer structure"]
impl crate::Writable for RegPa20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA20 to value 0x1900"]
impl crate::Resettable for RegPa20Spec {
    const RESET_VALUE: u32 = 0x1900;
}
