#[doc = "Register `REG_FE3ISR` reader"]
pub type R = crate::R<RegFe3isrSpec>;
#[doc = "Field `BIT_FS_BCNERLY0_INT` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB0 time."]
pub type BitFsBcnerly0IntR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNERLY0_MB1INT` reader - This interrupt is issued at the time set by DRVERLYINTregister before TBTT0_MB1 time."]
pub type BitFsBcnerly0Mb1intR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNERLY0_MB2INT` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB2 time."]
pub type BitFsBcnerly0Mb2intR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNERLY0_MB3INT` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB3 time."]
pub type BitFsBcnerly0Mb3intR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNERLY0_MB4INT` reader - This interrupt is issued at the time set by DRVERLYINTregister before TBTT0_MB4 time."]
pub type BitFsBcnerly0Mb4intR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNERLY0_MB5INT` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB5 time."]
pub type BitFsBcnerly0Mb5intR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNERLY0_MB6INT` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB6 time."]
pub type BitFsBcnerly0Mb6intR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNERLY0_MB7INT` reader - This interrupt is issued at the time set by DRVERLYINTregister before TBTT0_MB7 time."]
pub type BitFsBcnerly0Mb7intR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNERLY1_INT` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT1 time."]
pub type BitFsBcnerly1IntR = crate::BitReader;
#[doc = "Field `BIT_FS_on_SCK_INT` reader - This interrupt is issued at the time set by ALL DRVERLYINT register before TBTT time."]
pub type BitFsOnSckIntR = crate::BitReader;
#[doc = "Field `BIT_FS_TBTT0_INT` reader - the interrupt from the exact TBTT time for P0 MBSSID0."]
pub type BitFsTbtt0IntR = crate::BitReader;
#[doc = "Field `BIT_FS_PO_TBTT_AGG_INT` reader - the interrupt from the multiplt TBTT times for P0."]
pub type BitFsPoTbttAggIntR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNERLY2_INT` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 DISC time."]
pub type BitFsBcnerly2IntR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNERLY2_SYNC_2G_INT` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 SYNC 2.4G time."]
pub type BitFsBcnerly2Sync2gIntR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNERLY2_SYNC_5G_INT` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 SYNC 5G time."]
pub type BitFsBcnerly2Sync5gIntR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNELY0_AGGR_INT` reader - port 0 Beacon early aggregation interrupt"]
pub type BitFsBcnely0AggrIntR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNELY1_AGGR_INT` reader - port 1 Beacon early aggregation interrupt"]
pub type BitFsBcnely1AggrIntR = crate::BitReader;
#[doc = "Field `BIT_FS_BCNELY2_AGGR_INT` reader - port 2 Beacon early aggregation interrupt"]
pub type BitFsBcnely2AggrIntR = crate::BitReader;
#[doc = "Field `RSVD_7` reader - RSVD"]
pub type Rsvd7R = crate::BitReader;
#[doc = "Field `RSVD_6` reader - RSVD"]
pub type Rsvd6R = crate::BitReader;
#[doc = "Field `RSVD_5` reader - RSVD"]
pub type Rsvd5R = crate::BitReader;
#[doc = "Field `RSVD_4` reader - RSVD"]
pub type Rsvd4R = crate::BitReader;
#[doc = "Field `RSVD_3` reader - RSVD"]
pub type Rsvd3R = crate::BitReader;
#[doc = "Field `RSVD_2` reader - RSVD"]
pub type Rsvd2R = crate::BitReader;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::BitReader;
#[doc = "Field `BIT_FISR1_INT` reader - FISR1 Indicator (FISR1 and FIMR1 are true, this bit is set to 1)"]
pub type BitFisr1IntR = crate::BitReader;
#[doc = "Field `BIT_FISR_INT` reader - FISR Indicator (FISR and FIMR are true, this bit is set to 1)"]
pub type BitFisrIntR = crate::BitReader;
#[doc = "Field `BIT_FE6ISR_INT` reader - FE6ISR Indicator (FE6ISR and FE6IMR are true, this bit is set to 1)"]
pub type BitFe6isrIntR = crate::BitReader;
#[doc = "Field `BIT_FE5ISR_INT` reader - FE5ISR Indicator (FE5ISR and FE5IMR are true, this bit is set to 1)"]
pub type BitFe5isrIntR = crate::BitReader;
#[doc = "Field `BIT_FE4ISR_INT` reader - FE4ISR Indicator (FE4ISR and FE4IMR are true, this bit is set to 1)"]
pub type BitFe4isrIntR = crate::BitReader;
#[doc = "Field `BIT_FE2ISR_INT` reader - FE2ISR Indicator (FE2ISR and FE2IMR are true, this bit is set to 1)"]
pub type BitFe2isrIntR = crate::BitReader;
#[doc = "Field `BIT_FE1ISR_INT` reader - FE1ISR Indicator (FE1ISR and FE1IMR are true, this bit is set to 1)"]
pub type BitFe1isrIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB0 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly0_int(&self) -> BitFsBcnerly0IntR {
        BitFsBcnerly0IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This interrupt is issued at the time set by DRVERLYINTregister before TBTT0_MB1 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly0_mb1int(&self) -> BitFsBcnerly0Mb1intR {
        BitFsBcnerly0Mb1intR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB2 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly0_mb2int(&self) -> BitFsBcnerly0Mb2intR {
        BitFsBcnerly0Mb2intR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB3 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly0_mb3int(&self) -> BitFsBcnerly0Mb3intR {
        BitFsBcnerly0Mb3intR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This interrupt is issued at the time set by DRVERLYINTregister before TBTT0_MB4 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly0_mb4int(&self) -> BitFsBcnerly0Mb4intR {
        BitFsBcnerly0Mb4intR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB5 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly0_mb5int(&self) -> BitFsBcnerly0Mb5intR {
        BitFsBcnerly0Mb5intR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB6 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly0_mb6int(&self) -> BitFsBcnerly0Mb6intR {
        BitFsBcnerly0Mb6intR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This interrupt is issued at the time set by DRVERLYINTregister before TBTT0_MB7 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly0_mb7int(&self) -> BitFsBcnerly0Mb7intR {
        BitFsBcnerly0Mb7intR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This interrupt is issued at the time set by DRVERLYINT register before TBTT1 time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly1_int(&self) -> BitFsBcnerly1IntR {
        BitFsBcnerly1IntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This interrupt is issued at the time set by ALL DRVERLYINT register before TBTT time."]
    #[inline(always)]
    pub fn bit_fs_on_sck_int(&self) -> BitFsOnSckIntR {
        BitFsOnSckIntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - the interrupt from the exact TBTT time for P0 MBSSID0."]
    #[inline(always)]
    pub fn bit_fs_tbtt0_int(&self) -> BitFsTbtt0IntR {
        BitFsTbtt0IntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - the interrupt from the multiplt TBTT times for P0."]
    #[inline(always)]
    pub fn bit_fs_po_tbtt_agg_int(&self) -> BitFsPoTbttAggIntR {
        BitFsPoTbttAggIntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 DISC time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly2_int(&self) -> BitFsBcnerly2IntR {
        BitFsBcnerly2IntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 SYNC 2.4G time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly2_sync_2g_int(&self) -> BitFsBcnerly2Sync2gIntR {
        BitFsBcnerly2Sync2gIntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 SYNC 5G time."]
    #[inline(always)]
    pub fn bit_fs_bcnerly2_sync_5g_int(&self) -> BitFsBcnerly2Sync5gIntR {
        BitFsBcnerly2Sync5gIntR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - port 0 Beacon early aggregation interrupt"]
    #[inline(always)]
    pub fn bit_fs_bcnely0_aggr_int(&self) -> BitFsBcnely0AggrIntR {
        BitFsBcnely0AggrIntR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - port 1 Beacon early aggregation interrupt"]
    #[inline(always)]
    pub fn bit_fs_bcnely1_aggr_int(&self) -> BitFsBcnely1AggrIntR {
        BitFsBcnely1AggrIntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - port 2 Beacon early aggregation interrupt"]
    #[inline(always)]
    pub fn bit_fs_bcnely2_aggr_int(&self) -> BitFsBcnely2AggrIntR {
        BitFsBcnely2AggrIntR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RSVD"]
    #[inline(always)]
    pub fn rsvd_7(&self) -> Rsvd7R {
        Rsvd7R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RSVD"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> Rsvd6R {
        Rsvd6R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - RSVD"]
    #[inline(always)]
    pub fn rsvd_5(&self) -> Rsvd5R {
        Rsvd5R::new(((self.bits >> 20) & 1) != 0)
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
    pub fn bit_fisr1_int(&self) -> BitFisr1IntR {
        BitFisr1IntR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - FISR Indicator (FISR and FIMR are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_fisr_int(&self) -> BitFisrIntR {
        BitFisrIntR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - FE6ISR Indicator (FE6ISR and FE6IMR are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_fe6isr_int(&self) -> BitFe6isrIntR {
        BitFe6isrIntR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - FE5ISR Indicator (FE5ISR and FE5IMR are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_fe5isr_int(&self) -> BitFe5isrIntR {
        BitFe5isrIntR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - FE4ISR Indicator (FE4ISR and FE4IMR are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_fe4isr_int(&self) -> BitFe4isrIntR {
        BitFe4isrIntR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - FE2ISR Indicator (FE2ISR and FE2IMR are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_fe2isr_int(&self) -> BitFe2isrIntR {
        BitFe2isrIntR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - FE1ISR Indicator (FE1ISR and FE1IMR are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_fe1isr_int(&self) -> BitFe1isrIntR {
        BitFe1isrIntR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fe3isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFe3isrSpec;
impl crate::RegisterSpec for RegFe3isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fe3isr::R`](R) reader structure"]
impl crate::Readable for RegFe3isrSpec {}
#[doc = "`reset()` method sets REG_FE3ISR to value 0"]
impl crate::Resettable for RegFe3isrSpec {}
