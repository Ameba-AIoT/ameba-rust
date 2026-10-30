#[doc = "Register `REG_FE3IMR` reader"]
pub type R = crate::R<RegFe3imrSpec>;
#[doc = "Register `REG_FE3IMR` writer"]
pub type W = crate::W<RegFe3imrSpec>;
#[doc = "Field `BIT_FS_BCNERLY0_INT_EN` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB0 time."]
pub type BitFsBcnerly0IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNERLY0_INT_EN` writer - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB0 time."]
pub type BitFsBcnerly0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNERLY0_MB1INT_EN` reader - This interrupt is issued at the time set by DRVERLYINTregister before TBTT0_MB1 time."]
pub type BitFsBcnerly0Mb1intEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNERLY0_MB1INT_EN` writer - This interrupt is issued at the time set by DRVERLYINTregister before TBTT0_MB1 time."]
pub type BitFsBcnerly0Mb1intEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNERLY0_MB2INT_EN` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB2 time."]
pub type BitFsBcnerly0Mb2intEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNERLY0_MB2INT_EN` writer - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB2 time."]
pub type BitFsBcnerly0Mb2intEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNERLY0_MB3INT_EN` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB3 time."]
pub type BitFsBcnerly0Mb3intEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNERLY0_MB3INT_EN` writer - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB3 time."]
pub type BitFsBcnerly0Mb3intEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNERLY0_MB4INT_EN` reader - This interrupt is issued at the time set by DRVERLYINTregister before TBTT0_MB4 time."]
pub type BitFsBcnerly0Mb4intEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNERLY0_MB4INT_EN` writer - This interrupt is issued at the time set by DRVERLYINTregister before TBTT0_MB4 time."]
pub type BitFsBcnerly0Mb4intEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNERLY0_MB5INT_EN` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB5 time."]
pub type BitFsBcnerly0Mb5intEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNERLY0_MB5INT_EN` writer - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB5 time."]
pub type BitFsBcnerly0Mb5intEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNERLY0_MB6INT_EN` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB6 time."]
pub type BitFsBcnerly0Mb6intEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNERLY0_MB6INT_EN` writer - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB6 time."]
pub type BitFsBcnerly0Mb6intEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNERLY0_MB7INT_EN` reader - This interrupt is issued at the time set by DRVERLYINTregister before TBTT0_MB7 time."]
pub type BitFsBcnerly0Mb7intEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNERLY0_MB7INT_EN` writer - This interrupt is issued at the time set by DRVERLYINTregister before TBTT0_MB7 time."]
pub type BitFsBcnerly0Mb7intEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNERLY1_INT_EN` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT1 time."]
pub type BitFsBcnerly1IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNERLY1_INT_EN` writer - This interrupt is issued at the time set by DRVERLYINT register before TBTT1 time."]
pub type BitFsBcnerly1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_on_SCK_INT_EN` reader - This interrupt is issued at the time set by ALL DRVERLYINT register before TBTT time."]
pub type BitFsOnSckIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_on_SCK_INT_EN` writer - This interrupt is issued at the time set by ALL DRVERLYINT register before TBTT time."]
pub type BitFsOnSckIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TBTT0_INT_EN` reader - the interrupt from the exact TBTT time for P0 MBSSID0."]
pub type BitFsTbtt0IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TBTT0_INT_EN` writer - the interrupt from the exact TBTT time for P0 MBSSID0."]
pub type BitFsTbtt0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PO_TBTT_AGG_INT_EN` reader - the interrupt from the multiplt TBTT times for P0."]
pub type BitFsPoTbttAggIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PO_TBTT_AGG_INT_EN` writer - the interrupt from the multiplt TBTT times for P0."]
pub type BitFsPoTbttAggIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNERLY2_INT_EN` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 DISC time."]
pub type BitFsBcnerly2IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNERLY2_INT_EN` writer - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 DISC time."]
pub type BitFsBcnerly2IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNERLY2_SYNC_2G_INT_EN` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 SYNC 2.4G time."]
pub type BitFsBcnerly2Sync2gIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNERLY2_SYNC_2G_INT_EN` writer - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 SYNC 2.4G time."]
pub type BitFsBcnerly2Sync2gIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNERLY2_SYNC_5G_INT_EN` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 SYNC 5G time."]
pub type BitFsBcnerly2Sync5gIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNERLY2_SYNC_5G_INT_EN` writer - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 SYNC 5G time."]
pub type BitFsBcnerly2Sync5gIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNELY0_AGGR_INT_EN` reader - port 0 Beacon early aggregation interrupt"]
pub type BitFsBcnely0AggrIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNELY0_AGGR_INT_EN` writer - port 0 Beacon early aggregation interrupt"]
pub type BitFsBcnely0AggrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNELY1_AGGR_INT_EN` reader - port 1 Beacon early aggregation interrupt"]
pub type BitFsBcnely1AggrIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNELY1_AGGR_INT_EN` writer - port 1 Beacon early aggregation interrupt"]
pub type BitFsBcnely1AggrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_BCNELY2_AGGR_INT_EN` reader - port 2 Beacon early aggregation interrupt"]
pub type BitFsBcnely2AggrIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNELY2_AGGR_INT_EN` writer - port 2 Beacon early aggregation interrupt"]
pub type BitFsBcnely2AggrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_6` reader - RSVD"]
pub type Rsvd6R = crate::BitReader;
#[doc = "Field `RSVD_6` writer - RSVD"]
pub type Rsvd6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_5` reader - RSVD"]
pub type Rsvd5R = crate::BitReader;
#[doc = "Field `RSVD_5` writer - RSVD"]
pub type Rsvd5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_4` reader - RSVD"]
pub type Rsvd4R = crate::BitReader;
#[doc = "Field `RSVD_4` writer - RSVD"]
pub type Rsvd4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_3` reader - RSVD"]
pub type Rsvd3R = crate::BitReader;
#[doc = "Field `RSVD_3` writer - RSVD"]
pub type Rsvd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_2` reader - RSVD"]
pub type Rsvd2R = crate::BitReader;
#[doc = "Field `RSVD_2` writer - RSVD"]
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::BitReader;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FISR1_INT_EN` reader - FISR1 Indicator (FISR1 and FIMR1 are true, this bit is set to 1)"]
pub type BitFisr1IntEnR = crate::BitReader;
#[doc = "Field `BIT_FISR1_INT_EN` writer - FISR1 Indicator (FISR1 and FIMR1 are true, this bit is set to 1)"]
pub type BitFisr1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FISR_INT_EN` reader - FISR Indicator (FISR and FIMR are true, this bit is set to 1)"]
pub type BitFisrIntEnR = crate::BitReader;
#[doc = "Field `BIT_FISR_INT_EN` writer - FISR Indicator (FISR and FIMR are true, this bit is set to 1)"]
pub type BitFisrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FE6ISR_INT_EN` reader - FE6ISR Indicator (FE6ISR and FE6IMR are true, this bit is set to 1)"]
pub type BitFe6isrIntEnR = crate::BitReader;
#[doc = "Field `BIT_FE6ISR_INT_EN` writer - FE6ISR Indicator (FE6ISR and FE6IMR are true, this bit is set to 1)"]
pub type BitFe6isrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FE5ISR_INT_EN` reader - FE5ISR Indicator (FE5ISR and FE5IMR are true, this bit is set to 1)"]
pub type BitFe5isrIntEnR = crate::BitReader;
#[doc = "Field `BIT_FE5ISR_INT_EN` writer - FE5ISR Indicator (FE5ISR and FE5IMR are true, this bit is set to 1)"]
pub type BitFe5isrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FE4ISR_INT_EN` reader - FE4ISR Indicator (FE4ISR and FE4IMR are true, this bit is set to 1)"]
pub type BitFe4isrIntEnR = crate::BitReader;
#[doc = "Field `BIT_FE4ISR_INT_EN` writer - FE4ISR Indicator (FE4ISR and FE4IMR are true, this bit is set to 1)"]
pub type BitFe4isrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FE2ISR_INT_EN` reader - FE2ISR Indicator (FE2ISR and FE2IMR are true, this bit is set to 1)"]
pub type BitFe2isrIntEnR = crate::BitReader;
#[doc = "Field `BIT_FE2ISR_INT_EN` writer - FE2ISR Indicator (FE2ISR and FE2IMR are true, this bit is set to 1)"]
pub type BitFe2isrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FE1ISR_INT_EN` reader - FE1ISR Indicator (FE1ISR and FE1IMR are true, this bit is set to 1)"]
pub type BitFe1isrIntEnR = crate::BitReader;
#[doc = "Field `BIT_FE1ISR_INT_EN` writer - FE1ISR Indicator (FE1ISR and FE1IMR are true, this bit is set to 1)"]
pub type BitFe1isrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB0 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly0_int_en(&self) -> BitFsBcnerly0IntEnR {
        BitFsBcnerly0IntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This interrupt is issued at the time set by DRVERLYINTregister before TBTT0_MB1 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly0_mb1int_en(&self) -> BitFsBcnerly0Mb1intEnR {
        BitFsBcnerly0Mb1intEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB2 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly0_mb2int_en(&self) -> BitFsBcnerly0Mb2intEnR {
        BitFsBcnerly0Mb2intEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB3 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly0_mb3int_en(&self) -> BitFsBcnerly0Mb3intEnR {
        BitFsBcnerly0Mb3intEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This interrupt is issued at the time set by DRVERLYINTregister before TBTT0_MB4 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly0_mb4int_en(&self) -> BitFsBcnerly0Mb4intEnR {
        BitFsBcnerly0Mb4intEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB5 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly0_mb5int_en(&self) -> BitFsBcnerly0Mb5intEnR {
        BitFsBcnerly0Mb5intEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB6 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly0_mb6int_en(&self) -> BitFsBcnerly0Mb6intEnR {
        BitFsBcnerly0Mb6intEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This interrupt is issued at the time set by DRVERLYINTregister before TBTT0_MB7 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly0_mb7int_en(&self) -> BitFsBcnerly0Mb7intEnR {
        BitFsBcnerly0Mb7intEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This interrupt is issued at the time set by DRVERLYINT register before TBTT1 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly1_int_en(&self) -> BitFsBcnerly1IntEnR {
        BitFsBcnerly1IntEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This interrupt is issued at the time set by ALL DRVERLYINT register before TBTT time."]
    #[inline(always)]
    pub fn bit_fs_on_sck_int_en(&self) -> BitFsOnSckIntEnR {
        BitFsOnSckIntEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - the interrupt from the exact TBTT time for P0 MBSSID0."]
    #[inline(always)]
    pub fn bit_fs_tbtt0_int_en(&self) -> BitFsTbtt0IntEnR {
        BitFsTbtt0IntEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - the interrupt from the multiplt TBTT times for P0."]
    #[inline(always)]
    pub fn bit_fs_po_tbtt_agg_int_en(&self) -> BitFsPoTbttAggIntEnR {
        BitFsPoTbttAggIntEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 DISC time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly2_int_en(&self) -> BitFsBcnerly2IntEnR {
        BitFsBcnerly2IntEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 SYNC 2.4G time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly2_sync_2g_int_en(&self) -> BitFsBcnerly2Sync2gIntEnR {
        BitFsBcnerly2Sync2gIntEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 SYNC 5G time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly2_sync_5g_int_en(&self) -> BitFsBcnerly2Sync5gIntEnR {
        BitFsBcnerly2Sync5gIntEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - port 0 Beacon early aggregation interrupt"]
    #[inline(always)]
    pub fn bit_fs_bcnely0_aggr_int_en(&self) -> BitFsBcnely0AggrIntEnR {
        BitFsBcnely0AggrIntEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - port 1 Beacon early aggregation interrupt"]
    #[inline(always)]
    pub fn bit_fs_bcnely1_aggr_int_en(&self) -> BitFsBcnely1AggrIntEnR {
        BitFsBcnely1AggrIntEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - port 2 Beacon early aggregation interrupt"]
    #[inline(always)]
    pub fn bit_fs_bcnely2_aggr_int_en(&self) -> BitFsBcnely2AggrIntEnR {
        BitFsBcnely2AggrIntEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RSVD"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> Rsvd6R {
        Rsvd6R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RSVD"]
    #[inline(always)]
    pub fn rsvd_5(&self) -> Rsvd5R {
        Rsvd5R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - RSVD"]
    #[inline(always)]
    pub fn rsvd_4(&self) -> Rsvd4R {
        Rsvd4R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - RSVD"]
    #[inline(always)]
    pub fn rsvd_3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - FISR1 Indicator (FISR1 and FIMR1 are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_fisr1_int_en(&self) -> BitFisr1IntEnR {
        BitFisr1IntEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - FISR Indicator (FISR and FIMR are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_fisr_int_en(&self) -> BitFisrIntEnR {
        BitFisrIntEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - FE6ISR Indicator (FE6ISR and FE6IMR are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_fe6isr_int_en(&self) -> BitFe6isrIntEnR {
        BitFe6isrIntEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - FE5ISR Indicator (FE5ISR and FE5IMR are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_fe5isr_int_en(&self) -> BitFe5isrIntEnR {
        BitFe5isrIntEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - FE4ISR Indicator (FE4ISR and FE4IMR are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_fe4isr_int_en(&self) -> BitFe4isrIntEnR {
        BitFe4isrIntEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - FE2ISR Indicator (FE2ISR and FE2IMR are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_fe2isr_int_en(&self) -> BitFe2isrIntEnR {
        BitFe2isrIntEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - FE1ISR Indicator (FE1ISR and FE1IMR are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_fe1isr_int_en(&self) -> BitFe1isrIntEnR {
        BitFe1isrIntEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB0 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly0_int_en(&mut self) -> BitFsBcnerly0IntEnW<'_, RegFe3imrSpec> {
        BitFsBcnerly0IntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - This interrupt is issued at the time set by DRVERLYINTregister before TBTT0_MB1 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly0_mb1int_en(&mut self) -> BitFsBcnerly0Mb1intEnW<'_, RegFe3imrSpec> {
        BitFsBcnerly0Mb1intEnW::new(self, 1)
    }
    #[doc = "Bit 2 - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB2 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly0_mb2int_en(&mut self) -> BitFsBcnerly0Mb2intEnW<'_, RegFe3imrSpec> {
        BitFsBcnerly0Mb2intEnW::new(self, 2)
    }
    #[doc = "Bit 3 - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB3 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly0_mb3int_en(&mut self) -> BitFsBcnerly0Mb3intEnW<'_, RegFe3imrSpec> {
        BitFsBcnerly0Mb3intEnW::new(self, 3)
    }
    #[doc = "Bit 4 - This interrupt is issued at the time set by DRVERLYINTregister before TBTT0_MB4 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly0_mb4int_en(&mut self) -> BitFsBcnerly0Mb4intEnW<'_, RegFe3imrSpec> {
        BitFsBcnerly0Mb4intEnW::new(self, 4)
    }
    #[doc = "Bit 5 - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB5 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly0_mb5int_en(&mut self) -> BitFsBcnerly0Mb5intEnW<'_, RegFe3imrSpec> {
        BitFsBcnerly0Mb5intEnW::new(self, 5)
    }
    #[doc = "Bit 6 - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB6 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly0_mb6int_en(&mut self) -> BitFsBcnerly0Mb6intEnW<'_, RegFe3imrSpec> {
        BitFsBcnerly0Mb6intEnW::new(self, 6)
    }
    #[doc = "Bit 7 - This interrupt is issued at the time set by DRVERLYINTregister before TBTT0_MB7 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly0_mb7int_en(&mut self) -> BitFsBcnerly0Mb7intEnW<'_, RegFe3imrSpec> {
        BitFsBcnerly0Mb7intEnW::new(self, 7)
    }
    #[doc = "Bit 8 - This interrupt is issued at the time set by DRVERLYINT register before TBTT1 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly1_int_en(&mut self) -> BitFsBcnerly1IntEnW<'_, RegFe3imrSpec> {
        BitFsBcnerly1IntEnW::new(self, 8)
    }
    #[doc = "Bit 9 - This interrupt is issued at the time set by ALL DRVERLYINT register before TBTT time."]
    #[inline(always)]
    pub fn bit_fs_on_sck_int_en(&mut self) -> BitFsOnSckIntEnW<'_, RegFe3imrSpec> {
        BitFsOnSckIntEnW::new(self, 9)
    }
    #[doc = "Bit 10 - the interrupt from the exact TBTT time for P0 MBSSID0."]
    #[inline(always)]
    pub fn bit_fs_tbtt0_int_en(&mut self) -> BitFsTbtt0IntEnW<'_, RegFe3imrSpec> {
        BitFsTbtt0IntEnW::new(self, 10)
    }
    #[doc = "Bit 11 - the interrupt from the multiplt TBTT times for P0."]
    #[inline(always)]
    pub fn bit_fs_po_tbtt_agg_int_en(&mut self) -> BitFsPoTbttAggIntEnW<'_, RegFe3imrSpec> {
        BitFsPoTbttAggIntEnW::new(self, 11)
    }
    #[doc = "Bit 12 - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 DISC time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly2_int_en(&mut self) -> BitFsBcnerly2IntEnW<'_, RegFe3imrSpec> {
        BitFsBcnerly2IntEnW::new(self, 12)
    }
    #[doc = "Bit 13 - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 SYNC 2.4G time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly2_sync_2g_int_en(
        &mut self,
    ) -> BitFsBcnerly2Sync2gIntEnW<'_, RegFe3imrSpec> {
        BitFsBcnerly2Sync2gIntEnW::new(self, 13)
    }
    #[doc = "Bit 14 - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 SYNC 5G time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly2_sync_5g_int_en(
        &mut self,
    ) -> BitFsBcnerly2Sync5gIntEnW<'_, RegFe3imrSpec> {
        BitFsBcnerly2Sync5gIntEnW::new(self, 14)
    }
    #[doc = "Bit 15 - port 0 Beacon early aggregation interrupt"]
    #[inline(always)]
    pub fn bit_fs_bcnely0_aggr_int_en(&mut self) -> BitFsBcnely0AggrIntEnW<'_, RegFe3imrSpec> {
        BitFsBcnely0AggrIntEnW::new(self, 15)
    }
    #[doc = "Bit 16 - port 1 Beacon early aggregation interrupt"]
    #[inline(always)]
    pub fn bit_fs_bcnely1_aggr_int_en(&mut self) -> BitFsBcnely1AggrIntEnW<'_, RegFe3imrSpec> {
        BitFsBcnely1AggrIntEnW::new(self, 16)
    }
    #[doc = "Bit 17 - port 2 Beacon early aggregation interrupt"]
    #[inline(always)]
    pub fn bit_fs_bcnely2_aggr_int_en(&mut self) -> BitFsBcnely2AggrIntEnW<'_, RegFe3imrSpec> {
        BitFsBcnely2AggrIntEnW::new(self, 17)
    }
    #[doc = "Bit 18 - RSVD"]
    #[inline(always)]
    pub fn rsvd_6(&mut self) -> Rsvd6W<'_, RegFe3imrSpec> {
        Rsvd6W::new(self, 18)
    }
    #[doc = "Bit 19 - RSVD"]
    #[inline(always)]
    pub fn rsvd_5(&mut self) -> Rsvd5W<'_, RegFe3imrSpec> {
        Rsvd5W::new(self, 19)
    }
    #[doc = "Bit 21 - RSVD"]
    #[inline(always)]
    pub fn rsvd_4(&mut self) -> Rsvd4W<'_, RegFe3imrSpec> {
        Rsvd4W::new(self, 21)
    }
    #[doc = "Bit 22 - RSVD"]
    #[inline(always)]
    pub fn rsvd_3(&mut self) -> Rsvd3W<'_, RegFe3imrSpec> {
        Rsvd3W::new(self, 22)
    }
    #[doc = "Bit 23 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&mut self) -> Rsvd2W<'_, RegFe3imrSpec> {
        Rsvd2W::new(self, 23)
    }
    #[doc = "Bit 24 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegFe3imrSpec> {
        RsvdW::new(self, 24)
    }
    #[doc = "Bit 25 - FISR1 Indicator (FISR1 and FIMR1 are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_fisr1_int_en(&mut self) -> BitFisr1IntEnW<'_, RegFe3imrSpec> {
        BitFisr1IntEnW::new(self, 25)
    }
    #[doc = "Bit 26 - FISR Indicator (FISR and FIMR are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_fisr_int_en(&mut self) -> BitFisrIntEnW<'_, RegFe3imrSpec> {
        BitFisrIntEnW::new(self, 26)
    }
    #[doc = "Bit 27 - FE6ISR Indicator (FE6ISR and FE6IMR are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_fe6isr_int_en(&mut self) -> BitFe6isrIntEnW<'_, RegFe3imrSpec> {
        BitFe6isrIntEnW::new(self, 27)
    }
    #[doc = "Bit 28 - FE5ISR Indicator (FE5ISR and FE5IMR are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_fe5isr_int_en(&mut self) -> BitFe5isrIntEnW<'_, RegFe3imrSpec> {
        BitFe5isrIntEnW::new(self, 28)
    }
    #[doc = "Bit 29 - FE4ISR Indicator (FE4ISR and FE4IMR are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_fe4isr_int_en(&mut self) -> BitFe4isrIntEnW<'_, RegFe3imrSpec> {
        BitFe4isrIntEnW::new(self, 29)
    }
    #[doc = "Bit 30 - FE2ISR Indicator (FE2ISR and FE2IMR are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_fe2isr_int_en(&mut self) -> BitFe2isrIntEnW<'_, RegFe3imrSpec> {
        BitFe2isrIntEnW::new(self, 30)
    }
    #[doc = "Bit 31 - FE1ISR Indicator (FE1ISR and FE1IMR are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_fe1isr_int_en(&mut self) -> BitFe1isrIntEnW<'_, RegFe3imrSpec> {
        BitFe1isrIntEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fe3imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fe3imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFe3imrSpec;
impl crate::RegisterSpec for RegFe3imrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fe3imr::R`](R) reader structure"]
impl crate::Readable for RegFe3imrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_fe3imr::W`](W) writer structure"]
impl crate::Writable for RegFe3imrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FE3IMR to value 0"]
impl crate::Resettable for RegFe3imrSpec {}
