#[doc = "Register `HASH_input_len_2` reader"]
pub type R = crate::R<HashInputLen2Spec>;
#[doc = "Register `HASH_input_len_2` writer"]
pub type W = crate::W<HashInputLen2Spec>;
#[doc = "Field `HASH_input_len2` reader - Hash length\\[32*(3-2)+31:32*(3-2)\\]"]
pub type HashInputLen2R = crate::FieldReader<u32>;
#[doc = "Field `HASH_input_len2` writer - Hash length\\[32*(3-2)+31:32*(3-2)\\]"]
pub type HashInputLen2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash length\\[32*(3-2)+31:32*(3-2)\\]"]
    #[inline(always)]
    pub fn hash_input_len2(&self) -> HashInputLen2R {
        HashInputLen2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash length\\[32*(3-2)+31:32*(3-2)\\]"]
    #[inline(always)]
    pub fn hash_input_len2(&mut self) -> HashInputLen2W<'_, HashInputLen2Spec> {
        HashInputLen2W::new(self, 0)
    }
}
#[doc = "Total input byte length register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_input_len_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_input_len_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashInputLen2Spec;
impl crate::RegisterSpec for HashInputLen2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_input_len_2::R`](R) reader structure"]
impl crate::Readable for HashInputLen2Spec {}
#[doc = "`write(|w| ..)` method takes [`hash_input_len_2::W`](W) writer structure"]
impl crate::Writable for HashInputLen2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HASH_input_len_2 to value 0"]
impl crate::Resettable for HashInputLen2Spec {}
