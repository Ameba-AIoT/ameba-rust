#[doc = "Register `ANAPAR_XTAL_ON_2` reader"]
pub type R = crate::R<AnaparXtalOn2Spec>;
#[doc = "Register `ANAPAR_XTAL_ON_2` writer"]
pub type W = crate::W<AnaparXtalOn2Spec>;
#[doc = "Field `AAC_MODE` reader - AAC_MODE for LPS 00: all XOXI cap off 01: cap from XTAL_SC_LPS 10: cap from XTAL_SC_INIT 11: cap from XTAL_SC_XI/XO"]
pub type AacModeR = crate::FieldReader;
#[doc = "Field `AAC_MODE` writer - AAC_MODE for LPS 00: all XOXI cap off 01: cap from XTAL_SC_LPS 10: cap from XTAL_SC_INIT 11: cap from XTAL_SC_XI/XO"]
pub type AacModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EN_XTAL_DRV_DIGI_DIV2` reader - IQKPLL enable clock source control, 1:enable, 0: gate"]
pub type EnXtalDrvDigiDiv2R = crate::BitReader;
#[doc = "Field `EN_XTAL_DRV_DIGI_DIV2` writer - IQKPLL enable clock source control, 1:enable, 0: gate"]
pub type EnXtalDrvDigiDiv2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_XTAL_DRV_IQK_BCN` reader - IQKPLL enable clock source control, 1:enable, 0: gate"]
pub type EnXtalDrvIqkBcnR = crate::BitReader;
#[doc = "Field `EN_XTAL_DRV_IQK_BCN` writer - IQKPLL enable clock source control, 1:enable, 0: gate"]
pub type EnXtalDrvIqkBcnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_XTAL_AAC_GM` reader - 1: enable xtal aac gm control"]
pub type EnXtalAacGmR = crate::BitReader;
#[doc = "Field `EN_XTAL_AAC_GM` writer - 1: enable xtal aac gm control"]
pub type EnXtalAacGmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GM_MANUAL` reader - Gm manual at EN_XTAL_AAC_DIGI=0"]
pub type GmManualR = crate::FieldReader;
#[doc = "Field `GM_MANUAL` writer - Gm manual at EN_XTAL_AAC_DIGI=0"]
pub type GmManualW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EN_XTAL_BDRF` reader - Balance dynamic RF for duty cycle"]
pub type EnXtalBdrfR = crate::BitReader;
#[doc = "Field `EN_XTAL_BDRF` writer - Balance dynamic RF for duty cycle"]
pub type EnXtalBdrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_DUMMY` reader - Dummy control bits"]
pub type XtalDummyR = crate::FieldReader;
#[doc = "Field `XTAL_DUMMY` writer - Dummy control bits"]
pub type XtalDummyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XTAL_SEL_TOK_FST` reader - used for HP2NM or NM2HP XTAL start-up process reg value: cycles / 40MHz / 26MHz 000: 2^10 / 0.025m / 0.05 001: 2^11 / 0.05m / 0.1m 010: 2^12 / 0.1m / 0.2m 011: 2^13 / 0.2m / 0.3m 100: 2^14 / 0.4m / 0.6m 101: 2^15 / 0.8m / 1.2m 110: 2^16 / 1.6m / 2.5m 111: 2^17 / 3.2m / 4.9m XTAL mode switch process reg value: cycles / 40MHz / 26MHz 000: 2^10 / 0.0125m / 0.025 001: 2^11 / 0.025m / 0.05m 010: 2^12 / 0.05m / 0.1m 011: 2^13 / 0.1m / 0.2m 100: 2^14 / 0.2m / 0.3m 101: 2^15 / 0.4m / 0.6m 110: 2^16 / 0.8m / 1.25m 111: 2^17 / 1.6m / 2.45m"]
pub type XtalSelTokFstR = crate::FieldReader;
#[doc = "Field `XTAL_SEL_TOK_FST` writer - used for HP2NM or NM2HP XTAL start-up process reg value: cycles / 40MHz / 26MHz 000: 2^10 / 0.025m / 0.05 001: 2^11 / 0.05m / 0.1m 010: 2^12 / 0.1m / 0.2m 011: 2^13 / 0.2m / 0.3m 100: 2^14 / 0.4m / 0.6m 101: 2^15 / 0.8m / 1.2m 110: 2^16 / 1.6m / 2.5m 111: 2^17 / 3.2m / 4.9m XTAL mode switch process reg value: cycles / 40MHz / 26MHz 000: 2^10 / 0.0125m / 0.025 001: 2^11 / 0.025m / 0.05m 010: 2^12 / 0.05m / 0.1m 011: 2^13 / 0.1m / 0.2m 100: 2^14 / 0.2m / 0.3m 101: 2^15 / 0.4m / 0.6m 110: 2^16 / 0.8m / 1.25m 111: 2^17 / 1.6m / 2.45m"]
pub type XtalSelTokFstW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - AAC_MODE for LPS 00: all XOXI cap off 01: cap from XTAL_SC_LPS 10: cap from XTAL_SC_INIT 11: cap from XTAL_SC_XI/XO"]
    #[inline(always)]
    pub fn aac_mode(&self) -> AacModeR {
        AacModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - IQKPLL enable clock source control, 1:enable, 0: gate"]
    #[inline(always)]
    pub fn en_xtal_drv_digi_div2(&self) -> EnXtalDrvDigiDiv2R {
        EnXtalDrvDigiDiv2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IQKPLL enable clock source control, 1:enable, 0: gate"]
    #[inline(always)]
    pub fn en_xtal_drv_iqk_bcn(&self) -> EnXtalDrvIqkBcnR {
        EnXtalDrvIqkBcnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: enable xtal aac gm control"]
    #[inline(always)]
    pub fn en_xtal_aac_gm(&self) -> EnXtalAacGmR {
        EnXtalAacGmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:9 - Gm manual at EN_XTAL_AAC_DIGI=0"]
    #[inline(always)]
    pub fn gm_manual(&self) -> GmManualR {
        GmManualR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - Balance dynamic RF for duty cycle"]
    #[inline(always)]
    pub fn en_xtal_bdrf(&self) -> EnXtalBdrfR {
        EnXtalBdrfR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Dummy control bits"]
    #[inline(always)]
    pub fn xtal_dummy(&self) -> XtalDummyR {
        XtalDummyR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 29:31 - used for HP2NM or NM2HP XTAL start-up process reg value: cycles / 40MHz / 26MHz 000: 2^10 / 0.025m / 0.05 001: 2^11 / 0.05m / 0.1m 010: 2^12 / 0.1m / 0.2m 011: 2^13 / 0.2m / 0.3m 100: 2^14 / 0.4m / 0.6m 101: 2^15 / 0.8m / 1.2m 110: 2^16 / 1.6m / 2.5m 111: 2^17 / 3.2m / 4.9m XTAL mode switch process reg value: cycles / 40MHz / 26MHz 000: 2^10 / 0.0125m / 0.025 001: 2^11 / 0.025m / 0.05m 010: 2^12 / 0.05m / 0.1m 011: 2^13 / 0.1m / 0.2m 100: 2^14 / 0.2m / 0.3m 101: 2^15 / 0.4m / 0.6m 110: 2^16 / 0.8m / 1.25m 111: 2^17 / 1.6m / 2.45m"]
    #[inline(always)]
    pub fn xtal_sel_tok_fst(&self) -> XtalSelTokFstR {
        XtalSelTokFstR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - AAC_MODE for LPS 00: all XOXI cap off 01: cap from XTAL_SC_LPS 10: cap from XTAL_SC_INIT 11: cap from XTAL_SC_XI/XO"]
    #[inline(always)]
    pub fn aac_mode(&mut self) -> AacModeW<'_, AnaparXtalOn2Spec> {
        AacModeW::new(self, 0)
    }
    #[doc = "Bit 2 - IQKPLL enable clock source control, 1:enable, 0: gate"]
    #[inline(always)]
    pub fn en_xtal_drv_digi_div2(&mut self) -> EnXtalDrvDigiDiv2W<'_, AnaparXtalOn2Spec> {
        EnXtalDrvDigiDiv2W::new(self, 2)
    }
    #[doc = "Bit 3 - IQKPLL enable clock source control, 1:enable, 0: gate"]
    #[inline(always)]
    pub fn en_xtal_drv_iqk_bcn(&mut self) -> EnXtalDrvIqkBcnW<'_, AnaparXtalOn2Spec> {
        EnXtalDrvIqkBcnW::new(self, 3)
    }
    #[doc = "Bit 4 - 1: enable xtal aac gm control"]
    #[inline(always)]
    pub fn en_xtal_aac_gm(&mut self) -> EnXtalAacGmW<'_, AnaparXtalOn2Spec> {
        EnXtalAacGmW::new(self, 4)
    }
    #[doc = "Bits 5:9 - Gm manual at EN_XTAL_AAC_DIGI=0"]
    #[inline(always)]
    pub fn gm_manual(&mut self) -> GmManualW<'_, AnaparXtalOn2Spec> {
        GmManualW::new(self, 5)
    }
    #[doc = "Bit 11 - Balance dynamic RF for duty cycle"]
    #[inline(always)]
    pub fn en_xtal_bdrf(&mut self) -> EnXtalBdrfW<'_, AnaparXtalOn2Spec> {
        EnXtalBdrfW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Dummy control bits"]
    #[inline(always)]
    pub fn xtal_dummy(&mut self) -> XtalDummyW<'_, AnaparXtalOn2Spec> {
        XtalDummyW::new(self, 12)
    }
    #[doc = "Bits 29:31 - used for HP2NM or NM2HP XTAL start-up process reg value: cycles / 40MHz / 26MHz 000: 2^10 / 0.025m / 0.05 001: 2^11 / 0.05m / 0.1m 010: 2^12 / 0.1m / 0.2m 011: 2^13 / 0.2m / 0.3m 100: 2^14 / 0.4m / 0.6m 101: 2^15 / 0.8m / 1.2m 110: 2^16 / 1.6m / 2.5m 111: 2^17 / 3.2m / 4.9m XTAL mode switch process reg value: cycles / 40MHz / 26MHz 000: 2^10 / 0.0125m / 0.025 001: 2^11 / 0.025m / 0.05m 010: 2^12 / 0.05m / 0.1m 011: 2^13 / 0.1m / 0.2m 100: 2^14 / 0.2m / 0.3m 101: 2^15 / 0.4m / 0.6m 110: 2^16 / 0.8m / 1.25m 111: 2^17 / 1.6m / 2.45m"]
    #[inline(always)]
    pub fn xtal_sel_tok_fst(&mut self) -> XtalSelTokFstW<'_, AnaparXtalOn2Spec> {
        XtalSelTokFstW::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`anapar_xtal_on_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anapar_xtal_on_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaparXtalOn2Spec;
impl crate::RegisterSpec for AnaparXtalOn2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anapar_xtal_on_2::R`](R) reader structure"]
impl crate::Readable for AnaparXtalOn2Spec {}
#[doc = "`write(|w| ..)` method takes [`anapar_xtal_on_2::W`](W) writer structure"]
impl crate::Writable for AnaparXtalOn2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANAPAR_XTAL_ON_2 to value 0x08e9"]
impl crate::Resettable for AnaparXtalOn2Spec {
    const RESET_VALUE: u32 = 0x08e9;
}
