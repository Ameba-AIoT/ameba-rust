#[doc = "Register `LLP` reader"]
pub type R = crate::R<LlpSpec>;
#[doc = "Register `LLP` writer"]
pub type W = crate::W<LlpSpec>;
#[doc = "Field `LLP` reader - Linked List Pointer Note: Must align to 4 Bytes. Bits\\[1:0\\] is read only and return 0 on a read."]
pub type LlpR = crate::FieldReader<u32>;
#[doc = "Field `LLP` writer - Linked List Pointer Note: Must align to 4 Bytes. Bits\\[1:0\\] is read only and return 0 on a read."]
pub type LlpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Linked List Pointer Note: Must align to 4 Bytes. Bits\\[1:0\\] is read only and return 0 on a read."]
    #[inline(always)]
    pub fn llp(&self) -> LlpR {
        LlpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Linked List Pointer Note: Must align to 4 Bytes. Bits\\[1:0\\] is read only and return 0 on a read."]
    #[inline(always)]
    pub fn llp(&mut self) -> LlpW<'_, LlpSpec> {
        LlpW::new(self, 0)
    }
}
#[doc = "Linked List Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`llp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LlpSpec;
impl crate::RegisterSpec for LlpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`llp::R`](R) reader structure"]
impl crate::Readable for LlpSpec {}
#[doc = "`write(|w| ..)` method takes [`llp::W`](W) writer structure"]
impl crate::Writable for LlpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LLP to value 0"]
impl crate::Resettable for LlpSpec {}
