#[doc = "Register `REG_MU_BF_OPTION` reader"]
pub type R = crate::R<RegMuBfOptionSpec>;
#[doc = "Register `REG_MU_BF_OPTION` writer"]
pub type W = crate::W<RegMuBfOptionSpec>;
#[doc = "Field `BIT_WMAC_MU_BFEE_DIS` reader - If set to 1, HW will be SU only beamformee and not response to NDPA or Beamforming report poll."]
pub type BitWmacMuBfeeDisR = crate::BitReader;
#[doc = "Field `BIT_WMAC_MU_BFEE_DIS` writer - If set to 1, HW will be SU only beamformee and not response to NDPA or Beamforming report poll."]
pub type BitWmacMuBfeeDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_MU_BFEE_PORT_SEL` reader - Enable software to set initial state of MU arbiter"]
pub type BitWmacMuBfeePortSelR = crate::FieldReader;
#[doc = "Field `BIT_WMAC_MU_BFEE_PORT_SEL` writer - Enable software to set initial state of MU arbiter"]
pub type BitWmacMuBfeePortSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_WMAC_RESP_NONSTA1_DIS` reader - Disable response BA when receive non-STA1 user position MU PPDU"]
pub type BitWmacRespNonsta1DisR = crate::BitReader;
#[doc = "Field `BIT_WMAC_RESP_NONSTA1_DIS` writer - Disable response BA when receive non-STA1 user position MU PPDU"]
pub type BitWmacRespNonsta1DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_PAUSE_BB_CLR_TH` reader - MAC_PAUSE_BB High to Low clear threshold of csidma syncff read-write pointer"]
pub type BitWmacPauseBbClrThR = crate::FieldReader;
#[doc = "Field `BIT_WMAC_PAUSE_BB_CLR_TH` writer - MAC_PAUSE_BB High to Low clear threshold of csidma syncff read-write pointer"]
pub type BitWmacPauseBbClrThW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_ARB_SW_STATE` reader - MU arbiter software initial state"]
pub type BitArbSwStateR = crate::FieldReader;
#[doc = "Field `BIT_ARB_SW_STATE` writer - MU arbiter software initial state"]
pub type BitArbSwStateW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BIT_ARB_SW_EN` reader - Enable software to set initial state of MU arbiter"]
pub type BitArbSwEnR = crate::BitReader;
#[doc = "Field `BIT_ARB_SW_EN` writer - Enable software to set initial state of MU arbiter"]
pub type BitArbSwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ARB_HW_ADAPT_EN` reader - Enable hardware to arbitrate data according to rate offset of user0 and user1"]
pub type BitArbHwAdaptEnR = crate::BitReader;
#[doc = "Field `BIT_ARB_HW_ADAPT_EN` writer - Enable hardware to arbitrate data according to rate offset of user0 and user1"]
pub type BitArbHwAdaptEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WMAC_CSI_PARA_RDY_DLYCNT` reader - 1:Cycle number from txctl state machine IDLE(VBON falling) to CSI_standby falling. 2:Half of this value is txctl state machine IDLE(VBON falling) to CSI_parameter_ready rising cycle number."]
pub type WmacCsiParaRdyDlycntR = crate::FieldReader;
#[doc = "Field `WMAC_CSI_PARA_RDY_DLYCNT` writer - 1:Cycle number from txctl state machine IDLE(VBON falling) to CSI_standby falling. 2:Half of this value is txctl state machine IDLE(VBON falling) to CSI_parameter_ready rising cycle number."]
pub type WmacCsiParaRdyDlycntW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WMAC_NOCHK_BFPOLL_BMP` reader - Set this bit to ignore retransmission bitmap of rx beamforming report poll"]
pub type WmacNochkBfpollBmpR = crate::BitReader;
#[doc = "Field `WMAC_NOCHK_BFPOLL_BMP` writer - Set this bit to ignore retransmission bitmap of rx beamforming report poll"]
pub type WmacNochkBfpollBmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - If set to 1, HW will be SU only beamformee and not response to NDPA or Beamforming report poll."]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee_dis(&self) -> BitWmacMuBfeeDisR {
        BitWmacMuBfeeDisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Enable software to set initial state of MU arbiter"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee_port_sel(&self) -> BitWmacMuBfeePortSelR {
        BitWmacMuBfeePortSelR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 7 - Disable response BA when receive non-STA1 user position MU PPDU"]
    #[inline(always)]
    pub fn bit_wmac_resp_nonsta1_dis(&self) -> BitWmacRespNonsta1DisR {
        BitWmacRespNonsta1DisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - MAC_PAUSE_BB High to Low clear threshold of csidma syncff read-write pointer"]
    #[inline(always)]
    pub fn bit_wmac_pause_bb_clr_th(&self) -> BitWmacPauseBbClrThR {
        BitWmacPauseBbClrThR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:21 - MU arbiter software initial state"]
    #[inline(always)]
    pub fn bit_arb_sw_state(&self) -> BitArbSwStateR {
        BitArbSwStateR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Enable software to set initial state of MU arbiter"]
    #[inline(always)]
    pub fn bit_arb_sw_en(&self) -> BitArbSwEnR {
        BitArbSwEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable hardware to arbitrate data according to rate offset of user0 and user1"]
    #[inline(always)]
    pub fn bit_arb_hw_adapt_en(&self) -> BitArbHwAdaptEnR {
        BitArbHwAdaptEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:28 - 1:Cycle number from txctl state machine IDLE(VBON falling) to CSI_standby falling. 2:Half of this value is txctl state machine IDLE(VBON falling) to CSI_parameter_ready rising cycle number."]
    #[inline(always)]
    pub fn wmac_csi_para_rdy_dlycnt(&self) -> WmacCsiParaRdyDlycntR {
        WmacCsiParaRdyDlycntR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Set this bit to ignore retransmission bitmap of rx beamforming report poll"]
    #[inline(always)]
    pub fn wmac_nochk_bfpoll_bmp(&self) -> WmacNochkBfpollBmpR {
        WmacNochkBfpollBmpR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If set to 1, HW will be SU only beamformee and not response to NDPA or Beamforming report poll."]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee_dis(&mut self) -> BitWmacMuBfeeDisW<'_, RegMuBfOptionSpec> {
        BitWmacMuBfeeDisW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Enable software to set initial state of MU arbiter"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee_port_sel(&mut self) -> BitWmacMuBfeePortSelW<'_, RegMuBfOptionSpec> {
        BitWmacMuBfeePortSelW::new(self, 1)
    }
    #[doc = "Bit 7 - Disable response BA when receive non-STA1 user position MU PPDU"]
    #[inline(always)]
    pub fn bit_wmac_resp_nonsta1_dis(&mut self) -> BitWmacRespNonsta1DisW<'_, RegMuBfOptionSpec> {
        BitWmacRespNonsta1DisW::new(self, 7)
    }
    #[doc = "Bits 8:15 - MAC_PAUSE_BB High to Low clear threshold of csidma syncff read-write pointer"]
    #[inline(always)]
    pub fn bit_wmac_pause_bb_clr_th(&mut self) -> BitWmacPauseBbClrThW<'_, RegMuBfOptionSpec> {
        BitWmacPauseBbClrThW::new(self, 8)
    }
    #[doc = "Bits 16:21 - MU arbiter software initial state"]
    #[inline(always)]
    pub fn bit_arb_sw_state(&mut self) -> BitArbSwStateW<'_, RegMuBfOptionSpec> {
        BitArbSwStateW::new(self, 16)
    }
    #[doc = "Bit 22 - Enable software to set initial state of MU arbiter"]
    #[inline(always)]
    pub fn bit_arb_sw_en(&mut self) -> BitArbSwEnW<'_, RegMuBfOptionSpec> {
        BitArbSwEnW::new(self, 22)
    }
    #[doc = "Bit 23 - Enable hardware to arbitrate data according to rate offset of user0 and user1"]
    #[inline(always)]
    pub fn bit_arb_hw_adapt_en(&mut self) -> BitArbHwAdaptEnW<'_, RegMuBfOptionSpec> {
        BitArbHwAdaptEnW::new(self, 23)
    }
    #[doc = "Bits 24:28 - 1:Cycle number from txctl state machine IDLE(VBON falling) to CSI_standby falling. 2:Half of this value is txctl state machine IDLE(VBON falling) to CSI_parameter_ready rising cycle number."]
    #[inline(always)]
    pub fn wmac_csi_para_rdy_dlycnt(&mut self) -> WmacCsiParaRdyDlycntW<'_, RegMuBfOptionSpec> {
        WmacCsiParaRdyDlycntW::new(self, 24)
    }
    #[doc = "Bit 31 - Set this bit to ignore retransmission bitmap of rx beamforming report poll"]
    #[inline(always)]
    pub fn wmac_nochk_bfpoll_bmp(&mut self) -> WmacNochkBfpollBmpW<'_, RegMuBfOptionSpec> {
        WmacNochkBfpollBmpW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mu_bf_option::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mu_bf_option::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMuBfOptionSpec;
impl crate::RegisterSpec for RegMuBfOptionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mu_bf_option::R`](R) reader structure"]
impl crate::Readable for RegMuBfOptionSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mu_bf_option::W`](W) writer structure"]
impl crate::Writable for RegMuBfOptionSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MU_BF_OPTION to value 0x1f2a_4800"]
impl crate::Resettable for RegMuBfOptionSpec {
    const RESET_VALUE: u32 = 0x1f2a_4800;
}
