#[doc = "Register `REG_PC29` reader"]
pub type R = crate::R<RegPc29Spec>;
#[doc = "Register `REG_PC29` writer"]
pub type W = crate::W<RegPc29Spec>;
#[doc = "Field `pC29_sel` reader - PAD pinmux function ID selection"]
pub type PC29SelR = crate::FieldReader;
#[doc = "Field `pC29_sel` writer - PAD pinmux function ID selection"]
pub type PC29SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pC29_ie` reader - PAD input enable control"]
pub type PC29IeR = crate::BitReader;
#[doc = "Field `pC29_ie` writer - PAD input enable control"]
pub type PC29IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC29_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC29PupdcR = crate::BitReader;
#[doc = "Field `pC29_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PC29PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC29_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC29E2R = crate::BitReader;
#[doc = "Field `pC29_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC29E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC29_smt` reader - PAD Schmit control"]
pub type PC29SmtR = crate::BitReader;
#[doc = "Field `pC29_smt` writer - PAD Schmit control"]
pub type PC29SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC29_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PC29SrR = crate::BitReader;
#[doc = "Field `pC29_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PC29SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC29_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC29E3R = crate::BitReader;
#[doc = "Field `pC29_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PC29E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pC29_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC29DisR = crate::BitReader;
#[doc = "Field `pC29_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PC29DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c29_sel(&self) -> PC29SelR {
        PC29SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c29_ie(&self) -> PC29IeR {
        PC29IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c29_pupdc(&self) -> PC29PupdcR {
        PC29PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c29_e2(&self) -> PC29E2R {
        PC29E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c29_smt(&self) -> PC29SmtR {
        PC29SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c29_sr(&self) -> PC29SrR {
        PC29SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c29_e3(&self) -> PC29E3R {
        PC29E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c29_dis(&self) -> PC29DisR {
        PC29DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_c29_sel(&mut self) -> PC29SelW<'_, RegPc29Spec> {
        PC29SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_c29_ie(&mut self) -> PC29IeW<'_, RegPc29Spec> {
        PC29IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_c29_pupdc(&mut self) -> PC29PupdcW<'_, RegPc29Spec> {
        PC29PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c29_e2(&mut self) -> PC29E2W<'_, RegPc29Spec> {
        PC29E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_c29_smt(&mut self) -> PC29SmtW<'_, RegPc29Spec> {
        PC29SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_c29_sr(&mut self) -> PC29SrW<'_, RegPc29Spec> {
        PC29SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_c29_e3(&mut self) -> PC29E3W<'_, RegPc29Spec> {
        PC29E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_c29_dis(&mut self) -> PC29DisW<'_, RegPc29Spec> {
        PC29DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pc29::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pc29::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPc29Spec;
impl crate::RegisterSpec for RegPc29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pc29::R`](R) reader structure"]
impl crate::Readable for RegPc29Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pc29::W`](W) writer structure"]
impl crate::Writable for RegPc29Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PC29 to value 0x1900"]
impl crate::Resettable for RegPc29Spec {
    const RESET_VALUE: u32 = 0x1900;
}
