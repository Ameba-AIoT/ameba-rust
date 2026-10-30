#[doc = "Register `ghash_key_3` reader"]
pub type R = crate::R<GhashKey3Spec>;
#[doc = "Register `ghash_key_3` writer"]
pub type W = crate::W<GhashKey3Spec>;
#[doc = "Field `ghash_key_3` reader - GHASH key register bit\\[32*3+31:32*3\\] If using GCM mode without AAD, need to first calculate the ghash key and write it to the register. To generate the ghash key, apply the AES block cipher to the \"zero\" block with actual key. The ghash_keyorder depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, ghash_key = {ghash_key_3\\[7:0\\], ghash_key_3\\[15:8\\], ghash_key_3\\[23:16\\], ghash_key_3\\[31:24\\], ghash_key_2\\[7:0\\], ghash_key_2\\[15:8\\], ghash_key_2\\[23:16\\], ghash_key_2\\[31:24\\], ghash_key_1\\[7:0\\], ghash_key_1\\[15:8\\], ghash_key_1\\[23:16\\], ghash_key_1\\[31:24\\], ghash_key_0\\[7:0\\], ghash_key_0\\[15:8\\], ghash_key_0\\[23:16\\], ghash_key_0\\[31:24\\]} If byte_swap = 1, ghash_key = {ghash_key_3, ghash_key_2, ghash_key_1, ghash_key_0}"]
pub type GhashKey3R = crate::FieldReader<u32>;
#[doc = "Field `ghash_key_3` writer - GHASH key register bit\\[32*3+31:32*3\\] If using GCM mode without AAD, need to first calculate the ghash key and write it to the register. To generate the ghash key, apply the AES block cipher to the \"zero\" block with actual key. The ghash_keyorder depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, ghash_key = {ghash_key_3\\[7:0\\], ghash_key_3\\[15:8\\], ghash_key_3\\[23:16\\], ghash_key_3\\[31:24\\], ghash_key_2\\[7:0\\], ghash_key_2\\[15:8\\], ghash_key_2\\[23:16\\], ghash_key_2\\[31:24\\], ghash_key_1\\[7:0\\], ghash_key_1\\[15:8\\], ghash_key_1\\[23:16\\], ghash_key_1\\[31:24\\], ghash_key_0\\[7:0\\], ghash_key_0\\[15:8\\], ghash_key_0\\[23:16\\], ghash_key_0\\[31:24\\]} If byte_swap = 1, ghash_key = {ghash_key_3, ghash_key_2, ghash_key_1, ghash_key_0}"]
pub type GhashKey3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GHASH key register bit\\[32*3+31:32*3\\] If using GCM mode without AAD, need to first calculate the ghash key and write it to the register. To generate the ghash key, apply the AES block cipher to the \"zero\" block with actual key. The ghash_keyorder depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, ghash_key = {ghash_key_3\\[7:0\\], ghash_key_3\\[15:8\\], ghash_key_3\\[23:16\\], ghash_key_3\\[31:24\\], ghash_key_2\\[7:0\\], ghash_key_2\\[15:8\\], ghash_key_2\\[23:16\\], ghash_key_2\\[31:24\\], ghash_key_1\\[7:0\\], ghash_key_1\\[15:8\\], ghash_key_1\\[23:16\\], ghash_key_1\\[31:24\\], ghash_key_0\\[7:0\\], ghash_key_0\\[15:8\\], ghash_key_0\\[23:16\\], ghash_key_0\\[31:24\\]} If byte_swap = 1, ghash_key = {ghash_key_3, ghash_key_2, ghash_key_1, ghash_key_0}"]
    #[inline(always)]
    pub fn ghash_key_3(&self) -> GhashKey3R {
        GhashKey3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GHASH key register bit\\[32*3+31:32*3\\] If using GCM mode without AAD, need to first calculate the ghash key and write it to the register. To generate the ghash key, apply the AES block cipher to the \"zero\" block with actual key. The ghash_keyorder depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, ghash_key = {ghash_key_3\\[7:0\\], ghash_key_3\\[15:8\\], ghash_key_3\\[23:16\\], ghash_key_3\\[31:24\\], ghash_key_2\\[7:0\\], ghash_key_2\\[15:8\\], ghash_key_2\\[23:16\\], ghash_key_2\\[31:24\\], ghash_key_1\\[7:0\\], ghash_key_1\\[15:8\\], ghash_key_1\\[23:16\\], ghash_key_1\\[31:24\\], ghash_key_0\\[7:0\\], ghash_key_0\\[15:8\\], ghash_key_0\\[23:16\\], ghash_key_0\\[31:24\\]} If byte_swap = 1, ghash_key = {ghash_key_3, ghash_key_2, ghash_key_1, ghash_key_0}"]
    #[inline(always)]
    pub fn ghash_key_3(&mut self) -> GhashKey3W<'_, GhashKey3Spec> {
        GhashKey3W::new(self, 0)
    }
}
#[doc = "GHASH key register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ghash_key_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghash_key_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GhashKey3Spec;
impl crate::RegisterSpec for GhashKey3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghash_key_3::R`](R) reader structure"]
impl crate::Readable for GhashKey3Spec {}
#[doc = "`write(|w| ..)` method takes [`ghash_key_3::W`](W) writer structure"]
impl crate::Writable for GhashKey3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ghash_key_3 to value 0"]
impl crate::Resettable for GhashKey3Spec {}
