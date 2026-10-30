#[doc = "Register `SWINT` writer"]
pub type W = crate::W<SwintSpec>;
#[doc = "Field `SWINT` writer - Write 1 will force ISR register bit 10 set to 1"]
pub type SwintW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 24 - Write 1 will force ISR register bit 10 set to 1"]
    #[inline(always)]
    pub fn swint(&mut self) -> SwintW<'_, SwintSpec> {
        SwintW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swint::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwintSpec;
impl crate::RegisterSpec for SwintSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swint::W`](W) writer structure"]
impl crate::Writable for SwintSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWINT to value 0"]
impl crate::Resettable for SwintSpec {}
