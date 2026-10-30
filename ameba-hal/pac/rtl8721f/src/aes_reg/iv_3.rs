#[doc = "Register `iv_3` reader"]
pub type R = crate::R<Iv3Spec>;
#[doc = "Register `iv_3` writer"]
pub type W = crate::W<Iv3Spec>;
#[doc = "Field `iv_3` reader - IV register bit\\[32*3+31:32*3\\] Except for ECB mode, other modes require IV as input. The user should set the IV before using the AES engine. The IV order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, iv = {iv_3\\[7:0\\], iv_3\\[15:8\\], iv_3\\[23:16\\], iv_3\\[31:24\\], iv_2\\[7:0\\], iv_2\\[15:8\\], iv_2\\[23:16\\], iv_2\\[31:24\\], iv_1\\[7:0\\], iv_1\\[15:8\\], iv_1\\[23:16\\], iv_1\\[31:24\\], iv_0\\[7:0\\], iv_0\\[15:8\\], iv_0\\[23:16\\], iv_0\\[31:24\\]} If byte_swap = 1, iv = {iv_3, iv_2, iv_1, iv_0}"]
pub type Iv3R = crate::FieldReader<u32>;
#[doc = "Field `iv_3` writer - IV register bit\\[32*3+31:32*3\\] Except for ECB mode, other modes require IV as input. The user should set the IV before using the AES engine. The IV order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, iv = {iv_3\\[7:0\\], iv_3\\[15:8\\], iv_3\\[23:16\\], iv_3\\[31:24\\], iv_2\\[7:0\\], iv_2\\[15:8\\], iv_2\\[23:16\\], iv_2\\[31:24\\], iv_1\\[7:0\\], iv_1\\[15:8\\], iv_1\\[23:16\\], iv_1\\[31:24\\], iv_0\\[7:0\\], iv_0\\[15:8\\], iv_0\\[23:16\\], iv_0\\[31:24\\]} If byte_swap = 1, iv = {iv_3, iv_2, iv_1, iv_0}"]
pub type Iv3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IV register bit\\[32*3+31:32*3\\] Except for ECB mode, other modes require IV as input. The user should set the IV before using the AES engine. The IV order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, iv = {iv_3\\[7:0\\], iv_3\\[15:8\\], iv_3\\[23:16\\], iv_3\\[31:24\\], iv_2\\[7:0\\], iv_2\\[15:8\\], iv_2\\[23:16\\], iv_2\\[31:24\\], iv_1\\[7:0\\], iv_1\\[15:8\\], iv_1\\[23:16\\], iv_1\\[31:24\\], iv_0\\[7:0\\], iv_0\\[15:8\\], iv_0\\[23:16\\], iv_0\\[31:24\\]} If byte_swap = 1, iv = {iv_3, iv_2, iv_1, iv_0}"]
    #[inline(always)]
    pub fn iv_3(&self) -> Iv3R {
        Iv3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IV register bit\\[32*3+31:32*3\\] Except for ECB mode, other modes require IV as input. The user should set the IV before using the AES engine. The IV order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, iv = {iv_3\\[7:0\\], iv_3\\[15:8\\], iv_3\\[23:16\\], iv_3\\[31:24\\], iv_2\\[7:0\\], iv_2\\[15:8\\], iv_2\\[23:16\\], iv_2\\[31:24\\], iv_1\\[7:0\\], iv_1\\[15:8\\], iv_1\\[23:16\\], iv_1\\[31:24\\], iv_0\\[7:0\\], iv_0\\[15:8\\], iv_0\\[23:16\\], iv_0\\[31:24\\]} If byte_swap = 1, iv = {iv_3, iv_2, iv_1, iv_0}"]
    #[inline(always)]
    pub fn iv_3(&mut self) -> Iv3W<'_, Iv3Spec> {
        Iv3W::new(self, 0)
    }
}
#[doc = "IV register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`iv_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iv3Spec;
impl crate::RegisterSpec for Iv3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iv_3::R`](R) reader structure"]
impl crate::Readable for Iv3Spec {}
#[doc = "`write(|w| ..)` method takes [`iv_3::W`](W) writer structure"]
impl crate::Writable for Iv3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets iv_3 to value 0"]
impl crate::Resettable for Iv3Spec {}
