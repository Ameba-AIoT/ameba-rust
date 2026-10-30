#[doc = "Register `chn0_aes_int_mask` reader"]
pub type R = crate::R<Chn0AesIntMaskSpec>;
#[doc = "Register `chn0_aes_int_mask` writer"]
pub type W = crate::W<Chn0AesIntMaskSpec>;
#[doc = "Field `aes_slave_done_mask` reader - Interrupt aes_slave_done enable."]
pub type AesSlaveDoneMaskR = crate::BitReader;
#[doc = "Field `aes_slave_done_mask` writer - Interrupt aes_slave_done enable."]
pub type AesSlaveDoneMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `error_block_size_mask` reader - Interrupt error_block_size enable."]
pub type ErrorBlockSizeMaskR = crate::BitReader;
#[doc = "Field `error_block_size_mask` writer - Interrupt error_block_size enable."]
pub type ErrorBlockSizeMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `error_decode_cipher_mask` reader - Interrupt error_decode_cipher enable."]
pub type ErrorDecodeCipherMaskR = crate::BitReader;
#[doc = "Field `error_decode_cipher_mask` writer - Interrupt error_decode_cipher enable."]
pub type ErrorDecodeCipherMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `error_ende_circuit_mask` reader - Interrupt error_ende_circuit enable."]
pub type ErrorEndeCircuitMaskR = crate::BitReader;
#[doc = "Field `error_ende_circuit_mask` writer - Interrupt error_ende_circuit enable."]
pub type ErrorEndeCircuitMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `error_cpu_fifo_mask` reader - Interrupt error_cpu_fifo enable. *available after version 2024/08/19"]
pub type ErrorCpuFifoMaskR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt aes_slave_done enable."]
    #[inline(always)]
    pub fn aes_slave_done_mask(&self) -> AesSlaveDoneMaskR {
        AesSlaveDoneMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt error_block_size enable."]
    #[inline(always)]
    pub fn error_block_size_mask(&self) -> ErrorBlockSizeMaskR {
        ErrorBlockSizeMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt error_decode_cipher enable."]
    #[inline(always)]
    pub fn error_decode_cipher_mask(&self) -> ErrorDecodeCipherMaskR {
        ErrorDecodeCipherMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt error_ende_circuit enable."]
    #[inline(always)]
    pub fn error_ende_circuit_mask(&self) -> ErrorEndeCircuitMaskR {
        ErrorEndeCircuitMaskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt error_cpu_fifo enable. *available after version 2024/08/19"]
    #[inline(always)]
    pub fn error_cpu_fifo_mask(&self) -> ErrorCpuFifoMaskR {
        ErrorCpuFifoMaskR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt aes_slave_done enable."]
    #[inline(always)]
    pub fn aes_slave_done_mask(&mut self) -> AesSlaveDoneMaskW<'_, Chn0AesIntMaskSpec> {
        AesSlaveDoneMaskW::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt error_block_size enable."]
    #[inline(always)]
    pub fn error_block_size_mask(&mut self) -> ErrorBlockSizeMaskW<'_, Chn0AesIntMaskSpec> {
        ErrorBlockSizeMaskW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt error_decode_cipher enable."]
    #[inline(always)]
    pub fn error_decode_cipher_mask(&mut self) -> ErrorDecodeCipherMaskW<'_, Chn0AesIntMaskSpec> {
        ErrorDecodeCipherMaskW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt error_ende_circuit enable."]
    #[inline(always)]
    pub fn error_ende_circuit_mask(&mut self) -> ErrorEndeCircuitMaskW<'_, Chn0AesIntMaskSpec> {
        ErrorEndeCircuitMaskW::new(self, 3)
    }
}
#[doc = "Enable interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`chn0_aes_int_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn0_aes_int_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chn0AesIntMaskSpec;
impl crate::RegisterSpec for Chn0AesIntMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chn0_aes_int_mask::R`](R) reader structure"]
impl crate::Readable for Chn0AesIntMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`chn0_aes_int_mask::W`](W) writer structure"]
impl crate::Writable for Chn0AesIntMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets chn0_aes_int_mask to value 0x1f"]
impl crate::Resettable for Chn0AesIntMaskSpec {
    const RESET_VALUE: u32 = 0x1f;
}
