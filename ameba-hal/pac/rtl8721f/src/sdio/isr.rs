#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<IsrSpec>;
#[doc = "Field `write_data` writer - 1 to set, 0 to clear bit with 1."]
pub type WriteDataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Int1` reader - SDIO Int1. DMA done."]
pub type Int1R = crate::BitReader;
#[doc = "Field `Int1` writer - SDIO Int1. DMA done."]
pub type Int1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Int3` reader - SD Int3.SB1 wlast/rlast."]
pub type Int3R = crate::BitReader;
#[doc = "Field `Int3` writer - SD Int3.SB1 wlast/rlast."]
pub type Int3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Int4` reader - SDIO Int4. SDIO IP Int."]
pub type Int4R = crate::BitReader;
#[doc = "Field `Int4` writer - SDIO Int4. SDIO IP Int."]
pub type Int4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - SDIO Int1. DMA done."]
    #[inline(always)]
    pub fn int1(&self) -> Int1R {
        Int1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SD Int3.SB1 wlast/rlast."]
    #[inline(always)]
    pub fn int3(&self) -> Int3R {
        Int3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SDIO Int4. SDIO IP Int."]
    #[inline(always)]
    pub fn int4(&self) -> Int4R {
        Int4R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1 to set, 0 to clear bit with 1."]
    #[inline(always)]
    pub fn write_data(&mut self) -> WriteDataW<'_, IsrSpec> {
        WriteDataW::new(self, 0)
    }
    #[doc = "Bit 1 - SDIO Int1. DMA done."]
    #[inline(always)]
    pub fn int1(&mut self) -> Int1W<'_, IsrSpec> {
        Int1W::new(self, 1)
    }
    #[doc = "Bit 3 - SD Int3.SB1 wlast/rlast."]
    #[inline(always)]
    pub fn int3(&mut self) -> Int3W<'_, IsrSpec> {
        Int3W::new(self, 3)
    }
    #[doc = "Bit 4 - SDIO Int4. SDIO IP Int."]
    #[inline(always)]
    pub fn int4(&mut self) -> Int4W<'_, IsrSpec> {
        Int4W::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for IsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
