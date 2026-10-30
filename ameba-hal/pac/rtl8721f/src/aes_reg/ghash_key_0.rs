#[doc = "Register `ghash_key_0` reader"]
pub type R = crate::R<GhashKey0Spec>;
#[doc = "Register `ghash_key_0` writer"]
pub type W = crate::W<GhashKey0Spec>;
#[doc = "Field `ghash_key_0` reader - GHASH key register bit\\[32*0+31:32*0\\] If using GCM mode without AAD, need to first calculate the ghash key and write it to the register. To generate the ghash key, apply the AES block cipher to the \"zero\" block with actual key. The ghash_keyorder depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, ghash_key = {ghash_key_3\\[7:0\\], ghash_key_3\\[15:8\\], ghash_key_3\\[23:16\\], ghash_key_3\\[31:24\\], ghash_key_2\\[7:0\\], ghash_key_2\\[15:8\\], ghash_key_2\\[23:16\\], ghash_key_2\\[31:24\\], ghash_key_1\\[7:0\\], ghash_key_1\\[15:8\\], ghash_key_1\\[23:16\\], ghash_key_1\\[31:24\\], ghash_key_0\\[7:0\\], ghash_key_0\\[15:8\\], ghash_key_0\\[23:16\\], ghash_key_0\\[31:24\\]} If byte_swap = 1, ghash_key = {ghash_key_3, ghash_key_2, ghash_key_1, ghash_key_0}"]
pub type GhashKey0R = crate::FieldReader<u32>;
#[doc = "Field `ghash_key_0` writer - GHASH key register bit\\[32*0+31:32*0\\] If using GCM mode without AAD, need to first calculate the ghash key and write it to the register. To generate the ghash key, apply the AES block cipher to the \"zero\" block with actual key. The ghash_keyorder depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, ghash_key = {ghash_key_3\\[7:0\\], ghash_key_3\\[15:8\\], ghash_key_3\\[23:16\\], ghash_key_3\\[31:24\\], ghash_key_2\\[7:0\\], ghash_key_2\\[15:8\\], ghash_key_2\\[23:16\\], ghash_key_2\\[31:24\\], ghash_key_1\\[7:0\\], ghash_key_1\\[15:8\\], ghash_key_1\\[23:16\\], ghash_key_1\\[31:24\\], ghash_key_0\\[7:0\\], ghash_key_0\\[15:8\\], ghash_key_0\\[23:16\\], ghash_key_0\\[31:24\\]} If byte_swap = 1, ghash_key = {ghash_key_3, ghash_key_2, ghash_key_1, ghash_key_0}"]
pub type GhashKey0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GHASH key register bit\\[32*0+31:32*0\\] If using GCM mode without AAD, need to first calculate the ghash key and write it to the register. To generate the ghash key, apply the AES block cipher to the \"zero\" block with actual key. The ghash_keyorder depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, ghash_key = {ghash_key_3\\[7:0\\], ghash_key_3\\[15:8\\], ghash_key_3\\[23:16\\], ghash_key_3\\[31:24\\], ghash_key_2\\[7:0\\], ghash_key_2\\[15:8\\], ghash_key_2\\[23:16\\], ghash_key_2\\[31:24\\], ghash_key_1\\[7:0\\], ghash_key_1\\[15:8\\], ghash_key_1\\[23:16\\], ghash_key_1\\[31:24\\], ghash_key_0\\[7:0\\], ghash_key_0\\[15:8\\], ghash_key_0\\[23:16\\], ghash_key_0\\[31:24\\]} If byte_swap = 1, ghash_key = {ghash_key_3, ghash_key_2, ghash_key_1, ghash_key_0}"]
    #[inline(always)]
    pub fn ghash_key_0(&self) -> GhashKey0R {
        GhashKey0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GHASH key register bit\\[32*0+31:32*0\\] If using GCM mode without AAD, need to first calculate the ghash key and write it to the register. To generate the ghash key, apply the AES block cipher to the \"zero\" block with actual key. The ghash_keyorder depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, ghash_key = {ghash_key_3\\[7:0\\], ghash_key_3\\[15:8\\], ghash_key_3\\[23:16\\], ghash_key_3\\[31:24\\], ghash_key_2\\[7:0\\], ghash_key_2\\[15:8\\], ghash_key_2\\[23:16\\], ghash_key_2\\[31:24\\], ghash_key_1\\[7:0\\], ghash_key_1\\[15:8\\], ghash_key_1\\[23:16\\], ghash_key_1\\[31:24\\], ghash_key_0\\[7:0\\], ghash_key_0\\[15:8\\], ghash_key_0\\[23:16\\], ghash_key_0\\[31:24\\]} If byte_swap = 1, ghash_key = {ghash_key_3, ghash_key_2, ghash_key_1, ghash_key_0}"]
    #[inline(always)]
    pub fn ghash_key_0(&mut self) -> GhashKey0W<'_, GhashKey0Spec> {
        GhashKey0W::new(self, 0)
    }
}
#[doc = "GHASH key register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ghash_key_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghash_key_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GhashKey0Spec;
impl crate::RegisterSpec for GhashKey0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghash_key_0::R`](R) reader structure"]
impl crate::Readable for GhashKey0Spec {}
#[doc = "`write(|w| ..)` method takes [`ghash_key_0::W`](W) writer structure"]
impl crate::Writable for GhashKey0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ghash_key_0 to value 0"]
impl crate::Resettable for GhashKey0Spec {}
