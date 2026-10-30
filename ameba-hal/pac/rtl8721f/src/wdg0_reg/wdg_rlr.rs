#[doc = "Register `WDG_RLR` reader"]
pub type R = crate::R<WdgRlrSpec>;
#[doc = "Register `WDG_RLR` writer"]
pub type W = crate::W<WdgRlrSpec>;
#[doc = "Field `RELOAD` reader - Reload value for watchdog counter Default : 64s (@100k)"]
pub type ReloadR = crate::FieldReader<u16>;
#[doc = "Field `RELOAD` writer - Reload value for watchdog counter Default : 64s (@100k)"]
pub type ReloadW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PRER` reader - Prescaler counter, configuration only allowed before wdg enable WDG: 0x63 System wdg: 0x1F"]
pub type PrerR = crate::FieldReader;
#[doc = "Field `PRER` writer - Prescaler counter, configuration only allowed before wdg enable WDG: 0x63 System wdg: 0x1F"]
pub type PrerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - Reload value for watchdog counter Default : 64s (@100k)"]
    #[inline(always)]
    pub fn reload(&self) -> ReloadR {
        ReloadR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Prescaler counter, configuration only allowed before wdg enable WDG: 0x63 System wdg: 0x1F"]
    #[inline(always)]
    pub fn prer(&self) -> PrerR {
        PrerR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Reload value for watchdog counter Default : 64s (@100k)"]
    #[inline(always)]
    pub fn reload(&mut self) -> ReloadW<'_, WdgRlrSpec> {
        ReloadW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Prescaler counter, configuration only allowed before wdg enable WDG: 0x63 System wdg: 0x1F"]
    #[inline(always)]
    pub fn prer(&mut self) -> PrerW<'_, WdgRlrSpec> {
        PrerW::new(self, 16)
    }
}
#[doc = "WDG Relaod register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdg_rlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdg_rlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdgRlrSpec;
impl crate::RegisterSpec for WdgRlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdg_rlr::R`](R) reader structure"]
impl crate::Readable for WdgRlrSpec {}
#[doc = "`write(|w| ..)` method takes [`wdg_rlr::W`](W) writer structure"]
impl crate::Writable for WdgRlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDG_RLR to value 0x0063_ffff"]
impl crate::Resettable for WdgRlrSpec {
    const RESET_VALUE: u32 = 0x0063_ffff;
}
