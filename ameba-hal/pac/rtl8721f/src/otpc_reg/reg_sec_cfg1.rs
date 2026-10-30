#[doc = "Register `REG_SEC_CFG1` reader"]
pub type R = crate::R<RegSecCfg1Spec>;
#[doc = "Field `SWD_PWD_EN` reader - "]
pub type SwdPwdEnR = crate::BitReader;
#[doc = "Field `SWD_DBGEN` reader - "]
pub type SwdDbgenR = crate::BitReader;
#[doc = "Field `SWD_NIDEN` reader - "]
pub type SwdNidenR = crate::BitReader;
#[doc = "Field `SWD_SPIDEN` reader - "]
pub type SwdSpidenR = crate::BitReader;
#[doc = "Field `SWD_SPNIDEN` reader - "]
pub type SwdSpnidenR = crate::BitReader;
#[doc = "Field `SWD_PWD_R_Protection_EN` reader - "]
pub type SwdPwdRProtectionEnR = crate::BitReader;
#[doc = "Field `SWD_PWD_W_Forbidden_EN` reader - "]
pub type SwdPwdWForbiddenEnR = crate::BitReader;
#[doc = "Field `HUK_W_Forbidden_EN` reader - "]
pub type HukWForbiddenEnR = crate::BitReader;
#[doc = "Field `SWD_SoftWare_CTRL_EN` reader - "]
pub type SwdSoftWareCtrlEnR = crate::BitReader;
#[doc = "Field `PK1_W_Forbidden_EN` reader - "]
pub type Pk1WForbiddenEnR = crate::BitReader;
#[doc = "Field `PK2_W_Forbidden_EN` reader - "]
pub type Pk2WForbiddenEnR = crate::BitReader;
#[doc = "Field `S_IPSEC_Key1_R_Protection_EN` reader - "]
pub type SIpsecKey1RProtectionEnR = crate::BitReader;
#[doc = "Field `S_IPSEC_Key1_W_Forbidden_EN` reader - "]
pub type SIpsecKey1WForbiddenEnR = crate::BitReader;
#[doc = "Field `S_IPSEC_Key2_R_Protection_EN` reader - "]
pub type SIpsecKey2RProtectionEnR = crate::BitReader;
#[doc = "Field `S_IPSEC_Key2_W_Forbidden_EN` reader - "]
pub type SIpsecKey2WForbiddenEnR = crate::BitReader;
#[doc = "Field `NS_IPSEC_Key1_R_Protection_EN` reader - "]
pub type NsIpsecKey1RProtectionEnR = crate::BitReader;
#[doc = "Field `NS_IPSEC_Key1_W_Forbidden_EN` reader - "]
pub type NsIpsecKey1WForbiddenEnR = crate::BitReader;
#[doc = "Field `NS_IPSEC_Key2_R_Protection_EN` reader - "]
pub type NsIpsecKey2RProtectionEnR = crate::BitReader;
#[doc = "Field `NS_IPSEC_Key2_W_Forbidden_EN` reader - "]
pub type NsIpsecKey2WForbiddenEnR = crate::BitReader;
#[doc = "Field `ECDSA_PRI_KEY1_R_Protection_EN` reader - "]
pub type EcdsaPriKey1RProtectionEnR = crate::BitReader;
#[doc = "Field `ECDSA_PRI_KEY1_W_Forbidden_EN` reader - "]
pub type EcdsaPriKey1WForbiddenEnR = crate::BitReader;
#[doc = "Field `ECDSA_PRI_KEY2_R_Protection_EN` reader - "]
pub type EcdsaPriKey2RProtectionEnR = crate::BitReader;
#[doc = "Field `ECDSA_PRI_KEY2_W_Forbidden_EN` reader - "]
pub type EcdsaPriKey2WForbiddenEnR = crate::BitReader;
#[doc = "Field `RSIP_KEY1_R_Protection_EN` reader - "]
pub type RsipKey1RProtectionEnR = crate::BitReader;
#[doc = "Field `RSIP_KEY1_W_Forbidden_EN` reader - "]
pub type RsipKey1WForbiddenEnR = crate::BitReader;
#[doc = "Field `RSIP_KEY2_R_Protection_EN` reader - "]
pub type RsipKey2RProtectionEnR = crate::BitReader;
#[doc = "Field `RSIP_KEY2_W_Forbidden_EN` reader - "]
pub type RsipKey2WForbiddenEnR = crate::BitReader;
#[doc = "Field `RSIP_MODE_W_Forbidden_EN` reader - "]
pub type RsipModeWForbiddenEnR = crate::BitReader;
#[doc = "Field `SIC_SECURE_EN` reader - "]
pub type SicSecureEnR = crate::BitReader;
#[doc = "Field `CPU_PC_DBG_EN` reader - "]
pub type CpuPcDbgEnR = crate::BitReader;
#[doc = "Field `udf1_Trustzone_EN` reader - "]
pub type Udf1TrustzoneEnR = crate::BitReader;
#[doc = "Field `udf2_Trustzone_EN` reader - "]
pub type Udf2TrustzoneEnR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swd_pwd_en(&self) -> SwdPwdEnR {
        SwdPwdEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swd_dbgen(&self) -> SwdDbgenR {
        SwdDbgenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn swd_niden(&self) -> SwdNidenR {
        SwdNidenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn swd_spiden(&self) -> SwdSpidenR {
        SwdSpidenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn swd_spniden(&self) -> SwdSpnidenR {
        SwdSpnidenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn swd_pwd_r_protection_en(&self) -> SwdPwdRProtectionEnR {
        SwdPwdRProtectionEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn swd_pwd_w_forbidden_en(&self) -> SwdPwdWForbiddenEnR {
        SwdPwdWForbiddenEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn huk_w_forbidden_en(&self) -> HukWForbiddenEnR {
        HukWForbiddenEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swd_soft_ware_ctrl_en(&self) -> SwdSoftWareCtrlEnR {
        SwdSoftWareCtrlEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pk1_w_forbidden_en(&self) -> Pk1WForbiddenEnR {
        Pk1WForbiddenEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pk2_w_forbidden_en(&self) -> Pk2WForbiddenEnR {
        Pk2WForbiddenEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn s_ipsec_key1_r_protection_en(&self) -> SIpsecKey1RProtectionEnR {
        SIpsecKey1RProtectionEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn s_ipsec_key1_w_forbidden_en(&self) -> SIpsecKey1WForbiddenEnR {
        SIpsecKey1WForbiddenEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn s_ipsec_key2_r_protection_en(&self) -> SIpsecKey2RProtectionEnR {
        SIpsecKey2RProtectionEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn s_ipsec_key2_w_forbidden_en(&self) -> SIpsecKey2WForbiddenEnR {
        SIpsecKey2WForbiddenEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ns_ipsec_key1_r_protection_en(&self) -> NsIpsecKey1RProtectionEnR {
        NsIpsecKey1RProtectionEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ns_ipsec_key1_w_forbidden_en(&self) -> NsIpsecKey1WForbiddenEnR {
        NsIpsecKey1WForbiddenEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ns_ipsec_key2_r_protection_en(&self) -> NsIpsecKey2RProtectionEnR {
        NsIpsecKey2RProtectionEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ns_ipsec_key2_w_forbidden_en(&self) -> NsIpsecKey2WForbiddenEnR {
        NsIpsecKey2WForbiddenEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ecdsa_pri_key1_r_protection_en(&self) -> EcdsaPriKey1RProtectionEnR {
        EcdsaPriKey1RProtectionEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ecdsa_pri_key1_w_forbidden_en(&self) -> EcdsaPriKey1WForbiddenEnR {
        EcdsaPriKey1WForbiddenEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ecdsa_pri_key2_r_protection_en(&self) -> EcdsaPriKey2RProtectionEnR {
        EcdsaPriKey2RProtectionEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ecdsa_pri_key2_w_forbidden_en(&self) -> EcdsaPriKey2WForbiddenEnR {
        EcdsaPriKey2WForbiddenEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rsip_key1_r_protection_en(&self) -> RsipKey1RProtectionEnR {
        RsipKey1RProtectionEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rsip_key1_w_forbidden_en(&self) -> RsipKey1WForbiddenEnR {
        RsipKey1WForbiddenEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rsip_key2_r_protection_en(&self) -> RsipKey2RProtectionEnR {
        RsipKey2RProtectionEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rsip_key2_w_forbidden_en(&self) -> RsipKey2WForbiddenEnR {
        RsipKey2WForbiddenEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rsip_mode_w_forbidden_en(&self) -> RsipModeWForbiddenEnR {
        RsipModeWForbiddenEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sic_secure_en(&self) -> SicSecureEnR {
        SicSecureEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cpu_pc_dbg_en(&self) -> CpuPcDbgEnR {
        CpuPcDbgEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn udf1_trustzone_en(&self) -> Udf1TrustzoneEnR {
        Udf1TrustzoneEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn udf2_trustzone_en(&self) -> Udf2TrustzoneEnR {
        Udf2TrustzoneEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sec_cfg1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSecCfg1Spec;
impl crate::RegisterSpec for RegSecCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sec_cfg1::R`](R) reader structure"]
impl crate::Readable for RegSecCfg1Spec {}
#[doc = "`reset()` method sets REG_SEC_CFG1 to value 0"]
impl crate::Resettable for RegSecCfg1Spec {}
