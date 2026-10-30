#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `idle` reader - Indicates whether there is any outstanding transaction."]
pub type IdleR = crate::BitReader;
#[doc = "Field `key_init_done` reader - Indicates whether the initialization of the key is completed. Only used in MIX mode."]
pub type KeyInitDoneR = crate::BitReader;
#[doc = "Field `intr_err_tag` reader - Indicates a GCM tag mismatch error occurred."]
pub type IntrErrTagR = crate::BitReader;
#[doc = "Field `intr_err_resp` reader - Indicates a the error response is received from slave."]
pub type IntrErrRespR = crate::BitReader;
#[doc = "Field `intr_err_apb` reader - Indicates whether the setting configuration failed. When a transaction is in progress, all settings except clear interruption are invalid."]
pub type IntrErrApbR = crate::BitReader;
#[doc = "Field `intr_err_init` reader - Indicates the transaction was received before the key initialization is completed."]
pub type IntrErrInitR = crate::BitReader;
#[doc = "Field `remap_region_error` reader - Indicates the read address hits multiple remapping region."]
pub type RemapRegionErrorR = crate::BitReader;
#[doc = "Field `remap_addr_error` reader - Indicates the address after adding offset size cross 256MB boundaries"]
pub type RemapAddrErrorR = crate::BitReader;
#[doc = "Field `control_locked` reader - Indicates whether the setting is locked. If it is locked, only Clr_intr and Maintenance registers can be set"]
pub type ControlLockedR = crate::BitReader;
#[doc = "Field `key_locked` reader - Indicates whether the keyx is locked. If it is locked, keyx cannot be written or read"]
pub type KeyLockedR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Indicates whether there is any outstanding transaction."]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates whether the initialization of the key is completed. Only used in MIX mode."]
    #[inline(always)]
    pub fn key_init_done(&self) -> KeyInitDoneR {
        KeyInitDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates a GCM tag mismatch error occurred."]
    #[inline(always)]
    pub fn intr_err_tag(&self) -> IntrErrTagR {
        IntrErrTagR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates a the error response is received from slave."]
    #[inline(always)]
    pub fn intr_err_resp(&self) -> IntrErrRespR {
        IntrErrRespR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates whether the setting configuration failed. When a transaction is in progress, all settings except clear interruption are invalid."]
    #[inline(always)]
    pub fn intr_err_apb(&self) -> IntrErrApbR {
        IntrErrApbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates the transaction was received before the key initialization is completed."]
    #[inline(always)]
    pub fn intr_err_init(&self) -> IntrErrInitR {
        IntrErrInitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates the read address hits multiple remapping region."]
    #[inline(always)]
    pub fn remap_region_error(&self) -> RemapRegionErrorR {
        RemapRegionErrorR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates the address after adding offset size cross 256MB boundaries"]
    #[inline(always)]
    pub fn remap_addr_error(&self) -> RemapAddrErrorR {
        RemapAddrErrorR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates whether the setting is locked. If it is locked, only Clr_intr and Maintenance registers can be set"]
    #[inline(always)]
    pub fn control_locked(&self) -> ControlLockedR {
        ControlLockedR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:16 - Indicates whether the keyx is locked. If it is locked, keyx cannot be written or read"]
    #[inline(always)]
    pub fn key_locked(&self) -> KeyLockedR {
        KeyLockedR::new(((self.bits >> 9) & 0xff) as u8)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0x01"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0x01;
}
