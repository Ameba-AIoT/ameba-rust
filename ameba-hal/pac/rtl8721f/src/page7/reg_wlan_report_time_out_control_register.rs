#[doc = "Register `REG_WLAN_Report_Time_Out_Control_Register` reader"]
pub type R = crate::R<RegWlanReportTimeOutControlRegisterSpec>;
#[doc = "Register `REG_WLAN_Report_Time_Out_Control_Register` writer"]
pub type W = crate::W<RegWlanReportTimeOutControlRegisterSpec>;
#[doc = "Field `BIT_WLAN_RPT_TO` reader - When WLAN HW received query packet, HW timer counts. If timer equals to WLAN_RPT_TO and HW can\"t send packet, it will give up query packet at this time. When BT sent another query packet, WLAN HW need to send the previous packet and it shouldn't trigger an interrupt to firmware. Unit: 4us."]
pub type BitWlanRptToR = crate::FieldReader;
#[doc = "Field `BIT_WLAN_RPT_TO` writer - When WLAN HW received query packet, HW timer counts. If timer equals to WLAN_RPT_TO and HW can\"t send packet, it will give up query packet at this time. When BT sent another query packet, WLAN HW need to send the previous packet and it shouldn't trigger an interrupt to firmware. Unit: 4us."]
pub type BitWlanRptToW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_ISOLATION_EN` reader - Isolation table enable bit. 0: disable 1: enable"]
pub type BitIsolationEnR = crate::BitReader;
#[doc = "Field `BIT_ISOLATION_EN` writer - Isolation table enable bit. 0: disable 1: enable"]
pub type BitIsolationEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ISOLATION_CHK_0` reader - 0: leave PTA 1: enter PTA"]
pub type BitIsolationChk0R = crate::FieldReader<u32>;
#[doc = "Field `BIT_ISOLATION_CHK_0` writer - 0: leave PTA 1: enter PTA"]
pub type BitIsolationChk0W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:7 - When WLAN HW received query packet, HW timer counts. If timer equals to WLAN_RPT_TO and HW can\"t send packet, it will give up query packet at this time. When BT sent another query packet, WLAN HW need to send the previous packet and it shouldn't trigger an interrupt to firmware. Unit: 4us."]
    #[inline(always)]
    pub fn bit_wlan_rpt_to(&self) -> BitWlanRptToR {
        BitWlanRptToR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Isolation table enable bit. 0: disable 1: enable"]
    #[inline(always)]
    pub fn bit_isolation_en(&self) -> BitIsolationEnR {
        BitIsolationEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 0: leave PTA 1: enter PTA"]
    #[inline(always)]
    pub fn bit_isolation_chk_0(&self) -> BitIsolationChk0R {
        BitIsolationChk0R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - When WLAN HW received query packet, HW timer counts. If timer equals to WLAN_RPT_TO and HW can\"t send packet, it will give up query packet at this time. When BT sent another query packet, WLAN HW need to send the previous packet and it shouldn't trigger an interrupt to firmware. Unit: 4us."]
    #[inline(always)]
    pub fn bit_wlan_rpt_to(
        &mut self,
    ) -> BitWlanRptToW<'_, RegWlanReportTimeOutControlRegisterSpec> {
        BitWlanRptToW::new(self, 0)
    }
    #[doc = "Bit 8 - Isolation table enable bit. 0: disable 1: enable"]
    #[inline(always)]
    pub fn bit_isolation_en(
        &mut self,
    ) -> BitIsolationEnW<'_, RegWlanReportTimeOutControlRegisterSpec> {
        BitIsolationEnW::new(self, 8)
    }
    #[doc = "Bits 9:31 - 0: leave PTA 1: enter PTA"]
    #[inline(always)]
    pub fn bit_isolation_chk_0(
        &mut self,
    ) -> BitIsolationChk0W<'_, RegWlanReportTimeOutControlRegisterSpec> {
        BitIsolationChk0W::new(self, 9)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wlan_report_time_out_control_register::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wlan_report_time_out_control_register::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWlanReportTimeOutControlRegisterSpec;
impl crate::RegisterSpec for RegWlanReportTimeOutControlRegisterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wlan_report_time_out_control_register::R`](R) reader structure"]
impl crate::Readable for RegWlanReportTimeOutControlRegisterSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wlan_report_time_out_control_register::W`](W) writer structure"]
impl crate::Writable for RegWlanReportTimeOutControlRegisterSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WLAN_Report_Time_Out_Control_Register to value 0"]
impl crate::Resettable for RegWlanReportTimeOutControlRegisterSpec {}
