#[doc = "Register `TIM_EN` reader"]
pub type R = crate::R<TimEnSpec>;
#[doc = "Register `TIM_EN` writer"]
pub type W = crate::W<TimEnSpec>;
#[doc = "Field `cnt_start` writer - enable the counter. Poll CNT_RUN to see the counter status. If CNT_RUN is 1, it means that the counter has been enabled internally."]
pub type CntStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnt_stop` writer - disable the counter. Poll CNT_RUN to see the counter status. If CNT_RUN is 0, it means that the counter has been disabled internally."]
pub type CntStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEN` reader - counter run status * 0: counter is disabled * 1: counter is enabled"]
pub type CenR = crate::BitReader;
#[doc = "Field `cnt_en` reader - counter working status * 0: counter is stoppped * 1: counter is working"]
pub type CntEnR = crate::BitReader;
impl R {
    #[doc = "Bit 8 - counter run status * 0: counter is disabled * 1: counter is enabled"]
    #[inline(always)]
    pub fn cen(&self) -> CenR {
        CenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - counter working status * 0: counter is stoppped * 1: counter is working"]
    #[inline(always)]
    pub fn cnt_en(&self) -> CntEnR {
        CntEnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable the counter. Poll CNT_RUN to see the counter status. If CNT_RUN is 1, it means that the counter has been enabled internally."]
    #[inline(always)]
    pub fn cnt_start(&mut self) -> CntStartW<'_, TimEnSpec> {
        CntStartW::new(self, 0)
    }
    #[doc = "Bit 1 - disable the counter. Poll CNT_RUN to see the counter status. If CNT_RUN is 0, it means that the counter has been disabled internally."]
    #[inline(always)]
    pub fn cnt_stop(&mut self) -> CntStopW<'_, TimEnSpec> {
        CntStopW::new(self, 1)
    }
}
#[doc = "TIM enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimEnSpec;
impl crate::RegisterSpec for TimEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_en::R`](R) reader structure"]
impl crate::Readable for TimEnSpec {}
#[doc = "`write(|w| ..)` method takes [`tim_en::W`](W) writer structure"]
impl crate::Writable for TimEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIM_EN to value 0"]
impl crate::Resettable for TimEnSpec {}
