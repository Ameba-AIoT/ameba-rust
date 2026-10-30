#[doc = "Register `REG_PB20` reader"]
pub type R = crate::R<RegPb20Spec>;
#[doc = "Register `REG_PB20` writer"]
pub type W = crate::W<RegPb20Spec>;
#[doc = "Field `pB20_sel` reader - PAD pinmux function ID selection"]
pub type PB20SelR = crate::FieldReader;
#[doc = "Field `pB20_sel` writer - PAD pinmux function ID selection"]
pub type PB20SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pB20_ie` reader - PAD input enable control"]
pub type PB20IeR = crate::BitReader;
#[doc = "Field `pB20_ie` writer - PAD input enable control"]
pub type PB20IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB20_pupdc` reader - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB20PupdcR = crate::BitReader;
#[doc = "Field `pB20_pupdc` writer - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
pub type PB20PupdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB20_e2` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB20E2R = crate::BitReader;
#[doc = "Field `pB20_e2` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB20E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB20_smt` reader - PAD Schmit control"]
pub type PB20SmtR = crate::BitReader;
#[doc = "Field `pB20_smt` writer - PAD Schmit control"]
pub type PB20SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB20_sr` reader - PAD slew rate control. 1: slow 0: fast"]
pub type PB20SrR = crate::BitReader;
#[doc = "Field `pB20_sr` writer - PAD slew rate control. 1: slow 0: fast"]
pub type PB20SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB20_e3` reader - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB20E3R = crate::BitReader;
#[doc = "Field `pB20_e3` writer - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
pub type PB20E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pB20_dis` reader - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB20DisR = crate::BitReader;
#[doc = "Field `pB20_dis` writer - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
pub type PB20DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b20_sel(&self) -> PB20SelR {
        PB20SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b20_ie(&self) -> PB20IeR {
        PB20IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b20_pupdc(&self) -> PB20PupdcR {
        PB20PupdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b20_e2(&self) -> PB20E2R {
        PB20E2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b20_smt(&self) -> PB20SmtR {
        PB20SmtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b20_sr(&self) -> PB20SrR {
        PB20SrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b20_e3(&self) -> PB20E3R {
        PB20E3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b20_dis(&self) -> PB20DisR {
        PB20DisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAD pinmux function ID selection"]
    #[inline(always)]
    pub fn p_b20_sel(&mut self) -> PB20SelW<'_, RegPb20Spec> {
        PB20SelW::new(self, 0)
    }
    #[doc = "Bit 8 - PAD input enable control"]
    #[inline(always)]
    pub fn p_b20_ie(&mut self) -> PB20IeW<'_, RegPb20Spec> {
        PB20IeW::new(self, 8)
    }
    #[doc = "Bit 10 - Some pad may have two type of PU/PD resistor, this bit can select it. 1: small resistor 0: big resistor"]
    #[inline(always)]
    pub fn p_b20_pupdc(&mut self) -> PB20PupdcW<'_, RegPb20Spec> {
        PB20PupdcW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b20_e2(&mut self) -> PB20E2W<'_, RegPb20Spec> {
        PB20E2W::new(self, 11)
    }
    #[doc = "Bit 12 - PAD Schmit control"]
    #[inline(always)]
    pub fn p_b20_smt(&mut self) -> PB20SmtW<'_, RegPb20Spec> {
        PB20SmtW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD slew rate control. 1: slow 0: fast"]
    #[inline(always)]
    pub fn p_b20_sr(&mut self) -> PB20SrW<'_, RegPb20Spec> {
        PB20SrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD driving ability control. 0: low 1: high The actual driving current is depend on pad type."]
    #[inline(always)]
    pub fn p_b20_e3(&mut self) -> PB20E3W<'_, RegPb20Spec> {
        PB20E3W::new(self, 14)
    }
    #[doc = "Bit 15 - PAD shutdown, only when the whole group's dis signals are all 1, the PAD SHDN will pull down to shutdown the group pad. 1. disable pad 0. enable pad"]
    #[inline(always)]
    pub fn p_b20_dis(&mut self) -> PB20DisW<'_, RegPb20Spec> {
        PB20DisW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pb20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pb20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPb20Spec;
impl crate::RegisterSpec for RegPb20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pb20::R`](R) reader structure"]
impl crate::Readable for RegPb20Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_pb20::W`](W) writer structure"]
impl crate::Writable for RegPb20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PB20 to value 0x1900"]
impl crate::Resettable for RegPb20Spec {
    const RESET_VALUE: u32 = 0x1900;
}
