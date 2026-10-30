#[doc = "Register `REG_FWISR` reader"]
pub type R = crate::R<RegFwisrSpec>;
#[doc = "Field `BIT_FS_PTCL_HANDLE_OK_INT` reader - TX Finish (Ack/BA process Finish) Interrupt."]
pub type BitFsPtclHandleOkIntR = crate::BitReader;
#[doc = "Field `BIT_FS_TXCCX_INT` reader - CCX PKT TX Report Interrupt"]
pub type BitFsTxccxIntR = crate::BitReader;
#[doc = "Field `BIT_FS_ERRORHDL_INT` reader - FWHW/ TXDMA/ RXDMA/ WMAC error status interrupt(0x210\\[31:0\\], 0x288\\[31:0\\], 0x4E2\\[15:0\\], 0x6B8\\[31:0\\]). WMAC error state will be recovered by HW, the others won't."]
pub type BitFsErrorhdlIntR = crate::BitReader;
#[doc = "Field `BIT_FS_TX_CLOSE_INT` reader - TX DONE INTERRUPT"]
pub type BitFsTxCloseIntR = crate::BitReader;
#[doc = "Field `BIT_FS_H2CCMD_INT` reader - Host To CPU Message Interrupt"]
pub type BitFsH2ccmdIntR = crate::BitReader;
#[doc = "Field `BIT_FS_HRCV_INT` reader - Host to CPU Message Interrupt, generated on writing to 0x1CF"]
pub type BitFsHrcvIntR = crate::BitReader;
#[doc = "Field `BIT_FS_C2H_W_READY_INT` reader - C2H register ready for write interrupt"]
pub type BitFsC2hWReadyIntR = crate::BitReader;
#[doc = "Field `BIT_FS_TXRPT_INT` reader - TX report interrupt"]
pub type BitFsTxrptIntR = crate::BitReader;
#[doc = "Field `RSVD` reader - rsvd"]
pub type RsvdR = crate::BitReader;
#[doc = "Field `BIT_FS_CPUMGQ_ERR_INT` reader - CPU management queue polled packet not TX OK while new packet comes in, enable to issue interrupt or not"]
pub type BitFsCpumgqErrIntR = crate::BitReader;
#[doc = "Field `BIT_FS_MGNTQ_FF_RELEASE_INT` reader - MgntQ ReadPtr FIFO release interrupt (for 8822B)"]
pub type BitFsMgntqFfReleaseIntR = crate::BitReader;
#[doc = "Field `BIT_FS_CPUMGN_POLLED_PKT_DONE_INT` reader - CPU management queue polled packet released done interrupt enable (check cpu management queue current flow)"]
pub type BitFsCpumgnPolledPktDoneIntR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNERR_MB0_INT` reader - MBSSID0 TX beacon Error interrupt"]
pub type BitFsTxbcnerrMb0IntR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNERR_MB1_INT` reader - MBSSID1 TX beacon Error interrupt"]
pub type BitFsTxbcnerrMb1IntR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNERR_MB2_INT` reader - MBSSID2 TX beacon Error interrupt"]
pub type BitFsTxbcnerrMb2IntR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNERR_MB3_INT` reader - MBSSID3 TX beacon Error interrupt"]
pub type BitFsTxbcnerrMb3IntR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNERR_MB4_INT` reader - MBSSID4 TX beacon Error interrupt"]
pub type BitFsTxbcnerrMb4IntR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNERR_MB5_INT` reader - MBSSID5 TX beacon Error interrupt"]
pub type BitFsTxbcnerrMb5IntR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNERR_MB6_INT` reader - MBSSID6 TX beacon Error interrupt"]
pub type BitFsTxbcnerrMb6IntR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNERR_MB7_INT` reader - MBSSID7 TX beacon Error interrupt"]
pub type BitFsTxbcnerrMb7IntR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNOK_MB0_INT` reader - MBSSID0 TX beacon OK interrupt"]
pub type BitFsTxbcnokMb0IntR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNOK_MB1_INT` reader - MBSSID1 TX beacon OK interrupt"]
pub type BitFsTxbcnokMb1IntR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNOK_MB2_INT` reader - MBSSID2 TX beacon OK interrupt"]
pub type BitFsTxbcnokMb2IntR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNOK_MB3_INT` reader - MBSSID3 TX beacon OK interrupt"]
pub type BitFsTxbcnokMb3IntR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNOK_MB4_INT` reader - MBSSID4 TX beacon OK interrupt"]
pub type BitFsTxbcnokMb4IntR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNOK_MB5_INT` reader - MBSSID5 TX beacon OK interrupt"]
pub type BitFsTxbcnokMb5IntR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNOK_MB6_INT` reader - MBSSID6 TX beacon OK interrupt"]
pub type BitFsTxbcnokMb6IntR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNOK_MB7_INT` reader - MBSSID7 TX beacon OK interrupt"]
pub type BitFsTxbcnokMb7IntR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TX Finish (Ack/BA process Finish) Interrupt."]
    #[inline(always)]
    pub fn bit_fs_ptcl_handle_ok_int(&self) -> BitFsPtclHandleOkIntR {
        BitFsPtclHandleOkIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CCX PKT TX Report Interrupt"]
    #[inline(always)]
    pub fn bit_fs_txccx_int(&self) -> BitFsTxccxIntR {
        BitFsTxccxIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FWHW/ TXDMA/ RXDMA/ WMAC error status interrupt(0x210\\[31:0\\], 0x288\\[31:0\\], 0x4E2\\[15:0\\], 0x6B8\\[31:0\\]). WMAC error state will be recovered by HW, the others won't."]
    #[inline(always)]
    pub fn bit_fs_errorhdl_int(&self) -> BitFsErrorhdlIntR {
        BitFsErrorhdlIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX DONE INTERRUPT"]
    #[inline(always)]
    pub fn bit_fs_tx_close_int(&self) -> BitFsTxCloseIntR {
        BitFsTxCloseIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Host To CPU Message Interrupt"]
    #[inline(always)]
    pub fn bit_fs_h2ccmd_int(&self) -> BitFsH2ccmdIntR {
        BitFsH2ccmdIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Host to CPU Message Interrupt, generated on writing to 0x1CF"]
    #[inline(always)]
    pub fn bit_fs_hrcv_int(&self) -> BitFsHrcvIntR {
        BitFsHrcvIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - C2H register ready for write interrupt"]
    #[inline(always)]
    pub fn bit_fs_c2h_w_ready_int(&self) -> BitFsC2hWReadyIntR {
        BitFsC2hWReadyIntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX report interrupt"]
    #[inline(always)]
    pub fn bit_fs_txrpt_int(&self) -> BitFsTxrptIntR {
        BitFsTxrptIntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - rsvd"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU management queue polled packet not TX OK while new packet comes in, enable to issue interrupt or not"]
    #[inline(always)]
    pub fn bit_fs_cpumgq_err_int(&self) -> BitFsCpumgqErrIntR {
        BitFsCpumgqErrIntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - MgntQ ReadPtr FIFO release interrupt (for 8822B)"]
    #[inline(always)]
    pub fn bit_fs_mgntq_ff_release_int(&self) -> BitFsMgntqFfReleaseIntR {
        BitFsMgntqFfReleaseIntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU management queue polled packet released done interrupt enable (check cpu management queue current flow)"]
    #[inline(always)]
    pub fn bit_fs_cpumgn_polled_pkt_done_int(&self) -> BitFsCpumgnPolledPktDoneIntR {
        BitFsCpumgnPolledPktDoneIntR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MBSSID0 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_mb0_int(&self) -> BitFsTxbcnerrMb0IntR {
        BitFsTxbcnerrMb0IntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MBSSID1 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_mb1_int(&self) -> BitFsTxbcnerrMb1IntR {
        BitFsTxbcnerrMb1IntR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MBSSID2 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_mb2_int(&self) -> BitFsTxbcnerrMb2IntR {
        BitFsTxbcnerrMb2IntR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MBSSID3 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_mb3_int(&self) -> BitFsTxbcnerrMb3IntR {
        BitFsTxbcnerrMb3IntR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MBSSID4 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_mb4_int(&self) -> BitFsTxbcnerrMb4IntR {
        BitFsTxbcnerrMb4IntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MBSSID5 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_mb5_int(&self) -> BitFsTxbcnerrMb5IntR {
        BitFsTxbcnerrMb5IntR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MBSSID6 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_mb6_int(&self) -> BitFsTxbcnerrMb6IntR {
        BitFsTxbcnerrMb6IntR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MBSSID7 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_mb7_int(&self) -> BitFsTxbcnerrMb7IntR {
        BitFsTxbcnerrMb7IntR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MBSSID0 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_mb0_int(&self) -> BitFsTxbcnokMb0IntR {
        BitFsTxbcnokMb0IntR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MBSSID1 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_mb1_int(&self) -> BitFsTxbcnokMb1IntR {
        BitFsTxbcnokMb1IntR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - MBSSID2 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_mb2_int(&self) -> BitFsTxbcnokMb2IntR {
        BitFsTxbcnokMb2IntR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - MBSSID3 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_mb3_int(&self) -> BitFsTxbcnokMb3IntR {
        BitFsTxbcnokMb3IntR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - MBSSID4 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_mb4_int(&self) -> BitFsTxbcnokMb4IntR {
        BitFsTxbcnokMb4IntR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - MBSSID5 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_mb5_int(&self) -> BitFsTxbcnokMb5IntR {
        BitFsTxbcnokMb5IntR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - MBSSID6 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_mb6_int(&self) -> BitFsTxbcnokMb6IntR {
        BitFsTxbcnokMb6IntR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - MBSSID7 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_mb7_int(&self) -> BitFsTxbcnokMb7IntR {
        BitFsTxbcnokMb7IntR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Firmware Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fwisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFwisrSpec;
impl crate::RegisterSpec for RegFwisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fwisr::R`](R) reader structure"]
impl crate::Readable for RegFwisrSpec {}
#[doc = "`reset()` method sets REG_FWISR to value 0"]
impl crate::Resettable for RegFwisrSpec {}
