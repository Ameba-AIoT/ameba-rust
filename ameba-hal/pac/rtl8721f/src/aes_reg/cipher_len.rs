#[doc = "Register `cipher_len` reader"]
pub type R = crate::R<CipherLenSpec>;
#[doc = "Register `cipher_len` writer"]
pub type W = crate::W<CipherLenSpec>;
#[doc = "Field `cipher_len` reader - In the CFB, OFB and CTR mode, whether the last block is 16-byte aligned affects the length of the output. In the XTS and CMAC mode, the last block requires additional computation. The user needs to provide the total length for reference. In the GCM mode, the user needs to inform the length of the plaintext or ciphertext. The plaintext or ciphertext does not need to be given at once. But every time add data before the end of the calculation, the user needs to update the data length. Will be used in the last GMAC calculation of GCM."]
pub type CipherLenR = crate::FieldReader<u32>;
#[doc = "Field `cipher_len` writer - In the CFB, OFB and CTR mode, whether the last block is 16-byte aligned affects the length of the output. In the XTS and CMAC mode, the last block requires additional computation. The user needs to provide the total length for reference. In the GCM mode, the user needs to inform the length of the plaintext or ciphertext. The plaintext or ciphertext does not need to be given at once. But every time add data before the end of the calculation, the user needs to update the data length. Will be used in the last GMAC calculation of GCM."]
pub type CipherLenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - In the CFB, OFB and CTR mode, whether the last block is 16-byte aligned affects the length of the output. In the XTS and CMAC mode, the last block requires additional computation. The user needs to provide the total length for reference. In the GCM mode, the user needs to inform the length of the plaintext or ciphertext. The plaintext or ciphertext does not need to be given at once. But every time add data before the end of the calculation, the user needs to update the data length. Will be used in the last GMAC calculation of GCM."]
    #[inline(always)]
    pub fn cipher_len(&self) -> CipherLenR {
        CipherLenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - In the CFB, OFB and CTR mode, whether the last block is 16-byte aligned affects the length of the output. In the XTS and CMAC mode, the last block requires additional computation. The user needs to provide the total length for reference. In the GCM mode, the user needs to inform the length of the plaintext or ciphertext. The plaintext or ciphertext does not need to be given at once. But every time add data before the end of the calculation, the user needs to update the data length. Will be used in the last GMAC calculation of GCM."]
    #[inline(always)]
    pub fn cipher_len(&mut self) -> CipherLenW<'_, CipherLenSpec> {
        CipherLenW::new(self, 0)
    }
}
#[doc = "Input length register\n\nYou can [`read`](crate::Reg::read) this register and get [`cipher_len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cipher_len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CipherLenSpec;
impl crate::RegisterSpec for CipherLenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cipher_len::R`](R) reader structure"]
impl crate::Readable for CipherLenSpec {}
#[doc = "`write(|w| ..)` method takes [`cipher_len::W`](W) writer structure"]
impl crate::Writable for CipherLenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets cipher_len to value 0"]
impl crate::Resettable for CipherLenSpec {}
