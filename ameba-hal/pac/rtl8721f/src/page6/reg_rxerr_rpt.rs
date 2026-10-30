#[doc = "Register `REG_RXERR_RPT` reader"]
pub type R = crate::R<RegRxerrRptSpec>;
#[doc = "Register `REG_RXERR_RPT` writer"]
pub type W = crate::W<RegRxerrRptSpec>;
#[doc = "Field `BIT_RPT_COUNTER` reader - The reported counter value corresponding to Report type selected."]
pub type BitRptCounterR = crate::FieldReader<u16>;
#[doc = "Field `BIT_RPT_COUNTER` writer - The reported counter value corresponding to Report type selected."]
pub type BitRptCounterW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_UD_Type` reader - User defined: Type"]
pub type BitUdTypeR = crate::FieldReader;
#[doc = "Field `BIT_UD_Type` writer - User defined: Type"]
pub type BitUdTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_UD_Sub_Type` reader - User defined: SubType"]
pub type BitUdSubTypeR = crate::FieldReader;
#[doc = "Field `BIT_UD_Sub_Type` writer - User defined: SubType"]
pub type BitUdSubTypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_UD_Select_BSSID_0` reader - Select BSSID bit0"]
pub type BitUdSelectBssid0R = crate::BitReader;
#[doc = "Field `BIT_UD_Select_BSSID_0` writer - Select BSSID bit0"]
pub type BitUdSelectBssid0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_W1S` reader - Write One Setting, it is only valid for RXERR_RPT_SEL (4'd12 ~ 4'd15)"]
pub type BitW1sR = crate::BitReader;
#[doc = "Field `BIT_W1S` writer - Write One Setting, it is only valid for RXERR_RPT_SEL (4'd12 ~ 4'd15)"]
pub type BitW1sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_UD_Select_BSSID_1` reader - {24,22} Select matched BSSID: 2'b0: Port 0 2'b1: Port 1 otherwise, do not care bssid match or not"]
pub type BitUdSelectBssid1R = crate::BitReader;
#[doc = "Field `BIT_UD_Select_BSSID_1` writer - {24,22} Select matched BSSID: 2'b0: Port 0 2'b1: Port 1 otherwise, do not care bssid match or not"]
pub type BitUdSelectBssid1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXERR_RPT_SEL_V1_5` reader - Rx packet counter report selection bit5"]
pub type BitRxerrRptSelV1_5R = crate::BitReader;
#[doc = "Field `BIT_RXERR_RPT_SEL_V1_5` writer - Rx packet counter report selection bit5"]
pub type BitRxerrRptSelV1_5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXERR_RPT_SEL_V1_4` reader - Rx packet counter report selection bit4"]
pub type BitRxerrRptSelV1_4R = crate::BitReader;
#[doc = "Field `BIT_RXERR_RPT_SEL_V1_4` writer - Rx packet counter report selection bit4"]
pub type BitRxerrRptSelV1_4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXERR_RPT_RST` writer - Write \"1\" to set the counter to zero. One pulse"]
pub type BitRxerrRptRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXERR_RPT_SEL_V1_3_0` reader - Rx packet counter report selection bit3~bit0 RXERR_RPT_SEL\\[25,26,31:28\\]: Counter description 6'd0: OFDM MPDU OK counter 6'd1: OFDM MPDU Fail counter 6'd2: OFDM False Alarm counter 6'd3: CCK MPDU OK counter 6'd4: CCK MPDU Fail counter 6'd5: CCK False Alarm counter; 6'd6: HT MPDU OK counter 6'd7: HT MPDU Fail counter 6'd8: HT PPDU PLCP ok counter 6'd9: HT False Alarm counter 6'd10: Rx full drop packet counter 6'd11: RSVD 6'd12: user define 0 counter 6'd13: user define 1 counter 6'd14: user define 2 counter 6'd15: user define 3 counter 6'd16: VHT SU MPDU OK counter 6'd17: VHT SU MPDU Fail counter 6'd18: VHT SU PPDU plcp ok counter 6'd19: VHT SU False Alarm counter 6'd20: invalid_pkt counter 6'd21: VHT MU MPDU OK counter 6'd22: VHT MU MPDU Fail counter 6'd23: VHT MU PPDU plcp ok counter 6'd24: VHT MU False Alarm counter 6'd25: HE SU MPDU OK counter 6'd26: HE SU MPDU Fail counter 6'd27: HE SU PPDU plcp ok counter 6'd28: HE SU False Alarm counter 6'd29: HE MU MPDU OK counter 6'd30: HE MU MPDU Fail counter 6'd31: HE MU PPDU plcp ok counter 6'd32: HE MU False Alarm counter 6'd33: drop packet counter"]
pub type BitRxerrRptSelV1_3_0R = crate::FieldReader;
#[doc = "Field `BIT_RXERR_RPT_SEL_V1_3_0` writer - Rx packet counter report selection bit3~bit0 RXERR_RPT_SEL\\[25,26,31:28\\]: Counter description 6'd0: OFDM MPDU OK counter 6'd1: OFDM MPDU Fail counter 6'd2: OFDM False Alarm counter 6'd3: CCK MPDU OK counter 6'd4: CCK MPDU Fail counter 6'd5: CCK False Alarm counter; 6'd6: HT MPDU OK counter 6'd7: HT MPDU Fail counter 6'd8: HT PPDU PLCP ok counter 6'd9: HT False Alarm counter 6'd10: Rx full drop packet counter 6'd11: RSVD 6'd12: user define 0 counter 6'd13: user define 1 counter 6'd14: user define 2 counter 6'd15: user define 3 counter 6'd16: VHT SU MPDU OK counter 6'd17: VHT SU MPDU Fail counter 6'd18: VHT SU PPDU plcp ok counter 6'd19: VHT SU False Alarm counter 6'd20: invalid_pkt counter 6'd21: VHT MU MPDU OK counter 6'd22: VHT MU MPDU Fail counter 6'd23: VHT MU PPDU plcp ok counter 6'd24: VHT MU False Alarm counter 6'd25: HE SU MPDU OK counter 6'd26: HE SU MPDU Fail counter 6'd27: HE SU PPDU plcp ok counter 6'd28: HE SU False Alarm counter 6'd29: HE MU MPDU OK counter 6'd30: HE MU MPDU Fail counter 6'd31: HE MU PPDU plcp ok counter 6'd32: HE MU False Alarm counter 6'd33: drop packet counter"]
pub type BitRxerrRptSelV1_3_0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - The reported counter value corresponding to Report type selected."]
    #[inline(always)]
    pub fn bit_rpt_counter(&self) -> BitRptCounterR {
        BitRptCounterR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - User defined: Type"]
    #[inline(always)]
    pub fn bit_ud_type(&self) -> BitUdTypeR {
        BitUdTypeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:21 - User defined: SubType"]
    #[inline(always)]
    pub fn bit_ud_sub_type(&self) -> BitUdSubTypeR {
        BitUdSubTypeR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - Select BSSID bit0"]
    #[inline(always)]
    pub fn bit_ud_select_bssid_0(&self) -> BitUdSelectBssid0R {
        BitUdSelectBssid0R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Write One Setting, it is only valid for RXERR_RPT_SEL (4'd12 ~ 4'd15)"]
    #[inline(always)]
    pub fn bit_w1s(&self) -> BitW1sR {
        BitW1sR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - {24,22} Select matched BSSID: 2'b0: Port 0 2'b1: Port 1 otherwise, do not care bssid match or not"]
    #[inline(always)]
    pub fn bit_ud_select_bssid_1(&self) -> BitUdSelectBssid1R {
        BitUdSelectBssid1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx packet counter report selection bit5"]
    #[inline(always)]
    pub fn bit_rxerr_rpt_sel_v1_5(&self) -> BitRxerrRptSelV1_5R {
        BitRxerrRptSelV1_5R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Rx packet counter report selection bit4"]
    #[inline(always)]
    pub fn bit_rxerr_rpt_sel_v1_4(&self) -> BitRxerrRptSelV1_4R {
        BitRxerrRptSelV1_4R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Rx packet counter report selection bit3~bit0 RXERR_RPT_SEL\\[25,26,31:28\\]: Counter description 6'd0: OFDM MPDU OK counter 6'd1: OFDM MPDU Fail counter 6'd2: OFDM False Alarm counter 6'd3: CCK MPDU OK counter 6'd4: CCK MPDU Fail counter 6'd5: CCK False Alarm counter; 6'd6: HT MPDU OK counter 6'd7: HT MPDU Fail counter 6'd8: HT PPDU PLCP ok counter 6'd9: HT False Alarm counter 6'd10: Rx full drop packet counter 6'd11: RSVD 6'd12: user define 0 counter 6'd13: user define 1 counter 6'd14: user define 2 counter 6'd15: user define 3 counter 6'd16: VHT SU MPDU OK counter 6'd17: VHT SU MPDU Fail counter 6'd18: VHT SU PPDU plcp ok counter 6'd19: VHT SU False Alarm counter 6'd20: invalid_pkt counter 6'd21: VHT MU MPDU OK counter 6'd22: VHT MU MPDU Fail counter 6'd23: VHT MU PPDU plcp ok counter 6'd24: VHT MU False Alarm counter 6'd25: HE SU MPDU OK counter 6'd26: HE SU MPDU Fail counter 6'd27: HE SU PPDU plcp ok counter 6'd28: HE SU False Alarm counter 6'd29: HE MU MPDU OK counter 6'd30: HE MU MPDU Fail counter 6'd31: HE MU PPDU plcp ok counter 6'd32: HE MU False Alarm counter 6'd33: drop packet counter"]
    #[inline(always)]
    pub fn bit_rxerr_rpt_sel_v1_3_0(&self) -> BitRxerrRptSelV1_3_0R {
        BitRxerrRptSelV1_3_0R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - The reported counter value corresponding to Report type selected."]
    #[inline(always)]
    pub fn bit_rpt_counter(&mut self) -> BitRptCounterW<'_, RegRxerrRptSpec> {
        BitRptCounterW::new(self, 0)
    }
    #[doc = "Bits 16:17 - User defined: Type"]
    #[inline(always)]
    pub fn bit_ud_type(&mut self) -> BitUdTypeW<'_, RegRxerrRptSpec> {
        BitUdTypeW::new(self, 16)
    }
    #[doc = "Bits 18:21 - User defined: SubType"]
    #[inline(always)]
    pub fn bit_ud_sub_type(&mut self) -> BitUdSubTypeW<'_, RegRxerrRptSpec> {
        BitUdSubTypeW::new(self, 18)
    }
    #[doc = "Bit 22 - Select BSSID bit0"]
    #[inline(always)]
    pub fn bit_ud_select_bssid_0(&mut self) -> BitUdSelectBssid0W<'_, RegRxerrRptSpec> {
        BitUdSelectBssid0W::new(self, 22)
    }
    #[doc = "Bit 23 - Write One Setting, it is only valid for RXERR_RPT_SEL (4'd12 ~ 4'd15)"]
    #[inline(always)]
    pub fn bit_w1s(&mut self) -> BitW1sW<'_, RegRxerrRptSpec> {
        BitW1sW::new(self, 23)
    }
    #[doc = "Bit 24 - {24,22} Select matched BSSID: 2'b0: Port 0 2'b1: Port 1 otherwise, do not care bssid match or not"]
    #[inline(always)]
    pub fn bit_ud_select_bssid_1(&mut self) -> BitUdSelectBssid1W<'_, RegRxerrRptSpec> {
        BitUdSelectBssid1W::new(self, 24)
    }
    #[doc = "Bit 25 - Rx packet counter report selection bit5"]
    #[inline(always)]
    pub fn bit_rxerr_rpt_sel_v1_5(&mut self) -> BitRxerrRptSelV1_5W<'_, RegRxerrRptSpec> {
        BitRxerrRptSelV1_5W::new(self, 25)
    }
    #[doc = "Bit 26 - Rx packet counter report selection bit4"]
    #[inline(always)]
    pub fn bit_rxerr_rpt_sel_v1_4(&mut self) -> BitRxerrRptSelV1_4W<'_, RegRxerrRptSpec> {
        BitRxerrRptSelV1_4W::new(self, 26)
    }
    #[doc = "Bit 27 - Write \"1\" to set the counter to zero. One pulse"]
    #[inline(always)]
    pub fn bit_rxerr_rpt_rst(&mut self) -> BitRxerrRptRstW<'_, RegRxerrRptSpec> {
        BitRxerrRptRstW::new(self, 27)
    }
    #[doc = "Bits 28:31 - Rx packet counter report selection bit3~bit0 RXERR_RPT_SEL\\[25,26,31:28\\]: Counter description 6'd0: OFDM MPDU OK counter 6'd1: OFDM MPDU Fail counter 6'd2: OFDM False Alarm counter 6'd3: CCK MPDU OK counter 6'd4: CCK MPDU Fail counter 6'd5: CCK False Alarm counter; 6'd6: HT MPDU OK counter 6'd7: HT MPDU Fail counter 6'd8: HT PPDU PLCP ok counter 6'd9: HT False Alarm counter 6'd10: Rx full drop packet counter 6'd11: RSVD 6'd12: user define 0 counter 6'd13: user define 1 counter 6'd14: user define 2 counter 6'd15: user define 3 counter 6'd16: VHT SU MPDU OK counter 6'd17: VHT SU MPDU Fail counter 6'd18: VHT SU PPDU plcp ok counter 6'd19: VHT SU False Alarm counter 6'd20: invalid_pkt counter 6'd21: VHT MU MPDU OK counter 6'd22: VHT MU MPDU Fail counter 6'd23: VHT MU PPDU plcp ok counter 6'd24: VHT MU False Alarm counter 6'd25: HE SU MPDU OK counter 6'd26: HE SU MPDU Fail counter 6'd27: HE SU PPDU plcp ok counter 6'd28: HE SU False Alarm counter 6'd29: HE MU MPDU OK counter 6'd30: HE MU MPDU Fail counter 6'd31: HE MU PPDU plcp ok counter 6'd32: HE MU False Alarm counter 6'd33: drop packet counter"]
    #[inline(always)]
    pub fn bit_rxerr_rpt_sel_v1_3_0(&mut self) -> BitRxerrRptSelV1_3_0W<'_, RegRxerrRptSpec> {
        BitRxerrRptSelV1_3_0W::new(self, 28)
    }
}
#[doc = "RX Error Report Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rxerr_rpt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rxerr_rpt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxerrRptSpec;
impl crate::RegisterSpec for RegRxerrRptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rxerr_rpt::R`](R) reader structure"]
impl crate::Readable for RegRxerrRptSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rxerr_rpt::W`](W) writer structure"]
impl crate::Writable for RegRxerrRptSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RXERR_RPT to value 0"]
impl crate::Resettable for RegRxerrRptSpec {}
