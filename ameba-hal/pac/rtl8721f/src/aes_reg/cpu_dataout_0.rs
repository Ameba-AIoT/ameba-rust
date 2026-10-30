#[doc = "Register `cpu_dataout_0` reader"]
pub type R = crate::R<CpuDataout0Spec>;
#[doc = "Field `cpu_dataout_0` reader - Output register bit\\[32*0+31:32*0\\] of the AES engine in the slave mode, only used in Slave_mode. By waiting for the interrupt or polling the aes_slave_done signal to know whether the AES operation is completed. The result of AES will be stored in this register. The data order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, rdata = {cpu_dataout_3\\[7:0\\], cpu_dataout_3\\[15:8\\], cpu_dataout_3\\[23:16\\], cpu_dataout_3\\[31:24\\], cpu_dataout_2\\[7:0\\], cpu_dataout_2\\[15:8\\], cpu_dataout_2\\[23:16\\], cpu_dataout_2\\[31:24\\], cpu_dataout_1\\[7:0\\], cpu_dataout_1\\[15:8\\], cpu_dataout_1\\[23:16\\], cpu_dataout_1\\[31:24\\], cpu_dataout_0\\[7:0\\], cpu_dataout_0\\[15:8\\], cpu_dataout_0\\[23:16\\], cpu_dataout_0\\[31:24\\]} If byte_swap = 1,r data = {cpu_dataout_3, cpu_dataout_2, cpu_dataout_1, cpu_dataout_0} Note that when user starts to read output data, 128bits (4 words) should be read together, that is, user is not allowed to read-write interleave. In other word, no partial read (3 words/2 words/1 words) is allowed, and no write can be inserted between reads."]
pub type CpuDataout0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Output register bit\\[32*0+31:32*0\\] of the AES engine in the slave mode, only used in Slave_mode. By waiting for the interrupt or polling the aes_slave_done signal to know whether the AES operation is completed. The result of AES will be stored in this register. The data order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, rdata = {cpu_dataout_3\\[7:0\\], cpu_dataout_3\\[15:8\\], cpu_dataout_3\\[23:16\\], cpu_dataout_3\\[31:24\\], cpu_dataout_2\\[7:0\\], cpu_dataout_2\\[15:8\\], cpu_dataout_2\\[23:16\\], cpu_dataout_2\\[31:24\\], cpu_dataout_1\\[7:0\\], cpu_dataout_1\\[15:8\\], cpu_dataout_1\\[23:16\\], cpu_dataout_1\\[31:24\\], cpu_dataout_0\\[7:0\\], cpu_dataout_0\\[15:8\\], cpu_dataout_0\\[23:16\\], cpu_dataout_0\\[31:24\\]} If byte_swap = 1,r data = {cpu_dataout_3, cpu_dataout_2, cpu_dataout_1, cpu_dataout_0} Note that when user starts to read output data, 128bits (4 words) should be read together, that is, user is not allowed to read-write interleave. In other word, no partial read (3 words/2 words/1 words) is allowed, and no write can be inserted between reads."]
    #[inline(always)]
    pub fn cpu_dataout_0(&self) -> CpuDataout0R {
        CpuDataout0R::new(self.bits)
    }
}
#[doc = "CPU Data Output register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_dataout_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuDataout0Spec;
impl crate::RegisterSpec for CpuDataout0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_dataout_0::R`](R) reader structure"]
impl crate::Readable for CpuDataout0Spec {}
#[doc = "`reset()` method sets cpu_dataout_0 to value 0"]
impl crate::Resettable for CpuDataout0Spec {}
