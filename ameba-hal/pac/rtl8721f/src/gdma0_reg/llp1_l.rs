#[doc = "Register `LLP1_L` reader"]
pub type R = crate::R<Llp1LSpec>;
#[doc = "Register `LLP1_L` writer"]
pub type W = crate::W<Llp1LSpec>;
#[doc = "Field `LLP1_L_LOC` reader - Starting Address In Memory of next LLI if block chaining is enabled. Note that the two LSBs of the starting address are not stored because the address is assumed to be aligned to a 32-bit boundary.LLI accesses are always 32-bit accesses aligned to 32-bit boundaries and cannot be changed or programmed to anything other than 32-bit."]
pub type Llp1LLocR = crate::FieldReader<u32>;
#[doc = "Field `LLP1_L_LOC` writer - Starting Address In Memory of next LLI if block chaining is enabled. Note that the two LSBs of the starting address are not stored because the address is assumed to be aligned to a 32-bit boundary.LLI accesses are always 32-bit accesses aligned to 32-bit boundaries and cannot be changed or programmed to anything other than 32-bit."]
pub type Llp1LLocW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Starting Address In Memory of next LLI if block chaining is enabled. Note that the two LSBs of the starting address are not stored because the address is assumed to be aligned to a 32-bit boundary.LLI accesses are always 32-bit accesses aligned to 32-bit boundaries and cannot be changed or programmed to anything other than 32-bit."]
    #[inline(always)]
    pub fn llp1_l_loc(&self) -> Llp1LLocR {
        Llp1LLocR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Starting Address In Memory of next LLI if block chaining is enabled. Note that the two LSBs of the starting address are not stored because the address is assumed to be aligned to a 32-bit boundary.LLI accesses are always 32-bit accesses aligned to 32-bit boundaries and cannot be changed or programmed to anything other than 32-bit."]
    #[inline(always)]
    pub fn llp1_l_loc(&mut self) -> Llp1LLocW<'_, Llp1LSpec> {
        Llp1LLocW::new(self, 2)
    }
}
#[doc = "Channel 1 low-32 bits Linked List Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`llp1_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp1_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Llp1LSpec;
impl crate::RegisterSpec for Llp1LSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`llp1_l::R`](R) reader structure"]
impl crate::Readable for Llp1LSpec {}
#[doc = "`write(|w| ..)` method takes [`llp1_l::W`](W) writer structure"]
impl crate::Writable for Llp1LSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LLP1_L to value 0"]
impl crate::Resettable for Llp1LSpec {}
