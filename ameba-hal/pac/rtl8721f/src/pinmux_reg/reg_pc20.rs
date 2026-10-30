#[doc = "Register `REG_PC20` reader"]
pub type R = crate::R<RegPc20Spec>;
#[doc = "Register `REG_PC20` writer"]
pub type W = crate::W<RegPc20Spec>;
#[doc = "Field `pC20_sel` reader - PAD pinmux function ID selection"]
pub type PC20SelR = crate::FieldReader;
#[doc = "Field `pC20_sel` writer - PAD pinmux function ID selection"]
pub type PC20SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC20_ie` reader - PAD input enable control"]
pub type PC20IeR = crate::BitReader;
#[doc = "Field `pC20_ie` writer - PAD input enable control"]
pub type PC20IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC20_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC20PupdcR = crate::BitReader;
#[doc = "Field `pC20_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC20PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC20_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC20E2R = crate::BitReader;
#[doc = "Field `pC20_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC20E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC20_smt` reader - PAD Schmit control"]
pub type PC20SmtR = crate::BitReader;
#[doc = "Field `pC20_smt` writer - PAD Schmit control"]
pub type PC20SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC20_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC20SrR = crate::BitReader;
#[doc = "Field `pC20_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC20SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC20_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC20E3R = crate::BitReader;
#[doc = "Field `pC20_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC20E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC20_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC20DisR = crate::BitReader;
#[doc = "Field `pC20_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC20DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c20_sel(&self) -> PC20SelR {
        PC20SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c20_ie(&self) -> PC20IeR {
        PC20IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c20_pupdc(&self) -> PC20PupdcR {
        PC20PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c20_e2(&self) -> PC20E2R {
        PC20E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c20_smt(&self) -> PC20SmtR {
        PC20SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c20_sr(&self) -> PC20SrR {
        PC20SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c20_e3(&self) -> PC20E3R {
        PC20E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c20_dis(&self) -> PC20DisR {
        PC20DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c20_sel(&mut self) -> PC20SelW<'_, RegPc20Spec> {
        PC20SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c20_ie(&mut self) -> PC20IeW<'_, RegPc20Spec> {
        PC20IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c20_pupdc(&mut self) -> PC20PupdcW<'_, RegPc20Spec> {
        PC20PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c20_e2(&mut self) -> PC20E2W<'_, RegPc20Spec> {
        PC20E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c20_smt(&mut self) -> PC20SmtW<'_, RegPc20Spec> {
        PC20SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c20_sr(&mut self) -> PC20SrW<'_, RegPc20Spec> {
        PC20SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c20_e3(&mut self) -> PC20E3W<'_, RegPc20Spec> {
        PC20E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c20_dis(&mut self) -> PC20DisW<'_, RegPc20Spec> {
        PC20DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc20Spec;
impl crate::RegisterSpec for RegPc20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc20::R`](R) reader structure"]
impl crate::Readable for RegPc20Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc20::W`](W) writer structure"]
impl crate::Writable for RegPc20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC20 to value 0x1900"]
impl crate::Resettable for RegPc20Spec {
    const RESET_VALUE: u32 = 0x1900;
}
