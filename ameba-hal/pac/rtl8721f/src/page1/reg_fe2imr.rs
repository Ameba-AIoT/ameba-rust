#[doc = "Register `REG_FE2IMR` reader"]
pub type R = crate::R<RegFe2imrSpec>;
#[doc = "Register `REG_FE2IMR` writer"]
pub type W = crate::W<RegFe2imrSpec>;
#[doc = "Field `BIT_FS_TBTT0_MB1INT_EN` reader - the interrupt from the exact TBTT time for P0 MBSSID1."]
pub type BitFsTbtt0Mb1intEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TBTT0_MB1INT_EN` writer - the interrupt from the exact TBTT time for P0 MBSSID1."]
pub type BitFsTbtt0Mb1intEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TBTT0_MB2INT_EN` reader - the interrupt from the exact TBTT time for P0 MBSSID2."]
pub type BitFsTbtt0Mb2intEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TBTT0_MB2INT_EN` writer - the interrupt from the exact TBTT time for P0 MBSSID2."]
pub type BitFsTbtt0Mb2intEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TBTT0_MB3INT_EN` reader - the interrupt from the exact TBTT time for P0 MBSSID3."]
pub type BitFsTbtt0Mb3intEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TBTT0_MB3INT_EN` writer - the interrupt from the exact TBTT time for P0 MBSSID3."]
pub type BitFsTbtt0Mb3intEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TBTT0_MB4INT_EN` reader - the interrupt from the exact TBTT time for P0 MBSSID4."]
pub type BitFsTbtt0Mb4intEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TBTT0_MB4INT_EN` writer - the interrupt from the exact TBTT time for P0 MBSSID4."]
pub type BitFsTbtt0Mb4intEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TBTT0_MB5INT_EN` reader - the interrupt from the exact TBTT time for P0 MBSSID5."]
pub type BitFsTbtt0Mb5intEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TBTT0_MB5INT_EN` writer - the interrupt from the exact TBTT time for P0 MBSSID5."]
pub type BitFsTbtt0Mb5intEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TBTT0_MB6INT_EN` reader - the interrupt from the exact TBTT time for P0 MBSSID6."]
pub type BitFsTbtt0Mb6intEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TBTT0_MB6INT_EN` writer - the interrupt from the exact TBTT time for P0 MBSSID6."]
pub type BitFsTbtt0Mb6intEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TBTT0_MB7INT_EN` reader - the interrupt from the exact TBTT time for P0 MBSSID7."]
pub type BitFsTbtt0Mb7intEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TBTT0_MB7INT_EN` writer - the interrupt from the exact TBTT time for P0 MBSSID7."]
pub type BitFsTbtt0Mb7intEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TBTT1INT_EN` reader - the interrupt from the exact TBTT time for P1."]
pub type BitFsTbtt1intEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TBTT1INT_EN` writer - the interrupt from the exact TBTT time for P1."]
pub type BitFsTbtt1intEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TBTT2_DISC_INT_EN` reader - the interrupt from the exact TBTT time for P2."]
pub type BitFsTbtt2DiscIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TBTT2_DISC_INT_EN` writer - the interrupt from the exact TBTT time for P2."]
pub type BitFsTbtt2DiscIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TBTT2_SYNC_2G_INT_EN` reader - the interrupt from the exact TBTT time for P2 SYNC_2G."]
pub type BitFsTbtt2Sync2gIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TBTT2_SYNC_2G_INT_EN` writer - the interrupt from the exact TBTT time for P2 SYNC_2G."]
pub type BitFsTbtt2Sync2gIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TBTT2_SYNC_5G_INT_EN` reader - the interrupt from the exact TBTT time for P2 SYNC_5G."]
pub type BitFsTbtt2Sync5gIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TBTT2_SYNC_5G_INT_EN` writer - the interrupt from the exact TBTT time for P2 SYNC_5G."]
pub type BitFsTbtt2Sync5gIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TBTTALL_INT_EN` reader - the interrupt from the exact TBTT time for all port."]
pub type BitFsTbttallIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TBTTALL_INT_EN` writer - the interrupt from the exact TBTT time for all port."]
pub type BitFsTbttallIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNDMA2_INT_EN` reader - When BCNDMA interval arrives before TBTT2_DISC, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma2IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNDMA2_INT_EN` writer - When BCNDMA interval arrives before TBTT2_DISC, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndma2IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNDMA_P2_2G_INT_EN` reader - When BCNDMA interval arrives before TBTT2_2G_SYNC, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndmaP2_2gIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNDMA_P2_2G_INT_EN` writer - When BCNDMA interval arrives before TBTT2_2G_SYNC, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndmaP2_2gIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNDMA_P2_5G_INT_EN` reader - When BCNDMA interval arrives before TBTT2_5G_SYNC, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndmaP2_5gIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNDMA_P2_5G_INT_EN` writer - When BCNDMA interval arrives before TBTT2_5G_SYNC, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitFsBcndmaP2_5gIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXBCNERR_PORT2_INT_EN` reader - PORT2 TX DISC beacon Error interrupt"]
pub type BitFsTxbcnerrPort2IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNERR_PORT2_INT_EN` writer - PORT2 TX DISC beacon Error interrupt"]
pub type BitFsTxbcnerrPort2IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXBCNERR_PORT2_SYNC_INT_EN` reader - PORT2 TX SYNC beacon Error interrupt"]
pub type BitFsTxbcnerrPort2SyncIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNERR_PORT2_SYNC_INT_EN` writer - PORT2 TX SYNC beacon Error interrupt"]
pub type BitFsTxbcnerrPort2SyncIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXBCNOK_PORT2_INT_EN` reader - PORT2 TX DISC beacon OK interrupt"]
pub type BitFsTxbcnokPort2IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNOK_PORT2_INT_EN` writer - PORT2 TX DISC beacon OK interrupt"]
pub type BitFsTxbcnokPort2IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXBCNOK_PORT2_SYNC_INT_EN` reader - PORT2 TX SYNC beacon OK interrupt"]
pub type BitFsTxbcnokPort2SyncIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXBCNOK_PORT2_SYNC_INT_EN` writer - PORT2 TX SYNC beacon OK interrupt"]
pub type BitFsTxbcnokPort2SyncIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_DWWIN_END_INT_EN` reader - discovery window end interrupt for NAN"]
pub type BitFsDwwinEndIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_DWWIN_END_INT_EN` writer - discovery window end interrupt for NAN"]
pub type BitFsDwwinEndIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_EVTQ_START_INT_EN` reader - event queue start interrupt for NAN"]
pub type BitFsEvtqStartIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_EVTQ_START_INT_EN` writer - event queue start interrupt for NAN"]
pub type BitFsEvtqStartIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_EVTQ_TXDONE_INT_EN` reader - event queue done interrupt for NAN"]
pub type BitFsEvtqTxdoneIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_EVTQ_TXDONE_INT_EN` writer - event queue done interrupt for NAN"]
pub type BitFsEvtqTxdoneIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_RXBCN_NAN_RPT_OK_EN` reader - port2 rxbcn rpt and write to txrptbuf ok"]
pub type BitFsRxbcnNanRptOkEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RXBCN_NAN_RPT_OK_EN` writer - port2 rxbcn rpt and write to txrptbuf ok"]
pub type BitFsRxbcnNanRptOkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - the interrupt from the exact TBTT time for P0 MBSSID1."]
    #[inline(always)]
    pub fn bit_fs_tbtt0_mb1int_en(&self) -> BitFsTbtt0Mb1intEnR {
        BitFsTbtt0Mb1intEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the interrupt from the exact TBTT time for P0 MBSSID2."]
    #[inline(always)]
    pub fn bit_fs_tbtt0_mb2int_en(&self) -> BitFsTbtt0Mb2intEnR {
        BitFsTbtt0Mb2intEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the interrupt from the exact TBTT time for P0 MBSSID3."]
    #[inline(always)]
    pub fn bit_fs_tbtt0_mb3int_en(&self) -> BitFsTbtt0Mb3intEnR {
        BitFsTbtt0Mb3intEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - the interrupt from the exact TBTT time for P0 MBSSID4."]
    #[inline(always)]
    pub fn bit_fs_tbtt0_mb4int_en(&self) -> BitFsTbtt0Mb4intEnR {
        BitFsTbtt0Mb4intEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - the interrupt from the exact TBTT time for P0 MBSSID5."]
    #[inline(always)]
    pub fn bit_fs_tbtt0_mb5int_en(&self) -> BitFsTbtt0Mb5intEnR {
        BitFsTbtt0Mb5intEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - the interrupt from the exact TBTT time for P0 MBSSID6."]
    #[inline(always)]
    pub fn bit_fs_tbtt0_mb6int_en(&self) -> BitFsTbtt0Mb6intEnR {
        BitFsTbtt0Mb6intEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - the interrupt from the exact TBTT time for P0 MBSSID7."]
    #[inline(always)]
    pub fn bit_fs_tbtt0_mb7int_en(&self) -> BitFsTbtt0Mb7intEnR {
        BitFsTbtt0Mb7intEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - the interrupt from the exact TBTT time for P1."]
    #[inline(always)]
    pub fn bit_fs_tbtt1int_en(&self) -> BitFsTbtt1intEnR {
        BitFsTbtt1intEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - the interrupt from the exact TBTT time for P2."]
    #[inline(always)]
    pub fn bit_fs_tbtt2_disc_int_en(&self) -> BitFsTbtt2DiscIntEnR {
        BitFsTbtt2DiscIntEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - the interrupt from the exact TBTT time for P2 SYNC_2G."]
    #[inline(always)]
    pub fn bit_fs_tbtt2_sync_2g_int_en(&self) -> BitFsTbtt2Sync2gIntEnR {
        BitFsTbtt2Sync2gIntEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - the interrupt from the exact TBTT time for P2 SYNC_5G."]
    #[inline(always)]
    pub fn bit_fs_tbtt2_sync_5g_int_en(&self) -> BitFsTbtt2Sync5gIntEnR {
        BitFsTbtt2Sync5gIntEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - the interrupt from the exact TBTT time for all port."]
    #[inline(always)]
    pub fn bit_fs_tbttall_int_en(&self) -> BitFsTbttallIntEnR {
        BitFsTbttallIntEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When BCNDMA interval arrives before TBTT2_DISC, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma2_int_en(&self) -> BitFsBcndma2IntEnR {
        BitFsBcndma2IntEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - When BCNDMA interval arrives before TBTT2_2G_SYNC, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma_p2_2g_int_en(&self) -> BitFsBcndmaP2_2gIntEnR {
        BitFsBcndmaP2_2gIntEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When BCNDMA interval arrives before TBTT2_5G_SYNC, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma_p2_5g_int_en(&self) -> BitFsBcndmaP2_5gIntEnR {
        BitFsBcndmaP2_5gIntEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - PORT2 TX DISC beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_port2_int_en(&self) -> BitFsTxbcnerrPort2IntEnR {
        BitFsTxbcnerrPort2IntEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PORT2 TX SYNC beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_port2_sync_int_en(&self) -> BitFsTxbcnerrPort2SyncIntEnR {
        BitFsTxbcnerrPort2SyncIntEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PORT2 TX DISC beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_port2_int_en(&self) -> BitFsTxbcnokPort2IntEnR {
        BitFsTxbcnokPort2IntEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PORT2 TX SYNC beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_port2_sync_int_en(&self) -> BitFsTxbcnokPort2SyncIntEnR {
        BitFsTxbcnokPort2SyncIntEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - discovery window end interrupt for NAN"]
    #[inline(always)]
    pub fn bit_fs_dwwin_end_int_en(&self) -> BitFsDwwinEndIntEnR {
        BitFsDwwinEndIntEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - event queue start interrupt for NAN"]
    #[inline(always)]
    pub fn bit_fs_evtq_start_int_en(&self) -> BitFsEvtqStartIntEnR {
        BitFsEvtqStartIntEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - event queue done interrupt for NAN"]
    #[inline(always)]
    pub fn bit_fs_evtq_txdone_int_en(&self) -> BitFsEvtqTxdoneIntEnR {
        BitFsEvtqTxdoneIntEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - port2 rxbcn rpt and write to txrptbuf ok"]
    #[inline(always)]
    pub fn bit_fs_rxbcn_nan_rpt_ok_en(&self) -> BitFsRxbcnNanRptOkEnR {
        BitFsRxbcnNanRptOkEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - the interrupt from the exact TBTT time for P0 MBSSID1."]
    #[inline(always)]
    pub fn bit_fs_tbtt0_mb1int_en(&mut self) -> BitFsTbtt0Mb1intEnW<'_, RegFe2imrSpec> {
        BitFsTbtt0Mb1intEnW::new(self, 1)
    }
    #[doc = "Bit 2 - the interrupt from the exact TBTT time for P0 MBSSID2."]
    #[inline(always)]
    pub fn bit_fs_tbtt0_mb2int_en(&mut self) -> BitFsTbtt0Mb2intEnW<'_, RegFe2imrSpec> {
        BitFsTbtt0Mb2intEnW::new(self, 2)
    }
    #[doc = "Bit 3 - the interrupt from the exact TBTT time for P0 MBSSID3."]
    #[inline(always)]
    pub fn bit_fs_tbtt0_mb3int_en(&mut self) -> BitFsTbtt0Mb3intEnW<'_, RegFe2imrSpec> {
        BitFsTbtt0Mb3intEnW::new(self, 3)
    }
    #[doc = "Bit 4 - the interrupt from the exact TBTT time for P0 MBSSID4."]
    #[inline(always)]
    pub fn bit_fs_tbtt0_mb4int_en(&mut self) -> BitFsTbtt0Mb4intEnW<'_, RegFe2imrSpec> {
        BitFsTbtt0Mb4intEnW::new(self, 4)
    }
    #[doc = "Bit 5 - the interrupt from the exact TBTT time for P0 MBSSID5."]
    #[inline(always)]
    pub fn bit_fs_tbtt0_mb5int_en(&mut self) -> BitFsTbtt0Mb5intEnW<'_, RegFe2imrSpec> {
        BitFsTbtt0Mb5intEnW::new(self, 5)
    }
    #[doc = "Bit 6 - the interrupt from the exact TBTT time for P0 MBSSID6."]
    #[inline(always)]
    pub fn bit_fs_tbtt0_mb6int_en(&mut self) -> BitFsTbtt0Mb6intEnW<'_, RegFe2imrSpec> {
        BitFsTbtt0Mb6intEnW::new(self, 6)
    }
    #[doc = "Bit 7 - the interrupt from the exact TBTT time for P0 MBSSID7."]
    #[inline(always)]
    pub fn bit_fs_tbtt0_mb7int_en(&mut self) -> BitFsTbtt0Mb7intEnW<'_, RegFe2imrSpec> {
        BitFsTbtt0Mb7intEnW::new(self, 7)
    }
    #[doc = "Bit 8 - the interrupt from the exact TBTT time for P1."]
    #[inline(always)]
    pub fn bit_fs_tbtt1int_en(&mut self) -> BitFsTbtt1intEnW<'_, RegFe2imrSpec> {
        BitFsTbtt1intEnW::new(self, 8)
    }
    #[doc = "Bit 9 - the interrupt from the exact TBTT time for P2."]
    #[inline(always)]
    pub fn bit_fs_tbtt2_disc_int_en(&mut self) -> BitFsTbtt2DiscIntEnW<'_, RegFe2imrSpec> {
        BitFsTbtt2DiscIntEnW::new(self, 9)
    }
    #[doc = "Bit 10 - the interrupt from the exact TBTT time for P2 SYNC_2G."]
    #[inline(always)]
    pub fn bit_fs_tbtt2_sync_2g_int_en(&mut self) -> BitFsTbtt2Sync2gIntEnW<'_, RegFe2imrSpec> {
        BitFsTbtt2Sync2gIntEnW::new(self, 10)
    }
    #[doc = "Bit 11 - the interrupt from the exact TBTT time for P2 SYNC_5G."]
    #[inline(always)]
    pub fn bit_fs_tbtt2_sync_5g_int_en(&mut self) -> BitFsTbtt2Sync5gIntEnW<'_, RegFe2imrSpec> {
        BitFsTbtt2Sync5gIntEnW::new(self, 11)
    }
    #[doc = "Bit 12 - the interrupt from the exact TBTT time for all port."]
    #[inline(always)]
    pub fn bit_fs_tbttall_int_en(&mut self) -> BitFsTbttallIntEnW<'_, RegFe2imrSpec> {
        BitFsTbttallIntEnW::new(self, 12)
    }
    #[doc = "Bit 13 - When BCNDMA interval arrives before TBTT2_DISC, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma2_int_en(&mut self) -> BitFsBcndma2IntEnW<'_, RegFe2imrSpec> {
        BitFsBcndma2IntEnW::new(self, 13)
    }
    #[doc = "Bit 14 - When BCNDMA interval arrives before TBTT2_2G_SYNC, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma_p2_2g_int_en(&mut self) -> BitFsBcndmaP2_2gIntEnW<'_, RegFe2imrSpec> {
        BitFsBcndmaP2_2gIntEnW::new(self, 14)
    }
    #[doc = "Bit 15 - When BCNDMA interval arrives before TBTT2_5G_SYNC, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_fs_bcndma_p2_5g_int_en(&mut self) -> BitFsBcndmaP2_5gIntEnW<'_, RegFe2imrSpec> {
        BitFsBcndmaP2_5gIntEnW::new(self, 15)
    }
    #[doc = "Bit 24 - PORT2 TX DISC beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_port2_int_en(&mut self) -> BitFsTxbcnerrPort2IntEnW<'_, RegFe2imrSpec> {
        BitFsTxbcnerrPort2IntEnW::new(self, 24)
    }
    #[doc = "Bit 25 - PORT2 TX SYNC beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnerr_port2_sync_int_en(
        &mut self,
    ) -> BitFsTxbcnerrPort2SyncIntEnW<'_, RegFe2imrSpec> {
        BitFsTxbcnerrPort2SyncIntEnW::new(self, 25)
    }
    #[doc = "Bit 26 - PORT2 TX DISC beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_port2_int_en(&mut self) -> BitFsTxbcnokPort2IntEnW<'_, RegFe2imrSpec> {
        BitFsTxbcnokPort2IntEnW::new(self, 26)
    }
    #[doc = "Bit 27 - PORT2 TX SYNC beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_fs_txbcnok_port2_sync_int_en(
        &mut self,
    ) -> BitFsTxbcnokPort2SyncIntEnW<'_, RegFe2imrSpec> {
        BitFsTxbcnokPort2SyncIntEnW::new(self, 27)
    }
    #[doc = "Bit 28 - discovery window end interrupt for NAN"]
    #[inline(always)]
    pub fn bit_fs_dwwin_end_int_en(&mut self) -> BitFsDwwinEndIntEnW<'_, RegFe2imrSpec> {
        BitFsDwwinEndIntEnW::new(self, 28)
    }
    #[doc = "Bit 29 - event queue start interrupt for NAN"]
    #[inline(always)]
    pub fn bit_fs_evtq_start_int_en(&mut self) -> BitFsEvtqStartIntEnW<'_, RegFe2imrSpec> {
        BitFsEvtqStartIntEnW::new(self, 29)
    }
    #[doc = "Bit 30 - event queue done interrupt for NAN"]
    #[inline(always)]
    pub fn bit_fs_evtq_txdone_int_en(&mut self) -> BitFsEvtqTxdoneIntEnW<'_, RegFe2imrSpec> {
        BitFsEvtqTxdoneIntEnW::new(self, 30)
    }
    #[doc = "Bit 31 - port2 rxbcn rpt and write to txrptbuf ok"]
    #[inline(always)]
    pub fn bit_fs_rxbcn_nan_rpt_ok_en(&mut self) -> BitFsRxbcnNanRptOkEnW<'_, RegFe2imrSpec> {
        BitFsRxbcnNanRptOkEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fe2imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fe2imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFe2imrSpec;
impl crate::RegisterSpec for RegFe2imrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fe2imr::R`](R) reader structure"]
impl crate::Readable for RegFe2imrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_fe2imr::W`](W) writer structure"]
impl crate::Writable for RegFe2imrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FE2IMR to value 0"]
impl crate::Resettable for RegFe2imrSpec {}
