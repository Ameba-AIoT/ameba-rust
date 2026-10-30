#[doc = "Register `ISREN` reader"]
pub type R = crate::R<IsrenSpec>;
#[doc = "Register `ISREN` writer"]
pub type W = crate::W<IsrenSpec>;
#[doc = "Field `write_data` writer - 1 to set, 0 to clear bit with 1."]
pub type WriteDataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Int1En` reader - SDIO Int1 Enable Dma done Int Enable"]
pub type Int1enR = crate::BitReader;
#[doc = "Field `Int1En` writer - SDIO Int1 Enable Dma done Int Enable"]
pub type Int1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Int3En` reader - SDIO Int3 Enable SB1 wlast/rlast Int Enable"]
pub type Int3enR = crate::BitReader;
#[doc = "Field `Int3En` writer - SDIO Int3 Enable SB1 wlast/rlast Int Enable"]
pub type Int3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Int4En` reader - SDIO Int4 Enable IP Int Enable"]
pub type Int4enR = crate::BitReader;
#[doc = "Field `Int4En` writer - SDIO Int4 Enable IP Int Enable"]
pub type Int4enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - SDIO Int1 Enable Dma done Int Enable"]
    #[inline(always)]
    pub fn int1en(&self) -> Int1enR {
        Int1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SDIO Int3 Enable SB1 wlast/rlast Int Enable"]
    #[inline(always)]
    pub fn int3en(&self) -> Int3enR {
        Int3enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SDIO Int4 Enable IP Int Enable"]
    #[inline(always)]
    pub fn int4en(&self) -> Int4enR {
        Int4enR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1 to set, 0 to clear bit with 1."]
    #[inline(always)]
    pub fn write_data(&mut self) -> WriteDataW<'_, IsrenSpec> {
        WriteDataW::new(self, 0)
    }
    #[doc = "Bit 1 - SDIO Int1 Enable Dma done Int Enable"]
    #[inline(always)]
    pub fn int1en(&mut self) -> Int1enW<'_, IsrenSpec> {
        Int1enW::new(self, 1)
    }
    #[doc = "Bit 3 - SDIO Int3 Enable SB1 wlast/rlast Int Enable"]
    #[inline(always)]
    pub fn int3en(&mut self) -> Int3enW<'_, IsrenSpec> {
        Int3enW::new(self, 3)
    }
    #[doc = "Bit 4 - SDIO Int4 Enable IP Int Enable"]
    #[inline(always)]
    pub fn int4en(&mut self) -> Int4enW<'_, IsrenSpec> {
        Int4enW::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`isren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrenSpec;
impl crate::RegisterSpec for IsrenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isren::R`](R) reader structure"]
impl crate::Readable for IsrenSpec {}
#[doc = "`write(|w| ..)` method takes [`isren::W`](W) writer structure"]
impl crate::Writable for IsrenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ISREN to value 0"]
impl crate::Resettable for IsrenSpec {}
