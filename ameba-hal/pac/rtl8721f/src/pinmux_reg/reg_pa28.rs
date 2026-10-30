#[doc = "Register `REG_PA28` reader"]
pub type R = crate::R<RegPa28Spec>;
#[doc = "Register `REG_PA28` writer"]
pub type W = crate::W<RegPa28Spec>;
#[doc = "Field `pA28_sel` reader - PAD pinmux function ID selection"]
pub type PA28SelR = crate::FieldReader;
#[doc = "Field `pA28_sel` writer - PAD pinmux function ID selection"]
pub type PA28SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pA28_ie` reader - PAD input enable control"]
pub type PA28IeR = crate::BitReader;
#[doc = "Field `pA28_ie` writer - PAD input enable control"]
pub type PA28IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA28_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA28PupdcR = crate::BitReader;
#[doc = "Field `pA28_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PA28PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA28_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA28E2R = crate::BitReader;
#[doc = "Field `pA28_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA28E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA28_smt` reader - PAD Schmit control"]
pub type PA28SmtR = crate::BitReader;
#[doc = "Field `pA28_smt` writer - PAD Schmit control"]
pub type PA28SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA28_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PA28SrR = crate::BitReader;
#[doc = "Field `pA28_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PA28SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA28_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA28E3R = crate::BitReader;
#[doc = "Field `pA28_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PA28E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pA28_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA28DisR = crate::BitReader;
#[doc = "Field `pA28_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PA28DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a28_sel(&self) -> PA28SelR {
        PA28SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a28_ie(&self) -> PA28IeR {
        PA28IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a28_pupdc(&self) -> PA28PupdcR {
        PA28PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a28_e2(&self) -> PA28E2R {
        PA28E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a28_smt(&self) -> PA28SmtR {
        PA28SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a28_sr(&self) -> PA28SrR {
        PA28SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a28_e3(&self) -> PA28E3R {
        PA28E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a28_dis(&self) -> PA28DisR {
        PA28DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_a28_sel(&mut self) -> PA28SelW<'_, RegPa28Spec> {
        PA28SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_a28_ie(&mut self) -> PA28IeW<'_, RegPa28Spec> {
        PA28IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_a28_pupdc(&mut self) -> PA28PupdcW<'_, RegPa28Spec> {
        PA28PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a28_e2(&mut self) -> PA28E2W<'_, RegPa28Spec> {
        PA28E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_a28_smt(&mut self) -> PA28SmtW<'_, RegPa28Spec> {
        PA28SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_a28_sr(&mut self) -> PA28SrW<'_, RegPa28Spec> {
        PA28SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_a28_e3(&mut self) -> PA28E3W<'_, RegPa28Spec> {
        PA28E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_a28_dis(&mut self) -> PA28DisW<'_, RegPa28Spec> {
        PA28DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pa28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pa28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPa28Spec;
impl crate::RegisterSpec for RegPa28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pa28::R`](R) reader structure"]
impl crate::Readable for RegPa28Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pa28::W`](W) writer structure"]
impl crate::Writable for RegPa28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PA28 to value 0x1900"]
impl crate::Resettable for RegPa28Spec {
    const RESET_VALUE: u32 = 0x1900;
}
