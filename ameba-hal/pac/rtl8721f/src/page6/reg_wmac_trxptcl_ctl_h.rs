#[doc = "Register `REG_WMAC_TRXPTCL_CTL_H` reader"]
pub type R = crate::R<RegWmacTrxptclCtlHSpec>;
#[doc = "Register `REG_WMAC_TRXPTCL_CTL_H` writer"]
pub type W = crate::W<RegWmacTrxptclCtlHSpec>;
#[doc = "Field `BIT_EN_TXCTS_INTXOP_V1` reader - Enable to send CTS for responding RX RTS during our TXOP period"]
pub type BitEnTxctsIntxopV1R = crate::BitReader;
#[doc = "Field `BIT_EN_TXCTS_INTXOP_V1` writer - Enable to send CTS for responding RX RTS during our TXOP period"]
pub type BitEnTxctsIntxopV1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_TXCTS_INRXNAV_V1` reader - Enable to send CTS in Rx NAV"]
pub type BitEnTxctsInrxnavV1R = crate::BitReader;
#[doc = "Field `BIT_EN_TXCTS_INRXNAV_V1` writer - Enable to send CTS in Rx NAV"]
pub type BitEnTxctsInrxnavV1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_TXACKBA_INTXOP_V1` reader - If this bit is set, then MAC will Tx ACK/BA to respond previous rx my unitcast packets send by another STA during my TXOP"]
pub type BitEnTxackbaIntxopV1R = crate::BitReader;
#[doc = "Field `BIT_EN_TXACKBA_INTXOP_V1` writer - If this bit is set, then MAC will Tx ACK/BA to respond previous rx my unitcast packets send by another STA during my TXOP"]
pub type BitEnTxackbaIntxopV1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_TXACKBA_INTX_RDG_V1` reader - If this bit is set, then MAC will Tx ACK/BA to respond previous rx my unicast packets send by another STA during TX RDG"]
pub type BitEnTxackbaIntxRdgV1R = crate::BitReader;
#[doc = "Field `BIT_EN_TXACKBA_INTX_RDG_V1` writer - If this bit is set, then MAC will Tx ACK/BA to respond previous rx my unicast packets send by another STA during TX RDG"]
pub type BitEnTxackbaIntxRdgV1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_TXCTS_INFULL_V1` reader - Disable to send CTS for responding RX control wrapper RTS dropped by rxpktbuffer full"]
pub type BitDisTxctsInfullV1R = crate::BitReader;
#[doc = "Field `BIT_DIS_TXCTS_INFULL_V1` writer - Disable to send CTS for responding RX control wrapper RTS dropped by rxpktbuffer full"]
pub type BitDisTxctsInfullV1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_TXCFE_INFULL_V1` reader - Disable to send CF-END for responding RX control wrapper CF-END dropped by rxpktbuffer full. This bit is valid when RESP_DCFE_EN is set to 1"]
pub type BitDisTxcfeInfullV1R = crate::BitReader;
#[doc = "Field `BIT_DIS_TXCFE_INFULL_V1` writer - Disable to send CF-END for responding RX control wrapper CF-END dropped by rxpktbuffer full. This bit is valid when RESP_DCFE_EN is set to 1"]
pub type BitDisTxcfeInfullV1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_TXBA_RXBARINFULL_V1` reader - Disable to send BA for responding RX control wrapper BAR dropped by rxpkbuffer full"]
pub type BitDisTxbaRxbarinfullV1R = crate::BitReader;
#[doc = "Field `BIT_DIS_TXBA_RXBARINFULL_V1` writer - Disable to send BA for responding RX control wrapper BAR dropped by rxpkbuffer full"]
pub type BitDisTxbaRxbarinfullV1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_TXBA_AMPDUFCSERR_V1` reader - Disable to send BA for responding RX AMPDU with FCS error subframe"]
pub type BitDisTxbaAmpdufcserrV1R = crate::BitReader;
#[doc = "Field `BIT_DIS_TXBA_AMPDUFCSERR_V1` writer - Disable to send BA for responding RX AMPDU with FCS error subframe"]
pub type BitDisTxbaAmpdufcserrV1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_TXCTS_TO_TXOPOWNER_INRXNAV_V1` reader - together with bit41, implement 802.11ac txop owner function"]
pub type BitEnTxctsToTxopownerInrxnavV1R = crate::BitReader;
#[doc = "Field `BIT_EN_TXCTS_TO_TXOPOWNER_INRXNAV_V1` writer - together with bit41, implement 802.11ac txop owner function"]
pub type BitEnTxctsToTxopownerInrxnavV1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_SAVE_ALL_TXOPADDR_V1` reader - "]
pub type BitEnSaveAllTxopaddrV1R = crate::BitReader;
#[doc = "Field `BIT_EN_SAVE_ALL_TXOPADDR_V1` writer - "]
pub type BitEnSaveAllTxopaddrV1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXBA_IGNOREA2_V1` reader - Disable check A2 field of RX BA"]
pub type BitRxbaIgnorea2V1R = crate::BitReader;
#[doc = "Field `BIT_RXBA_IGNOREA2_V1` writer - Disable check A2 field of RX BA"]
pub type BitRxbaIgnorea2V1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_ACK_NON_EOF` reader - response to trigger+mgt with eof=0"]
pub type BitRespAckNonEofR = crate::BitReader;
#[doc = "Field `BIT_RESP_ACK_NON_EOF` writer - response to trigger+mgt with eof=0"]
pub type BitRespAckNonEofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ACKBAR_ACKPCHK` reader - If set 1, then check ACK Policy of BAR control field to determine whether send response or not. When this bit set to 1 and a. ACK Policy of Rx BAR control field is 0, then response type is controlled by ACLBAR_TYPESEL. b. ACK Policy of Rx BAR control field is 1, then don't send response If set 0, then ignore the ACK policy of RX BAR control field and response type controlled by ACKBA_TYPESEL ACKBA_ACKPCHK\\[0\\]: used for Basic BAR ACKBA_ACKPCHK\\[1\\]: used for Compression BAR ACKBA_ACKPCHK\\[2\\]: RSVD ACKBA_ACKPCHK\\[3\\]: used for MTID BAR"]
pub type BitAckbarAckpchkR = crate::FieldReader;
#[doc = "Field `BIT_ACKBAR_ACKPCHK` writer - If set 1, then check ACK Policy of BAR control field to determine whether send response or not. When this bit set to 1 and a. ACK Policy of Rx BAR control field is 0, then response type is controlled by ACLBAR_TYPESEL. b. ACK Policy of Rx BAR control field is 1, then don't send response If set 0, then ignore the ACK policy of RX BAR control field and response type controlled by ACKBA_TYPESEL ACKBA_ACKPCHK\\[0\\]: used for Basic BAR ACKBA_ACKPCHK\\[1\\]: used for Compression BAR ACKBA_ACKPCHK\\[2\\]: RSVD ACKBA_ACKPCHK\\[3\\]: used for MTID BAR"]
pub type BitAckbarAckpchkW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_ACKBAR_TYPESEL` reader - Determine response type for RX BAR, and the definition as the bellow 00: NAV 01: Send BA 10: Send ACK ACKBAR_TYPESEL\\[1:0\\]: used for Basic BAR ACKBAR_TYPESEL\\[3:2\\]: used for Compression BAR ACKBAR_TYPESEL\\[5:4\\]: RSVD ACKBAR_TYPESEL\\[7:6\\]: used for MTID BAR"]
pub type BitAckbarTypeselR = crate::FieldReader;
#[doc = "Field `BIT_ACKBAR_TYPESEL` writer - Determine response type for RX BAR, and the definition as the bellow 00: NAV 01: Send BA 10: Send ACK ACKBAR_TYPESEL\\[1:0\\]: used for Basic BAR ACKBAR_TYPESEL\\[3:2\\]: used for Compression BAR ACKBAR_TYPESEL\\[5:4\\]: RSVD ACKBAR_TYPESEL\\[7:6\\]: used for MTID BAR"]
pub type BitAckbarTypeselW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_ACKBA_ACKPCHK` reader - If set 1, then check ACK Policy of BA control field to determine whether send response or not. When this bit set to 1 and a. ACK Policy of Rx BA control field is 0, then response type is controlled by ACLBA_TYPESEL. b. ACK Policy of Rx BA control field is 1, then don't send response If set 0, then ignore the ACK policy of RX BA control field and response type controlled by ACKBA_TYPESEL ACKBA_ACKPCHK\\[0\\]: used for Basic BA ACKBA_ACKPCHK\\[1\\]: used for Compression BA ACKBA_ACKPCHK\\[2\\]: RSVD ACKBA_ACKPCHK\\[3\\]: used for MTID BA"]
pub type BitAckbaAckpchkR = crate::FieldReader;
#[doc = "Field `BIT_ACKBA_ACKPCHK` writer - If set 1, then check ACK Policy of BA control field to determine whether send response or not. When this bit set to 1 and a. ACK Policy of Rx BA control field is 0, then response type is controlled by ACLBA_TYPESEL. b. ACK Policy of Rx BA control field is 1, then don't send response If set 0, then ignore the ACK policy of RX BA control field and response type controlled by ACKBA_TYPESEL ACKBA_ACKPCHK\\[0\\]: used for Basic BA ACKBA_ACKPCHK\\[1\\]: used for Compression BA ACKBA_ACKPCHK\\[2\\]: RSVD ACKBA_ACKPCHK\\[3\\]: used for MTID BA"]
pub type BitAckbaAckpchkW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_ACKBA_TYPSEL` reader - Determine response type for RX BA, and the definition as the bellow 0: NAV 1: Send ACK ACKBA_TYPESEL\\[0\\]: used for Basic BA ACKBA_TYPESEL\\[1\\]: used for Compression BA ACKBA_TYPESEL\\[2\\]: RSVD ACKBA_TYPESEL\\[3\\]: used for MTID BA"]
pub type BitAckbaTypselR = crate::FieldReader;
#[doc = "Field `BIT_ACKBA_TYPSEL` writer - Determine response type for RX BA, and the definition as the bellow 0: NAV 1: Send ACK ACKBA_TYPESEL\\[0\\]: used for Basic BA ACKBA_TYPESEL\\[1\\]: used for Compression BA ACKBA_TYPESEL\\[2\\]: RSVD ACKBA_TYPESEL\\[3\\]: used for MTID BA"]
pub type BitAckbaTypselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Enable to send CTS for responding RX RTS during our TXOP period"]
    #[inline(always)]
    pub fn bit_en_txcts_intxop_v1(&self) -> BitEnTxctsIntxopV1R {
        BitEnTxctsIntxopV1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable to send CTS in Rx NAV"]
    #[inline(always)]
    pub fn bit_en_txcts_inrxnav_v1(&self) -> BitEnTxctsInrxnavV1R {
        BitEnTxctsInrxnavV1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If this bit is set, then MAC will Tx ACK/BA to respond previous rx my unitcast packets send by another STA during my TXOP"]
    #[inline(always)]
    pub fn bit_en_txackba_intxop_v1(&self) -> BitEnTxackbaIntxopV1R {
        BitEnTxackbaIntxopV1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If this bit is set, then MAC will Tx ACK/BA to respond previous rx my unicast packets send by another STA during TX RDG"]
    #[inline(always)]
    pub fn bit_en_txackba_intx_rdg_v1(&self) -> BitEnTxackbaIntxRdgV1R {
        BitEnTxackbaIntxRdgV1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Disable to send CTS for responding RX control wrapper RTS dropped by rxpktbuffer full"]
    #[inline(always)]
    pub fn bit_dis_txcts_infull_v1(&self) -> BitDisTxctsInfullV1R {
        BitDisTxctsInfullV1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable to send CF-END for responding RX control wrapper CF-END dropped by rxpktbuffer full. This bit is valid when RESP_DCFE_EN is set to 1"]
    #[inline(always)]
    pub fn bit_dis_txcfe_infull_v1(&self) -> BitDisTxcfeInfullV1R {
        BitDisTxcfeInfullV1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable to send BA for responding RX control wrapper BAR dropped by rxpkbuffer full"]
    #[inline(always)]
    pub fn bit_dis_txba_rxbarinfull_v1(&self) -> BitDisTxbaRxbarinfullV1R {
        BitDisTxbaRxbarinfullV1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Disable to send BA for responding RX AMPDU with FCS error subframe"]
    #[inline(always)]
    pub fn bit_dis_txba_ampdufcserr_v1(&self) -> BitDisTxbaAmpdufcserrV1R {
        BitDisTxbaAmpdufcserrV1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - together with bit41, implement 802.11ac txop owner function"]
    #[inline(always)]
    pub fn bit_en_txcts_to_txopowner_inrxnav_v1(&self) -> BitEnTxctsToTxopownerInrxnavV1R {
        BitEnTxctsToTxopownerInrxnavV1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bit_en_save_all_txopaddr_v1(&self) -> BitEnSaveAllTxopaddrV1R {
        BitEnSaveAllTxopaddrV1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Disable check A2 field of RX BA"]
    #[inline(always)]
    pub fn bit_rxba_ignorea2_v1(&self) -> BitRxbaIgnorea2V1R {
        BitRxbaIgnorea2V1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - response to trigger+mgt with eof=0"]
    #[inline(always)]
    pub fn bit_resp_ack_non_eof(&self) -> BitRespAckNonEofR {
        BitRespAckNonEofR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - If set 1, then check ACK Policy of BAR control field to determine whether send response or not. When this bit set to 1 and a. ACK Policy of Rx BAR control field is 0, then response type is controlled by ACLBAR_TYPESEL. b. ACK Policy of Rx BAR control field is 1, then don't send response If set 0, then ignore the ACK policy of RX BAR control field and response type controlled by ACKBA_TYPESEL ACKBA_ACKPCHK\\[0\\]: used for Basic BAR ACKBA_ACKPCHK\\[1\\]: used for Compression BAR ACKBA_ACKPCHK\\[2\\]: RSVD ACKBA_ACKPCHK\\[3\\]: used for MTID BAR"]
    #[inline(always)]
    pub fn bit_ackbar_ackpchk(&self) -> BitAckbarAckpchkR {
        BitAckbarAckpchkR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Determine response type for RX BAR, and the definition as the bellow 00: NAV 01: Send BA 10: Send ACK ACKBAR_TYPESEL\\[1:0\\]: used for Basic BAR ACKBAR_TYPESEL\\[3:2\\]: used for Compression BAR ACKBAR_TYPESEL\\[5:4\\]: RSVD ACKBAR_TYPESEL\\[7:6\\]: used for MTID BAR"]
    #[inline(always)]
    pub fn bit_ackbar_typesel(&self) -> BitAckbarTypeselR {
        BitAckbarTypeselR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - If set 1, then check ACK Policy of BA control field to determine whether send response or not. When this bit set to 1 and a. ACK Policy of Rx BA control field is 0, then response type is controlled by ACLBA_TYPESEL. b. ACK Policy of Rx BA control field is 1, then don't send response If set 0, then ignore the ACK policy of RX BA control field and response type controlled by ACKBA_TYPESEL ACKBA_ACKPCHK\\[0\\]: used for Basic BA ACKBA_ACKPCHK\\[1\\]: used for Compression BA ACKBA_ACKPCHK\\[2\\]: RSVD ACKBA_ACKPCHK\\[3\\]: used for MTID BA"]
    #[inline(always)]
    pub fn bit_ackba_ackpchk(&self) -> BitAckbaAckpchkR {
        BitAckbaAckpchkR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Determine response type for RX BA, and the definition as the bellow 0: NAV 1: Send ACK ACKBA_TYPESEL\\[0\\]: used for Basic BA ACKBA_TYPESEL\\[1\\]: used for Compression BA ACKBA_TYPESEL\\[2\\]: RSVD ACKBA_TYPESEL\\[3\\]: used for MTID BA"]
    #[inline(always)]
    pub fn bit_ackba_typsel(&self) -> BitAckbaTypselR {
        BitAckbaTypselR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable to send CTS for responding RX RTS during our TXOP period"]
    #[inline(always)]
    pub fn bit_en_txcts_intxop_v1(&mut self) -> BitEnTxctsIntxopV1W<'_, RegWmacTrxptclCtlHSpec> {
        BitEnTxctsIntxopV1W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable to send CTS in Rx NAV"]
    #[inline(always)]
    pub fn bit_en_txcts_inrxnav_v1(&mut self) -> BitEnTxctsInrxnavV1W<'_, RegWmacTrxptclCtlHSpec> {
        BitEnTxctsInrxnavV1W::new(self, 1)
    }
    #[doc = "Bit 2 - If this bit is set, then MAC will Tx ACK/BA to respond previous rx my unitcast packets send by another STA during my TXOP"]
    #[inline(always)]
    pub fn bit_en_txackba_intxop_v1(
        &mut self,
    ) -> BitEnTxackbaIntxopV1W<'_, RegWmacTrxptclCtlHSpec> {
        BitEnTxackbaIntxopV1W::new(self, 2)
    }
    #[doc = "Bit 3 - If this bit is set, then MAC will Tx ACK/BA to respond previous rx my unicast packets send by another STA during TX RDG"]
    #[inline(always)]
    pub fn bit_en_txackba_intx_rdg_v1(
        &mut self,
    ) -> BitEnTxackbaIntxRdgV1W<'_, RegWmacTrxptclCtlHSpec> {
        BitEnTxackbaIntxRdgV1W::new(self, 3)
    }
    #[doc = "Bit 4 - Disable to send CTS for responding RX control wrapper RTS dropped by rxpktbuffer full"]
    #[inline(always)]
    pub fn bit_dis_txcts_infull_v1(&mut self) -> BitDisTxctsInfullV1W<'_, RegWmacTrxptclCtlHSpec> {
        BitDisTxctsInfullV1W::new(self, 4)
    }
    #[doc = "Bit 5 - Disable to send CF-END for responding RX control wrapper CF-END dropped by rxpktbuffer full. This bit is valid when RESP_DCFE_EN is set to 1"]
    #[inline(always)]
    pub fn bit_dis_txcfe_infull_v1(&mut self) -> BitDisTxcfeInfullV1W<'_, RegWmacTrxptclCtlHSpec> {
        BitDisTxcfeInfullV1W::new(self, 5)
    }
    #[doc = "Bit 6 - Disable to send BA for responding RX control wrapper BAR dropped by rxpkbuffer full"]
    #[inline(always)]
    pub fn bit_dis_txba_rxbarinfull_v1(
        &mut self,
    ) -> BitDisTxbaRxbarinfullV1W<'_, RegWmacTrxptclCtlHSpec> {
        BitDisTxbaRxbarinfullV1W::new(self, 6)
    }
    #[doc = "Bit 7 - Disable to send BA for responding RX AMPDU with FCS error subframe"]
    #[inline(always)]
    pub fn bit_dis_txba_ampdufcserr_v1(
        &mut self,
    ) -> BitDisTxbaAmpdufcserrV1W<'_, RegWmacTrxptclCtlHSpec> {
        BitDisTxbaAmpdufcserrV1W::new(self, 7)
    }
    #[doc = "Bit 8 - together with bit41, implement 802.11ac txop owner function"]
    #[inline(always)]
    pub fn bit_en_txcts_to_txopowner_inrxnav_v1(
        &mut self,
    ) -> BitEnTxctsToTxopownerInrxnavV1W<'_, RegWmacTrxptclCtlHSpec> {
        BitEnTxctsToTxopownerInrxnavV1W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bit_en_save_all_txopaddr_v1(
        &mut self,
    ) -> BitEnSaveAllTxopaddrV1W<'_, RegWmacTrxptclCtlHSpec> {
        BitEnSaveAllTxopaddrV1W::new(self, 9)
    }
    #[doc = "Bit 10 - Disable check A2 field of RX BA"]
    #[inline(always)]
    pub fn bit_rxba_ignorea2_v1(&mut self) -> BitRxbaIgnorea2V1W<'_, RegWmacTrxptclCtlHSpec> {
        BitRxbaIgnorea2V1W::new(self, 10)
    }
    #[doc = "Bit 11 - response to trigger+mgt with eof=0"]
    #[inline(always)]
    pub fn bit_resp_ack_non_eof(&mut self) -> BitRespAckNonEofW<'_, RegWmacTrxptclCtlHSpec> {
        BitRespAckNonEofW::new(self, 11)
    }
    #[doc = "Bits 12:15 - If set 1, then check ACK Policy of BAR control field to determine whether send response or not. When this bit set to 1 and a. ACK Policy of Rx BAR control field is 0, then response type is controlled by ACLBAR_TYPESEL. b. ACK Policy of Rx BAR control field is 1, then don't send response If set 0, then ignore the ACK policy of RX BAR control field and response type controlled by ACKBA_TYPESEL ACKBA_ACKPCHK\\[0\\]: used for Basic BAR ACKBA_ACKPCHK\\[1\\]: used for Compression BAR ACKBA_ACKPCHK\\[2\\]: RSVD ACKBA_ACKPCHK\\[3\\]: used for MTID BAR"]
    #[inline(always)]
    pub fn bit_ackbar_ackpchk(&mut self) -> BitAckbarAckpchkW<'_, RegWmacTrxptclCtlHSpec> {
        BitAckbarAckpchkW::new(self, 12)
    }
    #[doc = "Bits 16:23 - Determine response type for RX BAR, and the definition as the bellow 00: NAV 01: Send BA 10: Send ACK ACKBAR_TYPESEL\\[1:0\\]: used for Basic BAR ACKBAR_TYPESEL\\[3:2\\]: used for Compression BAR ACKBAR_TYPESEL\\[5:4\\]: RSVD ACKBAR_TYPESEL\\[7:6\\]: used for MTID BAR"]
    #[inline(always)]
    pub fn bit_ackbar_typesel(&mut self) -> BitAckbarTypeselW<'_, RegWmacTrxptclCtlHSpec> {
        BitAckbarTypeselW::new(self, 16)
    }
    #[doc = "Bits 24:27 - If set 1, then check ACK Policy of BA control field to determine whether send response or not. When this bit set to 1 and a. ACK Policy of Rx BA control field is 0, then response type is controlled by ACLBA_TYPESEL. b. ACK Policy of Rx BA control field is 1, then don't send response If set 0, then ignore the ACK policy of RX BA control field and response type controlled by ACKBA_TYPESEL ACKBA_ACKPCHK\\[0\\]: used for Basic BA ACKBA_ACKPCHK\\[1\\]: used for Compression BA ACKBA_ACKPCHK\\[2\\]: RSVD ACKBA_ACKPCHK\\[3\\]: used for MTID BA"]
    #[inline(always)]
    pub fn bit_ackba_ackpchk(&mut self) -> BitAckbaAckpchkW<'_, RegWmacTrxptclCtlHSpec> {
        BitAckbaAckpchkW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Determine response type for RX BA, and the definition as the bellow 0: NAV 1: Send ACK ACKBA_TYPESEL\\[0\\]: used for Basic BA ACKBA_TYPESEL\\[1\\]: used for Compression BA ACKBA_TYPESEL\\[2\\]: RSVD ACKBA_TYPESEL\\[3\\]: used for MTID BA"]
    #[inline(always)]
    pub fn bit_ackba_typsel(&mut self) -> BitAckbaTypselW<'_, RegWmacTrxptclCtlHSpec> {
        BitAckbaTypselW::new(self, 28)
    }
}
#[doc = "WMAC TX/RX Protocol Control Register Higher Bit\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wmac_trxptcl_ctl_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wmac_trxptcl_ctl_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWmacTrxptclCtlHSpec;
impl crate::RegisterSpec for RegWmacTrxptclCtlHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wmac_trxptcl_ctl_h::R`](R) reader structure"]
impl crate::Readable for RegWmacTrxptclCtlHSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wmac_trxptcl_ctl_h::W`](W) writer structure"]
impl crate::Writable for RegWmacTrxptclCtlHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WMAC_TRXPTCL_CTL_H to value 0x0004_2007"]
impl crate::Resettable for RegWmacTrxptclCtlHSpec {
    const RESET_VALUE: u32 = 0x0004_2007;
}
