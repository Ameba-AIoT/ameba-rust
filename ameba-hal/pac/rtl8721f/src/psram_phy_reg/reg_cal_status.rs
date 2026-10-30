#[doc = "Register `REG_CAL_STATUS` reader"]
pub type R = crate::R<RegCalStatusSpec>;
#[doc = "Field `CFG_CAL_CUR_N` reader - Current value N read from hardware."]
pub type CfgCalCurNR = crate::FieldReader;
#[doc = "Field `CFG_CAL_CUR_J` reader - Current value J read from hardware."]
pub type CfgCalCurJR = crate::FieldReader;
#[doc = "Field `CFG_CAL_PDST` reader - Calibration check result for last read burst . \\[0\\] : indicate phase n+j , if 1 , means dn == dn+j \\[1\\] : indicate phase n-j , if 1 , means dn == dn-j \\[2\\] : if 1 , means the state is under j = jmin condition"]
pub type CfgCalPdstR = crate::FieldReader;
#[doc = "Field `CFG_CAL_CUR_ST` reader - Calibration current state . Cleared by write all 1 ."]
pub type CfgCalCurStR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Current value N read from hardware."]
    #[inline(always)]
    pub fn cfg_cal_cur_n(&self) -> CfgCalCurNR {
        CfgCalCurNR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Current value J read from hardware."]
    #[inline(always)]
    pub fn cfg_cal_cur_j(&self) -> CfgCalCurJR {
        CfgCalCurJR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - Calibration check result for last read burst . \\[0\\] : indicate phase n+j , if 1 , means dn == dn+j \\[1\\] : indicate phase n-j , if 1 , means dn == dn-j \\[2\\] : if 1 , means the state is under j = jmin condition"]
    #[inline(always)]
    pub fn cfg_cal_pdst(&self) -> CfgCalPdstR {
        CfgCalPdstR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:28 - Calibration current state . Cleared by write all 1 ."]
    #[inline(always)]
    pub fn cfg_cal_cur_st(&self) -> CfgCalCurStR {
        CfgCalCurStR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cal_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCalStatusSpec;
impl crate::RegisterSpec for RegCalStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cal_status::R`](R) reader structure"]
impl crate::Readable for RegCalStatusSpec {}
#[doc = "`reset()` method sets REG_CAL_STATUS to value 0"]
impl crate::Resettable for RegCalStatusSpec {}
