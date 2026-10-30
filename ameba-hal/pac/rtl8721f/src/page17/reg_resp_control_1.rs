#[doc = "Register `REG_RESP_CONTROL_1` reader"]
pub type R = crate::R<RegRespControl1Spec>;
#[doc = "Register `REG_RESP_CONTROL_1` writer"]
pub type W = crate::W<RegRespControl1Spec>;
#[doc = "Field `BIT_RESP_ALLACK_EN` reader - enable response with all-ack MBA"]
pub type BitRespAllackEnR = crate::BitReader;
#[doc = "Field `BIT_RESP_ALLACK_EN` writer - enable response with all-ack MBA"]
pub type BitRespAllackEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_USE_EARLY_FCS` reader - use fcs result from rxpktfifo instead of macrx_parser when receive packet used wep/tkip"]
pub type BitRespUseEarlyFcsR = crate::BitReader;
#[doc = "Field `BIT_RESP_USE_EARLY_FCS` writer - use fcs result from rxpktfifo instead of macrx_parser when receive packet used wep/tkip"]
pub type BitRespUseEarlyFcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_EARLY_START` reader - when set to 1:use mac header rdy when rx data frame and use aid fit when rx trigger frame, and use fcsok otherwise to start responding process when set to 0:use fcs ok to start responding process when rx any frame need reply"]
pub type BitRespEarlyStartR = crate::BitReader;
#[doc = "Field `BIT_RESP_EARLY_START` writer - when set to 1:use mac header rdy when rx data frame and use aid fit when rx trigger frame, and use fcsok otherwise to start responding process when set to 0:use fcs ok to start responding process when rx any frame need reply"]
pub type BitRespEarlyStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_NOMINAL_PAD` reader - nominal pkt pad info when response"]
pub type BitRespNominalPadR = crate::FieldReader;
#[doc = "Field `BIT_RESP_NOMINAL_PAD` writer - nominal pkt pad info when response"]
pub type BitRespNominalPadW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_RESP_BSR_FORMAT` reader - 0: legacy format of queue size 1: HE"]
pub type BitRespBsrFormatR = crate::BitReader;
#[doc = "Field `BIT_RESP_BSR_FORMAT` writer - 0: legacy format of queue size 1: HE"]
pub type BitRespBsrFormatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_DCM_SEL` reader - 1: follow ctrl reg(0x17b4\\[25\\]) 0: follow rxpkt"]
pub type BitRespDcmSelR = crate::BitReader;
#[doc = "Field `BIT_RESP_DCM_SEL` writer - 1: follow ctrl reg(0x17b4\\[25\\]) 0: follow rxpkt"]
pub type BitRespDcmSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_DOPPLER_SEL` reader - 1: follow ctrl reg(0x17E0\\[10\\]) 0: follow rxpkt"]
pub type BitRespDopplerSelR = crate::BitReader;
#[doc = "Field `BIT_RESP_DOPPLER_SEL` writer - 1: follow ctrl reg(0x17E0\\[10\\]) 0: follow rxpkt"]
pub type BitRespDopplerSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_DOPPLER_CTRL` reader - 0: Doppler unsupported 1: Doppler supported without moving 2: 10 symbol midamble periodicity 3: 20 symbol midamble periodicity"]
pub type BitRespDopplerCtrlR = crate::FieldReader;
#[doc = "Field `BIT_RESP_DOPPLER_CTRL` writer - 0: Doppler unsupported 1: Doppler supported without moving 2: 10 symbol midamble periodicity 3: 20 symbol midamble periodicity"]
pub type BitRespDopplerCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_RESP_GID_SEL` reader - HE SIG-A content 1 : group id=0, 0 : group id=63"]
pub type BitRespGidSelR = crate::BitReader;
#[doc = "Field `BIT_RESP_GID_SEL` writer - HE SIG-A content 1 : group id=0, 0 : group id=63"]
pub type BitRespGidSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_LDPC_SEL` reader - 1: follow ctrl reg(0x668\\[15\\]) 0: follow rxpkt"]
pub type BitRespLdpcSelR = crate::BitReader;
#[doc = "Field `BIT_RESP_LDPC_SEL` writer - 1: follow ctrl reg(0x668\\[15\\]) 0: follow rxpkt"]
pub type BitRespLdpcSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_ACTRL_CAS_EN` reader - If set to 1, HW would update CAS info to A-CTRL"]
pub type BitRespActrlCasEnR = crate::BitReader;
#[doc = "Field `BIT_RESP_ACTRL_CAS_EN` writer - If set to 1, HW would update CAS info to A-CTRL"]
pub type BitRespActrlCasEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_ACTRL_BSR_EN` reader - If set to 1, HW would update BSR info to A-CTRL"]
pub type BitRespActrlBsrEnR = crate::BitReader;
#[doc = "Field `BIT_RESP_ACTRL_BSR_EN` writer - If set to 1, HW would update BSR info to A-CTRL"]
pub type BitRespActrlBsrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_ACTRL_UPH_EN` reader - If set to 1, HW would update UPH info to A-CTRL"]
pub type BitRespActrlUphEnR = crate::BitReader;
#[doc = "Field `BIT_RESP_ACTRL_UPH_EN` writer - If set to 1, HW would update UPH info to A-CTRL"]
pub type BitRespActrlUphEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_ACTRL_BQR_EN` reader - If set to 1, HW would update BQR info to A-CTRL"]
pub type BitRespActrlBqrEnR = crate::BitReader;
#[doc = "Field `BIT_RESP_ACTRL_BQR_EN` writer - If set to 1, HW would update BQR info to A-CTRL"]
pub type BitRespActrlBqrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_TXRAW_IGNOREA1` reader - If set to 1, will not check A1 of rxpkt when txraw"]
pub type BitWmacTxrawIgnorea1R = crate::BitReader;
#[doc = "Field `BIT_WMAC_TXRAW_IGNOREA1` writer - If set to 1, will not check A1 of rxpkt when txraw"]
pub type BitWmacTxrawIgnorea1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_CHK_FCS_ABORT_TX` reader - If set to 1, will not response ack if fcs is not rdy when sifs reach if set to 0, will response ack if fcs is not rdy when sifs reach"]
pub type BitRespChkFcsAbortTxR = crate::BitReader;
#[doc = "Field `BIT_RESP_CHK_FCS_ABORT_TX` writer - If set to 1, will not response ack if fcs is not rdy when sifs reach if set to 0, will response ack if fcs is not rdy when sifs reach"]
pub type BitRespChkFcsAbortTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_TX_WITH_FCSERR` reader - 1:if resp_chk_fcs is low and mactx starts to generate mpdu,will tx response with wrong crc32"]
pub type BitRespTxWithFcserrR = crate::BitReader;
#[doc = "Field `BIT_RESP_TX_WITH_FCSERR` writer - 1:if resp_chk_fcs is low and mactx starts to generate mpdu,will tx response with wrong crc32"]
pub type BitRespTxWithFcserrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - enable response with all-ack MBA"]
    #[inline(always)]
    pub fn bit_resp_allack_en(&self) -> BitRespAllackEnR {
        BitRespAllackEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - use fcs result from rxpktfifo instead of macrx_parser when receive packet used wep/tkip"]
    #[inline(always)]
    pub fn bit_resp_use_early_fcs(&self) -> BitRespUseEarlyFcsR {
        BitRespUseEarlyFcsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - when set to 1:use mac header rdy when rx data frame and use aid fit when rx trigger frame, and use fcsok otherwise to start responding process when set to 0:use fcs ok to start responding process when rx any frame need reply"]
    #[inline(always)]
    pub fn bit_resp_early_start(&self) -> BitRespEarlyStartR {
        BitRespEarlyStartR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - nominal pkt pad info when response"]
    #[inline(always)]
    pub fn bit_resp_nominal_pad(&self) -> BitRespNominalPadR {
        BitRespNominalPadR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - 0: legacy format of queue size 1: HE"]
    #[inline(always)]
    pub fn bit_resp_bsr_format(&self) -> BitRespBsrFormatR {
        BitRespBsrFormatR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: follow ctrl reg(0x17b4\\[25\\]) 0: follow rxpkt"]
    #[inline(always)]
    pub fn bit_resp_dcm_sel(&self) -> BitRespDcmSelR {
        BitRespDcmSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: follow ctrl reg(0x17E0\\[10\\]) 0: follow rxpkt"]
    #[inline(always)]
    pub fn bit_resp_doppler_sel(&self) -> BitRespDopplerSelR {
        BitRespDopplerSelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - 0: Doppler unsupported 1: Doppler supported without moving 2: 10 symbol midamble periodicity 3: 20 symbol midamble periodicity"]
    #[inline(always)]
    pub fn bit_resp_doppler_ctrl(&self) -> BitRespDopplerCtrlR {
        BitRespDopplerCtrlR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - HE SIG-A content 1 : group id=0, 0 : group id=63"]
    #[inline(always)]
    pub fn bit_resp_gid_sel(&self) -> BitRespGidSelR {
        BitRespGidSelR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: follow ctrl reg(0x668\\[15\\]) 0: follow rxpkt"]
    #[inline(always)]
    pub fn bit_resp_ldpc_sel(&self) -> BitRespLdpcSelR {
        BitRespLdpcSelR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - If set to 1, HW would update CAS info to A-CTRL"]
    #[inline(always)]
    pub fn bit_resp_actrl_cas_en(&self) -> BitRespActrlCasEnR {
        BitRespActrlCasEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - If set to 1, HW would update BSR info to A-CTRL"]
    #[inline(always)]
    pub fn bit_resp_actrl_bsr_en(&self) -> BitRespActrlBsrEnR {
        BitRespActrlBsrEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - If set to 1, HW would update UPH info to A-CTRL"]
    #[inline(always)]
    pub fn bit_resp_actrl_uph_en(&self) -> BitRespActrlUphEnR {
        BitRespActrlUphEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - If set to 1, HW would update BQR info to A-CTRL"]
    #[inline(always)]
    pub fn bit_resp_actrl_bqr_en(&self) -> BitRespActrlBqrEnR {
        BitRespActrlBqrEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - If set to 1, will not check A1 of rxpkt when txraw"]
    #[inline(always)]
    pub fn bit_wmac_txraw_ignorea1(&self) -> BitWmacTxrawIgnorea1R {
        BitWmacTxrawIgnorea1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - If set to 1, will not response ack if fcs is not rdy when sifs reach if set to 0, will response ack if fcs is not rdy when sifs reach"]
    #[inline(always)]
    pub fn bit_resp_chk_fcs_abort_tx(&self) -> BitRespChkFcsAbortTxR {
        BitRespChkFcsAbortTxR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1:if resp_chk_fcs is low and mactx starts to generate mpdu,will tx response with wrong crc32"]
    #[inline(always)]
    pub fn bit_resp_tx_with_fcserr(&self) -> BitRespTxWithFcserrR {
        BitRespTxWithFcserrR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - enable response with all-ack MBA"]
    #[inline(always)]
    pub fn bit_resp_allack_en(&mut self) -> BitRespAllackEnW<'_, RegRespControl1Spec> {
        BitRespAllackEnW::new(self, 1)
    }
    #[doc = "Bit 2 - use fcs result from rxpktfifo instead of macrx_parser when receive packet used wep/tkip"]
    #[inline(always)]
    pub fn bit_resp_use_early_fcs(&mut self) -> BitRespUseEarlyFcsW<'_, RegRespControl1Spec> {
        BitRespUseEarlyFcsW::new(self, 2)
    }
    #[doc = "Bit 3 - when set to 1:use mac header rdy when rx data frame and use aid fit when rx trigger frame, and use fcsok otherwise to start responding process when set to 0:use fcs ok to start responding process when rx any frame need reply"]
    #[inline(always)]
    pub fn bit_resp_early_start(&mut self) -> BitRespEarlyStartW<'_, RegRespControl1Spec> {
        BitRespEarlyStartW::new(self, 3)
    }
    #[doc = "Bits 4:5 - nominal pkt pad info when response"]
    #[inline(always)]
    pub fn bit_resp_nominal_pad(&mut self) -> BitRespNominalPadW<'_, RegRespControl1Spec> {
        BitRespNominalPadW::new(self, 4)
    }
    #[doc = "Bit 6 - 0: legacy format of queue size 1: HE"]
    #[inline(always)]
    pub fn bit_resp_bsr_format(&mut self) -> BitRespBsrFormatW<'_, RegRespControl1Spec> {
        BitRespBsrFormatW::new(self, 6)
    }
    #[doc = "Bit 7 - 1: follow ctrl reg(0x17b4\\[25\\]) 0: follow rxpkt"]
    #[inline(always)]
    pub fn bit_resp_dcm_sel(&mut self) -> BitRespDcmSelW<'_, RegRespControl1Spec> {
        BitRespDcmSelW::new(self, 7)
    }
    #[doc = "Bit 8 - 1: follow ctrl reg(0x17E0\\[10\\]) 0: follow rxpkt"]
    #[inline(always)]
    pub fn bit_resp_doppler_sel(&mut self) -> BitRespDopplerSelW<'_, RegRespControl1Spec> {
        BitRespDopplerSelW::new(self, 8)
    }
    #[doc = "Bits 9:10 - 0: Doppler unsupported 1: Doppler supported without moving 2: 10 symbol midamble periodicity 3: 20 symbol midamble periodicity"]
    #[inline(always)]
    pub fn bit_resp_doppler_ctrl(&mut self) -> BitRespDopplerCtrlW<'_, RegRespControl1Spec> {
        BitRespDopplerCtrlW::new(self, 9)
    }
    #[doc = "Bit 11 - HE SIG-A content 1 : group id=0, 0 : group id=63"]
    #[inline(always)]
    pub fn bit_resp_gid_sel(&mut self) -> BitRespGidSelW<'_, RegRespControl1Spec> {
        BitRespGidSelW::new(self, 11)
    }
    #[doc = "Bit 12 - 1: follow ctrl reg(0x668\\[15\\]) 0: follow rxpkt"]
    #[inline(always)]
    pub fn bit_resp_ldpc_sel(&mut self) -> BitRespLdpcSelW<'_, RegRespControl1Spec> {
        BitRespLdpcSelW::new(self, 12)
    }
    #[doc = "Bit 13 - If set to 1, HW would update CAS info to A-CTRL"]
    #[inline(always)]
    pub fn bit_resp_actrl_cas_en(&mut self) -> BitRespActrlCasEnW<'_, RegRespControl1Spec> {
        BitRespActrlCasEnW::new(self, 13)
    }
    #[doc = "Bit 14 - If set to 1, HW would update BSR info to A-CTRL"]
    #[inline(always)]
    pub fn bit_resp_actrl_bsr_en(&mut self) -> BitRespActrlBsrEnW<'_, RegRespControl1Spec> {
        BitRespActrlBsrEnW::new(self, 14)
    }
    #[doc = "Bit 15 - If set to 1, HW would update UPH info to A-CTRL"]
    #[inline(always)]
    pub fn bit_resp_actrl_uph_en(&mut self) -> BitRespActrlUphEnW<'_, RegRespControl1Spec> {
        BitRespActrlUphEnW::new(self, 15)
    }
    #[doc = "Bit 16 - If set to 1, HW would update BQR info to A-CTRL"]
    #[inline(always)]
    pub fn bit_resp_actrl_bqr_en(&mut self) -> BitRespActrlBqrEnW<'_, RegRespControl1Spec> {
        BitRespActrlBqrEnW::new(self, 16)
    }
    #[doc = "Bit 17 - If set to 1, will not check A1 of rxpkt when txraw"]
    #[inline(always)]
    pub fn bit_wmac_txraw_ignorea1(&mut self) -> BitWmacTxrawIgnorea1W<'_, RegRespControl1Spec> {
        BitWmacTxrawIgnorea1W::new(self, 17)
    }
    #[doc = "Bit 18 - If set to 1, will not response ack if fcs is not rdy when sifs reach if set to 0, will response ack if fcs is not rdy when sifs reach"]
    #[inline(always)]
    pub fn bit_resp_chk_fcs_abort_tx(&mut self) -> BitRespChkFcsAbortTxW<'_, RegRespControl1Spec> {
        BitRespChkFcsAbortTxW::new(self, 18)
    }
    #[doc = "Bit 19 - 1:if resp_chk_fcs is low and mactx starts to generate mpdu,will tx response with wrong crc32"]
    #[inline(always)]
    pub fn bit_resp_tx_with_fcserr(&mut self) -> BitRespTxWithFcserrW<'_, RegRespControl1Spec> {
        BitRespTxWithFcserrW::new(self, 19)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_resp_control_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_resp_control_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRespControl1Spec;
impl crate::RegisterSpec for RegRespControl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_resp_control_1::R`](R) reader structure"]
impl crate::Readable for RegRespControl1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_resp_control_1::W`](W) writer structure"]
impl crate::Writable for RegRespControl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RESP_CONTROL_1 to value 0x0003_f80c"]
impl crate::Resettable for RegRespControl1Spec {
    const RESET_VALUE: u32 = 0x0003_f80c;
}
