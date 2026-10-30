#[doc = "Register `REG_NAV_THRSHOLD` reader"]
pub type R = crate::R<RegNavThrsholdSpec>;
#[doc = "Register `REG_NAV_THRSHOLD` writer"]
pub type W = crate::W<RegNavThrsholdSpec>;
#[doc = "Field `BIT_RTS_RST_DUR` reader - RTS NAV Reset Time. This is the value in us, not including the time of CTS frame, to reset the NAV set by an RTS frame if no PHY indication occurred during this time. Refer to 802.11 1999 Section 9.2.5.4 for detailed description."]
pub type BitRtsRstDurR = crate::FieldReader;
#[doc = "Field `BIT_RTS_RST_DUR` writer - RTS NAV Reset Time. This is the value in us, not including the time of CTS frame, to reset the NAV set by an RTS frame if no PHY indication occurred during this time. Refer to 802.11 1999 Section 9.2.5.4 for detailed description."]
pub type BitRtsRstDurW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_RxMyRTS_NAV` reader - RX my RTS NAV. As rx my unicast RTS and RxMyRTS_NAV is not equal to zero, then rx NAV is set to wait response transition time and RxMyRTS_NAV to avoid contention with the TXOP originator. If RxMyRTS_NAV is equal to zero, this mechanism is disabled"]
pub type BitRxMyRtsNavR = crate::FieldReader;
#[doc = "Field `BIT_RxMyRTS_NAV` writer - RX my RTS NAV. As rx my unicast RTS and RxMyRTS_NAV is not equal to zero, then rx NAV is set to wait response transition time and RxMyRTS_NAV to avoid contention with the TXOP originator. If RxMyRTS_NAV is equal to zero, this mechanism is disabled"]
pub type BitRxMyRtsNavW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_NAV_UPBDY` reader - When NAV update is beyond this value, then NAV update is aborted. When this field is 0x00, NAV_UPPER function is disable. The unit is 128us."]
pub type BitNavUpbdyR = crate::FieldReader;
#[doc = "Field `BIT_NAV_UPBDY` writer - When NAV update is beyond this value, then NAV update is aborted. When this field is 0x00, NAV_UPPER function is disable. The unit is 128us."]
pub type BitNavUpbdyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RTS NAV Reset Time. This is the value in us, not including the time of CTS frame, to reset the NAV set by an RTS frame if no PHY indication occurred during this time. Refer to 802.11 1999 Section 9.2.5.4 for detailed description."]
    #[inline(always)]
    pub fn bit_rts_rst_dur(&self) -> BitRtsRstDurR {
        BitRtsRstDurR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - RX my RTS NAV. As rx my unicast RTS and RxMyRTS_NAV is not equal to zero, then rx NAV is set to wait response transition time and RxMyRTS_NAV to avoid contention with the TXOP originator. If RxMyRTS_NAV is equal to zero, this mechanism is disabled"]
    #[inline(always)]
    pub fn bit_rx_my_rts_nav(&self) -> BitRxMyRtsNavR {
        BitRxMyRtsNavR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - When NAV update is beyond this value, then NAV update is aborted. When this field is 0x00, NAV_UPPER function is disable. The unit is 128us."]
    #[inline(always)]
    pub fn bit_nav_upbdy(&self) -> BitNavUpbdyR {
        BitNavUpbdyR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RTS NAV Reset Time. This is the value in us, not including the time of CTS frame, to reset the NAV set by an RTS frame if no PHY indication occurred during this time. Refer to 802.11 1999 Section 9.2.5.4 for detailed description."]
    #[inline(always)]
    pub fn bit_rts_rst_dur(&mut self) -> BitRtsRstDurW<'_, RegNavThrsholdSpec> {
        BitRtsRstDurW::new(self, 0)
    }
    #[doc = "Bits 8:11 - RX my RTS NAV. As rx my unicast RTS and RxMyRTS_NAV is not equal to zero, then rx NAV is set to wait response transition time and RxMyRTS_NAV to avoid contention with the TXOP originator. If RxMyRTS_NAV is equal to zero, this mechanism is disabled"]
    #[inline(always)]
    pub fn bit_rx_my_rts_nav(&mut self) -> BitRxMyRtsNavW<'_, RegNavThrsholdSpec> {
        BitRxMyRtsNavW::new(self, 8)
    }
    #[doc = "Bits 16:23 - When NAV update is beyond this value, then NAV update is aborted. When this field is 0x00, NAV_UPPER function is disable. The unit is 128us."]
    #[inline(always)]
    pub fn bit_nav_upbdy(&mut self) -> BitNavUpbdyW<'_, RegNavThrsholdSpec> {
        BitNavUpbdyW::new(self, 16)
    }
}
#[doc = "NAV Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_nav_thrshold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_nav_thrshold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNavThrsholdSpec;
impl crate::RegisterSpec for RegNavThrsholdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_nav_thrshold::R`](R) reader structure"]
impl crate::Readable for RegNavThrsholdSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_nav_thrshold::W`](W) writer structure"]
impl crate::Writable for RegNavThrsholdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NAV_THRSHOLD to value 0x0426"]
impl crate::Resettable for RegNavThrsholdSpec {
    const RESET_VALUE: u32 = 0x0426;
}
