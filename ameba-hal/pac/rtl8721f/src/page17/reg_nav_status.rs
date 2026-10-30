#[doc = "Register `REG_NAV_STATUS` reader"]
pub type R = crate::R<RegNavStatusSpec>;
#[doc = "Field `BIT_WMAC_BASIC_NAV_DUR` reader - basic_nav"]
pub type BitWmacBasicNavDurR = crate::FieldReader<u16>;
#[doc = "Field `BIT_WMAC_BASIC_NAV_UPD` reader - inter nav updated by 10: plcp header 01: mac header"]
pub type BitWmacBasicNavUpdR = crate::FieldReader;
#[doc = "Field `BIT_WMAC_INTRA_NAV_DUR` reader - intra_nav"]
pub type BitWmacIntraNavDurR = crate::FieldReader<u16>;
#[doc = "Field `BIT_WMAC_INTRA_NAV_UPD` reader - intra nav updated by 10: plcp header 01: mac header"]
pub type BitWmacIntraNavUpdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:13 - basic_nav"]
    #[inline(always)]
    pub fn bit_wmac_basic_nav_dur(&self) -> BitWmacBasicNavDurR {
        BitWmacBasicNavDurR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:15 - inter nav updated by 10: plcp header 01: mac header"]
    #[inline(always)]
    pub fn bit_wmac_basic_nav_upd(&self) -> BitWmacBasicNavUpdR {
        BitWmacBasicNavUpdR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:29 - intra_nav"]
    #[inline(always)]
    pub fn bit_wmac_intra_nav_dur(&self) -> BitWmacIntraNavDurR {
        BitWmacIntraNavDurR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bits 30:31 - intra nav updated by 10: plcp header 01: mac header"]
    #[inline(always)]
    pub fn bit_wmac_intra_nav_upd(&self) -> BitWmacIntraNavUpdR {
        BitWmacIntraNavUpdR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_nav_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNavStatusSpec;
impl crate::RegisterSpec for RegNavStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_nav_status::R`](R) reader structure"]
impl crate::Readable for RegNavStatusSpec {}
#[doc = "`reset()` method sets REG_NAV_STATUS to value 0"]
impl crate::Resettable for RegNavStatusSpec {}
