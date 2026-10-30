#[doc = "Register `REG_FE2ISR` reader"]
pub type R = crate::R<RegFe2isrSpec>;
#[doc = "Field `BIT_FS_TBTT0_MB1INT` reader - the interrupt from the exact TBTT time for P0 MBSSID1."]
pub type BitFsTbtt0Mb1intR = crate::BitReader;
#[doc = "Field `BIT_FS_TBTT0_MB2INT` reader - the interrupt from the exact TBTT time for P0 MBSSID2."]
pub type BitFsTbtt0Mb2intR = crate::BitReader;
#[doc = "Field `BIT_FS_TBTT0_MB3INT` reader - the interrupt from the exact TBTT time for P0 MBSSID3."]
pub type BitFsTbtt0Mb3intR = crate::BitReader;
#[doc = "Field `BIT_FS_TBTT0_MB4INT` reader - the interrupt from the exact TBTT time for P0 MBSSID4."]
pub type BitFsTbtt0Mb4intR = crate::BitReader;
#[doc = "Field `BIT_FS_TBTT0_MB5INT` reader - the interrupt from the exact TBTT time for P0 MBSSID5."]
pub type BitFsTbtt0Mb5intR = crate::BitReader;
#[doc = "Field `BIT_FS_TBTT0_MB6INT` reader - the interrupt from the exact TBTT time for P0 MBSSID6."]
pub type BitFsTbtt0Mb6intR = crate::BitReader;
#[doc = "Field `BIT_FS_TBTT0_MB7INT` reader - the interrupt from the exact TBTT time for P0 MBSSID7."]
pub type BitFsTbtt0Mb7intR = crate::BitReader;
#[doc = "Field `BIT_FS_TBTT1INT` reader - the interrupt from the exact TBTT time for P1."]
pub type BitFsTbtt1intR = crate::BitReader;
#[doc = "Field `BIT_FS_TBTT2_DISC_INT` reader - the interrupt from the exact TBTT time for P2."]
pub type BitFsTbtt2DiscIntR = crate::BitReader;
#[doc = "Field `BIT_FS_TBTT2_SYNC_2G_INT` reader - the interrupt from the exact TBTT time for P2 SYNC_2G."]
pub type BitFsTbtt2Sync2gIntR = crate::BitReader;
#[doc = "Field `BIT_FS_TBTT2_SYNC_5G_INT` reader - the interrupt from the exact TBTT time for P2 SYNC_5G."]
pub type BitFsTbtt2Sync5gIntR = crate::BitReader;
#[doc = "Field `BIT_FS_TBTTALL_INT` reader - the interrupt from the exact TBTT time for all port."]
pub type BitFsTbttallIntR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNDMA2_INT` reader - When BCNDMA interval arrives before TBTT2_DISC, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma2IntR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNDMA_P2_2G_INT` reader - When BCNDMA interval arrives before TBTT2_2G_SYNC, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndmaP2_2gIntR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNDMA_P2_5G_INT` reader - When BCNDMA interval arrives before TBTT2_5G_SYNC, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndmaP2_5gIntR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNERR_PORT2_INT` reader - PORT2 TX DISC beacon Error interrupt"]
pub type BitFsTxbcnerrPort2IntR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNERR_PORT2_SYNC_INT` reader - PORT2 TX SYNC beacon Error interrupt"]
pub type BitFsTxbcnerrPort2SyncIntR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNOK_PORT2_INT` reader - PORT2 TX DISC beacon OK interrupt"]
pub type BitFsTxbcnokPort2IntR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNOK_PORT2_SYNC_INT` reader - PORT2 TX SYNC beacon OK interrupt"]
pub type BitFsTxbcnokPort2SyncIntR = crate::BitReader;
#[doc = "Field `BIT_FS_DWWIN_END_INT` reader - discovery window end interrupt for NAN"]
pub type BitFsDwwinEndIntR = crate::BitReader;
#[doc = "Field `BIT_FS_EVTQ_START_INT` reader - event queue start interrupt for NAN"]
pub type BitFsEvtqStartIntR = crate::BitReader;
#[doc = "Field `BIT_FS_EVTQ_TXDONE_INT` reader - event queue done interrupt for NAN"]
pub type BitFsEvtqTxdoneIntR = crate::BitReader;
#[doc = "Field `BIT_FS_RXBCN_NAN_RPT_OK` reader - port2 rxbcn rpt and write to txrptbuf ok"]
pub type BitFsRxbcnNanRptOkR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - the interrupt from the exact TBTT time for P0 MBSSID1."]
    #[inline(always)]
    pub fn bit_fs_tbtt0_mb1int(&self) -> BitFsTbtt0Mb1intR {
        BitFsTbtt0Mb1intR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the interrupt from the exact TBTT time for P0 MBSSID2."]
    #[inline(always)]
    pub fn bit_fs_tbtt0_mb2int(&self) -> BitFsTbtt0Mb2intR {
        BitFsTbtt0Mb2intR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the interrupt from the exact TBTT time for P0 MBSSID3."]
    #[inline(always)]
    pub fn bit_fs_tbtt0_mb3int(&self) -> BitFsTbtt0Mb3intR {
        BitFsTbtt0Mb3intR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - the interrupt from the exact TBTT time for P0 MBSSID4."]
    #[inline(always)]
    pub fn bit_fs_tbtt0_mb4int(&self) -> BitFsTbtt0Mb4intR {
        BitFsTbtt0Mb4intR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - the interrupt from the exact TBTT time for P0 MBSSID5."]
    #[inline(always)]
    pub fn bit_fs_tbtt0_mb5int(&self) -> BitFsTbtt0Mb5intR {
        BitFsTbtt0Mb5intR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - the interrupt from the exact TBTT time for P0 MBSSID6."]
    #[inline(always)]
    pub fn bit_fs_tbtt0_mb6int(&self) -> BitFsTbtt0Mb6intR {
        BitFsTbtt0Mb6intR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - the interrupt from the exact TBTT time for P0 MBSSID7."]
    #[inline(always)]
    pub fn bit_fs_tbtt0_mb7int(&self) -> BitFsTbtt0Mb7intR {
        BitFsTbtt0Mb7intR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - the interrupt from the exact TBTT time for P1."]
    #[inline(always)]
    pub fn bit_fs_tbtt1int(&self) -> BitFsTbtt1intR {
        BitFsTbtt1intR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - the interrupt from the exact TBTT time for P2."]
    #[inline(always)]
    pub fn bit_fs_tbtt2_disc_int(&self) -> BitFsTbtt2DiscIntR {
        BitFsTbtt2DiscIntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - the interrupt from the exact TBTT time for P2 SYNC_2G."]
    #[inline(always)]
    pub fn bit_fs_tbtt2_sync_2g_int(&self) -> BitFsTbtt2Sync2gIntR {
        BitFsTbtt2Sync2gIntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - the interrupt from the exact TBTT time for P2 SYNC_5G."]
    #[inline(always)]
    pub fn bit_fs_tbtt2_sync_5g_int(&self) -> BitFsTbtt2Sync5gIntR {
        BitFsTbtt2Sync5gIntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - the interrupt from the exact TBTT time for all port."]
    #[inline(always)]
    pub fn bit_fs_tbttall_int(&self) -> BitFsTbttallIntR {
        BitFsTbttallIntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When BCNDMA interval arrives before TBTT2_DISC, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma2_int(&self) -> BitFsBcndma2IntR {
        BitFsBcndma2IntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - When BCNDMA interval arrives before TBTT2_2G_SYNC, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma_p2_2g_int(&self) -> BitFsBcndmaP2_2gIntR {
        BitFsBcndmaP2_2gIntR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When BCNDMA interval arrives before TBTT2_5G_SYNC, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma_p2_5g_int(&self) -> BitFsBcndmaP2_5gIntR {
        BitFsBcndmaP2_5gIntR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - PORT2 TX DISC beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_port2_int(&self) -> BitFsTxbcnerrPort2IntR {
        BitFsTxbcnerrPort2IntR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PORT2 TX SYNC beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_port2_sync_int(&self) -> BitFsTxbcnerrPort2SyncIntR {
        BitFsTxbcnerrPort2SyncIntR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PORT2 TX DISC beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_port2_int(&self) -> BitFsTxbcnokPort2IntR {
        BitFsTxbcnokPort2IntR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PORT2 TX SYNC beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_port2_sync_int(&self) -> BitFsTxbcnokPort2SyncIntR {
        BitFsTxbcnokPort2SyncIntR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - discovery window end interrupt for NAN"]
    #[inline(always)]
    pub fn bit_fs_dwwin_end_int(&self) -> BitFsDwwinEndIntR {
        BitFsDwwinEndIntR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - event queue start interrupt for NAN"]
    #[inline(always)]
    pub fn bit_fs_evtq_start_int(&self) -> BitFsEvtqStartIntR {
        BitFsEvtqStartIntR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - event queue done interrupt for NAN"]
    #[inline(always)]
    pub fn bit_fs_evtq_txdone_int(&self) -> BitFsEvtqTxdoneIntR {
        BitFsEvtqTxdoneIntR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - port2 rxbcn rpt and write to txrptbuf ok"]
    #[inline(always)]
    pub fn bit_fs_rxbcn_nan_rpt_ok(&self) -> BitFsRxbcnNanRptOkR {
        BitFsRxbcnNanRptOkR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fe2isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFe2isrSpec;
impl crate::RegisterSpec for RegFe2isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fe2isr::R`](R) reader structure"]
impl crate::Readable for RegFe2isrSpec {}
#[doc = "`reset()` method sets REG_FE2ISR to value 0"]
impl crate::Resettable for RegFe2isrSpec {}
