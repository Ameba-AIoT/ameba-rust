#[doc = "Register `REG_WLPMC_ISR` reader"]
pub type R = crate::R<RegWlpmcIsrSpec>;
#[doc = "Register `REG_WLPMC_ISR` writer"]
pub type W = crate::W<RegWlpmcIsrSpec>;
#[doc = "Field `WL_ON_ISR` reader - "]
pub type WlOnIsrR = crate::BitReader;
#[doc = "Field `WL_OFF_ISR` reader - "]
pub type WlOffIsrR = crate::BitReader;
#[doc = "Field `WL_ON_IMR` reader - "]
pub type WlOnImrR = crate::BitReader;
#[doc = "Field `WL_ON_IMR` writer - "]
pub type WlOnImrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WL_OFF_IMR` reader - "]
pub type WlOffImrR = crate::BitReader;
#[doc = "Field `WL_OFF_IMR` writer - "]
pub type WlOffImrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUMMY` reader - "]
pub type DummyR = crate::BitReader;
#[doc = "Field `DUMMY` writer - "]
pub type DummyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wl_on_isr(&self) -> WlOnIsrR {
        WlOnIsrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wl_off_isr(&self) -> WlOffIsrR {
        WlOffIsrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn wl_on_imr(&self) -> WlOnImrR {
        WlOnImrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn wl_off_imr(&self) -> WlOffImrR {
        WlOffImrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn wl_on_imr(&mut self) -> WlOnImrW<'_, RegWlpmcIsrSpec> {
        WlOnImrW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn wl_off_imr(&mut self) -> WlOffImrW<'_, RegWlpmcIsrSpec> {
        WlOffImrW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegWlpmcIsrSpec> {
        DummyW::new(self, 18)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wlpmc_isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wlpmc_isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWlpmcIsrSpec;
impl crate::RegisterSpec for RegWlpmcIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wlpmc_isr::R`](R) reader structure"]
impl crate::Readable for RegWlpmcIsrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wlpmc_isr::W`](W) writer structure"]
impl crate::Writable for RegWlpmcIsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WLPMC_ISR to value 0"]
impl crate::Resettable for RegWlpmcIsrSpec {}
