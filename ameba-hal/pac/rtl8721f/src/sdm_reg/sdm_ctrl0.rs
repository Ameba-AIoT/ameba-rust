#[doc = "Register `SDM_CTRL0` reader"]
pub type R = crate::R<SdmCtrl0Spec>;
#[doc = "Register `SDM_CTRL0` writer"]
pub type W = crate::W<SdmCtrl0Spec>;
#[doc = "Field `XTAL_SEL` reader - XTAL frequency select, default 40M * 4'h0: XTAL40M * 4'h1: XTAL25M * 4'h4: XTAL20M * 4'h5: XTAL26M * 4'hD: XTAL27M * 4'hE: XTAL24M"]
pub type XtalSelR = crate::FieldReader;
#[doc = "Field `XTAL_SEL` writer - XTAL frequency select, default 40M * 4'h0: XTAL40M * 4'h1: XTAL25M * 4'h4: XTAL20M * 4'h5: XTAL26M * 4'hD: XTAL27M * 4'hE: XTAL24M"]
pub type XtalSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TIMER_CAL_EN` reader - 1: Enable timer_cal"]
pub type TimerCalEnR = crate::BitReader;
#[doc = "Field `TIMER_CAL_EN` writer - 1: Enable timer_cal"]
pub type TimerCalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALWAYS_CAL_EN` reader - 1: Enable always_cal"]
pub type AlwaysCalEnR = crate::BitReader;
#[doc = "Field `ALWAYS_CAL_EN` writer - 1: Enable always_cal"]
pub type AlwaysCalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_LOSS_HIGH` reader - Decide 32k output compensation direction 1: Mask 0: Add"]
pub type TimeLossHighR = crate::BitReader;
#[doc = "Field `TIME_LOSS_HIGH` writer - Decide 32k output compensation direction 1: Mask 0: Add"]
pub type TimeLossHighW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_LOSS_SET` reader - Time loss set 1: User-defined"]
pub type TimeLossSetR = crate::BitReader;
#[doc = "Field `TIME_LOSS_SET` writer - Time loss set 1: User-defined"]
pub type TimeLossSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAL_DONE` reader - "]
pub type CalDoneR = crate::BitReader;
#[doc = "Field `SDM_MOD_SEL` reader - 0: Default mode 1: User-defined parameter mode"]
pub type SdmModSelR = crate::BitReader;
#[doc = "Field `SDM_MOD_SEL` writer - 0: Default mode 1: User-defined parameter mode"]
pub type SdmModSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDM_BYPASSS_MOD_EN` reader - Bypass mode enable"]
pub type SdmBypasssModEnR = crate::BitReader;
#[doc = "Field `SDM_BYPASSS_MOD_EN` writer - Bypass mode enable"]
pub type SdmBypasssModEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDM_RST` reader - 0: Reset SDM"]
pub type SdmRstR = crate::BitReader;
#[doc = "Field `SDM_RST` writer - 0: Reset SDM"]
pub type SdmRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDM_EN` reader - Enable SDM"]
pub type SdmEnR = crate::BitReader;
#[doc = "Field `SDM_EN` writer - Enable SDM"]
pub type SdmEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 8:11 - XTAL frequency select, default 40M * 4'h0: XTAL40M * 4'h1: XTAL25M * 4'h4: XTAL20M * 4'h5: XTAL26M * 4'hD: XTAL27M * 4'hE: XTAL24M"]
    #[inline(always)]
    pub fn xtal_sel(&self) -> XtalSelR {
        XtalSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - 1: Enable timer_cal"]
    #[inline(always)]
    pub fn timer_cal_en(&self) -> TimerCalEnR {
        TimerCalEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1: Enable always_cal"]
    #[inline(always)]
    pub fn always_cal_en(&self) -> AlwaysCalEnR {
        AlwaysCalEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Decide 32k output compensation direction 1: Mask 0: Add"]
    #[inline(always)]
    pub fn time_loss_high(&self) -> TimeLossHighR {
        TimeLossHighR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Time loss set 1: User-defined"]
    #[inline(always)]
    pub fn time_loss_set(&self) -> TimeLossSetR {
        TimeLossSetR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cal_done(&self) -> CalDoneR {
        CalDoneR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 0: Default mode 1: User-defined parameter mode"]
    #[inline(always)]
    pub fn sdm_mod_sel(&self) -> SdmModSelR {
        SdmModSelR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Bypass mode enable"]
    #[inline(always)]
    pub fn sdm_bypasss_mod_en(&self) -> SdmBypasssModEnR {
        SdmBypasssModEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 0: Reset SDM"]
    #[inline(always)]
    pub fn sdm_rst(&self) -> SdmRstR {
        SdmRstR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable SDM"]
    #[inline(always)]
    pub fn sdm_en(&self) -> SdmEnR {
        SdmEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:11 - XTAL frequency select, default 40M * 4'h0: XTAL40M * 4'h1: XTAL25M * 4'h4: XTAL20M * 4'h5: XTAL26M * 4'hD: XTAL27M * 4'hE: XTAL24M"]
    #[inline(always)]
    pub fn xtal_sel(&mut self) -> XtalSelW<'_, SdmCtrl0Spec> {
        XtalSelW::new(self, 8)
    }
    #[doc = "Bit 17 - 1: Enable timer_cal"]
    #[inline(always)]
    pub fn timer_cal_en(&mut self) -> TimerCalEnW<'_, SdmCtrl0Spec> {
        TimerCalEnW::new(self, 17)
    }
    #[doc = "Bit 18 - 1: Enable always_cal"]
    #[inline(always)]
    pub fn always_cal_en(&mut self) -> AlwaysCalEnW<'_, SdmCtrl0Spec> {
        AlwaysCalEnW::new(self, 18)
    }
    #[doc = "Bit 24 - Decide 32k output compensation direction 1: Mask 0: Add"]
    #[inline(always)]
    pub fn time_loss_high(&mut self) -> TimeLossHighW<'_, SdmCtrl0Spec> {
        TimeLossHighW::new(self, 24)
    }
    #[doc = "Bit 25 - Time loss set 1: User-defined"]
    #[inline(always)]
    pub fn time_loss_set(&mut self) -> TimeLossSetW<'_, SdmCtrl0Spec> {
        TimeLossSetW::new(self, 25)
    }
    #[doc = "Bit 28 - 0: Default mode 1: User-defined parameter mode"]
    #[inline(always)]
    pub fn sdm_mod_sel(&mut self) -> SdmModSelW<'_, SdmCtrl0Spec> {
        SdmModSelW::new(self, 28)
    }
    #[doc = "Bit 29 - Bypass mode enable"]
    #[inline(always)]
    pub fn sdm_bypasss_mod_en(&mut self) -> SdmBypasssModEnW<'_, SdmCtrl0Spec> {
        SdmBypasssModEnW::new(self, 29)
    }
    #[doc = "Bit 30 - 0: Reset SDM"]
    #[inline(always)]
    pub fn sdm_rst(&mut self) -> SdmRstW<'_, SdmCtrl0Spec> {
        SdmRstW::new(self, 30)
    }
    #[doc = "Bit 31 - Enable SDM"]
    #[inline(always)]
    pub fn sdm_en(&mut self) -> SdmEnW<'_, SdmCtrl0Spec> {
        SdmEnW::new(self, 31)
    }
}
#[doc = "SDM Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdm_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdm_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmCtrl0Spec;
impl crate::RegisterSpec for SdmCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdm_ctrl0::R`](R) reader structure"]
impl crate::Readable for SdmCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`sdm_ctrl0::W`](W) writer structure"]
impl crate::Writable for SdmCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDM_CTRL0 to value 0"]
impl crate::Resettable for SdmCtrl0Spec {}
