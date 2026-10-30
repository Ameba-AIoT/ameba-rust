#[doc = "Register `REG_SEC_CFG2` reader"]
pub type R = crate::R<RegSecCfg2Spec>;
#[doc = "Field `UART_DOWNLOAD_DISABLE` reader - "]
pub type UartDownloadDisableR = crate::BitReader;
#[doc = "Field `rsvd0_sw` reader - "]
pub type Rsvd0SwR = crate::BitReader;
#[doc = "Field `RSIP_EN` reader - "]
pub type RsipEnR = crate::BitReader;
#[doc = "Field `SECURE_BOOT_EN` reader - "]
pub type SecureBootEnR = crate::BitReader;
#[doc = "Field `BOOT_RAND_DELAY_EN` reader - "]
pub type BootRandDelayEnR = crate::BitReader;
#[doc = "Field `RDP_EN` reader - "]
pub type RdpEnR = crate::BitReader;
#[doc = "Field `Anti_Rollback_EN` reader - "]
pub type AntiRollbackEnR = crate::BitReader;
#[doc = "Field `FAULT_LOG_PRINT_DIS` reader - "]
pub type FaultLogPrintDisR = crate::BitReader;
#[doc = "Field `RSIP_MODE` reader - "]
pub type RsipModeR = crate::FieldReader;
#[doc = "Field `HUK_DERIV_EN` reader - "]
pub type HukDerivEnR = crate::BitReader;
#[doc = "Field `User_physical_Trustzone1_EN` reader - "]
pub type UserPhysicalTrustzone1EnR = crate::BitReader;
#[doc = "Field `User_physical_Trustzone2_EN` reader - "]
pub type UserPhysicalTrustzone2EnR = crate::BitReader;
#[doc = "Field `PK_IDX` reader - (ROTPK_hash_index)"]
pub type PkIdxR = crate::BitReader;
#[doc = "Field `SWTRIG_UART_DOWNLOAD_DISABLE` reader - "]
pub type SwtrigUartDownloadDisableR = crate::BitReader;
#[doc = "Field `SPIC_PG1B_DISABLE` reader - "]
pub type SpicPg1bDisableR = crate::BitReader;
#[doc = "Field `rsvd1_sw` reader - "]
pub type Rsvd1SwR = crate::FieldReader;
#[doc = "Field `SECURE_BOOT_AUTH_ALG` reader - "]
pub type SecureBootAuthAlgR = crate::FieldReader;
#[doc = "Field `SECURE_BOOT_HASH_ALG` reader - "]
pub type SecureBootHashAlgR = crate::FieldReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uart_download_disable(&self) -> UartDownloadDisableR {
        UartDownloadDisableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rsvd0_sw(&self) -> Rsvd0SwR {
        Rsvd0SwR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rsip_en(&self) -> RsipEnR {
        RsipEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn secure_boot_en(&self) -> SecureBootEnR {
        SecureBootEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn boot_rand_delay_en(&self) -> BootRandDelayEnR {
        BootRandDelayEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rdp_en(&self) -> RdpEnR {
        RdpEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn anti_rollback_en(&self) -> AntiRollbackEnR {
        AntiRollbackEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn fault_log_print_dis(&self) -> FaultLogPrintDisR {
        FaultLogPrintDisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rsip_mode(&self) -> RsipModeR {
        RsipModeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn huk_deriv_en(&self) -> HukDerivEnR {
        HukDerivEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn user_physical_trustzone1_en(&self) -> UserPhysicalTrustzone1EnR {
        UserPhysicalTrustzone1EnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn user_physical_trustzone2_en(&self) -> UserPhysicalTrustzone2EnR {
        UserPhysicalTrustzone2EnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - (ROTPK_hash_index)"]
    #[inline(always)]
    pub fn pk_idx(&self) -> PkIdxR {
        PkIdxR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn swtrig_uart_download_disable(&self) -> SwtrigUartDownloadDisableR {
        SwtrigUartDownloadDisableR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn spic_pg1b_disable(&self) -> SpicPg1bDisableR {
        SpicPg1bDisableR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn rsvd1_sw(&self) -> Rsvd1SwR {
        Rsvd1SwR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn secure_boot_auth_alg(&self) -> SecureBootAuthAlgR {
        SecureBootAuthAlgR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn secure_boot_hash_alg(&self) -> SecureBootHashAlgR {
        SecureBootHashAlgR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sec_cfg2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSecCfg2Spec;
impl crate::RegisterSpec for RegSecCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sec_cfg2::R`](R) reader structure"]
impl crate::Readable for RegSecCfg2Spec {}
#[doc = "`reset()` method sets REG_SEC_CFG2 to value 0"]
impl crate::Resettable for RegSecCfg2Spec {}
