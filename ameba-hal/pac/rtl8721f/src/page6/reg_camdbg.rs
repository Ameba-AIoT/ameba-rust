#[doc = "Register `REG_CAMDBG` reader"]
pub type R = crate::R<RegCamdbgSpec>;
#[doc = "Register `REG_CAMDBG` writer"]
pub type W = crate::W<RegCamdbgSpec>;
#[doc = "Field `BIT_CAMDBG_SEC_KEY_IDX_V2` reader - Security KEY INDEX"]
pub type BitCamdbgSecKeyIdxV2R = crate::FieldReader;
#[doc = "Field `BIT_CAMDBG_MIC_KEY_IDX_V3` reader - Mic KEY INDEX"]
pub type BitCamdbgMicKeyIdxV3R = crate::FieldReader;
#[doc = "Field `BIT_EXT_SECTYPE` reader - Found security key ext_sectype"]
pub type BitExtSectypeR = crate::BitReader;
#[doc = "Field `BIT_CAMDBG_SEC_TYPE_V1` reader - Found security key type"]
pub type BitCamdbgSecTypeV1R = crate::FieldReader;
#[doc = "Field `BIT_SEC_KEYFOUND_V1` reader - Security Key Found in CAM 0: key not found 1: key found"]
pub type BitSecKeyfoundV1R = crate::BitReader;
#[doc = "Field `BIT_KEYCAM_DBGSEL` reader - Select TX/RX CAM Information. Set to 1 to retrieve latest Tx search result. Set to 0 to retrieve latest Rx search result."]
pub type BitKeycamDbgselR = crate::BitReader;
#[doc = "Field `BIT_KEYCAM_DBGSEL` writer - Select TX/RX CAM Information. Set to 1 to retrieve latest Tx search result. Set to 0 to retrieve latest Rx search result."]
pub type BitKeycamDbgselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Security KEY INDEX"]
    #[inline(always)]
    pub fn bit_camdbg_sec_key_idx_v2(&self) -> BitCamdbgSecKeyIdxV2R {
        BitCamdbgSecKeyIdxV2R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Mic KEY INDEX"]
    #[inline(always)]
    pub fn bit_camdbg_mic_key_idx_v3(&self) -> BitCamdbgMicKeyIdxV3R {
        BitCamdbgMicKeyIdxV3R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - Found security key ext_sectype"]
    #[inline(always)]
    pub fn bit_ext_sectype(&self) -> BitExtSectypeR {
        BitExtSectypeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Found security key type"]
    #[inline(always)]
    pub fn bit_camdbg_sec_type_v1(&self) -> BitCamdbgSecTypeV1R {
        BitCamdbgSecTypeV1R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Security Key Found in CAM 0: key not found 1: key found"]
    #[inline(always)]
    pub fn bit_sec_keyfound_v1(&self) -> BitSecKeyfoundV1R {
        BitSecKeyfoundV1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - Select TX/RX CAM Information. Set to 1 to retrieve latest Tx search result. Set to 0 to retrieve latest Rx search result."]
    #[inline(always)]
    pub fn bit_keycam_dbgsel(&self) -> BitKeycamDbgselR {
        BitKeycamDbgselR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Select TX/RX CAM Information. Set to 1 to retrieve latest Tx search result. Set to 0 to retrieve latest Rx search result."]
    #[inline(always)]
    pub fn bit_keycam_dbgsel(&mut self) -> BitKeycamDbgselW<'_, RegCamdbgSpec> {
        BitKeycamDbgselW::new(self, 31)
    }
}
#[doc = "CAM Debug Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_camdbg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_camdbg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCamdbgSpec;
impl crate::RegisterSpec for RegCamdbgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_camdbg::R`](R) reader structure"]
impl crate::Readable for RegCamdbgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_camdbg::W`](W) writer structure"]
impl crate::Writable for RegCamdbgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CAMDBG to value 0"]
impl crate::Resettable for RegCamdbgSpec {}
