#[doc = "Register `EVENT_CTRL` reader"]
pub type R = crate::R<EventCtrlSpec>;
#[doc = "Register `EVENT_CTRL` writer"]
pub type W = crate::W<EventCtrlSpec>;
#[doc = "Field `EVENT_CTRL_EN` reader - Enable event counter."]
pub type EventCtrlEnR = crate::BitReader;
#[doc = "Field `EVENT_CTRL_EN` writer - Enable event counter."]
pub type EventCtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable event counter."]
    #[inline(always)]
    pub fn event_ctrl_en(&self) -> EventCtrlEnR {
        EventCtrlEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable event counter."]
    #[inline(always)]
    pub fn event_ctrl_en(&mut self) -> EventCtrlEnW<'_, EventCtrlSpec> {
        EventCtrlEnW::new(self, 0)
    }
}
#[doc = "Enable event counter\n\nYou can [`read`](crate::Reg::read) this register and get [`event_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`event_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventCtrlSpec;
impl crate::RegisterSpec for EventCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`event_ctrl::R`](R) reader structure"]
impl crate::Readable for EventCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`event_ctrl::W`](W) writer structure"]
impl crate::Writable for EventCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENT_CTRL to value 0x01"]
impl crate::Resettable for EventCtrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
