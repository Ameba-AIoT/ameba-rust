#[doc = "Register `chn_aes_mask_en_and_version` reader"]
pub type R = crate::R<ChnAesMaskEnAndVersionSpec>;
#[doc = "Field `gf_mask_en` reader - Indicate whether to use SCA countermeasure."]
pub type GfMaskEnR = crate::BitReader;
#[doc = "Field `trustzone_en` reader - Indicate whether to use TrustZone."]
pub type TrustzoneEnR = crate::BitReader;
#[doc = "Field `key_management_en` reader - Indicate whether to use key_management."]
pub type KeyManagementEnR = crate::BitReader;
#[doc = "Field `core_select_lock` reader - Indicate whether the core_select was locked by hardware pin."]
pub type CoreSelectLockR = crate::BitReader;
#[doc = "Field `dma_en` reader - Indicate whether DMA is used (suppoty DMA mode or not)"]
pub type DmaEnR = crate::BitReader;
#[doc = "Field `version` reader - The lalu_aes version number."]
pub type VersionR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Indicate whether to use SCA countermeasure."]
    #[inline(always)]
    pub fn gf_mask_en(&self) -> GfMaskEnR {
        GfMaskEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicate whether to use TrustZone."]
    #[inline(always)]
    pub fn trustzone_en(&self) -> TrustzoneEnR {
        TrustzoneEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicate whether to use key_management."]
    #[inline(always)]
    pub fn key_management_en(&self) -> KeyManagementEnR {
        KeyManagementEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicate whether the core_select was locked by hardware pin."]
    #[inline(always)]
    pub fn core_select_lock(&self) -> CoreSelectLockR {
        CoreSelectLockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicate whether DMA is used (suppoty DMA mode or not)"]
    #[inline(always)]
    pub fn dma_en(&self) -> DmaEnR {
        DmaEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:31 - The lalu_aes version number."]
    #[inline(always)]
    pub fn version(&self) -> VersionR {
        VersionR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Enable AES engine mask status register and version register\n\nYou can [`read`](crate::Reg::read) this register and get [`chn_aes_mask_en_and_version::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChnAesMaskEnAndVersionSpec;
impl crate::RegisterSpec for ChnAesMaskEnAndVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chn_aes_mask_en_and_version::R`](R) reader structure"]
impl crate::Readable for ChnAesMaskEnAndVersionSpec {}
#[doc = "`reset()` method sets chn_aes_mask_en_and_version to value 0x10"]
impl crate::Resettable for ChnAesMaskEnAndVersionSpec {
    const RESET_VALUE: u32 = 0x10;
}
