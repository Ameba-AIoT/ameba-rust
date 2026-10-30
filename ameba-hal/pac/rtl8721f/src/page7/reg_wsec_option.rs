#[doc = "Register `REG_WSEC_OPTION` reader"]
pub type R = crate::R<RegWsecOptionSpec>;
#[doc = "Register `REG_WSEC_OPTION` writer"]
pub type W = crate::W<RegWsecOptionSpec>;
#[doc = "Field `BIT_BT_TIME_CNT` reader - US timer counter boundary of BTCOEXIST"]
pub type BitBtTimeCntR = crate::FieldReader;
#[doc = "Field `BIT_BT_TIME_CNT` writer - US timer counter boundary of BTCOEXIST"]
pub type BitBtTimeCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_GCM_WAPI_FC_NOMASK` reader - If set to 1, enable old GCM-WAPI ADD format, don't mask FC bit4~6, bit8~15, If set to 0, enable new GCM-WAPI ADD format, mask FC bit15 to 0."]
pub type BitGcmWapiFcNomaskR = crate::BitReader;
#[doc = "Field `BIT_GCM_WAPI_FC_NOMASK` writer - If set to 1, enable old GCM-WAPI ADD format, don't mask FC bit4~6, bit8~15, If set to 0, enable new GCM-WAPI ADD format, mask FC bit15 to 0."]
pub type BitGcmWapiFcNomaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WAPI_QC_NOMASK` reader - If set to 1, enable old WAPI ADD format, don't mask QC bit4~6, bit8~15, If set to 0, enable new WAPI ADD format, mask QC bit4~6, bit8~15 to 0."]
pub type BitWapiQcNomaskR = crate::BitReader;
#[doc = "Field `BIT_WAPI_QC_NOMASK` writer - If set to 1, enable old WAPI ADD format, don't mask QC bit4~6, bit8~15, If set to 0, enable new WAPI ADD format, mask QC bit4~6, bit8~15 to 0."]
pub type BitWapiQcNomaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WAPI_FC_NOMASK` reader - If set to 1, enable old WAPI ADD format, don't mask FC bit4~6, bit8~15, If set to 0, enable new WAPI ADD format, mask FC bit15 to 0."]
pub type BitWapiFcNomaskR = crate::BitReader;
#[doc = "Field `BIT_WAPI_FC_NOMASK` writer - If set to 1, enable old WAPI ADD format, don't mask FC bit4~6, bit8~15, If set to 0, enable new WAPI ADD format, mask FC bit15 to 0."]
pub type BitWapiFcNomaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_SEC_PN_SEL` reader - Security GCMP header PN selection"]
pub type BitWmacSecPnSelR = crate::FieldReader;
#[doc = "Field `BIT_WMAC_SEC_PN_SEL` writer - Security GCMP header PN selection"]
pub type BitWmacSecPnSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_WMAC_SEC_MASKIV` reader - MASK IV\\[15:8\\] for GCMP security"]
pub type BitWmacSecMaskivR = crate::BitReader;
#[doc = "Field `BIT_WMAC_SEC_MASKIV` writer - MASK IV\\[15:8\\] for GCMP security"]
pub type BitWmacSecMaskivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TXENC_UNI_MGNT` reader - encryption enable of tx unicast management pkt, 0x680\\[2\\] shall also be set valid"]
pub type BitTxencUniMgntR = crate::BitReader;
#[doc = "Field `BIT_TXENC_UNI_MGNT` writer - encryption enable of tx unicast management pkt, 0x680\\[2\\] shall also be set valid"]
pub type BitTxencUniMgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXDEC_UNI_MGNT` reader - decryption enable of rx unicast management pkt, 0x680\\[3\\] shall also be set valid"]
pub type BitRxdecUniMgntR = crate::BitReader;
#[doc = "Field `BIT_RXDEC_UNI_MGNT` writer - decryption enable of rx unicast management pkt, 0x680\\[3\\] shall also be set valid"]
pub type BitRxdecUniMgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TXENC_BM_MGNT` reader - encryption enable of tx multicast management pkt, 0x680\\[2\\] shall also be set valid"]
pub type BitTxencBmMgntR = crate::BitReader;
#[doc = "Field `BIT_TXENC_BM_MGNT` writer - encryption enable of tx multicast management pkt, 0x680\\[2\\] shall also be set valid"]
pub type BitTxencBmMgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXDEC_BM_MGNT` reader - decryption enable of rx multicast management pkt, 0x680\\[3\\] shall also be set valid"]
pub type BitRxdecBmMgntR = crate::BitReader;
#[doc = "Field `BIT_RXDEC_BM_MGNT` writer - decryption enable of rx multicast management pkt, 0x680\\[3\\] shall also be set valid"]
pub type BitRxdecBmMgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXDEC_BM_DIS_CHKFC` reader - disenable check fc\\[14\\] when bip decryption"]
pub type BitRxdecBmDisChkfcR = crate::BitReader;
#[doc = "Field `BIT_RXDEC_BM_DIS_CHKFC` writer - disenable check fc\\[14\\] when bip decryption"]
pub type BitRxdecBmDisChkfcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - US timer counter boundary of BTCOEXIST"]
    #[inline(always)]
    pub fn bit_bt_time_cnt(&self) -> BitBtTimeCntR {
        BitBtTimeCntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 13 - If set to 1, enable old GCM-WAPI ADD format, don't mask FC bit4~6, bit8~15, If set to 0, enable new GCM-WAPI ADD format, mask FC bit15 to 0."]
    #[inline(always)]
    pub fn bit_gcm_wapi_fc_nomask(&self) -> BitGcmWapiFcNomaskR {
        BitGcmWapiFcNomaskR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - If set to 1, enable old WAPI ADD format, don't mask QC bit4~6, bit8~15, If set to 0, enable new WAPI ADD format, mask QC bit4~6, bit8~15 to 0."]
    #[inline(always)]
    pub fn bit_wapi_qc_nomask(&self) -> BitWapiQcNomaskR {
        BitWapiQcNomaskR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - If set to 1, enable old WAPI ADD format, don't mask FC bit4~6, bit8~15, If set to 0, enable new WAPI ADD format, mask FC bit15 to 0."]
    #[inline(always)]
    pub fn bit_wapi_fc_nomask(&self) -> BitWapiFcNomaskR {
        BitWapiFcNomaskR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Security GCMP header PN selection"]
    #[inline(always)]
    pub fn bit_wmac_sec_pn_sel(&self) -> BitWmacSecPnSelR {
        BitWmacSecPnSelR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - MASK IV\\[15:8\\] for GCMP security"]
    #[inline(always)]
    pub fn bit_wmac_sec_maskiv(&self) -> BitWmacSecMaskivR {
        BitWmacSecMaskivR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - encryption enable of tx unicast management pkt, 0x680\\[2\\] shall also be set valid"]
    #[inline(always)]
    pub fn bit_txenc_uni_mgnt(&self) -> BitTxencUniMgntR {
        BitTxencUniMgntR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - decryption enable of rx unicast management pkt, 0x680\\[3\\] shall also be set valid"]
    #[inline(always)]
    pub fn bit_rxdec_uni_mgnt(&self) -> BitRxdecUniMgntR {
        BitRxdecUniMgntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - encryption enable of tx multicast management pkt, 0x680\\[2\\] shall also be set valid"]
    #[inline(always)]
    pub fn bit_txenc_bm_mgnt(&self) -> BitTxencBmMgntR {
        BitTxencBmMgntR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - decryption enable of rx multicast management pkt, 0x680\\[3\\] shall also be set valid"]
    #[inline(always)]
    pub fn bit_rxdec_bm_mgnt(&self) -> BitRxdecBmMgntR {
        BitRxdecBmMgntR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - disenable check fc\\[14\\] when bip decryption"]
    #[inline(always)]
    pub fn bit_rxdec_bm_dis_chkfc(&self) -> BitRxdecBmDisChkfcR {
        BitRxdecBmDisChkfcR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - US timer counter boundary of BTCOEXIST"]
    #[inline(always)]
    pub fn bit_bt_time_cnt(&mut self) -> BitBtTimeCntW<'_, RegWsecOptionSpec> {
        BitBtTimeCntW::new(self, 0)
    }
    #[doc = "Bit 13 - If set to 1, enable old GCM-WAPI ADD format, don't mask FC bit4~6, bit8~15, If set to 0, enable new GCM-WAPI ADD format, mask FC bit15 to 0."]
    #[inline(always)]
    pub fn bit_gcm_wapi_fc_nomask(&mut self) -> BitGcmWapiFcNomaskW<'_, RegWsecOptionSpec> {
        BitGcmWapiFcNomaskW::new(self, 13)
    }
    #[doc = "Bit 14 - If set to 1, enable old WAPI ADD format, don't mask QC bit4~6, bit8~15, If set to 0, enable new WAPI ADD format, mask QC bit4~6, bit8~15 to 0."]
    #[inline(always)]
    pub fn bit_wapi_qc_nomask(&mut self) -> BitWapiQcNomaskW<'_, RegWsecOptionSpec> {
        BitWapiQcNomaskW::new(self, 14)
    }
    #[doc = "Bit 15 - If set to 1, enable old WAPI ADD format, don't mask FC bit4~6, bit8~15, If set to 0, enable new WAPI ADD format, mask FC bit15 to 0."]
    #[inline(always)]
    pub fn bit_wapi_fc_nomask(&mut self) -> BitWapiFcNomaskW<'_, RegWsecOptionSpec> {
        BitWapiFcNomaskW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Security GCMP header PN selection"]
    #[inline(always)]
    pub fn bit_wmac_sec_pn_sel(&mut self) -> BitWmacSecPnSelW<'_, RegWsecOptionSpec> {
        BitWmacSecPnSelW::new(self, 16)
    }
    #[doc = "Bit 18 - MASK IV\\[15:8\\] for GCMP security"]
    #[inline(always)]
    pub fn bit_wmac_sec_maskiv(&mut self) -> BitWmacSecMaskivW<'_, RegWsecOptionSpec> {
        BitWmacSecMaskivW::new(self, 18)
    }
    #[doc = "Bit 19 - encryption enable of tx unicast management pkt, 0x680\\[2\\] shall also be set valid"]
    #[inline(always)]
    pub fn bit_txenc_uni_mgnt(&mut self) -> BitTxencUniMgntW<'_, RegWsecOptionSpec> {
        BitTxencUniMgntW::new(self, 19)
    }
    #[doc = "Bit 20 - decryption enable of rx unicast management pkt, 0x680\\[3\\] shall also be set valid"]
    #[inline(always)]
    pub fn bit_rxdec_uni_mgnt(&mut self) -> BitRxdecUniMgntW<'_, RegWsecOptionSpec> {
        BitRxdecUniMgntW::new(self, 20)
    }
    #[doc = "Bit 21 - encryption enable of tx multicast management pkt, 0x680\\[2\\] shall also be set valid"]
    #[inline(always)]
    pub fn bit_txenc_bm_mgnt(&mut self) -> BitTxencBmMgntW<'_, RegWsecOptionSpec> {
        BitTxencBmMgntW::new(self, 21)
    }
    #[doc = "Bit 22 - decryption enable of rx multicast management pkt, 0x680\\[3\\] shall also be set valid"]
    #[inline(always)]
    pub fn bit_rxdec_bm_mgnt(&mut self) -> BitRxdecBmMgntW<'_, RegWsecOptionSpec> {
        BitRxdecBmMgntW::new(self, 22)
    }
    #[doc = "Bit 23 - disenable check fc\\[14\\] when bip decryption"]
    #[inline(always)]
    pub fn bit_rxdec_bm_dis_chkfc(&mut self) -> BitRxdecBmDisChkfcW<'_, RegWsecOptionSpec> {
        BitRxdecBmDisChkfcW::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wsec_option::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wsec_option::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWsecOptionSpec;
impl crate::RegisterSpec for RegWsecOptionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wsec_option::R`](R) reader structure"]
impl crate::Readable for RegWsecOptionSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wsec_option::W`](W) writer structure"]
impl crate::Writable for RegWsecOptionSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WSEC_OPTION to value 0x0078_0014"]
impl crate::Resettable for RegWsecOptionSpec {
    const RESET_VALUE: u32 = 0x0078_0014;
}
