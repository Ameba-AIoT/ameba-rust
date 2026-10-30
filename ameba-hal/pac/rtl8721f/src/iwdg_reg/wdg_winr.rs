#[doc = "Register `WDG_WINR` reader"]
pub type R = crate::R<WdgWinrSpec>;
#[doc = "Register `WDG_WINR` writer"]
pub type W = crate::W<WdgWinrSpec>;
#[doc = "Field `WINDOW` reader - Watchdog feed protect window register"]
pub type WindowR = crate::FieldReader<u16>;
#[doc = "Field `WINDOW` writer - Watchdog feed protect window register"]
pub type WindowW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Watchdog feed protect window register"]
    #[inline(always)]
    pub fn window(&self) -> WindowR {
        WindowR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Watchdog feed protect window register"]
    #[inline(always)]
    pub fn window(&mut self) -> WindowW<'_, WdgWinrSpec> {
        WindowW::new(self, 0)
    }
}
#[doc = "WDG Window Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdg_winr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdg_winr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdgWinrSpec;
impl crate::RegisterSpec for WdgWinrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdg_winr::R`](R) reader structure"]
impl crate::Readable for WdgWinrSpec {}
#[doc = "`write(|w| ..)` method takes [`wdg_winr::W`](W) writer structure"]
impl crate::Writable for WdgWinrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDG_WINR to value 0xffff"]
impl crate::Resettable for WdgWinrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
