#[doc = "Register `chn0_aes_int_status` reader"]
pub type R = crate::R<Chn0AesIntStatusSpec>;
#[doc = "Field `aes_slave_done_status` reader - Masked aes_slave_done."]
pub type AesSlaveDoneStatusR = crate::BitReader;
#[doc = "Field `error_block_size_status` reader - Masked error_block_size."]
pub type ErrorBlockSizeStatusR = crate::BitReader;
#[doc = "Field `error_decode_cipher_status` reader - Masked error_decode_cipher."]
pub type ErrorDecodeCipherStatusR = crate::BitReader;
#[doc = "Field `error_ende_circuit_status` reader - Masked error_ende_circuit."]
pub type ErrorEndeCircuitStatusR = crate::BitReader;
#[doc = "Field `error_cpu_fifo_status` reader - Masked error_cpu_fifo *available after version 2024/08/19"]
pub type ErrorCpuFifoStatusR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Masked aes_slave_done."]
    #[inline(always)]
    pub fn aes_slave_done_status(&self) -> AesSlaveDoneStatusR {
        AesSlaveDoneStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masked error_block_size."]
    #[inline(always)]
    pub fn error_block_size_status(&self) -> ErrorBlockSizeStatusR {
        ErrorBlockSizeStatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masked error_decode_cipher."]
    #[inline(always)]
    pub fn error_decode_cipher_status(&self) -> ErrorDecodeCipherStatusR {
        ErrorDecodeCipherStatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masked error_ende_circuit."]
    #[inline(always)]
    pub fn error_ende_circuit_status(&self) -> ErrorEndeCircuitStatusR {
        ErrorEndeCircuitStatusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Masked error_cpu_fifo *available after version 2024/08/19"]
    #[inline(always)]
    pub fn error_cpu_fifo_status(&self) -> ErrorCpuFifoStatusR {
        ErrorCpuFifoStatusR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Masked interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`chn0_aes_int_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chn0AesIntStatusSpec;
impl crate::RegisterSpec for Chn0AesIntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chn0_aes_int_status::R`](R) reader structure"]
impl crate::Readable for Chn0AesIntStatusSpec {}
#[doc = "`reset()` method sets chn0_aes_int_status to value 0x10"]
impl crate::Resettable for Chn0AesIntStatusSpec {
    const RESET_VALUE: u32 = 0x10;
}
