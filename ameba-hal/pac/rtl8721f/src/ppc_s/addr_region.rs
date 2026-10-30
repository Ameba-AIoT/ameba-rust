#[doc = "Register `ADDR_REGION` reader"]
pub type R = crate::R<AddrRegionSpec>;
#[doc = "Register `ADDR_REGION` writer"]
pub type W = crate::W<AddrRegionSpec>;
#[doc = "Field `ADDR_BASE` reader - The base address boundary of prefetch. (only consider address\\[31:16\\])"]
pub type AddrBaseR = crate::FieldReader<u16>;
#[doc = "Field `ADDR_BASE` writer - The base address boundary of prefetch. (only consider address\\[31:16\\])"]
pub type AddrBaseW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADDR_TOP` reader - The top address boundary of prefetch. (only consider address\\[31:16\\])"]
pub type AddrTopR = crate::FieldReader<u16>;
#[doc = "Field `ADDR_TOP` writer - The top address boundary of prefetch. (only consider address\\[31:16\\])"]
pub type AddrTopW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The base address boundary of prefetch. (only consider address\\[31:16\\])"]
    #[inline(always)]
    pub fn addr_base(&self) -> AddrBaseR {
        AddrBaseR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The top address boundary of prefetch. (only consider address\\[31:16\\])"]
    #[inline(always)]
    pub fn addr_top(&self) -> AddrTopR {
        AddrTopR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The base address boundary of prefetch. (only consider address\\[31:16\\])"]
    #[inline(always)]
    pub fn addr_base(&mut self) -> AddrBaseW<'_, AddrRegionSpec> {
        AddrBaseW::new(self, 0)
    }
    #[doc = "Bits 16:31 - The top address boundary of prefetch. (only consider address\\[31:16\\])"]
    #[inline(always)]
    pub fn addr_top(&mut self) -> AddrTopW<'_, AddrRegionSpec> {
        AddrTopW::new(self, 16)
    }
}
#[doc = "The address region that requires prefetch\n\nYou can [`read`](crate::Reg::read) this register and get [`addr_region::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr_region::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrRegionSpec;
impl crate::RegisterSpec for AddrRegionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr_region::R`](R) reader structure"]
impl crate::Readable for AddrRegionSpec {}
#[doc = "`write(|w| ..)` method takes [`addr_region::W`](W) writer structure"]
impl crate::Writable for AddrRegionSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDR_REGION to value 0x6fff_6000"]
impl crate::Resettable for AddrRegionSpec {
    const RESET_VALUE: u32 = 0x6fff_6000;
}
