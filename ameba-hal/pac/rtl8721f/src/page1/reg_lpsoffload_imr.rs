#[doc = "Register `REG_LPSOFFLOAD_IMR` reader"]
pub type R = crate::R<RegLpsoffloadImrSpec>;
#[doc = "Register `REG_LPSOFFLOAD_IMR` writer"]
pub type W = crate::W<RegLpsoffloadImrSpec>;
#[doc = "Field `BIT_LPS_RXBCN_TIMEUP_INT_EN` reader - When this bit is enable, the interrupt is issued when RX beacon frame timeout occurs in port0. 0x124\\[25\\]"]
pub type BitLpsRxbcnTimeupIntEnR = crate::BitReader;
#[doc = "Field `BIT_LPS_RXBCN_TIMEUP_INT_EN` writer - When this bit is enable, the interrupt is issued when RX beacon frame timeout occurs in port0. 0x124\\[25\\]"]
pub type BitLpsRxbcnTimeupIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_LPS_RX_BCN_ERR_P0_INT_EN` reader - When this bit is enable, the interrupt is issued when RX beacon frame with fcs error occurs in port0. 0x124\\[23\\]"]
pub type BitLpsRxBcnErrP0IntEnR = crate::BitReader;
#[doc = "Field `BIT_LPS_RX_BCN_ERR_P0_INT_EN` writer - When this bit is enable, the interrupt is issued when RX beacon frame with fcs error occurs in port0. 0x124\\[23\\]"]
pub type BitLpsRxBcnErrP0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_LPS_RX_BCN_P0_INT_EN` reader - When this bit is enable, the interrupt is issued when RX beacon frame occurs in port0. 0x124\\[21\\]"]
pub type BitLpsRxBcnP0IntEnR = crate::BitReader;
#[doc = "Field `BIT_LPS_RX_BCN_P0_INT_EN` writer - When this bit is enable, the interrupt is issued when RX beacon frame occurs in port0. 0x124\\[21\\]"]
pub type BitLpsRxBcnP0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_LPS_TBTT0_INT_EN` reader - the interrupt from the exact TBTT time for P0 MBSSID0. 0x112C\\[10\\]"]
pub type BitLpsTbtt0IntEnR = crate::BitReader;
#[doc = "Field `BIT_LPS_TBTT0_INT_EN` writer - the interrupt from the exact TBTT time for P0 MBSSID0. 0x112C\\[10\\]"]
pub type BitLpsTbtt0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_LPS_BCNELY0_AGGR_INT_EN` reader - port 0 Beacon early aggregation interrupt 0x112C\\[15\\]"]
pub type BitLpsBcnely0AggrIntEnR = crate::BitReader;
#[doc = "Field `BIT_LPS_BCNELY0_AGGR_INT_EN` writer - port 0 Beacon early aggregation interrupt 0x112C\\[15\\]"]
pub type BitLpsBcnely0AggrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_LPS_BCNERLY0_INT_EN` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB0 time. 0x112C\\[0\\]"]
pub type BitLpsBcnerly0IntEnR = crate::BitReader;
#[doc = "Field `BIT_LPS_BCNERLY0_INT_EN` writer - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB0 time. 0x112C\\[0\\]"]
pub type BitLpsBcnerly0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_LPS_TIM_NO_HIT_INT_EN` reader - when no my aid or aid is 0(0x16DC\\[8\\]) or my bssid is not in tim element(0x16DB\\[31\\]),generate this interrupt 0x11C4\\[21\\]"]
pub type BitLpsTimNoHitIntEnR = crate::BitReader;
#[doc = "Field `BIT_LPS_TIM_NO_HIT_INT_EN` writer - when no my aid or aid is 0(0x16DC\\[8\\]) or my bssid is not in tim element(0x16DB\\[31\\]),generate this interrupt 0x11C4\\[21\\]"]
pub type BitLpsTimNoHitIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_LPS_HIOE_CHN_OFF_INT_EN` reader - hioe_chn_off done interrupt 0X11C4\\[15\\]"]
pub type BitLpsHioeChnOffIntEnR = crate::BitReader;
#[doc = "Field `BIT_LPS_HIOE_CHN_OFF_INT_EN` writer - hioe_chn_off done interrupt 0X11C4\\[15\\]"]
pub type BitLpsHioeChnOffIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_LPS_HIOE_CHN_ON_INT_EN` reader - hioe_chn_on done interrupt 0X11C4\\[16\\]"]
pub type BitLpsHioeChnOnIntEnR = crate::BitReader;
#[doc = "Field `BIT_LPS_HIOE_CHN_ON_INT_EN` writer - hioe_chn_on done interrupt 0X11C4\\[16\\]"]
pub type BitLpsHioeChnOnIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_LPS_RXBCN_TSF_GT_THRES_INT_EN` reader - The interrupt is issued when difference of tsft in beacon frame of port0 is greater than threshold 0X124\\[28\\]"]
pub type BitLpsRxbcnTsfGtThresIntEnR = crate::BitReader;
#[doc = "Field `BIT_LPS_RXBCN_TSF_GT_THRES_INT_EN` writer - The interrupt is issued when difference of tsft in beacon frame of port0 is greater than threshold 0X124\\[28\\]"]
pub type BitLpsRxbcnTsfGtThresIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader<u32>;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `BIT_FS_HIOE_CHN1_INT_EXT_EN` reader - hioe_chn1 done interrupt 0X11C4\\[17\\]"]
pub type BitFsHioeChn1IntExtEnR = crate::BitReader;
#[doc = "Field `BIT_FS_HIOE_CHN1_INT_EXT_EN` writer - hioe_chn1 done interrupt 0X11C4\\[17\\]"]
pub type BitFsHioeChn1IntExtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_HIOE_CHN0_INT_EXT_EN` reader - hioe_chn0 done interrupt 0X11C4\\[18\\]"]
pub type BitFsHioeChn0IntExtEnR = crate::BitReader;
#[doc = "Field `BIT_FS_HIOE_CHN0_INT_EXT_EN` writer - hioe_chn0 done interrupt 0X11C4\\[18\\]"]
pub type BitFsHioeChn0IntExtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_EDCCA_ILDE_TOCNT_INT_EXT_EN` reader - when edcca is low after tbtt, start timer,when timeout for few times(0x180~0x188),set this interrupt 0X11C4\\[7\\]"]
pub type BitFsEdccaIldeTocntIntExtEnR = crate::BitReader;
#[doc = "Field `BIT_FS_EDCCA_ILDE_TOCNT_INT_EXT_EN` writer - when edcca is low after tbtt, start timer,when timeout for few times(0x180~0x188),set this interrupt 0X11C4\\[7\\]"]
pub type BitFsEdccaIldeTocntIntExtEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When this bit is enable, the interrupt is issued when RX beacon frame timeout occurs in port0. 0x124\\[25\\]"]
    #[inline(always)]
    pub fn bit_lps_rxbcn_timeup_int_en(&self) -> BitLpsRxbcnTimeupIntEnR {
        BitLpsRxbcnTimeupIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When this bit is enable, the interrupt is issued when RX beacon frame with fcs error occurs in port0. 0x124\\[23\\]"]
    #[inline(always)]
    pub fn bit_lps_rx_bcn_err_p0_int_en(&self) -> BitLpsRxBcnErrP0IntEnR {
        BitLpsRxBcnErrP0IntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When this bit is enable, the interrupt is issued when RX beacon frame occurs in port0. 0x124\\[21\\]"]
    #[inline(always)]
    pub fn bit_lps_rx_bcn_p0_int_en(&self) -> BitLpsRxBcnP0IntEnR {
        BitLpsRxBcnP0IntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the interrupt from the exact TBTT time for P0 MBSSID0. 0x112C\\[10\\]"]
    #[inline(always)]
    pub fn bit_lps_tbtt0_int_en(&self) -> BitLpsTbtt0IntEnR {
        BitLpsTbtt0IntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - port 0 Beacon early aggregation interrupt 0x112C\\[15\\]"]
    #[inline(always)]
    pub fn bit_lps_bcnely0_aggr_int_en(&self) -> BitLpsBcnely0AggrIntEnR {
        BitLpsBcnely0AggrIntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB0 time. 0x112C\\[0\\]"]
    #[inline(always)]
    pub fn bit_lps_bcnerly0_int_en(&self) -> BitLpsBcnerly0IntEnR {
        BitLpsBcnerly0IntEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - when no my aid or aid is 0(0x16DC\\[8\\]) or my bssid is not in tim element(0x16DB\\[31\\]),generate this interrupt 0x11C4\\[21\\]"]
    #[inline(always)]
    pub fn bit_lps_tim_no_hit_int_en(&self) -> BitLpsTimNoHitIntEnR {
        BitLpsTimNoHitIntEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - hioe_chn_off done interrupt 0X11C4\\[15\\]"]
    #[inline(always)]
    pub fn bit_lps_hioe_chn_off_int_en(&self) -> BitLpsHioeChnOffIntEnR {
        BitLpsHioeChnOffIntEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - hioe_chn_on done interrupt 0X11C4\\[16\\]"]
    #[inline(always)]
    pub fn bit_lps_hioe_chn_on_int_en(&self) -> BitLpsHioeChnOnIntEnR {
        BitLpsHioeChnOnIntEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt is issued when difference of tsft in beacon frame of port0 is greater than threshold 0X124\\[28\\]"]
    #[inline(always)]
    pub fn bit_lps_rxbcn_tsf_gt_thres_int_en(&self) -> BitLpsRxbcnTsfGtThresIntEnR {
        BitLpsRxbcnTsfGtThresIntEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 11:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 11) & 0x001f_ffff)
    }
    #[doc = "Bit 12 - hioe_chn1 done interrupt 0X11C4\\[17\\]"]
    #[inline(always)]
    pub fn bit_fs_hioe_chn1_int_ext_en(&self) -> BitFsHioeChn1IntExtEnR {
        BitFsHioeChn1IntExtEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - hioe_chn0 done interrupt 0X11C4\\[18\\]"]
    #[inline(always)]
    pub fn bit_fs_hioe_chn0_int_ext_en(&self) -> BitFsHioeChn0IntExtEnR {
        BitFsHioeChn0IntExtEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - when edcca is low after tbtt, start timer,when timeout for few times(0x180~0x188),set this interrupt 0X11C4\\[7\\]"]
    #[inline(always)]
    pub fn bit_fs_edcca_ilde_tocnt_int_ext_en(&self) -> BitFsEdccaIldeTocntIntExtEnR {
        BitFsEdccaIldeTocntIntExtEnR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When this bit is enable, the interrupt is issued when RX beacon frame timeout occurs in port0. 0x124\\[25\\]"]
    #[inline(always)]
    pub fn bit_lps_rxbcn_timeup_int_en(
        &mut self,
    ) -> BitLpsRxbcnTimeupIntEnW<'_, RegLpsoffloadImrSpec> {
        BitLpsRxbcnTimeupIntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - When this bit is enable, the interrupt is issued when RX beacon frame with fcs error occurs in port0. 0x124\\[23\\]"]
    #[inline(always)]
    pub fn bit_lps_rx_bcn_err_p0_int_en(
        &mut self,
    ) -> BitLpsRxBcnErrP0IntEnW<'_, RegLpsoffloadImrSpec> {
        BitLpsRxBcnErrP0IntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - When this bit is enable, the interrupt is issued when RX beacon frame occurs in port0. 0x124\\[21\\]"]
    #[inline(always)]
    pub fn bit_lps_rx_bcn_p0_int_en(&mut self) -> BitLpsRxBcnP0IntEnW<'_, RegLpsoffloadImrSpec> {
        BitLpsRxBcnP0IntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - the interrupt from the exact TBTT time for P0 MBSSID0. 0x112C\\[10\\]"]
    #[inline(always)]
    pub fn bit_lps_tbtt0_int_en(&mut self) -> BitLpsTbtt0IntEnW<'_, RegLpsoffloadImrSpec> {
        BitLpsTbtt0IntEnW::new(self, 3)
    }
    #[doc = "Bit 4 - port 0 Beacon early aggregation interrupt 0x112C\\[15\\]"]
    #[inline(always)]
    pub fn bit_lps_bcnely0_aggr_int_en(
        &mut self,
    ) -> BitLpsBcnely0AggrIntEnW<'_, RegLpsoffloadImrSpec> {
        BitLpsBcnely0AggrIntEnW::new(self, 4)
    }
    #[doc = "Bit 5 - This interrupt is issued at the time set by DRVERLYINT register before TBTT0_MB0 time. 0x112C\\[0\\]"]
    #[inline(always)]
    pub fn bit_lps_bcnerly0_int_en(&mut self) -> BitLpsBcnerly0IntEnW<'_, RegLpsoffloadImrSpec> {
        BitLpsBcnerly0IntEnW::new(self, 5)
    }
    #[doc = "Bit 6 - when no my aid or aid is 0(0x16DC\\[8\\]) or my bssid is not in tim element(0x16DB\\[31\\]),generate this interrupt 0x11C4\\[21\\]"]
    #[inline(always)]
    pub fn bit_lps_tim_no_hit_int_en(&mut self) -> BitLpsTimNoHitIntEnW<'_, RegLpsoffloadImrSpec> {
        BitLpsTimNoHitIntEnW::new(self, 6)
    }
    #[doc = "Bit 7 - hioe_chn_off done interrupt 0X11C4\\[15\\]"]
    #[inline(always)]
    pub fn bit_lps_hioe_chn_off_int_en(
        &mut self,
    ) -> BitLpsHioeChnOffIntEnW<'_, RegLpsoffloadImrSpec> {
        BitLpsHioeChnOffIntEnW::new(self, 7)
    }
    #[doc = "Bit 8 - hioe_chn_on done interrupt 0X11C4\\[16\\]"]
    #[inline(always)]
    pub fn bit_lps_hioe_chn_on_int_en(
        &mut self,
    ) -> BitLpsHioeChnOnIntEnW<'_, RegLpsoffloadImrSpec> {
        BitLpsHioeChnOnIntEnW::new(self, 8)
    }
    #[doc = "Bit 9 - The interrupt is issued when difference of tsft in beacon frame of port0 is greater than threshold 0X124\\[28\\]"]
    #[inline(always)]
    pub fn bit_lps_rxbcn_tsf_gt_thres_int_en(
        &mut self,
    ) -> BitLpsRxbcnTsfGtThresIntEnW<'_, RegLpsoffloadImrSpec> {
        BitLpsRxbcnTsfGtThresIntEnW::new(self, 9)
    }
    #[doc = "Bits 11:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegLpsoffloadImrSpec> {
        RsvdW::new(self, 11)
    }
    #[doc = "Bit 12 - hioe_chn1 done interrupt 0X11C4\\[17\\]"]
    #[inline(always)]
    pub fn bit_fs_hioe_chn1_int_ext_en(
        &mut self,
    ) -> BitFsHioeChn1IntExtEnW<'_, RegLpsoffloadImrSpec> {
        BitFsHioeChn1IntExtEnW::new(self, 12)
    }
    #[doc = "Bit 13 - hioe_chn0 done interrupt 0X11C4\\[18\\]"]
    #[inline(always)]
    pub fn bit_fs_hioe_chn0_int_ext_en(
        &mut self,
    ) -> BitFsHioeChn0IntExtEnW<'_, RegLpsoffloadImrSpec> {
        BitFsHioeChn0IntExtEnW::new(self, 13)
    }
    #[doc = "Bit 14 - when edcca is low after tbtt, start timer,when timeout for few times(0x180~0x188),set this interrupt 0X11C4\\[7\\]"]
    #[inline(always)]
    pub fn bit_fs_edcca_ilde_tocnt_int_ext_en(
        &mut self,
    ) -> BitFsEdccaIldeTocntIntExtEnW<'_, RegLpsoffloadImrSpec> {
        BitFsEdccaIldeTocntIntExtEnW::new(self, 14)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_lpsoffload_imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_lpsoffload_imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegLpsoffloadImrSpec;
impl crate::RegisterSpec for RegLpsoffloadImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_lpsoffload_imr::R`](R) reader structure"]
impl crate::Readable for RegLpsoffloadImrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_lpsoffload_imr::W`](W) writer structure"]
impl crate::Writable for RegLpsoffloadImrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_LPSOFFLOAD_IMR to value 0"]
impl crate::Resettable for RegLpsoffloadImrSpec {}
