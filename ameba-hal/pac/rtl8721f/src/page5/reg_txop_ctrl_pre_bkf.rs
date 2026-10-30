#[doc = "Register `REG_TXOP_CTRL_PRE_BKF` reader"]
pub type R = crate::R<RegTxopCtrlPreBkfSpec>;
#[doc = "Register `REG_TXOP_CTRL_PRE_BKF` writer"]
pub type W = crate::W<RegTxopCtrlPreBkfSpec>;
#[doc = "Field `BIT_TXOP_MIN` reader - if txop counter is more than this value, than tx pkt is considered not cross tbtt"]
pub type BitTxopMinR = crate::FieldReader<u16>;
#[doc = "Field `BIT_TXOP_MIN` writer - if txop counter is more than this value, than tx pkt is considered not cross tbtt"]
pub type BitTxopMinW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `BIT_NAV_BLK_MGQ` reader - rxnav block MGQ"]
pub type BitNavBlkMgqR = crate::BitReader;
#[doc = "Field `BIT_NAV_BLK_MGQ` writer - rxnav block MGQ"]
pub type BitNavBlkMgqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_NAV_BLK_HGQ` reader - rxnav block HGQ"]
pub type BitNavBlkHgqR = crate::BitReader;
#[doc = "Field `BIT_NAV_BLK_HGQ` writer - rxnav block HGQ"]
pub type BitNavBlkHgqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PRE_BKF_TIME` reader - when bkf timer count down this value, sel queue and gen txcmd will start"]
pub type BitPreBkfTimeR = crate::FieldReader;
#[doc = "Field `BIT_PRE_BKF_TIME` writer - when bkf timer count down this value, sel queue and gen txcmd will start"]
pub type BitPreBkfTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_NOT_CROSS_TXOP` reader - 1:tx pkt cross tbtt is prohibited 0:tx pkt cross tbtt is permitted"]
pub type BitNotCrossTxopR = crate::BitReader;
#[doc = "Field `BIT_NOT_CROSS_TXOP` writer - 1:tx pkt cross tbtt is prohibited 0:tx pkt cross tbtt is permitted"]
pub type BitNotCrossTxopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RTS_NAV_TXOP` reader - enable RTS NAV protect all txop"]
pub type BitRtsNavTxopR = crate::BitReader;
#[doc = "Field `BIT_RTS_NAV_TXOP` writer - enable RTS NAV protect all txop"]
pub type BitRtsNavTxopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DTIM_BYPASS` reader - 1: dtim counter will keep 0"]
pub type BitDtimBypassR = crate::BitReader;
#[doc = "Field `BIT_DTIM_BYPASS` writer - 1: dtim counter will keep 0"]
pub type BitDtimBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PRETX_US` reader - For wep/tkip, enable security engine earlier than mactx by this amount of time. Unit is microsecond (us)"]
pub type BitPretxUsR = crate::FieldReader;
#[doc = "Field `BIT_PRETX_US` writer - For wep/tkip, enable security engine earlier than mactx by this amount of time. Unit is microsecond (us)"]
pub type BitPretxUsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_NOPKT_END_RTSMF` reader - 1: enable exit rts hold mode when there is no corresponding ac pkt to tx"]
pub type BitNopktEndRtsmfR = crate::BitReader;
#[doc = "Field `BIT_NOPKT_END_RTSMF` writer - 1: enable exit rts hold mode when there is no corresponding ac pkt to tx"]
pub type BitNopktEndRtsmfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - if txop counter is more than this value, than tx pkt is considered not cross tbtt"]
    #[inline(always)]
    pub fn bit_txop_min(&self) -> BitTxopMinR {
        BitTxopMinR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 14 - rxnav block MGQ"]
    #[inline(always)]
    pub fn bit_nav_blk_mgq(&self) -> BitNavBlkMgqR {
        BitNavBlkMgqR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - rxnav block HGQ"]
    #[inline(always)]
    pub fn bit_nav_blk_hgq(&self) -> BitNavBlkHgqR {
        BitNavBlkHgqR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - when bkf timer count down this value, sel queue and gen txcmd will start"]
    #[inline(always)]
    pub fn bit_pre_bkf_time(&self) -> BitPreBkfTimeR {
        BitPreBkfTimeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 1:tx pkt cross tbtt is prohibited 0:tx pkt cross tbtt is permitted"]
    #[inline(always)]
    pub fn bit_not_cross_txop(&self) -> BitNotCrossTxopR {
        BitNotCrossTxopR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - enable RTS NAV protect all txop"]
    #[inline(always)]
    pub fn bit_rts_nav_txop(&self) -> BitRtsNavTxopR {
        BitRtsNavTxopR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: dtim counter will keep 0"]
    #[inline(always)]
    pub fn bit_dtim_bypass(&self) -> BitDtimBypassR {
        BitDtimBypassR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:30 - For wep/tkip, enable security engine earlier than mactx by this amount of time. Unit is microsecond (us)"]
    #[inline(always)]
    pub fn bit_pretx_us(&self) -> BitPretxUsR {
        BitPretxUsR::new(((self.bits >> 27) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - 1: enable exit rts hold mode when there is no corresponding ac pkt to tx"]
    #[inline(always)]
    pub fn bit_nopkt_end_rtsmf(&self) -> BitNopktEndRtsmfR {
        BitNopktEndRtsmfR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - if txop counter is more than this value, than tx pkt is considered not cross tbtt"]
    #[inline(always)]
    pub fn bit_txop_min(&mut self) -> BitTxopMinW<'_, RegTxopCtrlPreBkfSpec> {
        BitTxopMinW::new(self, 0)
    }
    #[doc = "Bit 14 - rxnav block MGQ"]
    #[inline(always)]
    pub fn bit_nav_blk_mgq(&mut self) -> BitNavBlkMgqW<'_, RegTxopCtrlPreBkfSpec> {
        BitNavBlkMgqW::new(self, 14)
    }
    #[doc = "Bit 15 - rxnav block HGQ"]
    #[inline(always)]
    pub fn bit_nav_blk_hgq(&mut self) -> BitNavBlkHgqW<'_, RegTxopCtrlPreBkfSpec> {
        BitNavBlkHgqW::new(self, 15)
    }
    #[doc = "Bits 16:23 - when bkf timer count down this value, sel queue and gen txcmd will start"]
    #[inline(always)]
    pub fn bit_pre_bkf_time(&mut self) -> BitPreBkfTimeW<'_, RegTxopCtrlPreBkfSpec> {
        BitPreBkfTimeW::new(self, 16)
    }
    #[doc = "Bit 24 - 1:tx pkt cross tbtt is prohibited 0:tx pkt cross tbtt is permitted"]
    #[inline(always)]
    pub fn bit_not_cross_txop(&mut self) -> BitNotCrossTxopW<'_, RegTxopCtrlPreBkfSpec> {
        BitNotCrossTxopW::new(self, 24)
    }
    #[doc = "Bit 25 - enable RTS NAV protect all txop"]
    #[inline(always)]
    pub fn bit_rts_nav_txop(&mut self) -> BitRtsNavTxopW<'_, RegTxopCtrlPreBkfSpec> {
        BitRtsNavTxopW::new(self, 25)
    }
    #[doc = "Bit 26 - 1: dtim counter will keep 0"]
    #[inline(always)]
    pub fn bit_dtim_bypass(&mut self) -> BitDtimBypassW<'_, RegTxopCtrlPreBkfSpec> {
        BitDtimBypassW::new(self, 26)
    }
    #[doc = "Bits 27:30 - For wep/tkip, enable security engine earlier than mactx by this amount of time. Unit is microsecond (us)"]
    #[inline(always)]
    pub fn bit_pretx_us(&mut self) -> BitPretxUsW<'_, RegTxopCtrlPreBkfSpec> {
        BitPretxUsW::new(self, 27)
    }
    #[doc = "Bit 31 - 1: enable exit rts hold mode when there is no corresponding ac pkt to tx"]
    #[inline(always)]
    pub fn bit_nopkt_end_rtsmf(&mut self) -> BitNopktEndRtsmfW<'_, RegTxopCtrlPreBkfSpec> {
        BitNopktEndRtsmfW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_txop_ctrl_pre_bkf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_txop_ctrl_pre_bkf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTxopCtrlPreBkfSpec;
impl crate::RegisterSpec for RegTxopCtrlPreBkfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_txop_ctrl_pre_bkf::R`](R) reader structure"]
impl crate::Readable for RegTxopCtrlPreBkfSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_txop_ctrl_pre_bkf::W`](W) writer structure"]
impl crate::Writable for RegTxopCtrlPreBkfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TXOP_CTRL_PRE_BKF to value 0xb00f_000f"]
impl crate::Resettable for RegTxopCtrlPreBkfSpec {
    const RESET_VALUE: u32 = 0xb00f_000f;
}
