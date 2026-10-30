#[doc = "Register `chn0_aes_int_raw` reader"]
pub type R = crate::R<Chn0AesIntRawSpec>;
#[doc = "Field `aes_slave_done_raw` reader - Set to 1 when aes has output data waiting for user to read (*after version 2024/08/19)"]
pub type AesSlaveDoneRawR = crate::BitReader;
#[doc = "Field `error_block_size_raw` reader - Set to 1 when the input size is not a multiple of 128 when using ECB or CBC mode."]
pub type ErrorBlockSizeRawR = crate::BitReader;
#[doc = "Field `error_decode_cipher_raw` reader - Set to 1 when the user sets a non-existent cipher mode."]
pub type ErrorDecodeCipherRawR = crate::BitReader;
#[doc = "Field `error_ende_circuit_raw` reader - Set to 1 when the user uses the corresponding operation when closing the encryption or decryption circuit."]
pub type ErrorEndeCircuitRawR = crate::BitReader;
#[doc = "Field `error_cpu_fifo_raw` reader - Set to 1 when the user uses slave mode and writes data when input data fifo is full or reads data when output data fifo is empty. (error condition is related to ctrl.block_pready) *available after version 2024/08/19"]
pub type ErrorCpuFifoRawR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set to 1 when aes has output data waiting for user to read (*after version 2024/08/19)"]
    #[inline(always)]
    pub fn aes_slave_done_raw(&self) -> AesSlaveDoneRawR {
        AesSlaveDoneRawR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set to 1 when the input size is not a multiple of 128 when using ECB or CBC mode."]
    #[inline(always)]
    pub fn error_block_size_raw(&self) -> ErrorBlockSizeRawR {
        ErrorBlockSizeRawR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set to 1 when the user sets a non-existent cipher mode."]
    #[inline(always)]
    pub fn error_decode_cipher_raw(&self) -> ErrorDecodeCipherRawR {
        ErrorDecodeCipherRawR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set to 1 when the user uses the corresponding operation when closing the encryption or decryption circuit."]
    #[inline(always)]
    pub fn error_ende_circuit_raw(&self) -> ErrorEndeCircuitRawR {
        ErrorEndeCircuitRawR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set to 1 when the user uses slave mode and writes data when input data fifo is full or reads data when output data fifo is empty. (error condition is related to ctrl.block_pready) *available after version 2024/08/19"]
    #[inline(always)]
    pub fn error_cpu_fifo_raw(&self) -> ErrorCpuFifoRawR {
        ErrorCpuFifoRawR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Raw interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`chn0_aes_int_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chn0AesIntRawSpec;
impl crate::RegisterSpec for Chn0AesIntRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chn0_aes_int_raw::R`](R) reader structure"]
impl crate::Readable for Chn0AesIntRawSpec {}
#[doc = "`reset()` method sets chn0_aes_int_raw to value 0"]
impl crate::Resettable for Chn0AesIntRawSpec {}
