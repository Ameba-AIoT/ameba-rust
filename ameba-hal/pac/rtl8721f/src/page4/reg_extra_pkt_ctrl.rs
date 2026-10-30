#[doc = "Register `REG_EXTRA_PKT_CTRL` reader"]
pub type R = crate::R<RegExtraPktCtrlSpec>;
#[doc = "Register `REG_EXTRA_PKT_CTRL` writer"]
pub type W = crate::W<RegExtraPktCtrlSpec>;
#[doc = "Field `BIT_SEL_DESC_INDEX` reader - 1: select rate index from txdesc"]
pub type BitSelDescIndexR = crate::BitReader;
#[doc = "Field `BIT_SEL_DESC_INDEX` writer - 1: select rate index from txdesc"]
pub type BitSelDescIndexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_IGNORE_QOS_NULL` reader - 1: will not TX QoS Null when rx Basic trigger"]
pub type BitIgnoreQosNullR = crate::BitReader;
#[doc = "Field `BIT_IGNORE_QOS_NULL` writer - 1: will not TX QoS Null when rx Basic trigger"]
pub type BitIgnoreQosNullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TXDESC_PWR_OFT` reader - indicate the tx power compensation of TXDESC.TX_PWR_OFT, unit: 1dbm"]
pub type BitTxdescPwrOftR = crate::FieldReader;
#[doc = "Field `BIT_TXDESC_PWR_OFT` writer - indicate the tx power compensation of TXDESC.TX_PWR_OFT, unit: 1dbm"]
pub type BitTxdescPwrOftW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_EN_CTRL_HTC` reader - 1: rts/cts2s/cts2ap/bar will carry HTC field by ctrl_wrapper frame, valid when txdesc.HTC = 1 0: rts/cts2s/cts2ap/bar will NOT carry HTC field even though txdesc.HTC = 1"]
pub type BitEnCtrlHtcR = crate::BitReader;
#[doc = "Field `BIT_EN_CTRL_HTC` writer - 1: rts/cts2s/cts2ap/bar will carry HTC field by ctrl_wrapper frame, valid when txdesc.HTC = 1 0: rts/cts2s/cts2ap/bar will NOT carry HTC field even though txdesc.HTC = 1"]
pub type BitEnCtrlHtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_ONLY_TB_HTC` reader - 1: only TB can carry HTC field"]
pub type BitEnOnlyTbHtcR = crate::BitReader;
#[doc = "Field `BIT_EN_ONLY_TB_HTC` writer - 1: only TB can carry HTC field"]
pub type BitEnOnlyTbHtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_HTC_RATE_ADPT` reader - 1: if tx data rate fallback to CCK/OFDM rate, HW will remove HTC field in mac header"]
pub type BitEnHtcRateAdptR = crate::BitReader;
#[doc = "Field `BIT_EN_HTC_RATE_ADPT` writer - 1: if tx data rate fallback to CCK/OFDM rate, HW will remove HTC field in mac header"]
pub type BitEnHtcRateAdptW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: select rate index from txdesc"]
    #[inline(always)]
    pub fn bit_sel_desc_index(&self) -> BitSelDescIndexR {
        BitSelDescIndexR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: will not TX QoS Null when rx Basic trigger"]
    #[inline(always)]
    pub fn bit_ignore_qos_null(&self) -> BitIgnoreQosNullR {
        BitIgnoreQosNullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - indicate the tx power compensation of TXDESC.TX_PWR_OFT, unit: 1dbm"]
    #[inline(always)]
    pub fn bit_txdesc_pwr_oft(&self) -> BitTxdescPwrOftR {
        BitTxdescPwrOftR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 1: rts/cts2s/cts2ap/bar will carry HTC field by ctrl_wrapper frame, valid when txdesc.HTC = 1 0: rts/cts2s/cts2ap/bar will NOT carry HTC field even though txdesc.HTC = 1"]
    #[inline(always)]
    pub fn bit_en_ctrl_htc(&self) -> BitEnCtrlHtcR {
        BitEnCtrlHtcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: only TB can carry HTC field"]
    #[inline(always)]
    pub fn bit_en_only_tb_htc(&self) -> BitEnOnlyTbHtcR {
        BitEnOnlyTbHtcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: if tx data rate fallback to CCK/OFDM rate, HW will remove HTC field in mac header"]
    #[inline(always)]
    pub fn bit_en_htc_rate_adpt(&self) -> BitEnHtcRateAdptR {
        BitEnHtcRateAdptR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: select rate index from txdesc"]
    #[inline(always)]
    pub fn bit_sel_desc_index(&mut self) -> BitSelDescIndexW<'_, RegExtraPktCtrlSpec> {
        BitSelDescIndexW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: will not TX QoS Null when rx Basic trigger"]
    #[inline(always)]
    pub fn bit_ignore_qos_null(&mut self) -> BitIgnoreQosNullW<'_, RegExtraPktCtrlSpec> {
        BitIgnoreQosNullW::new(self, 1)
    }
    #[doc = "Bits 4:7 - indicate the tx power compensation of TXDESC.TX_PWR_OFT, unit: 1dbm"]
    #[inline(always)]
    pub fn bit_txdesc_pwr_oft(&mut self) -> BitTxdescPwrOftW<'_, RegExtraPktCtrlSpec> {
        BitTxdescPwrOftW::new(self, 4)
    }
    #[doc = "Bit 8 - 1: rts/cts2s/cts2ap/bar will carry HTC field by ctrl_wrapper frame, valid when txdesc.HTC = 1 0: rts/cts2s/cts2ap/bar will NOT carry HTC field even though txdesc.HTC = 1"]
    #[inline(always)]
    pub fn bit_en_ctrl_htc(&mut self) -> BitEnCtrlHtcW<'_, RegExtraPktCtrlSpec> {
        BitEnCtrlHtcW::new(self, 8)
    }
    #[doc = "Bit 9 - 1: only TB can carry HTC field"]
    #[inline(always)]
    pub fn bit_en_only_tb_htc(&mut self) -> BitEnOnlyTbHtcW<'_, RegExtraPktCtrlSpec> {
        BitEnOnlyTbHtcW::new(self, 9)
    }
    #[doc = "Bit 10 - 1: if tx data rate fallback to CCK/OFDM rate, HW will remove HTC field in mac header"]
    #[inline(always)]
    pub fn bit_en_htc_rate_adpt(&mut self) -> BitEnHtcRateAdptW<'_, RegExtraPktCtrlSpec> {
        BitEnHtcRateAdptW::new(self, 10)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_extra_pkt_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_extra_pkt_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegExtraPktCtrlSpec;
impl crate::RegisterSpec for RegExtraPktCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_extra_pkt_ctrl::R`](R) reader structure"]
impl crate::Readable for RegExtraPktCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_extra_pkt_ctrl::W`](W) writer structure"]
impl crate::Writable for RegExtraPktCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_EXTRA_PKT_CTRL to value 0x0400"]
impl crate::Resettable for RegExtraPktCtrlSpec {
    const RESET_VALUE: u32 = 0x0400;
}
