#[doc = "Register `HASH_input_len_3` reader"]
pub type R = crate::R<HashInputLen3Spec>;
#[doc = "Register `HASH_input_len_3` writer"]
pub type W = crate::W<HashInputLen3Spec>;
#[doc = "Field `HASH_input_len3` reader - Hash length\\[32*(3-3)+31:32*(3-3)\\]"]
pub type HashInputLen3R = crate::FieldReader<u32>;
#[doc = "Field `HASH_input_len3` writer - Hash length\\[32*(3-3)+31:32*(3-3)\\]"]
pub type HashInputLen3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash length\\[32*(3-3)+31:32*(3-3)\\]"]
    #[inline(always)]
    pub fn hash_input_len3(&self) -> HashInputLen3R {
        HashInputLen3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash length\\[32*(3-3)+31:32*(3-3)\\]"]
    #[inline(always)]
    pub fn hash_input_len3(&mut self) -> HashInputLen3W<'_, HashInputLen3Spec> {
        HashInputLen3W::new(self, 0)
    }
}
#[doc = "Total input byte length register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_input_len_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_input_len_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashInputLen3Spec;
impl crate::RegisterSpec for HashInputLen3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_input_len_3::R`](R) reader structure"]
impl crate::Readable for HashInputLen3Spec {}
#[doc = "`write(|w| ..)` method takes [`hash_input_len_3::W`](W) writer structure"]
impl crate::Writable for HashInputLen3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HASH_input_len_3 to value 0"]
impl crate::Resettable for HashInputLen3Spec {}
