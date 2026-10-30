#[doc = "Register `LLP5_L` reader"]
pub type R = crate::R<Llp5LSpec>;
#[doc = "Register `LLP5_L` writer"]
pub type W = crate::W<Llp5LSpec>;
#[doc = "Field `LLP5_L_LOC` reader - Starting Address In Memory of next LLI if block chaining is enabled. Note that the two LSBs of the starting address are not stored because the address is assumed to be aligned to a 32-bit boundary.LLI accesses are always 32-bit accesses aligned to 32-bit boundaries and cannot be changed or programmed to anything other than 32-bit."]
pub type Llp5LLocR = crate::FieldReader<u32>;
#[doc = "Field `LLP5_L_LOC` writer - Starting Address In Memory of next LLI if block chaining is enabled. Note that the two LSBs of the starting address are not stored because the address is assumed to be aligned to a 32-bit boundary.LLI accesses are always 32-bit accesses aligned to 32-bit boundaries and cannot be changed or programmed to anything other than 32-bit."]
pub type Llp5LLocW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Starting Address In Memory of next LLI if block chaining is enabled. Note that the two LSBs of the starting address are not stored because the address is assumed to be aligned to a 32-bit boundary.LLI accesses are always 32-bit accesses aligned to 32-bit boundaries and cannot be changed or programmed to anything other than 32-bit."]
    #[inline(always)]
    pub fn llp5_l_loc(&self) -> Llp5LLocR {
        Llp5LLocR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Starting Address In Memory of next LLI if block chaining is enabled. Note that the two LSBs of the starting address are not stored because the address is assumed to be aligned to a 32-bit boundary.LLI accesses are always 32-bit accesses aligned to 32-bit boundaries and cannot be changed or programmed to anything other than 32-bit."]
    #[inline(always)]
    pub fn llp5_l_loc(&mut self) -> Llp5LLocW<'_, Llp5LSpec> {
        Llp5LLocW::new(self, 2)
    }
}
#[doc = "Channel 5 low-32 bits Linked List Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`llp5_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp5_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Llp5LSpec;
impl crate::RegisterSpec for Llp5LSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`llp5_l::R`](R) reader structure"]
impl crate::Readable for Llp5LSpec {}
#[doc = "`write(|w| ..)` method takes [`llp5_l::W`](W) writer structure"]
impl crate::Writable for Llp5LSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LLP5_L to value 0"]
impl crate::Resettable for Llp5LSpec {}
