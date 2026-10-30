#[doc = "Register `iv_1` reader"]
pub type R = crate::R<Iv1Spec>;
#[doc = "Register `iv_1` writer"]
pub type W = crate::W<Iv1Spec>;
#[doc = "Field `iv_1` reader - IV register bit\\[32*1+31:32*1\\] Except for ECB mode, other modes require IV as input. The user should set the IV before using the AES engine. The IV order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, iv = {iv_3\\[7:0\\], iv_3\\[15:8\\], iv_3\\[23:16\\], iv_3\\[31:24\\], iv_2\\[7:0\\], iv_2\\[15:8\\], iv_2\\[23:16\\], iv_2\\[31:24\\], iv_1\\[7:0\\], iv_1\\[15:8\\], iv_1\\[23:16\\], iv_1\\[31:24\\], iv_0\\[7:0\\], iv_0\\[15:8\\], iv_0\\[23:16\\], iv_0\\[31:24\\]} If byte_swap = 1, iv = {iv_3, iv_2, iv_1, iv_0}"]
pub type Iv1R = crate::FieldReader<u32>;
#[doc = "Field `iv_1` writer - IV register bit\\[32*1+31:32*1\\] Except for ECB mode, other modes require IV as input. The user should set the IV before using the AES engine. The IV order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, iv = {iv_3\\[7:0\\], iv_3\\[15:8\\], iv_3\\[23:16\\], iv_3\\[31:24\\], iv_2\\[7:0\\], iv_2\\[15:8\\], iv_2\\[23:16\\], iv_2\\[31:24\\], iv_1\\[7:0\\], iv_1\\[15:8\\], iv_1\\[23:16\\], iv_1\\[31:24\\], iv_0\\[7:0\\], iv_0\\[15:8\\], iv_0\\[23:16\\], iv_0\\[31:24\\]} If byte_swap = 1, iv = {iv_3, iv_2, iv_1, iv_0}"]
pub type Iv1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IV register bit\\[32*1+31:32*1\\] Except for ECB mode, other modes require IV as input. The user should set the IV before using the AES engine. The IV order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, iv = {iv_3\\[7:0\\], iv_3\\[15:8\\], iv_3\\[23:16\\], iv_3\\[31:24\\], iv_2\\[7:0\\], iv_2\\[15:8\\], iv_2\\[23:16\\], iv_2\\[31:24\\], iv_1\\[7:0\\], iv_1\\[15:8\\], iv_1\\[23:16\\], iv_1\\[31:24\\], iv_0\\[7:0\\], iv_0\\[15:8\\], iv_0\\[23:16\\], iv_0\\[31:24\\]} If byte_swap = 1, iv = {iv_3, iv_2, iv_1, iv_0}"]
    #[inline(always)]
    pub fn iv_1(&self) -> Iv1R {
        Iv1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IV register bit\\[32*1+31:32*1\\] Except for ECB mode, other modes require IV as input. The user should set the IV before using the AES engine. The IV order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, iv = {iv_3\\[7:0\\], iv_3\\[15:8\\], iv_3\\[23:16\\], iv_3\\[31:24\\], iv_2\\[7:0\\], iv_2\\[15:8\\], iv_2\\[23:16\\], iv_2\\[31:24\\], iv_1\\[7:0\\], iv_1\\[15:8\\], iv_1\\[23:16\\], iv_1\\[31:24\\], iv_0\\[7:0\\], iv_0\\[15:8\\], iv_0\\[23:16\\], iv_0\\[31:24\\]} If byte_swap = 1, iv = {iv_3, iv_2, iv_1, iv_0}"]
    #[inline(always)]
    pub fn iv_1(&mut self) -> Iv1W<'_, Iv1Spec> {
        Iv1W::new(self, 0)
    }
}
#[doc = "IV register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`iv_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iv1Spec;
impl crate::RegisterSpec for Iv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iv_1::R`](R) reader structure"]
impl crate::Readable for Iv1Spec {}
#[doc = "`write(|w| ..)` method takes [`iv_1::W`](W) writer structure"]
impl crate::Writable for Iv1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets iv_1 to value 0"]
impl crate::Resettable for Iv1Spec {}
