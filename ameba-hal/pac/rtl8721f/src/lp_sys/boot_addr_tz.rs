#[doc = "Register `BOOT_ADDR_TZ` reader"]
pub type R = crate::R<BootAddrTzSpec>;
#[doc = "Register `BOOT_ADDR_TZ` writer"]
pub type W = crate::W<BootAddrTzSpec>;
#[doc = "Field `dummy` reader - USED for SW"]
pub type DummyR = crate::FieldReader<u32>;
#[doc = "Field `dummy` writer - USED for SW"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USED for SW"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USED for SW"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, BootAddrTzSpec> {
        DummyW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`boot_addr_tz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot_addr_tz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BootAddrTzSpec;
impl crate::RegisterSpec for BootAddrTzSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot_addr_tz::R`](R) reader structure"]
impl crate::Readable for BootAddrTzSpec {}
#[doc = "`write(|w| ..)` method takes [`boot_addr_tz::W`](W) writer structure"]
impl crate::Writable for BootAddrTzSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BOOT_ADDR_TZ to value 0"]
impl crate::Resettable for BootAddrTzSpec {}
