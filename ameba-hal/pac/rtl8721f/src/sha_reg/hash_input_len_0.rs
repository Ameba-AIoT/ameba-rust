#[doc = "Register `HASH_input_len_0` reader"]
pub type R = crate::R<HashInputLen0Spec>;
#[doc = "Register `HASH_input_len_0` writer"]
pub type W = crate::W<HashInputLen0Spec>;
#[doc = "Field `HASH_input_len0` reader - Hash length\\[32*(3-0)+31:32*(3-0)\\]"]
pub type HashInputLen0R = crate::FieldReader<u32>;
#[doc = "Field `HASH_input_len0` writer - Hash length\\[32*(3-0)+31:32*(3-0)\\]"]
pub type HashInputLen0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash length\\[32*(3-0)+31:32*(3-0)\\]"]
    #[inline(always)]
    pub fn hash_input_len0(&self) -> HashInputLen0R {
        HashInputLen0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash length\\[32*(3-0)+31:32*(3-0)\\]"]
    #[inline(always)]
    pub fn hash_input_len0(&mut self) -> HashInputLen0W<'_, HashInputLen0Spec> {
        HashInputLen0W::new(self, 0)
    }
}
#[doc = "Total input byte length register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_input_len_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_input_len_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashInputLen0Spec;
impl crate::RegisterSpec for HashInputLen0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_input_len_0::R`](R) reader structure"]
impl crate::Readable for HashInputLen0Spec {}
#[doc = "`write(|w| ..)` method takes [`hash_input_len_0::W`](W) writer structure"]
impl crate::Writable for HashInputLen0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HASH_input_len_0 to value 0"]
impl crate::Resettable for HashInputLen0Spec {}
