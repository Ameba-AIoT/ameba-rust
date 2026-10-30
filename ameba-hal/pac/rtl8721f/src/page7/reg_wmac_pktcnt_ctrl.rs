#[doc = "Register `REG_WMAC_PKTCNT_CTRL` reader"]
pub type R = crate::R<RegWmacPktcntCtrlSpec>;
#[doc = "Register `REG_WMAC_PKTCNT_CTRL` writer"]
pub type W = crate::W<RegWmacPktcntCtrlSpec>;
#[doc = "Field `BIT_WMAC_PKTCNT_CFGAD` reader - Packet counter configure register read/write address"]
pub type BitWmacPktcntCfgadR = crate::FieldReader;
#[doc = "Field `BIT_WMAC_PKTCNT_CFGAD` writer - Packet counter configure register read/write address"]
pub type BitWmacPktcntCfgadW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_WMAC_PKTCNT_FEN` reader - Packet counter function enable"]
pub type BitWmacPktcntFenR = crate::BitReader;
#[doc = "Field `BIT_WMAC_PKTCNT_FEN` writer - Packet counter function enable"]
pub type BitWmacPktcntFenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_MACTX_CTL_RST` writer - write pluse to reset fsm of ppdu control in mactx_ctl"]
pub type BitWmacMactxCtlRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_TRXPTCL_RESP_RST` writer - write pluse to reset fsm of wmac_ptrxptcl_resp"]
pub type BitWmacTrxptclRespRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_TRXPTCL_TXCTL_RST` writer - write pluse to reset fsm of wmac_trxptcl_txctl"]
pub type BitWmacTrxptclTxctlRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_PSF_BCNTIMER_RST` writer - write pluse to reset fsm of wmac_psf_bcntimer"]
pub type BitWmacPsfBcntimerRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Packet counter configure register read/write address"]
    #[inline(always)]
    pub fn bit_wmac_pktcnt_cfgad(&self) -> BitWmacPktcntCfgadR {
        BitWmacPktcntCfgadR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Packet counter function enable"]
    #[inline(always)]
    pub fn bit_wmac_pktcnt_fen(&self) -> BitWmacPktcntFenR {
        BitWmacPktcntFenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Packet counter configure register read/write address"]
    #[inline(always)]
    pub fn bit_wmac_pktcnt_cfgad(&mut self) -> BitWmacPktcntCfgadW<'_, RegWmacPktcntCtrlSpec> {
        BitWmacPktcntCfgadW::new(self, 0)
    }
    #[doc = "Bit 8 - Packet counter function enable"]
    #[inline(always)]
    pub fn bit_wmac_pktcnt_fen(&mut self) -> BitWmacPktcntFenW<'_, RegWmacPktcntCtrlSpec> {
        BitWmacPktcntFenW::new(self, 8)
    }
    #[doc = "Bit 16 - write pluse to reset fsm of ppdu control in mactx_ctl"]
    #[inline(always)]
    pub fn bit_wmac_mactx_ctl_rst(&mut self) -> BitWmacMactxCtlRstW<'_, RegWmacPktcntCtrlSpec> {
        BitWmacMactxCtlRstW::new(self, 16)
    }
    #[doc = "Bit 17 - write pluse to reset fsm of wmac_ptrxptcl_resp"]
    #[inline(always)]
    pub fn bit_wmac_trxptcl_resp_rst(
        &mut self,
    ) -> BitWmacTrxptclRespRstW<'_, RegWmacPktcntCtrlSpec> {
        BitWmacTrxptclRespRstW::new(self, 17)
    }
    #[doc = "Bit 18 - write pluse to reset fsm of wmac_trxptcl_txctl"]
    #[inline(always)]
    pub fn bit_wmac_trxptcl_txctl_rst(
        &mut self,
    ) -> BitWmacTrxptclTxctlRstW<'_, RegWmacPktcntCtrlSpec> {
        BitWmacTrxptclTxctlRstW::new(self, 18)
    }
    #[doc = "Bit 19 - write pluse to reset fsm of wmac_psf_bcntimer"]
    #[inline(always)]
    pub fn bit_wmac_psf_bcntimer_rst(
        &mut self,
    ) -> BitWmacPsfBcntimerRstW<'_, RegWmacPktcntCtrlSpec> {
        BitWmacPsfBcntimerRstW::new(self, 19)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wmac_pktcnt_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wmac_pktcnt_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWmacPktcntCtrlSpec;
impl crate::RegisterSpec for RegWmacPktcntCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wmac_pktcnt_ctrl::R`](R) reader structure"]
impl crate::Readable for RegWmacPktcntCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wmac_pktcnt_ctrl::W`](W) writer structure"]
impl crate::Writable for RegWmacPktcntCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WMAC_PKTCNT_CTRL to value 0"]
impl crate::Resettable for RegWmacPktcntCtrlSpec {}
