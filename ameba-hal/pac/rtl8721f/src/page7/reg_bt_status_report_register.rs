#[doc = "Register `REG_BT_Status_Report_Register` reader"]
pub type R = crate::R<RegBtStatusReportRegisterSpec>;
#[doc = "Field `BIT_BT_CMD_ID` reader - command information from BT"]
pub type BitBtCmdIdR = crate::FieldReader;
#[doc = "Field `BIT_BT_PREDECT_STATUS` reader - When WLAN received CMD_ID 0x0, it will recorder to this"]
pub type BitBtPredectStatusR = crate::FieldReader;
#[doc = "Field `BIT_BT_POWER` reader - When WLAN received CMD_ID 0x2, it will recorder to this"]
pub type BitBtPowerR = crate::FieldReader;
#[doc = "Field `BIT_BT_PROFILE` reader - When WLAN received CMD_ID 0x3, it will recorder to this."]
pub type BitBtProfileR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - command information from BT"]
    #[inline(always)]
    pub fn bit_bt_cmd_id(&self) -> BitBtCmdIdR {
        BitBtCmdIdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - When WLAN received CMD_ID 0x0, it will recorder to this"]
    #[inline(always)]
    pub fn bit_bt_predect_status(&self) -> BitBtPredectStatusR {
        BitBtPredectStatusR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - When WLAN received CMD_ID 0x2, it will recorder to this"]
    #[inline(always)]
    pub fn bit_bt_power(&self) -> BitBtPowerR {
        BitBtPowerR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - When WLAN received CMD_ID 0x3, it will recorder to this."]
    #[inline(always)]
    pub fn bit_bt_profile(&self) -> BitBtProfileR {
        BitBtProfileR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bt_status_report_register::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBtStatusReportRegisterSpec;
impl crate::RegisterSpec for RegBtStatusReportRegisterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bt_status_report_register::R`](R) reader structure"]
impl crate::Readable for RegBtStatusReportRegisterSpec {}
#[doc = "`reset()` method sets REG_BT_Status_Report_Register to value 0"]
impl crate::Resettable for RegBtStatusReportRegisterSpec {}
