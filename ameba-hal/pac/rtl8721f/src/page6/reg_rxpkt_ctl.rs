#[doc = "Register `REG_RXPKT_CTL` reader"]
pub type R = crate::R<RegRxpktCtlSpec>;
#[doc = "Register `REG_RXPKT_CTL` writer"]
pub type W = crate::W<RegRxpktCtlSpec>;
#[doc = "Field `BIT_RXPKTLMT` reader - RX PKT Upper Limit. The unit is 512 bytes. When \"0\", no RX limit is forced."]
pub type BitRxpktlmtR = crate::FieldReader;
#[doc = "Field `BIT_RXPKTLMT` writer - RX PKT Upper Limit. The unit is 512 bytes. When \"0\", no RX limit is forced."]
pub type BitRxpktlmtW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BIT_RX_DLK_TIME` reader - macrx dead lock timer timeout value (unit in 4us). As data_on is deasserted and no RXD is received but macrx still waits coming RXD, the deadlock timer will be enabled in this case"]
pub type BitRxDlkTimeR = crate::FieldReader;
#[doc = "Field `BIT_RX_DLK_TIME` writer - macrx dead lock timer timeout value (unit in 4us). As data_on is deasserted and no RXD is received but macrx still waits coming RXD, the deadlock timer will be enabled in this case"]
pub type BitRxDlkTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_MBID_BCNNO` reader - "]
pub type BitMbidBcnnoR = crate::FieldReader;
#[doc = "Field `BIT_MBID_BCNNO` writer - "]
pub type BitMbidBcnnoW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_MBID_EN` reader - "]
pub type BitMbidEnR = crate::BitReader;
#[doc = "Field `BIT_MBID_EN` writer - "]
pub type BitMbidEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RX_DRVINFO_SZ` reader - RX DESC Driver Info Size: For RXPKTBUF layout, this field indicates the offset starting from the end of RXDESC to the beginning of RX packet. The unit is 8-byte. This value will be copied to DRV_INFO_SIZE field in RXDESC."]
pub type BitRxDrvinfoSzR = crate::FieldReader;
#[doc = "Field `BIT_RX_DRVINFO_SZ` writer - RX DESC Driver Info Size: For RXPKTBUF layout, this field indicates the offset starting from the end of RXDESC to the beginning of RX packet. The unit is 8-byte. This value will be copied to DRV_INFO_SIZE field in RXDESC."]
pub type BitRxDrvinfoSzW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_WMAC_ENSHIFT` reader - MAC header shift function enable signal in MACRX"]
pub type BitWmacEnshiftR = crate::BitReader;
#[doc = "Field `BIT_WMAC_ENSHIFT` writer - MAC header shift function enable signal in MACRX"]
pub type BitWmacEnshiftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_APP_MH_SHIFT_VAL` reader - When this bit is set to 1,append MAC header shift value in rx descriptor,otherwise, not append"]
pub type BitAppMhShiftValR = crate::BitReader;
#[doc = "Field `BIT_APP_MH_SHIFT_VAL` writer - When this bit is set to 1,append MAC header shift value in rx descriptor,otherwise, not append"]
pub type BitAppMhShiftValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_APP_PHYSTS_PER_SUBMPDU` reader - When this bit is set to 1,append phy status for each mpdu if phy status is valid. Otherwise,append phy status for each ppdu."]
pub type BitAppPhystsPerSubmpduR = crate::BitReader;
#[doc = "Field `BIT_APP_PHYSTS_PER_SUBMPDU` writer - When this bit is set to 1,append phy status for each mpdu if phy status is valid. Otherwise,append phy status for each ppdu."]
pub type BitAppPhystsPerSubmpduW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - RX PKT Upper Limit. The unit is 512 bytes. When \"0\", no RX limit is forced."]
    #[inline(always)]
    pub fn bit_rxpktlmt(&self) -> BitRxpktlmtR {
        BitRxpktlmtR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - macrx dead lock timer timeout value (unit in 4us). As data_on is deasserted and no RXD is received but macrx still waits coming RXD, the deadlock timer will be enabled in this case"]
    #[inline(always)]
    pub fn bit_rx_dlk_time(&self) -> BitRxDlkTimeR {
        BitRxDlkTimeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn bit_mbid_bcnno(&self) -> BitMbidBcnnoR {
        BitMbidBcnnoR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn bit_mbid_en(&self) -> BitMbidEnR {
        BitMbidEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - RX DESC Driver Info Size: For RXPKTBUF layout, this field indicates the offset starting from the end of RXDESC to the beginning of RX packet. The unit is 8-byte. This value will be copied to DRV_INFO_SIZE field in RXDESC."]
    #[inline(always)]
    pub fn bit_rx_drvinfo_sz(&self) -> BitRxDrvinfoSzR {
        BitRxDrvinfoSzR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - MAC header shift function enable signal in MACRX"]
    #[inline(always)]
    pub fn bit_wmac_enshift(&self) -> BitWmacEnshiftR {
        BitWmacEnshiftR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When this bit is set to 1,append MAC header shift value in rx descriptor,otherwise, not append"]
    #[inline(always)]
    pub fn bit_app_mh_shift_val(&self) -> BitAppMhShiftValR {
        BitAppMhShiftValR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When this bit is set to 1,append phy status for each mpdu if phy status is valid. Otherwise,append phy status for each ppdu."]
    #[inline(always)]
    pub fn bit_app_physts_per_submpdu(&self) -> BitAppPhystsPerSubmpduR {
        BitAppPhystsPerSubmpduR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - RX PKT Upper Limit. The unit is 512 bytes. When \"0\", no RX limit is forced."]
    #[inline(always)]
    pub fn bit_rxpktlmt(&mut self) -> BitRxpktlmtW<'_, RegRxpktCtlSpec> {
        BitRxpktlmtW::new(self, 0)
    }
    #[doc = "Bits 8:15 - macrx dead lock timer timeout value (unit in 4us). As data_on is deasserted and no RXD is received but macrx still waits coming RXD, the deadlock timer will be enabled in this case"]
    #[inline(always)]
    pub fn bit_rx_dlk_time(&mut self) -> BitRxDlkTimeW<'_, RegRxpktCtlSpec> {
        BitRxDlkTimeW::new(self, 8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn bit_mbid_bcnno(&mut self) -> BitMbidBcnnoW<'_, RegRxpktCtlSpec> {
        BitMbidBcnnoW::new(self, 20)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn bit_mbid_en(&mut self) -> BitMbidEnW<'_, RegRxpktCtlSpec> {
        BitMbidEnW::new(self, 23)
    }
    #[doc = "Bits 24:27 - RX DESC Driver Info Size: For RXPKTBUF layout, this field indicates the offset starting from the end of RXDESC to the beginning of RX packet. The unit is 8-byte. This value will be copied to DRV_INFO_SIZE field in RXDESC."]
    #[inline(always)]
    pub fn bit_rx_drvinfo_sz(&mut self) -> BitRxDrvinfoSzW<'_, RegRxpktCtlSpec> {
        BitRxDrvinfoSzW::new(self, 24)
    }
    #[doc = "Bit 29 - MAC header shift function enable signal in MACRX"]
    #[inline(always)]
    pub fn bit_wmac_enshift(&mut self) -> BitWmacEnshiftW<'_, RegRxpktCtlSpec> {
        BitWmacEnshiftW::new(self, 29)
    }
    #[doc = "Bit 30 - When this bit is set to 1,append MAC header shift value in rx descriptor,otherwise, not append"]
    #[inline(always)]
    pub fn bit_app_mh_shift_val(&mut self) -> BitAppMhShiftValW<'_, RegRxpktCtlSpec> {
        BitAppMhShiftValW::new(self, 30)
    }
    #[doc = "Bit 31 - When this bit is set to 1,append phy status for each mpdu if phy status is valid. Otherwise,append phy status for each ppdu."]
    #[inline(always)]
    pub fn bit_app_physts_per_submpdu(&mut self) -> BitAppPhystsPerSubmpduW<'_, RegRxpktCtlSpec> {
        BitAppPhystsPerSubmpduW::new(self, 31)
    }
}
#[doc = "RX Packet Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rxpkt_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rxpkt_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxpktCtlSpec;
impl crate::RegisterSpec for RegRxpktCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rxpkt_ctl::R`](R) reader structure"]
impl crate::Readable for RegRxpktCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rxpkt_ctl::W`](W) writer structure"]
impl crate::Writable for RegRxpktCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RXPKT_CTL to value 0x0400_0410"]
impl crate::Resettable for RegRxpktCtlSpec {
    const RESET_VALUE: u32 = 0x0400_0410;
}
