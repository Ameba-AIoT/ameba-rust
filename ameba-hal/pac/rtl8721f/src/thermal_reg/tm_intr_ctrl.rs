#[doc = "Register `TM_INTR_CTRL` reader"]
pub type R = crate::R<TmIntrCtrlSpec>;
#[doc = "Register `TM_INTR_CTRL` writer"]
pub type W = crate::W<TmIntrCtrlSpec>;
#[doc = "Field `imr_tm_high_wt` reader - Interrupt control for thermal meter detect over temperature warning. * 0: Disable over temperature warning detect interrupt * 1: Enable over temperature warning detect interrupt"]
pub type ImrTmHighWtR = crate::BitReader;
#[doc = "Field `imr_tm_high_wt` writer - Interrupt control for thermal meter detect over temperature warning. * 0: Disable over temperature warning detect interrupt * 1: Enable over temperature warning detect interrupt"]
pub type ImrTmHighWtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `imr_tm_low_wt` reader - Interrupt control for thermal meter detect low temperature. * 0: Disable low temperature detect interrupt * 1: Enable low temperature detect interrupt"]
pub type ImrTmLowWtR = crate::BitReader;
#[doc = "Field `imr_tm_low_wt` writer - Interrupt control for thermal meter detect low temperature. * 0: Disable low temperature detect interrupt * 1: Enable low temperature detect interrupt"]
pub type ImrTmLowWtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt control for thermal meter detect over temperature warning. * 0: Disable over temperature warning detect interrupt * 1: Enable over temperature warning detect interrupt"]
    #[inline(always)]
    pub fn imr_tm_high_wt(&self) -> ImrTmHighWtR {
        ImrTmHighWtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt control for thermal meter detect low temperature. * 0: Disable low temperature detect interrupt * 1: Enable low temperature detect interrupt"]
    #[inline(always)]
    pub fn imr_tm_low_wt(&self) -> ImrTmLowWtR {
        ImrTmLowWtR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt control for thermal meter detect over temperature warning. * 0: Disable over temperature warning detect interrupt * 1: Enable over temperature warning detect interrupt"]
    #[inline(always)]
    pub fn imr_tm_high_wt(&mut self) -> ImrTmHighWtW<'_, TmIntrCtrlSpec> {
        ImrTmHighWtW::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt control for thermal meter detect low temperature. * 0: Disable low temperature detect interrupt * 1: Enable low temperature detect interrupt"]
    #[inline(always)]
    pub fn imr_tm_low_wt(&mut self) -> ImrTmLowWtW<'_, TmIntrCtrlSpec> {
        ImrTmLowWtW::new(self, 1)
    }
}
#[doc = "Thermal Meter Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tm_intr_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tm_intr_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmIntrCtrlSpec;
impl crate::RegisterSpec for TmIntrCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tm_intr_ctrl::R`](R) reader structure"]
impl crate::Readable for TmIntrCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tm_intr_ctrl::W`](W) writer structure"]
impl crate::Writable for TmIntrCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TM_INTR_CTRL to value 0"]
impl crate::Resettable for TmIntrCtrlSpec {}
