#[doc = "Register `CIR` reader"]
pub type R = crate::R<CirSpec>;
#[doc = "Field `region_number` reader - Number of supported decryption regions"]
pub type RegionNumberR = crate::FieldReader;
#[doc = "Field `remap_number` reader - Number of supported remap regions"]
pub type RemapNumberR = crate::FieldReader;
#[doc = "Field `iv_number` reader - Number of supported IVs registers"]
pub type IvNumberR = crate::FieldReader;
#[doc = "Field `CTR_key_number` reader - Number of supported CTR key registers, valid when support_ctr or support_gcm is 1"]
pub type CtrKeyNumberR = crate::FieldReader;
#[doc = "Field `MIX_key_number` reader - Number of supported MIX key registers, valid when support_mix is 1"]
pub type MixKeyNumberR = crate::FieldReader;
#[doc = "Field `support_ctr` reader - Indicate whether CTR mode is supported."]
pub type SupportCtrR = crate::BitReader;
#[doc = "Field `support_mix` reader - Indicate whether MIX mode is supported."]
pub type SupportMixR = crate::BitReader;
#[doc = "Field `support_gcm` reader - Indicate whether GCM mode is supported."]
pub type SupportGcmR = crate::BitReader;
#[doc = "Field `key_source` reader - Indicates the key source 0: from dedicated key port 1: from APB register"]
pub type KeySourceR = crate::BitReader;
#[doc = "Field `line_size` reader - Cache line size 0: 16B = 1 AES block 1: 32B = 2 AES blocks 2: 64B = 4 AES blocks 3: 128B = 8 AES blocks Indicates how many 128-bit blocks are used to calculate the tag cache line size (Byte) = 16 * 2BLOCK_SIZE"]
pub type LineSizeR = crate::FieldReader;
#[doc = "Field `independent_tag_addr` reader - Indicates the storage location of the tag 0: consecutive space - interleaved with each data blocks1: separate space from data - use another base address"]
pub type IndependentTagAddrR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Number of supported decryption regions"]
    #[inline(always)]
    pub fn region_number(&self) -> RegionNumberR {
        RegionNumberR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Number of supported remap regions"]
    #[inline(always)]
    pub fn remap_number(&self) -> RemapNumberR {
        RemapNumberR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Number of supported IVs registers"]
    #[inline(always)]
    pub fn iv_number(&self) -> IvNumberR {
        IvNumberR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Number of supported CTR key registers, valid when support_ctr or support_gcm is 1"]
    #[inline(always)]
    pub fn ctr_key_number(&self) -> CtrKeyNumberR {
        CtrKeyNumberR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Number of supported MIX key registers, valid when support_mix is 1"]
    #[inline(always)]
    pub fn mix_key_number(&self) -> MixKeyNumberR {
        MixKeyNumberR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Indicate whether CTR mode is supported."]
    #[inline(always)]
    pub fn support_ctr(&self) -> SupportCtrR {
        SupportCtrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Indicate whether MIX mode is supported."]
    #[inline(always)]
    pub fn support_mix(&self) -> SupportMixR {
        SupportMixR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Indicate whether GCM mode is supported."]
    #[inline(always)]
    pub fn support_gcm(&self) -> SupportGcmR {
        SupportGcmR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Indicates the key source 0: from dedicated key port 1: from APB register"]
    #[inline(always)]
    pub fn key_source(&self) -> KeySourceR {
        KeySourceR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Cache line size 0: 16B = 1 AES block 1: 32B = 2 AES blocks 2: 64B = 4 AES blocks 3: 128B = 8 AES blocks Indicates how many 128-bit blocks are used to calculate the tag cache line size (Byte) = 16 * 2BLOCK_SIZE"]
    #[inline(always)]
    pub fn line_size(&self) -> LineSizeR {
        LineSizeR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Indicates the storage location of the tag 0: consecutive space - interleaved with each data blocks1: separate space from data - use another base address"]
    #[inline(always)]
    pub fn independent_tag_addr(&self) -> IndependentTagAddrR {
        IndependentTagAddrR::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "Component Identification Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CirSpec;
impl crate::RegisterSpec for CirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cir::R`](R) reader structure"]
impl crate::Readable for CirSpec {}
#[doc = "`reset()` method sets CIR to value 0"]
impl crate::Resettable for CirSpec {}
