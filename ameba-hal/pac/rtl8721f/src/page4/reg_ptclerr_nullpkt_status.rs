#[doc = "Register `REG_PTCLERR_NULLPKT_STATUS` reader"]
pub type R = crate::R<RegPtclerrNullpktStatusSpec>;
#[doc = "Field `BIT_P0_TX_NULL_0` reader - Port 0 Tx Null 0 packet ok indicator"]
pub type BitP0TxNull0R = crate::BitReader;
#[doc = "Field `BIT_P0_TX_NULL_1` reader - Port 0 Tx Null 1 packet ok indicator"]
pub type BitP0TxNull1R = crate::BitReader;
#[doc = "Field `BIT_P1_TX_NULL_0` reader - Port 1 Tx Null 0 packet ok indicator"]
pub type BitP1TxNull0R = crate::BitReader;
#[doc = "Field `BIT_P1_TX_NULL_1` reader - Port 1 Tx Null 1 packet ok indicator"]
pub type BitP1TxNull1R = crate::BitReader;
#[doc = "Field `BIT_CLR_UPT_TAIL_CNT` reader - write 1 to clear BIT_RD_UPT_TAIL_CNT, pulse"]
pub type BitClrUptTailCntR = crate::BitReader;
#[doc = "Field `BIT_RD_UPT_TAIL_CNT` reader - SW update free_tail page counter"]
pub type BitRdUptTailCntR = crate::FieldReader;
#[doc = "Field `BIT_PTCL_RATE_TABLE_INVALID` reader - during rate fallback process, if the all rate table is invalid, then assert ERROR FLAG"]
pub type BitPtclRateTableInvalidR = crate::BitReader;
#[doc = "Field `BIT_RD_PKTIN_ERR` reader - Indicates the pktin error in RDG mode"]
pub type BitRdPktinErrR = crate::FieldReader;
#[doc = "Field `BIT_RD_CTN_ERR` reader - Indicates the cnt error in RDG mode"]
pub type BitRdCtnErrR = crate::BitReader;
#[doc = "Field `BIT_RD_STATUS_ERR` reader - Indicates the status error in RDG mode"]
pub type BitRdStatusErrR = crate::BitReader;
#[doc = "Field `BIT_RD_TXHANG_ERR` reader - Indicates the txhang error in RDG mode"]
pub type BitRdTxhangErrR = crate::BitReader;
#[doc = "Field `BIT_RD_FTMT2R_ERR` reader - Indicates the timeout error in FTM t2r report"]
pub type BitRdFtmt2rErrR = crate::BitReader;
#[doc = "Field `BIT_PTCL_TOTAL_PG_8` reader - Total Page in PTCL"]
pub type BitPtclTotalPg8R = crate::BitReader;
#[doc = "Field `BIT_PTCL_TOTAL_PG0_7` reader - Total Page in PTCL"]
pub type BitPtclTotalPg0_7R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Port 0 Tx Null 0 packet ok indicator"]
    #[inline(always)]
    pub fn bit_p0_tx_null_0(&self) -> BitP0TxNull0R {
        BitP0TxNull0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port 0 Tx Null 1 packet ok indicator"]
    #[inline(always)]
    pub fn bit_p0_tx_null_1(&self) -> BitP0TxNull1R {
        BitP0TxNull1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port 1 Tx Null 0 packet ok indicator"]
    #[inline(always)]
    pub fn bit_p1_tx_null_0(&self) -> BitP1TxNull0R {
        BitP1TxNull0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port 1 Tx Null 1 packet ok indicator"]
    #[inline(always)]
    pub fn bit_p1_tx_null_1(&self) -> BitP1TxNull1R {
        BitP1TxNull1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 11 - write 1 to clear BIT_RD_UPT_TAIL_CNT, pulse"]
    #[inline(always)]
    pub fn bit_clr_upt_tail_cnt(&self) -> BitClrUptTailCntR {
        BitClrUptTailCntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SW update free_tail page counter"]
    #[inline(always)]
    pub fn bit_rd_upt_tail_cnt(&self) -> BitRdUptTailCntR {
        BitRdUptTailCntR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - during rate fallback process, if the all rate table is invalid, then assert ERROR FLAG"]
    #[inline(always)]
    pub fn bit_ptcl_rate_table_invalid(&self) -> BitPtclRateTableInvalidR {
        BitPtclRateTableInvalidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Indicates the pktin error in RDG mode"]
    #[inline(always)]
    pub fn bit_rd_pktin_err(&self) -> BitRdPktinErrR {
        BitRdPktinErrR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Indicates the cnt error in RDG mode"]
    #[inline(always)]
    pub fn bit_rd_ctn_err(&self) -> BitRdCtnErrR {
        BitRdCtnErrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Indicates the status error in RDG mode"]
    #[inline(always)]
    pub fn bit_rd_status_err(&self) -> BitRdStatusErrR {
        BitRdStatusErrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Indicates the txhang error in RDG mode"]
    #[inline(always)]
    pub fn bit_rd_txhang_err(&self) -> BitRdTxhangErrR {
        BitRdTxhangErrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Indicates the timeout error in FTM t2r report"]
    #[inline(always)]
    pub fn bit_rd_ftmt2r_err(&self) -> BitRdFtmt2rErrR {
        BitRdFtmt2rErrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Total Page in PTCL"]
    #[inline(always)]
    pub fn bit_ptcl_total_pg_8(&self) -> BitPtclTotalPg8R {
        BitPtclTotalPg8R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Total Page in PTCL"]
    #[inline(always)]
    pub fn bit_ptcl_total_pg0_7(&self) -> BitPtclTotalPg0_7R {
        BitPtclTotalPg0_7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ptclerr_nullpkt_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPtclerrNullpktStatusSpec;
impl crate::RegisterSpec for RegPtclerrNullpktStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ptclerr_nullpkt_status::R`](R) reader structure"]
impl crate::Readable for RegPtclerrNullpktStatusSpec {}
#[doc = "`reset()` method sets REG_PTCLERR_NULLPKT_STATUS to value 0"]
impl crate::Resettable for RegPtclerrNullpktStatusSpec {}
