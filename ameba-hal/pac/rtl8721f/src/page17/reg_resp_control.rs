#[doc = "Register `REG_RESP_CONTROL` reader"]
pub type R = crate::R<RegRespControlSpec>;
#[doc = "Register `REG_RESP_CONTROL` writer"]
pub type W = crate::W<RegRespControlSpec>;
#[doc = "Field `BIT_RESP_TXRATE` reader - tx rate use to decide response rate"]
pub type BitRespTxrateR = crate::FieldReader<u16>;
#[doc = "Field `BIT_RESP_TXRATE` writer - tx rate use to decide response rate"]
pub type BitRespTxrateW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `BIT_RESP_CHK_EDCCA_PER20` reader - check edcca_per20_bitmap_sifs\\[0\\] when response"]
pub type BitRespChkEdccaPer20R = crate::BitReader;
#[doc = "Field `BIT_RESP_CHK_EDCCA_PER20` writer - check edcca_per20_bitmap_sifs\\[0\\] when response"]
pub type BitRespChkEdccaPer20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_CHK_ZBCCA` reader - check zigbee cca when response"]
pub type BitRespChkZbccaR = crate::BitReader;
#[doc = "Field `BIT_RESP_CHK_ZBCCA` writer - check zigbee cca when response"]
pub type BitRespChkZbccaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_GI_LTF` reader - 5:1x+0.8 4:1x+1.6 3:2x+0.8 2:2x+1.6 1:4x+0.8 0:4x+3.2 not support 1x+1.6 when response"]
pub type BitRespGiLtfR = crate::FieldReader;
#[doc = "Field `BIT_RESP_GI_LTF` writer - 5:1x+0.8 4:1x+1.6 3:2x+0.8 2:2x+1.6 1:4x+0.8 0:4x+3.2 not support 1x+1.6 when response"]
pub type BitRespGiLtfW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_RESP_EARLY_LEG_SEC` reader - when receive pkt of tkip or wep, While phyrxon down, mac_rxactive on. Ignore mac rx busy or physts_on, respond immediately."]
pub type BitRespEarlyLegSecR = crate::BitReader;
#[doc = "Field `BIT_RESP_EARLY_LEG_SEC` writer - when receive pkt of tkip or wep, While phyrxon down, mac_rxactive on. Ignore mac rx busy or physts_on, respond immediately."]
pub type BitRespEarlyLegSecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_CHK_SEC_CCA80` reader - check sec 80 cca when response"]
pub type BitRespChkSecCca80R = crate::BitReader;
#[doc = "Field `BIT_RESP_CHK_SEC_CCA80` writer - check sec 80 cca when response"]
pub type BitRespChkSecCca80W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_CHK_SEC_CCA40` reader - check sec 40 cca when response"]
pub type BitRespChkSecCca40R = crate::BitReader;
#[doc = "Field `BIT_RESP_CHK_SEC_CCA40` writer - check sec 40 cca when response"]
pub type BitRespChkSecCca40W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_CHK_SEC_CCA20` reader - check sec 20 cca when response"]
pub type BitRespChkSecCca20R = crate::BitReader;
#[doc = "Field `BIT_RESP_CHK_SEC_CCA20` writer - check sec 20 cca when response"]
pub type BitRespChkSecCca20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_CHK_BTCCA` reader - check btcca when response"]
pub type BitRespChkBtccaR = crate::BitReader;
#[doc = "Field `BIT_RESP_CHK_BTCCA` writer - check btcca when response"]
pub type BitRespChkBtccaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_CHK_EDCCA` reader - check edcca when response"]
pub type BitRespChkEdccaR = crate::BitReader;
#[doc = "Field `BIT_RESP_CHK_EDCCA` writer - check edcca when response"]
pub type BitRespChkEdccaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_RXTRIG_CHK_INSIFS` reader - check whether channel is idle during sifs, if csreq=1 in trigger frame, will not reply with tb ppdu"]
pub type BitRespRxtrigChkInsifsR = crate::BitReader;
#[doc = "Field `BIT_RESP_RXTRIG_CHK_INSIFS` writer - check whether channel is idle during sifs, if csreq=1 in trigger frame, will not reply with tb ppdu"]
pub type BitRespRxtrigChkInsifsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_TBPPDU_CHK_PWR` reader - check tb ppdu tx power when rx trigger, if txpwr_ctrl send signal of abort,will not reply with tb ppdu"]
pub type BitRespTbppduChkPwrR = crate::BitReader;
#[doc = "Field `BIT_RESP_TBPPDU_CHK_PWR` writer - check tb ppdu tx power when rx trigger, if txpwr_ctrl send signal of abort,will not reply with tb ppdu"]
pub type BitRespTbppduChkPwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::BitReader;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_DCM_EN` reader - enable using dcm need to response"]
pub type BitRespDcmEnR = crate::BitReader;
#[doc = "Field `BIT_RESP_DCM_EN` writer - enable using dcm need to response"]
pub type BitRespDcmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_RATE_DOWNGRADE` reader - enable downgrade response rate from HE to VHT, VHT to HT, HE to HT when need to response"]
pub type BitRespRateDowngradeR = crate::BitReader;
#[doc = "Field `BIT_RESP_RATE_DOWNGRADE` writer - enable downgrade response rate from HE to VHT, VHT to HT, HE to HT when need to response"]
pub type BitRespRateDowngradeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_HTC_EN` reader - enable appending htc when need to response"]
pub type BitRespHtcEnR = crate::BitReader;
#[doc = "Field `BIT_RESP_HTC_EN` writer - enable appending htc when need to response"]
pub type BitRespHtcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_STBC_EN` reader - enable stbc when need to response"]
pub type BitRespStbcEnR = crate::BitReader;
#[doc = "Field `BIT_RESP_STBC_EN` writer - enable stbc when need to response"]
pub type BitRespStbcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_CHK_BASIC_NAV` reader - check basic nav when need to response"]
pub type BitRespChkBasicNavR = crate::BitReader;
#[doc = "Field `BIT_RESP_CHK_BASIC_NAV` writer - check basic nav when need to response"]
pub type BitRespChkBasicNavW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_CHK_INTRA_NAV` reader - check intra nav when need to response"]
pub type BitRespChkIntraNavR = crate::BitReader;
#[doc = "Field `BIT_RESP_CHK_INTRA_NAV` writer - check intra nav when need to response"]
pub type BitRespChkIntraNavW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_CHK_TXNAV` reader - check txnav when need to response"]
pub type BitRespChkTxnavR = crate::BitReader;
#[doc = "Field `BIT_RESP_CHK_TXNAV` writer - check txnav when need to response"]
pub type BitRespChkTxnavW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - tx rate use to decide response rate"]
    #[inline(always)]
    pub fn bit_resp_txrate(&self) -> BitRespTxrateR {
        BitRespTxrateR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 11 - check edcca_per20_bitmap_sifs\\[0\\] when response"]
    #[inline(always)]
    pub fn bit_resp_chk_edcca_per20(&self) -> BitRespChkEdccaPer20R {
        BitRespChkEdccaPer20R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - check zigbee cca when response"]
    #[inline(always)]
    pub fn bit_resp_chk_zbcca(&self) -> BitRespChkZbccaR {
        BitRespChkZbccaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - 5:1x+0.8 4:1x+1.6 3:2x+0.8 2:2x+1.6 1:4x+0.8 0:4x+3.2 not support 1x+1.6 when response"]
    #[inline(always)]
    pub fn bit_resp_gi_ltf(&self) -> BitRespGiLtfR {
        BitRespGiLtfR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - when receive pkt of tkip or wep, While phyrxon down, mac_rxactive on. Ignore mac rx busy or physts_on, respond immediately."]
    #[inline(always)]
    pub fn bit_resp_early_leg_sec(&self) -> BitRespEarlyLegSecR {
        BitRespEarlyLegSecR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - check sec 80 cca when response"]
    #[inline(always)]
    pub fn bit_resp_chk_sec_cca80(&self) -> BitRespChkSecCca80R {
        BitRespChkSecCca80R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - check sec 40 cca when response"]
    #[inline(always)]
    pub fn bit_resp_chk_sec_cca40(&self) -> BitRespChkSecCca40R {
        BitRespChkSecCca40R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - check sec 20 cca when response"]
    #[inline(always)]
    pub fn bit_resp_chk_sec_cca20(&self) -> BitRespChkSecCca20R {
        BitRespChkSecCca20R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - check btcca when response"]
    #[inline(always)]
    pub fn bit_resp_chk_btcca(&self) -> BitRespChkBtccaR {
        BitRespChkBtccaR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - check edcca when response"]
    #[inline(always)]
    pub fn bit_resp_chk_edcca(&self) -> BitRespChkEdccaR {
        BitRespChkEdccaR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - check whether channel is idle during sifs, if csreq=1 in trigger frame, will not reply with tb ppdu"]
    #[inline(always)]
    pub fn bit_resp_rxtrig_chk_insifs(&self) -> BitRespRxtrigChkInsifsR {
        BitRespRxtrigChkInsifsR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - check tb ppdu tx power when rx trigger, if txpwr_ctrl send signal of abort,will not reply with tb ppdu"]
    #[inline(always)]
    pub fn bit_resp_tbppdu_chk_pwr(&self) -> BitRespTbppduChkPwrR {
        BitRespTbppduChkPwrR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - enable using dcm need to response"]
    #[inline(always)]
    pub fn bit_resp_dcm_en(&self) -> BitRespDcmEnR {
        BitRespDcmEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - enable downgrade response rate from HE to VHT, VHT to HT, HE to HT when need to response"]
    #[inline(always)]
    pub fn bit_resp_rate_downgrade(&self) -> BitRespRateDowngradeR {
        BitRespRateDowngradeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - enable appending htc when need to response"]
    #[inline(always)]
    pub fn bit_resp_htc_en(&self) -> BitRespHtcEnR {
        BitRespHtcEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - enable stbc when need to response"]
    #[inline(always)]
    pub fn bit_resp_stbc_en(&self) -> BitRespStbcEnR {
        BitRespStbcEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - check basic nav when need to response"]
    #[inline(always)]
    pub fn bit_resp_chk_basic_nav(&self) -> BitRespChkBasicNavR {
        BitRespChkBasicNavR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - check intra nav when need to response"]
    #[inline(always)]
    pub fn bit_resp_chk_intra_nav(&self) -> BitRespChkIntraNavR {
        BitRespChkIntraNavR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - check txnav when need to response"]
    #[inline(always)]
    pub fn bit_resp_chk_txnav(&self) -> BitRespChkTxnavR {
        BitRespChkTxnavR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - tx rate use to decide response rate"]
    #[inline(always)]
    pub fn bit_resp_txrate(&mut self) -> BitRespTxrateW<'_, RegRespControlSpec> {
        BitRespTxrateW::new(self, 0)
    }
    #[doc = "Bit 11 - check edcca_per20_bitmap_sifs\\[0\\] when response"]
    #[inline(always)]
    pub fn bit_resp_chk_edcca_per20(&mut self) -> BitRespChkEdccaPer20W<'_, RegRespControlSpec> {
        BitRespChkEdccaPer20W::new(self, 11)
    }
    #[doc = "Bit 12 - check zigbee cca when response"]
    #[inline(always)]
    pub fn bit_resp_chk_zbcca(&mut self) -> BitRespChkZbccaW<'_, RegRespControlSpec> {
        BitRespChkZbccaW::new(self, 12)
    }
    #[doc = "Bits 13:15 - 5:1x+0.8 4:1x+1.6 3:2x+0.8 2:2x+1.6 1:4x+0.8 0:4x+3.2 not support 1x+1.6 when response"]
    #[inline(always)]
    pub fn bit_resp_gi_ltf(&mut self) -> BitRespGiLtfW<'_, RegRespControlSpec> {
        BitRespGiLtfW::new(self, 13)
    }
    #[doc = "Bit 16 - when receive pkt of tkip or wep, While phyrxon down, mac_rxactive on. Ignore mac rx busy or physts_on, respond immediately."]
    #[inline(always)]
    pub fn bit_resp_early_leg_sec(&mut self) -> BitRespEarlyLegSecW<'_, RegRespControlSpec> {
        BitRespEarlyLegSecW::new(self, 16)
    }
    #[doc = "Bit 17 - check sec 80 cca when response"]
    #[inline(always)]
    pub fn bit_resp_chk_sec_cca80(&mut self) -> BitRespChkSecCca80W<'_, RegRespControlSpec> {
        BitRespChkSecCca80W::new(self, 17)
    }
    #[doc = "Bit 18 - check sec 40 cca when response"]
    #[inline(always)]
    pub fn bit_resp_chk_sec_cca40(&mut self) -> BitRespChkSecCca40W<'_, RegRespControlSpec> {
        BitRespChkSecCca40W::new(self, 18)
    }
    #[doc = "Bit 19 - check sec 20 cca when response"]
    #[inline(always)]
    pub fn bit_resp_chk_sec_cca20(&mut self) -> BitRespChkSecCca20W<'_, RegRespControlSpec> {
        BitRespChkSecCca20W::new(self, 19)
    }
    #[doc = "Bit 20 - check btcca when response"]
    #[inline(always)]
    pub fn bit_resp_chk_btcca(&mut self) -> BitRespChkBtccaW<'_, RegRespControlSpec> {
        BitRespChkBtccaW::new(self, 20)
    }
    #[doc = "Bit 21 - check edcca when response"]
    #[inline(always)]
    pub fn bit_resp_chk_edcca(&mut self) -> BitRespChkEdccaW<'_, RegRespControlSpec> {
        BitRespChkEdccaW::new(self, 21)
    }
    #[doc = "Bit 22 - check whether channel is idle during sifs, if csreq=1 in trigger frame, will not reply with tb ppdu"]
    #[inline(always)]
    pub fn bit_resp_rxtrig_chk_insifs(
        &mut self,
    ) -> BitRespRxtrigChkInsifsW<'_, RegRespControlSpec> {
        BitRespRxtrigChkInsifsW::new(self, 22)
    }
    #[doc = "Bit 23 - check tb ppdu tx power when rx trigger, if txpwr_ctrl send signal of abort,will not reply with tb ppdu"]
    #[inline(always)]
    pub fn bit_resp_tbppdu_chk_pwr(&mut self) -> BitRespTbppduChkPwrW<'_, RegRespControlSpec> {
        BitRespTbppduChkPwrW::new(self, 23)
    }
    #[doc = "Bit 24 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegRespControlSpec> {
        RsvdW::new(self, 24)
    }
    #[doc = "Bit 25 - enable using dcm need to response"]
    #[inline(always)]
    pub fn bit_resp_dcm_en(&mut self) -> BitRespDcmEnW<'_, RegRespControlSpec> {
        BitRespDcmEnW::new(self, 25)
    }
    #[doc = "Bit 26 - enable downgrade response rate from HE to VHT, VHT to HT, HE to HT when need to response"]
    #[inline(always)]
    pub fn bit_resp_rate_downgrade(&mut self) -> BitRespRateDowngradeW<'_, RegRespControlSpec> {
        BitRespRateDowngradeW::new(self, 26)
    }
    #[doc = "Bit 27 - enable appending htc when need to response"]
    #[inline(always)]
    pub fn bit_resp_htc_en(&mut self) -> BitRespHtcEnW<'_, RegRespControlSpec> {
        BitRespHtcEnW::new(self, 27)
    }
    #[doc = "Bit 28 - enable stbc when need to response"]
    #[inline(always)]
    pub fn bit_resp_stbc_en(&mut self) -> BitRespStbcEnW<'_, RegRespControlSpec> {
        BitRespStbcEnW::new(self, 28)
    }
    #[doc = "Bit 29 - check basic nav when need to response"]
    #[inline(always)]
    pub fn bit_resp_chk_basic_nav(&mut self) -> BitRespChkBasicNavW<'_, RegRespControlSpec> {
        BitRespChkBasicNavW::new(self, 29)
    }
    #[doc = "Bit 30 - check intra nav when need to response"]
    #[inline(always)]
    pub fn bit_resp_chk_intra_nav(&mut self) -> BitRespChkIntraNavW<'_, RegRespControlSpec> {
        BitRespChkIntraNavW::new(self, 30)
    }
    #[doc = "Bit 31 - check txnav when need to response"]
    #[inline(always)]
    pub fn bit_resp_chk_txnav(&mut self) -> BitRespChkTxnavW<'_, RegRespControlSpec> {
        BitRespChkTxnavW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_resp_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_resp_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRespControlSpec;
impl crate::RegisterSpec for RegRespControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_resp_control::R`](R) reader structure"]
impl crate::Readable for RegRespControlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_resp_control::W`](W) writer structure"]
impl crate::Writable for RegRespControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RESP_CONTROL to value 0x62a1_0004"]
impl crate::Resettable for RegRespControlSpec {
    const RESET_VALUE: u32 = 0x62a1_0004;
}
