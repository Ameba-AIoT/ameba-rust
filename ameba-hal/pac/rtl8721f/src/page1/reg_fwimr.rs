#[doc = "Register `REG_FWIMR` reader"]
pub type R = crate::R<RegFwimrSpec>;
#[doc = "Register `REG_FWIMR` writer"]
pub type W = crate::W<RegFwimrSpec>;
#[doc = "Field `BIT_FS_PTCL_HANDLE_OK_INT_EN` reader - TX Finish (Ack/BA process Finish) Interrupt."]
pub type BitFsPtclHandleOkIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PTCL_HANDLE_OK_INT_EN` writer - TX Finish (Ack/BA process Finish) Interrupt."]
pub type BitFsPtclHandleOkIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXCCX_INT_EN` reader - CCX PKT TX Report Interrupt"]
pub type BitFsTxccxIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXCCX_INT_EN` writer - CCX PKT TX Report Interrupt"]
pub type BitFsTxccxIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_ERRORHDL_INT_EN` reader - FWHW/ TXDMA/ RXDMA/ WMAC error status interrupt(0x210\\[31:0\\], 0x288\\[31:0\\], 0x4E2\\[15:0\\], 0x6B8\\[31:0\\]). WMAC error state will be recovered by HW, the others won't."]
pub type BitFsErrorhdlIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_ERRORHDL_INT_EN` writer - FWHW/ TXDMA/ RXDMA/ WMAC error status interrupt(0x210\\[31:0\\], 0x288\\[31:0\\], 0x4E2\\[15:0\\], 0x6B8\\[31:0\\]). WMAC error state will be recovered by HW, the others won't."]
pub type BitFsErrorhdlIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TX_CLOSE_INT_EN` reader - TX DONE INTERRUPT"]
pub type BitFsTxCloseIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TX_CLOSE_INT_EN` writer - TX DONE INTERRUPT"]
pub type BitFsTxCloseIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_H2CCMD_INT_EN` reader - Host To CPU Message Interrupt"]
pub type BitFsH2ccmdIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_H2CCMD_INT_EN` writer - Host To CPU Message Interrupt"]
pub type BitFsH2ccmdIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_HRCV_INT_EN` reader - Host to CPU Message Interrupt, generated on writing to 0x1CF"]
pub type BitFsHrcvIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_HRCV_INT_EN` writer - Host to CPU Message Interrupt, generated on writing to 0x1CF"]
pub type BitFsHrcvIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_C2H_W_READY_INT_EN` reader - C2H register ready for write interrupt"]
pub type BitFsC2hWReadyIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_C2H_W_READY_INT_EN` writer - C2H register ready for write interrupt"]
pub type BitFsC2hWReadyIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXRPT_INT_EN` reader - TX report interrupt"]
pub type BitFsTxrptIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXRPT_INT_EN` writer - TX report interrupt"]
pub type BitFsTxrptIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_DDMA0_HP_INT_EN` reader - High priority ddma0 down interrupt"]
pub type BitFsDdma0HpIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_DDMA0_HP_INT_EN` writer - High priority ddma0 down interrupt"]
pub type BitFsDdma0HpIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_DDMA0_LP_INT_EN` reader - Low priority ddma0 down interrupt ."]
pub type BitFsDdma0LpIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_DDMA0_LP_INT_EN` writer - Low priority ddma0 down interrupt ."]
pub type BitFsDdma0LpIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - rsvd"]
pub type RsvdR = crate::BitReader;
#[doc = "Field `RSVD` writer - rsvd"]
pub type RsvdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_CPUMGQ_ERR_INT_EN` reader - CPU management queue polled packet not TX OK while new packet comes in, enable to issue interrupt or not"]
pub type BitFsCpumgqErrIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_CPUMGQ_ERR_INT_EN` writer - CPU management queue polled packet not TX OK while new packet comes in, enable to issue interrupt or not"]
pub type BitFsCpumgqErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_MGNTQ_FF_RELEASE_INT_EN` reader - MgntQ ReadPtr FIFO release interrupt (for 8822B)"]
pub type BitFsMgntqFfReleaseIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_MGNTQ_FF_RELEASE_INT_EN` writer - MgntQ ReadPtr FIFO release interrupt (for 8822B)"]
pub type BitFsMgntqFfReleaseIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_CPUMGN_POLLED_PKT_DONE_INT_EN` reader - CPU management queue polled packet released done interrupt enable (check cpu management queue current flow)"]
pub type BitFsCpumgnPolledPktDoneIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_CPUMGN_POLLED_PKT_DONE_INT_EN` writer - CPU management queue polled packet released done interrupt enable (check cpu management queue current flow)"]
pub type BitFsCpumgnPolledPktDoneIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXBCNERR_MB0_INT_EN` reader - MBSSID0 TX beacon Error interrupt"]
pub type BitFsTxbcnerrMb0IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNERR_MB0_INT_EN` writer - MBSSID0 TX beacon Error interrupt"]
pub type BitFsTxbcnerrMb0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXBCNERR_MB1_INT_EN` reader - MBSSID1 TX beacon Error interrupt"]
pub type BitFsTxbcnerrMb1IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNERR_MB1_INT_EN` writer - MBSSID1 TX beacon Error interrupt"]
pub type BitFsTxbcnerrMb1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXBCNERR_MB2_INT_EN` reader - MBSSID2 TX beacon Error interrupt"]
pub type BitFsTxbcnerrMb2IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNERR_MB2_INT_EN` writer - MBSSID2 TX beacon Error interrupt"]
pub type BitFsTxbcnerrMb2IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXBCNERR_MB3_INT_EN` reader - MBSSID3 TX beacon Error interrupt"]
pub type BitFsTxbcnerrMb3IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNERR_MB3_INT_EN` writer - MBSSID3 TX beacon Error interrupt"]
pub type BitFsTxbcnerrMb3IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXBCNERR_MB4_INT_EN` reader - MBSSID4 TX beacon Error interrupt"]
pub type BitFsTxbcnerrMb4IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNERR_MB4_INT_EN` writer - MBSSID4 TX beacon Error interrupt"]
pub type BitFsTxbcnerrMb4IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXBCNERR_MB5_INT_EN` reader - MBSSID5 TX beacon Error interrupt"]
pub type BitFsTxbcnerrMb5IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNERR_MB5_INT_EN` writer - MBSSID5 TX beacon Error interrupt"]
pub type BitFsTxbcnerrMb5IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXBCNERR_MB6_INT_EN` reader - MBSSID6 TX beacon Error interrupt"]
pub type BitFsTxbcnerrMb6IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNERR_MB6_INT_EN` writer - MBSSID6 TX beacon Error interrupt"]
pub type BitFsTxbcnerrMb6IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXBCNERR_MB7_INT_EN` reader - MBSSID7 TX beacon Error interrupt"]
pub type BitFsTxbcnerrMb7IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNERR_MB7_INT_EN` writer - MBSSID7 TX beacon Error interrupt"]
pub type BitFsTxbcnerrMb7IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXBCNOK_MB0_INT_EN` reader - MBSSID0 TX beacon OK interrupt"]
pub type BitFsTxbcnokMb0IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNOK_MB0_INT_EN` writer - MBSSID0 TX beacon OK interrupt"]
pub type BitFsTxbcnokMb0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXBCNOK_MB1_INT_EN` reader - MBSSID1 TX beacon OK interrupt"]
pub type BitFsTxbcnokMb1IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNOK_MB1_INT_EN` writer - MBSSID1 TX beacon OK interrupt"]
pub type BitFsTxbcnokMb1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXBCNOK_MB2_INT_EN` reader - MBSSID2 TX beacon OK interrupt"]
pub type BitFsTxbcnokMb2IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNOK_MB2_INT_EN` writer - MBSSID2 TX beacon OK interrupt"]
pub type BitFsTxbcnokMb2IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXBCNOK_MB3_INT_EN` reader - MBSSID3 TX beacon OK interrupt"]
pub type BitFsTxbcnokMb3IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNOK_MB3_INT_EN` writer - MBSSID3 TX beacon OK interrupt"]
pub type BitFsTxbcnokMb3IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXBCNOK_MB4_INT_EN` reader - MBSSID4 TX beacon OK interrupt"]
pub type BitFsTxbcnokMb4IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNOK_MB4_INT_EN` writer - MBSSID4 TX beacon OK interrupt"]
pub type BitFsTxbcnokMb4IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXBCNOK_MB5_INT_EN` reader - MBSSID5 TX beacon OK interrupt"]
pub type BitFsTxbcnokMb5IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNOK_MB5_INT_EN` writer - MBSSID5 TX beacon OK interrupt"]
pub type BitFsTxbcnokMb5IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXBCNOK_MB6_INT_EN` reader - MBSSID6 TX beacon OK interrupt"]
pub type BitFsTxbcnokMb6IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNOK_MB6_INT_EN` writer - MBSSID6 TX beacon OK interrupt"]
pub type BitFsTxbcnokMb6IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXBCNOK_MB7_INT_EN` reader - MBSSID7 TX beacon OK interrupt"]
pub type BitFsTxbcnokMb7IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNOK_MB7_INT_EN` writer - MBSSID7 TX beacon OK interrupt"]
pub type BitFsTxbcnokMb7IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TX Finish (Ack/BA process Finish) Interrupt."]
    #[inline(always)]
    pub fn bit_fs_ptcl_handle_ok_int_en(&self) -> BitFsPtclHandleOkIntEnR {
        BitFsPtclHandleOkIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CCX PKT TX Report Interrupt"]
    #[inline(always)]
    pub fn bit_fs_txccx_int_en(&self) -> BitFsTxccxIntEnR {
        BitFsTxccxIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FWHW/ TXDMA/ RXDMA/ WMAC error status interrupt(0x210\\[31:0\\], 0x288\\[31:0\\], 0x4E2\\[15:0\\], 0x6B8\\[31:0\\]). WMAC error state will be recovered by HW, the others won't."]
    #[inline(always)]
    pub fn bit_fs_errorhdl_int_en(&self) -> BitFsErrorhdlIntEnR {
        BitFsErrorhdlIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX DONE INTERRUPT"]
    #[inline(always)]
    pub fn bit_fs_tx_close_int_en(&self) -> BitFsTxCloseIntEnR {
        BitFsTxCloseIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Host To CPU Message Interrupt"]
    #[inline(always)]
    pub fn bit_fs_h2ccmd_int_en(&self) -> BitFsH2ccmdIntEnR {
        BitFsH2ccmdIntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Host to CPU Message Interrupt, generated on writing to 0x1CF"]
    #[inline(always)]
    pub fn bit_fs_hrcv_int_en(&self) -> BitFsHrcvIntEnR {
        BitFsHrcvIntEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - C2H register ready for write interrupt"]
    #[inline(always)]
    pub fn bit_fs_c2h_w_ready_int_en(&self) -> BitFsC2hWReadyIntEnR {
        BitFsC2hWReadyIntEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX report interrupt"]
    #[inline(always)]
    pub fn bit_fs_txrpt_int_en(&self) -> BitFsTxrptIntEnR {
        BitFsTxrptIntEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High priority ddma0 down interrupt"]
    #[inline(always)]
    pub fn bit_fs_ddma0_hp_int_en(&self) -> BitFsDdma0HpIntEnR {
        BitFsDdma0HpIntEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Low priority ddma0 down interrupt ."]
    #[inline(always)]
    pub fn bit_fs_ddma0_lp_int_en(&self) -> BitFsDdma0LpIntEnR {
        BitFsDdma0LpIntEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - rsvd"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU management queue polled packet not TX OK while new packet comes in, enable to issue interrupt or not"]
    #[inline(always)]
    pub fn bit_fs_cpumgq_err_int_en(&self) -> BitFsCpumgqErrIntEnR {
        BitFsCpumgqErrIntEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - MgntQ ReadPtr FIFO release interrupt (for 8822B)"]
    #[inline(always)]
    pub fn bit_fs_mgntq_ff_release_int_en(&self) -> BitFsMgntqFfReleaseIntEnR {
        BitFsMgntqFfReleaseIntEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU management queue polled packet released done interrupt enable (check cpu management queue current flow)"]
    #[inline(always)]
    pub fn bit_fs_cpumgn_polled_pkt_done_int_en(&self) -> BitFsCpumgnPolledPktDoneIntEnR {
        BitFsCpumgnPolledPktDoneIntEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MBSSID0 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_mb0_int_en(&self) -> BitFsTxbcnerrMb0IntEnR {
        BitFsTxbcnerrMb0IntEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MBSSID1 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_mb1_int_en(&self) -> BitFsTxbcnerrMb1IntEnR {
        BitFsTxbcnerrMb1IntEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MBSSID2 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_mb2_int_en(&self) -> BitFsTxbcnerrMb2IntEnR {
        BitFsTxbcnerrMb2IntEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MBSSID3 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_mb3_int_en(&self) -> BitFsTxbcnerrMb3IntEnR {
        BitFsTxbcnerrMb3IntEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MBSSID4 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_mb4_int_en(&self) -> BitFsTxbcnerrMb4IntEnR {
        BitFsTxbcnerrMb4IntEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MBSSID5 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_mb5_int_en(&self) -> BitFsTxbcnerrMb5IntEnR {
        BitFsTxbcnerrMb5IntEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MBSSID6 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_mb6_int_en(&self) -> BitFsTxbcnerrMb6IntEnR {
        BitFsTxbcnerrMb6IntEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MBSSID7 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_mb7_int_en(&self) -> BitFsTxbcnerrMb7IntEnR {
        BitFsTxbcnerrMb7IntEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MBSSID0 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_mb0_int_en(&self) -> BitFsTxbcnokMb0IntEnR {
        BitFsTxbcnokMb0IntEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MBSSID1 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_mb1_int_en(&self) -> BitFsTxbcnokMb1IntEnR {
        BitFsTxbcnokMb1IntEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - MBSSID2 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_mb2_int_en(&self) -> BitFsTxbcnokMb2IntEnR {
        BitFsTxbcnokMb2IntEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - MBSSID3 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_mb3_int_en(&self) -> BitFsTxbcnokMb3IntEnR {
        BitFsTxbcnokMb3IntEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - MBSSID4 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_mb4_int_en(&self) -> BitFsTxbcnokMb4IntEnR {
        BitFsTxbcnokMb4IntEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - MBSSID5 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_mb5_int_en(&self) -> BitFsTxbcnokMb5IntEnR {
        BitFsTxbcnokMb5IntEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - MBSSID6 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_mb6_int_en(&self) -> BitFsTxbcnokMb6IntEnR {
        BitFsTxbcnokMb6IntEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - MBSSID7 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_mb7_int_en(&self) -> BitFsTxbcnokMb7IntEnR {
        BitFsTxbcnokMb7IntEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX Finish (Ack/BA process Finish) Interrupt."]
    #[inline(always)]
    pub fn bit_fs_ptcl_handle_ok_int_en(&mut self) -> BitFsPtclHandleOkIntEnW<'_, RegFwimrSpec> {
        BitFsPtclHandleOkIntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - CCX PKT TX Report Interrupt"]
    #[inline(always)]
    pub fn bit_fs_txccx_int_en(&mut self) -> BitFsTxccxIntEnW<'_, RegFwimrSpec> {
        BitFsTxccxIntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - FWHW/ TXDMA/ RXDMA/ WMAC error status interrupt(0x210\\[31:0\\], 0x288\\[31:0\\], 0x4E2\\[15:0\\], 0x6B8\\[31:0\\]). WMAC error state will be recovered by HW, the others won't."]
    #[inline(always)]
    pub fn bit_fs_errorhdl_int_en(&mut self) -> BitFsErrorhdlIntEnW<'_, RegFwimrSpec> {
        BitFsErrorhdlIntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - TX DONE INTERRUPT"]
    #[inline(always)]
    pub fn bit_fs_tx_close_int_en(&mut self) -> BitFsTxCloseIntEnW<'_, RegFwimrSpec> {
        BitFsTxCloseIntEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Host To CPU Message Interrupt"]
    #[inline(always)]
    pub fn bit_fs_h2ccmd_int_en(&mut self) -> BitFsH2ccmdIntEnW<'_, RegFwimrSpec> {
        BitFsH2ccmdIntEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Host to CPU Message Interrupt, generated on writing to 0x1CF"]
    #[inline(always)]
    pub fn bit_fs_hrcv_int_en(&mut self) -> BitFsHrcvIntEnW<'_, RegFwimrSpec> {
        BitFsHrcvIntEnW::new(self, 5)
    }
    #[doc = "Bit 6 - C2H register ready for write interrupt"]
    #[inline(always)]
    pub fn bit_fs_c2h_w_ready_int_en(&mut self) -> BitFsC2hWReadyIntEnW<'_, RegFwimrSpec> {
        BitFsC2hWReadyIntEnW::new(self, 6)
    }
    #[doc = "Bit 7 - TX report interrupt"]
    #[inline(always)]
    pub fn bit_fs_txrpt_int_en(&mut self) -> BitFsTxrptIntEnW<'_, RegFwimrSpec> {
        BitFsTxrptIntEnW::new(self, 7)
    }
    #[doc = "Bit 8 - High priority ddma0 down interrupt"]
    #[inline(always)]
    pub fn bit_fs_ddma0_hp_int_en(&mut self) -> BitFsDdma0HpIntEnW<'_, RegFwimrSpec> {
        BitFsDdma0HpIntEnW::new(self, 8)
    }
    #[doc = "Bit 9 - Low priority ddma0 down interrupt ."]
    #[inline(always)]
    pub fn bit_fs_ddma0_lp_int_en(&mut self) -> BitFsDdma0LpIntEnW<'_, RegFwimrSpec> {
        BitFsDdma0LpIntEnW::new(self, 9)
    }
    #[doc = "Bit 10 - rsvd"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegFwimrSpec> {
        RsvdW::new(self, 10)
    }
    #[doc = "Bit 11 - CPU management queue polled packet not TX OK while new packet comes in, enable to issue interrupt or not"]
    #[inline(always)]
    pub fn bit_fs_cpumgq_err_int_en(&mut self) -> BitFsCpumgqErrIntEnW<'_, RegFwimrSpec> {
        BitFsCpumgqErrIntEnW::new(self, 11)
    }
    #[doc = "Bit 13 - MgntQ ReadPtr FIFO release interrupt (for 8822B)"]
    #[inline(always)]
    pub fn bit_fs_mgntq_ff_release_int_en(
        &mut self,
    ) -> BitFsMgntqFfReleaseIntEnW<'_, RegFwimrSpec> {
        BitFsMgntqFfReleaseIntEnW::new(self, 13)
    }
    #[doc = "Bit 15 - CPU management queue polled packet released done interrupt enable (check cpu management queue current flow)"]
    #[inline(always)]
    pub fn bit_fs_cpumgn_polled_pkt_done_int_en(
        &mut self,
    ) -> BitFsCpumgnPolledPktDoneIntEnW<'_, RegFwimrSpec> {
        BitFsCpumgnPolledPktDoneIntEnW::new(self, 15)
    }
    #[doc = "Bit 16 - MBSSID0 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_mb0_int_en(&mut self) -> BitFsTxbcnerrMb0IntEnW<'_, RegFwimrSpec> {
        BitFsTxbcnerrMb0IntEnW::new(self, 16)
    }
    #[doc = "Bit 17 - MBSSID1 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_mb1_int_en(&mut self) -> BitFsTxbcnerrMb1IntEnW<'_, RegFwimrSpec> {
        BitFsTxbcnerrMb1IntEnW::new(self, 17)
    }
    #[doc = "Bit 18 - MBSSID2 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_mb2_int_en(&mut self) -> BitFsTxbcnerrMb2IntEnW<'_, RegFwimrSpec> {
        BitFsTxbcnerrMb2IntEnW::new(self, 18)
    }
    #[doc = "Bit 19 - MBSSID3 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_mb3_int_en(&mut self) -> BitFsTxbcnerrMb3IntEnW<'_, RegFwimrSpec> {
        BitFsTxbcnerrMb3IntEnW::new(self, 19)
    }
    #[doc = "Bit 20 - MBSSID4 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_mb4_int_en(&mut self) -> BitFsTxbcnerrMb4IntEnW<'_, RegFwimrSpec> {
        BitFsTxbcnerrMb4IntEnW::new(self, 20)
    }
    #[doc = "Bit 21 - MBSSID5 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_mb5_int_en(&mut self) -> BitFsTxbcnerrMb5IntEnW<'_, RegFwimrSpec> {
        BitFsTxbcnerrMb5IntEnW::new(self, 21)
    }
    #[doc = "Bit 22 - MBSSID6 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_mb6_int_en(&mut self) -> BitFsTxbcnerrMb6IntEnW<'_, RegFwimrSpec> {
        BitFsTxbcnerrMb6IntEnW::new(self, 22)
    }
    #[doc = "Bit 23 - MBSSID7 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_mb7_int_en(&mut self) -> BitFsTxbcnerrMb7IntEnW<'_, RegFwimrSpec> {
        BitFsTxbcnerrMb7IntEnW::new(self, 23)
    }
    #[doc = "Bit 24 - MBSSID0 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_mb0_int_en(&mut self) -> BitFsTxbcnokMb0IntEnW<'_, RegFwimrSpec> {
        BitFsTxbcnokMb0IntEnW::new(self, 24)
    }
    #[doc = "Bit 25 - MBSSID1 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_mb1_int_en(&mut self) -> BitFsTxbcnokMb1IntEnW<'_, RegFwimrSpec> {
        BitFsTxbcnokMb1IntEnW::new(self, 25)
    }
    #[doc = "Bit 26 - MBSSID2 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_mb2_int_en(&mut self) -> BitFsTxbcnokMb2IntEnW<'_, RegFwimrSpec> {
        BitFsTxbcnokMb2IntEnW::new(self, 26)
    }
    #[doc = "Bit 27 - MBSSID3 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_mb3_int_en(&mut self) -> BitFsTxbcnokMb3IntEnW<'_, RegFwimrSpec> {
        BitFsTxbcnokMb3IntEnW::new(self, 27)
    }
    #[doc = "Bit 28 - MBSSID4 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_mb4_int_en(&mut self) -> BitFsTxbcnokMb4IntEnW<'_, RegFwimrSpec> {
        BitFsTxbcnokMb4IntEnW::new(self, 28)
    }
    #[doc = "Bit 29 - MBSSID5 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_mb5_int_en(&mut self) -> BitFsTxbcnokMb5IntEnW<'_, RegFwimrSpec> {
        BitFsTxbcnokMb5IntEnW::new(self, 29)
    }
    #[doc = "Bit 30 - MBSSID6 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_mb6_int_en(&mut self) -> BitFsTxbcnokMb6IntEnW<'_, RegFwimrSpec> {
        BitFsTxbcnokMb6IntEnW::new(self, 30)
    }
    #[doc = "Bit 31 - MBSSID7 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_mb7_int_en(&mut self) -> BitFsTxbcnokMb7IntEnW<'_, RegFwimrSpec> {
        BitFsTxbcnokMb7IntEnW::new(self, 31)
    }
}
#[doc = "Firmware Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fwimr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fwimr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFwimrSpec;
impl crate::RegisterSpec for RegFwimrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fwimr::R`](R) reader structure"]
impl crate::Readable for RegFwimrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_fwimr::W`](W) writer structure"]
impl crate::Writable for RegFwimrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FWIMR to value 0"]
impl crate::Resettable for RegFwimrSpec {}
