#[doc = "Register `REG_PC28` reader"]
pub type R = crate::R<RegPc28Spec>;
#[doc = "Register `REG_PC28` writer"]
pub type W = crate::W<RegPc28Spec>;
#[doc = "Field `pC28_sel` reader - PAD pinmux function ID selection"]
pub type PC28SelR = crate::FieldReader;
#[doc = "Field `pC28_sel` writer - PAD pinmux function ID selection"]
pub type PC28SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC28_ie` reader - PAD input enable control"]
pub type PC28IeR = crate::BitReader;
#[doc = "Field `pC28_ie` writer - PAD input enable control"]
pub type PC28IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC28_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC28PupdcR = crate::BitReader;
#[doc = "Field `pC28_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC28PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC28_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC28E2R = crate::BitReader;
#[doc = "Field `pC28_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC28E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC28_smt` reader - PAD Schmit control"]
pub type PC28SmtR = crate::BitReader;
#[doc = "Field `pC28_smt` writer - PAD Schmit control"]
pub type PC28SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC28_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC28SrR = crate::BitReader;
#[doc = "Field `pC28_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC28SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC28_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC28E3R = crate::BitReader;
#[doc = "Field `pC28_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC28E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC28_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC28DisR = crate::BitReader;
#[doc = "Field `pC28_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC28DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c28_sel(&self) -> PC28SelR {
        PC28SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c28_ie(&self) -> PC28IeR {
        PC28IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c28_pupdc(&self) -> PC28PupdcR {
        PC28PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c28_e2(&self) -> PC28E2R {
        PC28E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c28_smt(&self) -> PC28SmtR {
        PC28SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c28_sr(&self) -> PC28SrR {
        PC28SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c28_e3(&self) -> PC28E3R {
        PC28E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c28_dis(&self) -> PC28DisR {
        PC28DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c28_sel(&mut self) -> PC28SelW<'_, RegPc28Spec> {
        PC28SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c28_ie(&mut self) -> PC28IeW<'_, RegPc28Spec> {
        PC28IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c28_pupdc(&mut self) -> PC28PupdcW<'_, RegPc28Spec> {
        PC28PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c28_e2(&mut self) -> PC28E2W<'_, RegPc28Spec> {
        PC28E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c28_smt(&mut self) -> PC28SmtW<'_, RegPc28Spec> {
        PC28SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c28_sr(&mut self) -> PC28SrW<'_, RegPc28Spec> {
        PC28SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c28_e3(&mut self) -> PC28E3W<'_, RegPc28Spec> {
        PC28E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c28_dis(&mut self) -> PC28DisW<'_, RegPc28Spec> {
        PC28DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc28Spec;
impl crate::RegisterSpec for RegPc28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc28::R`](R) reader structure"]
impl crate::Readable for RegPc28Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc28::W`](W) writer structure"]
impl crate::Writable for RegPc28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC28 to value 0x1900"]
impl crate::Resettable for RegPc28Spec {
    const RESET_VALUE: u32 = 0x1900;
}
