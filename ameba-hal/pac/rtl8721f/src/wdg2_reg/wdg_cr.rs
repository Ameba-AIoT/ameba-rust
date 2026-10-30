#[doc = "Register `WDG_CR` reader"]
pub type R = crate::R<WdgCrSpec>;
#[doc = "Register `WDG_CR` writer"]
pub type W = crate::W<WdgCrSpec>;
#[doc = "Field `EICNT` reader - Early interrupt trigger threshold"]
pub type EicntR = crate::FieldReader<u16>;
#[doc = "Field `EICNT` writer - Early interrupt trigger threshold"]
pub type EicntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EIE` reader - Watchdog early interrupt enable"]
pub type EieR = crate::BitReader;
#[doc = "Field `EIE` writer - Watchdog early interrupt enable"]
pub type EieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIC` writer - Write '1' clear the early interrupt"]
pub type EicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPEN` reader - Low power enable 0: WDG will gating when system goes into sleep mode 1: WDG keep running when system goes into sleep mode"]
pub type LpenR = crate::BitReader;
#[doc = "Field `LPEN` writer - Low power enable 0: WDG will gating when system goes into sleep mode 1: WDG keep running when system goes into sleep mode"]
pub type LpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVU` reader - Watchdog early interrupt function update"]
pub type EvuR = crate::BitReader;
#[doc = "Field `RVU` reader - Watchdog counter update by reload command"]
pub type RvuR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Early interrupt trigger threshold"]
    #[inline(always)]
    pub fn eicnt(&self) -> EicntR {
        EicntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Watchdog early interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EieR {
        EieR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Low power enable 0: WDG will gating when system goes into sleep mode 1: WDG keep running when system goes into sleep mode"]
    #[inline(always)]
    pub fn lpen(&self) -> LpenR {
        LpenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30 - Watchdog early interrupt function update"]
    #[inline(always)]
    pub fn evu(&self) -> EvuR {
        EvuR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Watchdog counter update by reload command"]
    #[inline(always)]
    pub fn rvu(&self) -> RvuR {
        RvuR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Early interrupt trigger threshold"]
    #[inline(always)]
    pub fn eicnt(&mut self) -> EicntW<'_, WdgCrSpec> {
        EicntW::new(self, 0)
    }
    #[doc = "Bit 16 - Watchdog early interrupt enable"]
    #[inline(always)]
    pub fn eie(&mut self) -> EieW<'_, WdgCrSpec> {
        EieW::new(self, 16)
    }
    #[doc = "Bit 17 - Write '1' clear the early interrupt"]
    #[inline(always)]
    pub fn eic(&mut self) -> EicW<'_, WdgCrSpec> {
        EicW::new(self, 17)
    }
    #[doc = "Bit 24 - Low power enable 0: WDG will gating when system goes into sleep mode 1: WDG keep running when system goes into sleep mode"]
    #[inline(always)]
    pub fn lpen(&mut self) -> LpenW<'_, WdgCrSpec> {
        LpenW::new(self, 24)
    }
}
#[doc = "WDG Control regsietr\n\nYou can [`read`](crate::Reg::read) this register and get [`wdg_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdg_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdgCrSpec;
impl crate::RegisterSpec for WdgCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdg_cr::R`](R) reader structure"]
impl crate::Readable for WdgCrSpec {}
#[doc = "`write(|w| ..)` method takes [`wdg_cr::W`](W) writer structure"]
impl crate::Writable for WdgCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDG_CR to value 0x0100_0000"]
impl crate::Resettable for WdgCrSpec {
    const RESET_VALUE: u32 = 0x0100_0000;
}
