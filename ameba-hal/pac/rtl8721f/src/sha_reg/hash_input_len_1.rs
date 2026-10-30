#[doc = "Register `HASH_input_len_1` reader"]
pub type R = crate::R<HashInputLen1Spec>;
#[doc = "Register `HASH_input_len_1` writer"]
pub type W = crate::W<HashInputLen1Spec>;
#[doc = "Field `HASH_input_len1` reader - Hash length\\[32*(3-1)+31:32*(3-1)\\]"]
pub type HashInputLen1R = crate::FieldReader<u32>;
#[doc = "Field `HASH_input_len1` writer - Hash length\\[32*(3-1)+31:32*(3-1)\\]"]
pub type HashInputLen1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash length\\[32*(3-1)+31:32*(3-1)\\]"]
    #[inline(always)]
    pub fn hash_input_len1(&self) -> HashInputLen1R {
        HashInputLen1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash length\\[32*(3-1)+31:32*(3-1)\\]"]
    #[inline(always)]
    pub fn hash_input_len1(&mut self) -> HashInputLen1W<'_, HashInputLen1Spec> {
        HashInputLen1W::new(self, 0)
    }
}
#[doc = "Total input byte length register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_input_len_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_input_len_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashInputLen1Spec;
impl crate::RegisterSpec for HashInputLen1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_input_len_1::R`](R) reader structure"]
impl crate::Readable for HashInputLen1Spec {}
#[doc = "`write(|w| ..)` method takes [`hash_input_len_1::W`](W) writer structure"]
impl crate::Writable for HashInputLen1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HASH_input_len_1 to value 0"]
impl crate::Resettable for HashInputLen1Spec {}
