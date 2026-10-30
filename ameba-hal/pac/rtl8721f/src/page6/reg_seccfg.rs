#[doc = "Register `REG_SECCFG` reader"]
pub type R = crate::R<RegSeccfgSpec>;
#[doc = "Register `REG_SECCFG` writer"]
pub type W = crate::W<RegSeccfgSpec>;
#[doc = "Field `BIT_TXUHUSEDK` reader - Force Tx Unicast Use Default Key If this bit is set to 1, use default key for TX key search when pairwise key is not found."]
pub type BitTxuhusedkR = crate::BitReader;
#[doc = "Field `BIT_TXUHUSEDK` writer - Force Tx Unicast Use Default Key If this bit is set to 1, use default key for TX key search when pairwise key is not found."]
pub type BitTxuhusedkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXUHUSEDK` reader - Force RX Unicast Use Default Key If this bit is set to 1, use default key for RX key search when pairwise key is not found."]
pub type BitRxuhusedkR = crate::BitReader;
#[doc = "Field `BIT_RXUHUSEDK` writer - Force RX Unicast Use Default Key If this bit is set to 1, use default key for RX key search when pairwise key is not found."]
pub type BitRxuhusedkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TXENC` reader - Enable Tx Encryption Setting to 1 to enable Tx hardware encryption."]
pub type BitTxencR = crate::BitReader;
#[doc = "Field `BIT_TXENC` writer - Enable Tx Encryption Setting to 1 to enable Tx hardware encryption."]
pub type BitTxencW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXDEC` reader - Enable Rx Decryption Setting 1 to enable Rx hardware decryption."]
pub type BitRxdecR = crate::BitReader;
#[doc = "Field `BIT_RXDEC` writer - Enable Rx Decryption Setting 1 to enable Rx hardware decryption."]
pub type BitRxdecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SKBYA2` reader - Search Key by A2 Setting 1 to search security CAM by address 2 for TX and search CAM by address 1 for RX. Setting 0 to search security CAM by address 1 for TX and search CAM by address 2 for RX. Used in MBSSID client mode."]
pub type BitSkbya2R = crate::BitReader;
#[doc = "Field `BIT_SKBYA2` writer - Search Key by A2 Setting 1 to search security CAM by address 2 for TX and search CAM by address 1 for RX. Setting 0 to search security CAM by address 1 for TX and search CAM by address 2 for RX. Used in MBSSID client mode."]
pub type BitSkbya2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_NOSKMC` reader - No Key Search for Multicast Setting 1 to disable key search for multicast frames. Used in MBSSID AP mode. Software should manage security encryption and decryption."]
pub type BitNoskmcR = crate::BitReader;
#[doc = "Field `BIT_NOSKMC` writer - No Key Search for Multicast Setting 1 to disable key search for multicast frames. Used in MBSSID AP mode. Software should manage security encryption and decryption."]
pub type BitNoskmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TXBCUSEDK` reader - Force Tx Broadcast packets Use Default Key If this bit is set to 1, use default key for TX key search when transmitted packet is broadcast and multicast."]
pub type BitTxbcusedkR = crate::BitReader;
#[doc = "Field `BIT_TXBCUSEDK` writer - Force Tx Broadcast packets Use Default Key If this bit is set to 1, use default key for TX key search when transmitted packet is broadcast and multicast."]
pub type BitTxbcusedkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXBCUSEDK` reader - Force RX Broadcast packets Use Default Key If this bit is set to 1, use default key for RX key search when received packet is broadcast and multicast."]
pub type BitRxbcusedkR = crate::BitReader;
#[doc = "Field `BIT_RXBCUSEDK` writer - Force RX Broadcast packets Use Default Key If this bit is set to 1, use default key for RX key search when received packet is broadcast and multicast."]
pub type BitRxbcusedkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CHK_KEYID` reader - If this bit is set to 1, key search engine need to check if key ID matched. If this bit is set to 0, then key ID table must be filled with 2\"b00."]
pub type BitChkKeyidR = crate::BitReader;
#[doc = "Field `BIT_CHK_KEYID` writer - If this bit is set to 1, key search engine need to check if key ID matched. If this bit is set to 0, then key ID table must be filled with 2\"b00."]
pub type BitChkKeyidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_CKECK_BMC` reader - If this bit is set to 1, key search engine need to check if broadcast/multicast bit matched."]
pub type BitWmacCkeckBmcR = crate::BitReader;
#[doc = "Field `BIT_WMAC_CKECK_BMC` writer - If this bit is set to 1, key search engine need to check if broadcast/multicast bit matched."]
pub type BitWmacCkeckBmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_AES_SEL_QC_0` reader - 1:use QC\\[7\\] to compute ctr 0: not use QC\\[7\\] to compute ctr"]
pub type BitAesSelQc0R = crate::BitReader;
#[doc = "Field `BIT_AES_SEL_QC_0` writer - 1:use QC\\[7\\] to compute ctr 0: not use QC\\[7\\] to compute ctr"]
pub type BitAesSelQc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_AES_SEL_QC_1` reader - 1: use QC\\[7\\] to compute mic 0: not use QC\\[7\\] to compute mic"]
pub type BitAesSelQc1R = crate::BitReader;
#[doc = "Field `BIT_AES_SEL_QC_1` writer - 1: use QC\\[7\\] to compute mic 0: not use QC\\[7\\] to compute mic"]
pub type BitAesSelQc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_GCLK_TKIP` reader - Disable gated clock control for WEP/TKIP engine."]
pub type BitDisGclkTkipR = crate::BitReader;
#[doc = "Field `BIT_DIS_GCLK_TKIP` writer - Disable gated clock control for WEP/TKIP engine."]
pub type BitDisGclkTkipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_GCLK_AES` reader - Disable gated clock control for AES engine."]
pub type BitDisGclkAesR = crate::BitReader;
#[doc = "Field `BIT_DIS_GCLK_AES` writer - Disable gated clock control for AES engine."]
pub type BitDisGclkAesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_GCLK_WAPI` reader - Disable gated clock control for WAPI engine"]
pub type BitDisGclkWapiR = crate::BitReader;
#[doc = "Field `BIT_DIS_GCLK_WAPI` writer - Disable gated clock control for WAPI engine"]
pub type BitDisGclkWapiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXFILTER_CATEGORY_1` reader - Configured by FW, Refer to 0x68E\\[10\\] to decide accepting this frame or not"]
pub type BitRxfilterCategory1R = crate::FieldReader;
#[doc = "Field `BIT_RXFILTER_CATEGORY_1` writer - Configured by FW, Refer to 0x68E\\[10\\] to decide accepting this frame or not"]
pub type BitRxfilterCategory1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_RXFILTER_ACTION_1` reader - Configured by FW, Refer to 0x68E\\[10\\] to decide accepting this frame or not"]
pub type BitRxfilterAction1R = crate::FieldReader;
#[doc = "Field `BIT_RXFILTER_ACTION_1` writer - Configured by FW, Refer to 0x68E\\[10\\] to decide accepting this frame or not"]
pub type BitRxfilterAction1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Force Tx Unicast Use Default Key If this bit is set to 1, use default key for TX key search when pairwise key is not found."]
    #[inline(always)]
    pub fn bit_txuhusedk(&self) -> BitTxuhusedkR {
        BitTxuhusedkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force RX Unicast Use Default Key If this bit is set to 1, use default key for RX key search when pairwise key is not found."]
    #[inline(always)]
    pub fn bit_rxuhusedk(&self) -> BitRxuhusedkR {
        BitRxuhusedkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Tx Encryption Setting to 1 to enable Tx hardware encryption."]
    #[inline(always)]
    pub fn bit_txenc(&self) -> BitTxencR {
        BitTxencR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Rx Decryption Setting 1 to enable Rx hardware decryption."]
    #[inline(always)]
    pub fn bit_rxdec(&self) -> BitRxdecR {
        BitRxdecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Search Key by A2 Setting 1 to search security CAM by address 2 for TX and search CAM by address 1 for RX. Setting 0 to search security CAM by address 1 for TX and search CAM by address 2 for RX. Used in MBSSID client mode."]
    #[inline(always)]
    pub fn bit_skbya2(&self) -> BitSkbya2R {
        BitSkbya2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No Key Search for Multicast Setting 1 to disable key search for multicast frames. Used in MBSSID AP mode. Software should manage security encryption and decryption."]
    #[inline(always)]
    pub fn bit_noskmc(&self) -> BitNoskmcR {
        BitNoskmcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Force Tx Broadcast packets Use Default Key If this bit is set to 1, use default key for TX key search when transmitted packet is broadcast and multicast."]
    #[inline(always)]
    pub fn bit_txbcusedk(&self) -> BitTxbcusedkR {
        BitTxbcusedkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Force RX Broadcast packets Use Default Key If this bit is set to 1, use default key for RX key search when received packet is broadcast and multicast."]
    #[inline(always)]
    pub fn bit_rxbcusedk(&self) -> BitRxbcusedkR {
        BitRxbcusedkR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - If this bit is set to 1, key search engine need to check if key ID matched. If this bit is set to 0, then key ID table must be filled with 2\"b00."]
    #[inline(always)]
    pub fn bit_chk_keyid(&self) -> BitChkKeyidR {
        BitChkKeyidR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - If this bit is set to 1, key search engine need to check if broadcast/multicast bit matched."]
    #[inline(always)]
    pub fn bit_wmac_ckeck_bmc(&self) -> BitWmacCkeckBmcR {
        BitWmacCkeckBmcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - 1:use QC\\[7\\] to compute ctr 0: not use QC\\[7\\] to compute ctr"]
    #[inline(always)]
    pub fn bit_aes_sel_qc_0(&self) -> BitAesSelQc0R {
        BitAesSelQc0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: use QC\\[7\\] to compute mic 0: not use QC\\[7\\] to compute mic"]
    #[inline(always)]
    pub fn bit_aes_sel_qc_1(&self) -> BitAesSelQc1R {
        BitAesSelQc1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Disable gated clock control for WEP/TKIP engine."]
    #[inline(always)]
    pub fn bit_dis_gclk_tkip(&self) -> BitDisGclkTkipR {
        BitDisGclkTkipR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Disable gated clock control for AES engine."]
    #[inline(always)]
    pub fn bit_dis_gclk_aes(&self) -> BitDisGclkAesR {
        BitDisGclkAesR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Disable gated clock control for WAPI engine"]
    #[inline(always)]
    pub fn bit_dis_gclk_wapi(&self) -> BitDisGclkWapiR {
        BitDisGclkWapiR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Configured by FW, Refer to 0x68E\\[10\\] to decide accepting this frame or not"]
    #[inline(always)]
    pub fn bit_rxfilter_category_1(&self) -> BitRxfilterCategory1R {
        BitRxfilterCategory1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Configured by FW, Refer to 0x68E\\[10\\] to decide accepting this frame or not"]
    #[inline(always)]
    pub fn bit_rxfilter_action_1(&self) -> BitRxfilterAction1R {
        BitRxfilterAction1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Force Tx Unicast Use Default Key If this bit is set to 1, use default key for TX key search when pairwise key is not found."]
    #[inline(always)]
    pub fn bit_txuhusedk(&mut self) -> BitTxuhusedkW<'_, RegSeccfgSpec> {
        BitTxuhusedkW::new(self, 0)
    }
    #[doc = "Bit 1 - Force RX Unicast Use Default Key If this bit is set to 1, use default key for RX key search when pairwise key is not found."]
    #[inline(always)]
    pub fn bit_rxuhusedk(&mut self) -> BitRxuhusedkW<'_, RegSeccfgSpec> {
        BitRxuhusedkW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Tx Encryption Setting to 1 to enable Tx hardware encryption."]
    #[inline(always)]
    pub fn bit_txenc(&mut self) -> BitTxencW<'_, RegSeccfgSpec> {
        BitTxencW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Rx Decryption Setting 1 to enable Rx hardware decryption."]
    #[inline(always)]
    pub fn bit_rxdec(&mut self) -> BitRxdecW<'_, RegSeccfgSpec> {
        BitRxdecW::new(self, 3)
    }
    #[doc = "Bit 4 - Search Key by A2 Setting 1 to search security CAM by address 2 for TX and search CAM by address 1 for RX. Setting 0 to search security CAM by address 1 for TX and search CAM by address 2 for RX. Used in MBSSID client mode."]
    #[inline(always)]
    pub fn bit_skbya2(&mut self) -> BitSkbya2W<'_, RegSeccfgSpec> {
        BitSkbya2W::new(self, 4)
    }
    #[doc = "Bit 5 - No Key Search for Multicast Setting 1 to disable key search for multicast frames. Used in MBSSID AP mode. Software should manage security encryption and decryption."]
    #[inline(always)]
    pub fn bit_noskmc(&mut self) -> BitNoskmcW<'_, RegSeccfgSpec> {
        BitNoskmcW::new(self, 5)
    }
    #[doc = "Bit 6 - Force Tx Broadcast packets Use Default Key If this bit is set to 1, use default key for TX key search when transmitted packet is broadcast and multicast."]
    #[inline(always)]
    pub fn bit_txbcusedk(&mut self) -> BitTxbcusedkW<'_, RegSeccfgSpec> {
        BitTxbcusedkW::new(self, 6)
    }
    #[doc = "Bit 7 - Force RX Broadcast packets Use Default Key If this bit is set to 1, use default key for RX key search when received packet is broadcast and multicast."]
    #[inline(always)]
    pub fn bit_rxbcusedk(&mut self) -> BitRxbcusedkW<'_, RegSeccfgSpec> {
        BitRxbcusedkW::new(self, 7)
    }
    #[doc = "Bit 8 - If this bit is set to 1, key search engine need to check if key ID matched. If this bit is set to 0, then key ID table must be filled with 2\"b00."]
    #[inline(always)]
    pub fn bit_chk_keyid(&mut self) -> BitChkKeyidW<'_, RegSeccfgSpec> {
        BitChkKeyidW::new(self, 8)
    }
    #[doc = "Bit 9 - If this bit is set to 1, key search engine need to check if broadcast/multicast bit matched."]
    #[inline(always)]
    pub fn bit_wmac_ckeck_bmc(&mut self) -> BitWmacCkeckBmcW<'_, RegSeccfgSpec> {
        BitWmacCkeckBmcW::new(self, 9)
    }
    #[doc = "Bit 11 - 1:use QC\\[7\\] to compute ctr 0: not use QC\\[7\\] to compute ctr"]
    #[inline(always)]
    pub fn bit_aes_sel_qc_0(&mut self) -> BitAesSelQc0W<'_, RegSeccfgSpec> {
        BitAesSelQc0W::new(self, 11)
    }
    #[doc = "Bit 12 - 1: use QC\\[7\\] to compute mic 0: not use QC\\[7\\] to compute mic"]
    #[inline(always)]
    pub fn bit_aes_sel_qc_1(&mut self) -> BitAesSelQc1W<'_, RegSeccfgSpec> {
        BitAesSelQc1W::new(self, 12)
    }
    #[doc = "Bit 13 - Disable gated clock control for WEP/TKIP engine."]
    #[inline(always)]
    pub fn bit_dis_gclk_tkip(&mut self) -> BitDisGclkTkipW<'_, RegSeccfgSpec> {
        BitDisGclkTkipW::new(self, 13)
    }
    #[doc = "Bit 14 - Disable gated clock control for AES engine."]
    #[inline(always)]
    pub fn bit_dis_gclk_aes(&mut self) -> BitDisGclkAesW<'_, RegSeccfgSpec> {
        BitDisGclkAesW::new(self, 14)
    }
    #[doc = "Bit 15 - Disable gated clock control for WAPI engine"]
    #[inline(always)]
    pub fn bit_dis_gclk_wapi(&mut self) -> BitDisGclkWapiW<'_, RegSeccfgSpec> {
        BitDisGclkWapiW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Configured by FW, Refer to 0x68E\\[10\\] to decide accepting this frame or not"]
    #[inline(always)]
    pub fn bit_rxfilter_category_1(&mut self) -> BitRxfilterCategory1W<'_, RegSeccfgSpec> {
        BitRxfilterCategory1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Configured by FW, Refer to 0x68E\\[10\\] to decide accepting this frame or not"]
    #[inline(always)]
    pub fn bit_rxfilter_action_1(&mut self) -> BitRxfilterAction1W<'_, RegSeccfgSpec> {
        BitRxfilterAction1W::new(self, 24)
    }
}
#[doc = "Security Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_seccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_seccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSeccfgSpec;
impl crate::RegisterSpec for RegSeccfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_seccfg::R`](R) reader structure"]
impl crate::Readable for RegSeccfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_seccfg::W`](W) writer structure"]
impl crate::Writable for RegSeccfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SECCFG to value 0x0c"]
impl crate::Resettable for RegSeccfgSpec {
    const RESET_VALUE: u32 = 0x0c;
}
