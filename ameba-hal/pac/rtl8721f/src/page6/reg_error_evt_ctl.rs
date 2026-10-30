#[doc = "Register `REG_ERROR_EVT_CTL` reader"]
pub type R = crate::R<RegErrorEvtCtlSpec>;
#[doc = "Register `REG_ERROR_EVT_CTL` writer"]
pub type W = crate::W<RegErrorEvtCtlSpec>;
#[doc = "Field `BIT_MACTX_ERR_0` reader - Indicate TXD FIFO underflow,mpdu fetch zld"]
pub type BitMactxErr0R = crate::BitReader;
#[doc = "Field `BIT_MACTX_ERR_1` reader - Indicate TXD FIFO underflow,ppdu fetch zld"]
pub type BitMactxErr1R = crate::BitReader;
#[doc = "Field `BIT_MACTX_ERR_2` reader - phy_txen is set ,but wait phy_phytxon time out"]
pub type BitMactxErr2R = crate::BitReader;
#[doc = "Field `BIT_MACTX_ERR_3` reader - Indicate PHY fetch plcp before it is ready"]
pub type BitMactxErr3R = crate::BitReader;
#[doc = "Field `BIT_PRETX_ERRHDL_EN` reader - Pre tx error block enable signal"]
pub type BitPretxErrhdlEnR = crate::BitReader;
#[doc = "Field `BIT_PRETX_ERRHDL_EN` writer - Pre tx error block enable signal"]
pub type BitPretxErrhdlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MACRX_ERR_0` reader - Indicate MAC Rx occurs halt event"]
pub type BitMacrxErr0R = crate::BitReader;
#[doc = "Field `BIT_MACRX_ERR_1` reader - Indicate Rx packet length >= 12K bytes"]
pub type BitMacrxErr1R = crate::BitReader;
#[doc = "Field `BIT_MACRX_ERR_2` reader - Indicate MAC Rx occurs internal halt event"]
pub type BitMacrxErr2R = crate::BitReader;
#[doc = "Field `BIT_MACRX_ERR_3` reader - Indicate rxfifo or mac phy interface fifo overflow"]
pub type BitMacrxErr3R = crate::BitReader;
#[doc = "Field `BIT_MACRX_ERR_4` reader - Indicate last MPDU length of AMPDU or PPDU length is error."]
pub type BitMacrxErr4R = crate::BitReader;
#[doc = "Field `BIT_CSI_CHKSUM_ERROR` reader - Indicate MAC Rx csi rpt checksum error"]
pub type BitCsiChksumErrorR = crate::BitReader;
#[doc = "Field `BIT_CSIRPT_LEN_BB_MORE_THAN_MAC` reader - Indicate MAC Rx csi rxd number overflow"]
pub type BitCsirptLenBbMoreThanMacR = crate::BitReader;
#[doc = "Field `BIT_PRI_MASK_TBTT_P2_0` reader - Priority Mask for P2 NAN disc Beacon"]
pub type BitPriMaskTbttP2_0R = crate::BitReader;
#[doc = "Field `BIT_PRI_MASK_TBTT_P2_0` writer - Priority Mask for P2 NAN disc Beacon"]
pub type BitPriMaskTbttP2_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PRI_MASK_TBTT_P2_1` reader - Priority Mask for P2 NAN sync Beacon"]
pub type BitPriMaskTbttP2_1R = crate::BitReader;
#[doc = "Field `BIT_PRI_MASK_TBTT_P2_1` writer - Priority Mask for P2 NAN sync Beacon"]
pub type BitPriMaskTbttP2_1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicate TXD FIFO underflow,mpdu fetch zld"]
    #[inline(always)]
    pub fn bit_mactx_err_0(&self) -> BitMactxErr0R {
        BitMactxErr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicate TXD FIFO underflow,ppdu fetch zld"]
    #[inline(always)]
    pub fn bit_mactx_err_1(&self) -> BitMactxErr1R {
        BitMactxErr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - phy_txen is set ,but wait phy_phytxon time out"]
    #[inline(always)]
    pub fn bit_mactx_err_2(&self) -> BitMactxErr2R {
        BitMactxErr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicate PHY fetch plcp before it is ready"]
    #[inline(always)]
    pub fn bit_mactx_err_3(&self) -> BitMactxErr3R {
        BitMactxErr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 15 - Pre tx error block enable signal"]
    #[inline(always)]
    pub fn bit_pretx_errhdl_en(&self) -> BitPretxErrhdlEnR {
        BitPretxErrhdlEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Indicate MAC Rx occurs halt event"]
    #[inline(always)]
    pub fn bit_macrx_err_0(&self) -> BitMacrxErr0R {
        BitMacrxErr0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Indicate Rx packet length >= 12K bytes"]
    #[inline(always)]
    pub fn bit_macrx_err_1(&self) -> BitMacrxErr1R {
        BitMacrxErr1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Indicate MAC Rx occurs internal halt event"]
    #[inline(always)]
    pub fn bit_macrx_err_2(&self) -> BitMacrxErr2R {
        BitMacrxErr2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Indicate rxfifo or mac phy interface fifo overflow"]
    #[inline(always)]
    pub fn bit_macrx_err_3(&self) -> BitMacrxErr3R {
        BitMacrxErr3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Indicate last MPDU length of AMPDU or PPDU length is error."]
    #[inline(always)]
    pub fn bit_macrx_err_4(&self) -> BitMacrxErr4R {
        BitMacrxErr4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Indicate MAC Rx csi rpt checksum error"]
    #[inline(always)]
    pub fn bit_csi_chksum_error(&self) -> BitCsiChksumErrorR {
        BitCsiChksumErrorR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Indicate MAC Rx csi rxd number overflow"]
    #[inline(always)]
    pub fn bit_csirpt_len_bb_more_than_mac(&self) -> BitCsirptLenBbMoreThanMacR {
        BitCsirptLenBbMoreThanMacR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - Priority Mask for P2 NAN disc Beacon"]
    #[inline(always)]
    pub fn bit_pri_mask_tbtt_p2_0(&self) -> BitPriMaskTbttP2_0R {
        BitPriMaskTbttP2_0R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Priority Mask for P2 NAN sync Beacon"]
    #[inline(always)]
    pub fn bit_pri_mask_tbtt_p2_1(&self) -> BitPriMaskTbttP2_1R {
        BitPriMaskTbttP2_1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Pre tx error block enable signal"]
    #[inline(always)]
    pub fn bit_pretx_errhdl_en(&mut self) -> BitPretxErrhdlEnW<'_, RegErrorEvtCtlSpec> {
        BitPretxErrhdlEnW::new(self, 15)
    }
    #[doc = "Bit 30 - Priority Mask for P2 NAN disc Beacon"]
    #[inline(always)]
    pub fn bit_pri_mask_tbtt_p2_0(&mut self) -> BitPriMaskTbttP2_0W<'_, RegErrorEvtCtlSpec> {
        BitPriMaskTbttP2_0W::new(self, 30)
    }
    #[doc = "Bit 31 - Priority Mask for P2 NAN sync Beacon"]
    #[inline(always)]
    pub fn bit_pri_mask_tbtt_p2_1(&mut self) -> BitPriMaskTbttP2_1W<'_, RegErrorEvtCtlSpec> {
        BitPriMaskTbttP2_1W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_error_evt_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_error_evt_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegErrorEvtCtlSpec;
impl crate::RegisterSpec for RegErrorEvtCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_error_evt_ctl::R`](R) reader structure"]
impl crate::Readable for RegErrorEvtCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_error_evt_ctl::W`](W) writer structure"]
impl crate::Writable for RegErrorEvtCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ERROR_EVT_CTL to value 0"]
impl crate::Resettable for RegErrorEvtCtlSpec {}
