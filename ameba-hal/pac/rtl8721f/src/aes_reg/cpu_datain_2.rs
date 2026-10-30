#[doc = "Register `cpu_datain_2` reader"]
pub type R = crate::R<CpuDatain2Spec>;
#[doc = "Register `cpu_datain_2` writer"]
pub type W = crate::W<CpuDatain2Spec>;
#[doc = "Field `cpu_datain_2` reader - Input register bit\\[32*2+31:32*2\\] of the AES engine in the slave mode, only used in Slave_mode. As input to the engine, such as plaintext or ciphertext, the input order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, data = {cpu_datain_3\\[7:0\\], cpu_datain_3\\[15:8\\], cpu_datain_3\\[23:16\\], cpu_datain_3\\[31:24\\], cpu_datain_2\\[7:0\\], cpu_datain_2\\[15:8\\], cpu_datain_2\\[23:16\\], cpu_datain_2\\[31:24\\], cpu_datain_1\\[7:0\\], cpu_datain_1\\[15:8\\], cpu_datain_1\\[23:16\\], cpu_datain_1\\[31:24\\], cpu_datain_0\\[7:0\\], cpu_datain_0\\[15:8\\], cpu_datain_0\\[23:16\\], cpu_datain_0\\[31:24\\]} If byte_swap = 1, data = {cpu_datain_3, cpu_datain_2, cpu_datain_1, cpu_datain_0} Note that when user starts to write input data, 128bits (4 words) should be written together, that is, user is not allowed to read-write interleave. In other word, no partial write (3 words/2 words/1 words) is allowed, and no read can be inserted between writes."]
pub type CpuDatain2R = crate::FieldReader<u32>;
#[doc = "Field `cpu_datain_2` writer - Input register bit\\[32*2+31:32*2\\] of the AES engine in the slave mode, only used in Slave_mode. As input to the engine, such as plaintext or ciphertext, the input order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, data = {cpu_datain_3\\[7:0\\], cpu_datain_3\\[15:8\\], cpu_datain_3\\[23:16\\], cpu_datain_3\\[31:24\\], cpu_datain_2\\[7:0\\], cpu_datain_2\\[15:8\\], cpu_datain_2\\[23:16\\], cpu_datain_2\\[31:24\\], cpu_datain_1\\[7:0\\], cpu_datain_1\\[15:8\\], cpu_datain_1\\[23:16\\], cpu_datain_1\\[31:24\\], cpu_datain_0\\[7:0\\], cpu_datain_0\\[15:8\\], cpu_datain_0\\[23:16\\], cpu_datain_0\\[31:24\\]} If byte_swap = 1, data = {cpu_datain_3, cpu_datain_2, cpu_datain_1, cpu_datain_0} Note that when user starts to write input data, 128bits (4 words) should be written together, that is, user is not allowed to read-write interleave. In other word, no partial write (3 words/2 words/1 words) is allowed, and no read can be inserted between writes."]
pub type CpuDatain2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Input register bit\\[32*2+31:32*2\\] of the AES engine in the slave mode, only used in Slave_mode. As input to the engine, such as plaintext or ciphertext, the input order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, data = {cpu_datain_3\\[7:0\\], cpu_datain_3\\[15:8\\], cpu_datain_3\\[23:16\\], cpu_datain_3\\[31:24\\], cpu_datain_2\\[7:0\\], cpu_datain_2\\[15:8\\], cpu_datain_2\\[23:16\\], cpu_datain_2\\[31:24\\], cpu_datain_1\\[7:0\\], cpu_datain_1\\[15:8\\], cpu_datain_1\\[23:16\\], cpu_datain_1\\[31:24\\], cpu_datain_0\\[7:0\\], cpu_datain_0\\[15:8\\], cpu_datain_0\\[23:16\\], cpu_datain_0\\[31:24\\]} If byte_swap = 1, data = {cpu_datain_3, cpu_datain_2, cpu_datain_1, cpu_datain_0} Note that when user starts to write input data, 128bits (4 words) should be written together, that is, user is not allowed to read-write interleave. In other word, no partial write (3 words/2 words/1 words) is allowed, and no read can be inserted between writes."]
    #[inline(always)]
    pub fn cpu_datain_2(&self) -> CpuDatain2R {
        CpuDatain2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Input register bit\\[32*2+31:32*2\\] of the AES engine in the slave mode, only used in Slave_mode. As input to the engine, such as plaintext or ciphertext, the input order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, data = {cpu_datain_3\\[7:0\\], cpu_datain_3\\[15:8\\], cpu_datain_3\\[23:16\\], cpu_datain_3\\[31:24\\], cpu_datain_2\\[7:0\\], cpu_datain_2\\[15:8\\], cpu_datain_2\\[23:16\\], cpu_datain_2\\[31:24\\], cpu_datain_1\\[7:0\\], cpu_datain_1\\[15:8\\], cpu_datain_1\\[23:16\\], cpu_datain_1\\[31:24\\], cpu_datain_0\\[7:0\\], cpu_datain_0\\[15:8\\], cpu_datain_0\\[23:16\\], cpu_datain_0\\[31:24\\]} If byte_swap = 1, data = {cpu_datain_3, cpu_datain_2, cpu_datain_1, cpu_datain_0} Note that when user starts to write input data, 128bits (4 words) should be written together, that is, user is not allowed to read-write interleave. In other word, no partial write (3 words/2 words/1 words) is allowed, and no read can be inserted between writes."]
    #[inline(always)]
    pub fn cpu_datain_2(&mut self) -> CpuDatain2W<'_, CpuDatain2Spec> {
        CpuDatain2W::new(self, 0)
    }
}
#[doc = "CPU Data Input register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_datain_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_datain_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuDatain2Spec;
impl crate::RegisterSpec for CpuDatain2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_datain_2::R`](R) reader structure"]
impl crate::Readable for CpuDatain2Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_datain_2::W`](W) writer structure"]
impl crate::Writable for CpuDatain2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets cpu_datain_2 to value 0"]
impl crate::Resettable for CpuDatain2Spec {}
