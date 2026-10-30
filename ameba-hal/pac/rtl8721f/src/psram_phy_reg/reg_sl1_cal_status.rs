#[doc = "Register `REG_SL1_CAL_STATUS` reader"]
pub type R = crate::R<RegSl1CalStatusSpec>;
#[doc = "Field `SL1_CFG_CAL_CUR_N` reader - Current value N read from hardware."]
pub type Sl1CfgCalCurNR = crate::FieldReader;
#[doc = "Field `SL1_CFG_CAL_CUR_J` reader - Current value J read from hardware."]
pub type Sl1CfgCalCurJR = crate::FieldReader;
#[doc = "Field `SL1_CFG_CAL_PDST` reader - Calibration check result for last read burst . \\[0\\] : indicate phase n+j , if 1 , means dn == dn+j \\[1\\] : indicate phase n-j , if 1 , means dn == dn-j \\[2\\] : if 1 , means the state is under j = jmin condition"]
pub type Sl1CfgCalPdstR = crate::FieldReader;
#[doc = "Field `SL1_CFG_CAL_CUR_ST` reader - Calibration current state . Cleared by write all 1 ."]
pub type Sl1CfgCalCurStR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Current value N read from hardware."]
    #[inline(always)]
    pub fn sl1_cfg_cal_cur_n(&self) -> Sl1CfgCalCurNR {
        Sl1CfgCalCurNR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Current value J read from hardware."]
    #[inline(always)]
    pub fn sl1_cfg_cal_cur_j(&self) -> Sl1CfgCalCurJR {
        Sl1CfgCalCurJR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - Calibration check result for last read burst . \\[0\\] : indicate phase n+j , if 1 , means dn == dn+j \\[1\\] : indicate phase n-j , if 1 , means dn == dn-j \\[2\\] : if 1 , means the state is under j = jmin condition"]
    #[inline(always)]
    pub fn sl1_cfg_cal_pdst(&self) -> Sl1CfgCalPdstR {
        Sl1CfgCalPdstR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:28 - Calibration current state . Cleared by write all 1 ."]
    #[inline(always)]
    pub fn sl1_cfg_cal_cur_st(&self) -> Sl1CfgCalCurStR {
        Sl1CfgCalCurStR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sl1_cal_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSl1CalStatusSpec;
impl crate::RegisterSpec for RegSl1CalStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sl1_cal_status::R`](R) reader structure"]
impl crate::Readable for RegSl1CalStatusSpec {}
#[doc = "`reset()` method sets REG_SL1_CAL_STATUS to value 0"]
impl crate::Resettable for RegSl1CalStatusSpec {}
