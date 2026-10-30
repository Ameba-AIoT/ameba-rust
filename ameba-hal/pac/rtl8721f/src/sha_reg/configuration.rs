#[doc = "Register `Configuration` reader"]
pub type R = crate::R<ConfigurationSpec>;
#[doc = "Field `SHA2_CORE_EN` reader - 1: SHA2 is implmented 0: SHA2 is not implmented"]
pub type Sha2CoreEnR = crate::BitReader;
#[doc = "Field `SUPPORT_SHA512` reader - 0: disabled 1: enabled"]
pub type SupportSha512R = crate::BitReader;
#[doc = "Field `SUPPORT_TRUNCATED_SHA512` reader - 0: disabled 1: enabled"]
pub type SupportTruncatedSha512R = crate::BitReader;
#[doc = "Field `SUPPORT_HMAC_MODE` reader - 0: disabled 1: enabled"]
pub type SupportHmacModeR = crate::BitReader;
#[doc = "Field `SUPPORT_COPY_MODE` reader - 0: disabled 1: enabled"]
pub type SupportCopyModeR = crate::BitReader;
#[doc = "Field `EXTERNAL_KEY_EN` reader - 0: disabled 1: enabled"]
pub type ExternalKeyEnR = crate::BitReader;
#[doc = "Field `KEY_MANAGER_EN` reader - 0: disabled 1: enabled"]
pub type KeyManagerEnR = crate::BitReader;
#[doc = "Field `TRUST_ZONE_EN` reader - 0: disabled 1: enabled"]
pub type TrustZoneEnR = crate::BitReader;
#[doc = "Field `SHA1_CORE_EN` reader - 1: SHA1 is implmented 0: SHA1 is not implmented"]
pub type Sha1CoreEnR = crate::BitReader;
#[doc = "Field `SM3_CORE_EN` reader - 1: SM3 is implmented 0: SM3 is not implmented"]
pub type Sm3CoreEnR = crate::BitReader;
#[doc = "Field `RSVD_2` reader - Reserved"]
pub type Rsvd2R = crate::FieldReader;
#[doc = "Field `DISABLE_CORE_SHA2` reader - 1: SHA2 is disabled by hardware 0: SHA2 is enabled"]
pub type DisableCoreSha2R = crate::BitReader;
#[doc = "Field `DISABLE_CORE_SHA1` reader - 1: SHA1 is disabled by hardware 0: SHA1 is enabled"]
pub type DisableCoreSha1R = crate::BitReader;
#[doc = "Field `DISABLE_CORE_SM3` reader - 1: SM3 is disabled by hardware 0: SM3 is enabled"]
pub type DisableCoreSm3R = crate::BitReader;
#[doc = "Field `RSVD` reader - Reserved"]
pub type RsvdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 1: SHA2 is implmented 0: SHA2 is not implmented"]
    #[inline(always)]
    pub fn sha2_core_en(&self) -> Sha2CoreEnR {
        Sha2CoreEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: disabled 1: enabled"]
    #[inline(always)]
    pub fn support_sha512(&self) -> SupportSha512R {
        SupportSha512R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0: disabled 1: enabled"]
    #[inline(always)]
    pub fn support_truncated_sha512(&self) -> SupportTruncatedSha512R {
        SupportTruncatedSha512R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: disabled 1: enabled"]
    #[inline(always)]
    pub fn support_hmac_mode(&self) -> SupportHmacModeR {
        SupportHmacModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0: disabled 1: enabled"]
    #[inline(always)]
    pub fn support_copy_mode(&self) -> SupportCopyModeR {
        SupportCopyModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0: disabled 1: enabled"]
    #[inline(always)]
    pub fn external_key_en(&self) -> ExternalKeyEnR {
        ExternalKeyEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0: disabled 1: enabled"]
    #[inline(always)]
    pub fn key_manager_en(&self) -> KeyManagerEnR {
        KeyManagerEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0: disabled 1: enabled"]
    #[inline(always)]
    pub fn trust_zone_en(&self) -> TrustZoneEnR {
        TrustZoneEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: SHA1 is implmented 0: SHA1 is not implmented"]
    #[inline(always)]
    pub fn sha1_core_en(&self) -> Sha1CoreEnR {
        Sha1CoreEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: SM3 is implmented 0: SM3 is not implmented"]
    #[inline(always)]
    pub fn sm3_core_en(&self) -> Sm3CoreEnR {
        Sm3CoreEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Reserved"]
    #[inline(always)]
    pub fn rsvd_2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - 1: SHA2 is disabled by hardware 0: SHA2 is enabled"]
    #[inline(always)]
    pub fn disable_core_sha2(&self) -> DisableCoreSha2R {
        DisableCoreSha2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: SHA1 is disabled by hardware 0: SHA1 is enabled"]
    #[inline(always)]
    pub fn disable_core_sha1(&self) -> DisableCoreSha1R {
        DisableCoreSha1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1: SM3 is disabled by hardware 0: SM3 is enabled"]
    #[inline(always)]
    pub fn disable_core_sm3(&self) -> DisableCoreSm3R {
        DisableCoreSm3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:31 - Reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 15) & 0x0001_ffff)
    }
}
#[doc = "Configuration information register\n\nYou can [`read`](crate::Reg::read) this register and get [`configuration::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigurationSpec;
impl crate::RegisterSpec for ConfigurationSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`configuration::R`](R) reader structure"]
impl crate::Readable for ConfigurationSpec {}
#[doc = "`reset()` method sets Configuration to value 0"]
impl crate::Resettable for ConfigurationSpec {}
