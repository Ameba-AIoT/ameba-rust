#[doc = "Register `REG_LPS_CTRL` reader"]
pub type R = crate::R<RegLpsCtrlSpec>;
#[doc = "Register `REG_LPS_CTRL` writer"]
pub type W = crate::W<RegLpsCtrlSpec>;
#[doc = "Field `BIT_WL_NRM_MEMPM_SL` reader - MAC memory power mode when BIT_WL_LPS_MEMPM_EN is 0, 0 : Normal 1 : SD"]
pub type BitWlNrmMempmSlR = crate::BitReader;
#[doc = "Field `BIT_WL_NRM_MEMPM_SL` writer - MAC memory power mode when BIT_WL_LPS_MEMPM_EN is 0, 0 : Normal 1 : SD"]
pub type BitWlNrmMempmSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WL_LPS_MEMPM_EN` reader - enable LPS memory power mode control"]
pub type BitWlLpsMempmEnR = crate::BitReader;
#[doc = "Field `BIT_WL_LPS_MEMPM_EN` writer - enable LPS memory power mode control"]
pub type BitWlLpsMempmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - rsvd"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - rsvd"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BIT_WL_LPS_OPT_EMACPWR` reader - LPS option to enable WL MAC OFF block power 1: Enable WL MAC OFF block power 0: Disable WL MAC OFF block power"]
pub type BitWlLpsOptEmacpwrR = crate::BitReader;
#[doc = "Field `BIT_WL_LPS_OPT_EMACPWR` writer - LPS option to enable WL MAC OFF block power 1: Enable WL MAC OFF block power 0: Disable WL MAC OFF block power"]
pub type BitWlLpsOptEmacpwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WL_LPS_OPT_EMACFUN` reader - LPS option to reset WL MAC OFF block when enable WL MAC OFF power 1: not reset WL MAC OFF block 0: reset WL MAC OFF block"]
pub type BitWlLpsOptEmacfunR = crate::BitReader;
#[doc = "Field `BIT_WL_LPS_OPT_EMACFUN` writer - LPS option to reset WL MAC OFF block when enable WL MAC OFF power 1: not reset WL MAC OFF block 0: reset WL MAC OFF block"]
pub type BitWlLpsOptEmacfunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_LPS_MEMPM_MACOFF_PG_FTC` reader - MAC OFF FTC memory power mode when enable LPS memory power mode(for rxcsibuf) 00 : normal 01 : NAP 10 : RET 11 : SLP"]
pub type BitLpsMempmMacoffPgFtcR = crate::FieldReader;
#[doc = "Field `BIT_LPS_MEMPM_MACOFF_PG_FTC` writer - MAC OFF FTC memory power mode when enable LPS memory power mode(for rxcsibuf) 00 : normal 01 : NAP 10 : RET 11 : SLP"]
pub type BitLpsMempmMacoffPgFtcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_LPS_MEMPM_MACOFF_SNPS` reader - MAC OFF SNPS memory power mode when enable LPS memory power mode(txllt/rc4/txoqt/keysrch/rxbacam/dmafifo/txpwr) 0 : normal 1 : NAP"]
pub type BitLpsMempmMacoffSnpsR = crate::BitReader;
#[doc = "Field `BIT_LPS_MEMPM_MACOFF_SNPS` writer - MAC OFF SNPS memory power mode when enable LPS memory power mode(txllt/rc4/txoqt/keysrch/rxbacam/dmafifo/txpwr) 0 : normal 1 : NAP"]
pub type BitLpsMempmMacoffSnpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_LPS_MEMPM_MACOFF_FTC` reader - MAC OFF FTC/SNPS memory power mode when enable LPS memory power mode (for rxfifo/txfifo) 0 : normal 1 : NAP/LS"]
pub type BitLpsMempmMacoffFtcR = crate::BitReader;
#[doc = "Field `BIT_LPS_MEMPM_MACOFF_FTC` writer - MAC OFF FTC/SNPS memory power mode when enable LPS memory power mode (for rxfifo/txfifo) 0 : normal 1 : NAP/LS"]
pub type BitLpsMempmMacoffFtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_LPS_MEMPM_MACON_FTC_TXPKT` reader - MAC ON FTC memory power mode when enable LPS memory power mode (for txpktbuf high 8k) 00 : normal 01 : NAP 10 : RET 11 : SLP"]
pub type BitLpsMempmMaconFtcTxpktR = crate::FieldReader;
#[doc = "Field `BIT_LPS_MEMPM_MACON_FTC_TXPKT` writer - MAC ON FTC memory power mode when enable LPS memory power mode (for txpktbuf high 8k) 00 : normal 01 : NAP 10 : RET 11 : SLP"]
pub type BitLpsMempmMaconFtcTxpktW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_LPS_MEMPM_MACOFF_FTC_TXRPT` reader - MAC OFF FTC memory power mode when enable LPS memory power mode (for txrptbuf) 00 : normal 01 : NAP 10 : RET 11 : SLP"]
pub type BitLpsMempmMacoffFtcTxrptR = crate::FieldReader;
#[doc = "Field `BIT_LPS_MEMPM_MACOFF_FTC_TXRPT` writer - MAC OFF FTC memory power mode when enable LPS memory power mode (for txrptbuf) 00 : normal 01 : NAP 10 : RET 11 : SLP"]
pub type BitLpsMempmMacoffFtcTxrptW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_LPS_MEMPM_MACOFF_FTC_SHR` reader - MAC OFF FTC memory power mode when enable LPS memory power mode(for rxpktbuf/ txpktbuf low 24k\\[SHARE with system\\]) 00 : normal 01 : NAP 10 : RET 11 : SLP"]
pub type BitLpsMempmMacoffFtcShrR = crate::FieldReader;
#[doc = "Field `BIT_LPS_MEMPM_MACOFF_FTC_SHR` writer - MAC OFF FTC memory power mode when enable LPS memory power mode(for rxpktbuf/ txpktbuf low 24k\\[SHARE with system\\]) 00 : normal 01 : NAP 10 : RET 11 : SLP"]
pub type BitLpsMempmMacoffFtcShrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_WL_LPS_OPT_SAVE_REG` reader - LPS option to save retention FF 1: enable request to save retention FF 0: disable request to save retention FF"]
pub type BitWlLpsOptSaveRegR = crate::BitReader;
#[doc = "Field `BIT_WL_LPS_OPT_SAVE_REG` writer - LPS option to save retention FF 1: enable request to save retention FF 0: disable request to save retention FF"]
pub type BitWlLpsOptSaveRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_LPS_OPT_GTSFOFF` reader - LPS option to gate tsf_off clock 1:enable tsf_off clock when enter LPS CG 0:disable tsf_off clock when enter LPS CG"]
pub type BitLpsOptGtsfoffR = crate::BitReader;
#[doc = "Field `BIT_LPS_OPT_GTSFOFF` writer - LPS option to gate tsf_off clock 1:enable tsf_off clock when enter LPS CG 0:disable tsf_off clock when enter LPS CG"]
pub type BitLpsOptGtsfoffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MAC memory power mode when BIT_WL_LPS_MEMPM_EN is 0, 0 : Normal 1 : SD"]
    #[inline(always)]
    pub fn bit_wl_nrm_mempm_sl(&self) -> BitWlNrmMempmSlR {
        BitWlNrmMempmSlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable LPS memory power mode control"]
    #[inline(always)]
    pub fn bit_wl_lps_mempm_en(&self) -> BitWlLpsMempmEnR {
        BitWlLpsMempmEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - rsvd"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - LPS option to enable WL MAC OFF block power 1: Enable WL MAC OFF block power 0: Disable WL MAC OFF block power"]
    #[inline(always)]
    pub fn bit_wl_lps_opt_emacpwr(&self) -> BitWlLpsOptEmacpwrR {
        BitWlLpsOptEmacpwrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - LPS option to reset WL MAC OFF block when enable WL MAC OFF power 1: not reset WL MAC OFF block 0: reset WL MAC OFF block"]
    #[inline(always)]
    pub fn bit_wl_lps_opt_emacfun(&self) -> BitWlLpsOptEmacfunR {
        BitWlLpsOptEmacfunR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 15:16 - MAC OFF FTC memory power mode when enable LPS memory power mode(for rxcsibuf) 00 : normal 01 : NAP 10 : RET 11 : SLP"]
    #[inline(always)]
    pub fn bit_lps_mempm_macoff_pg_ftc(&self) -> BitLpsMempmMacoffPgFtcR {
        BitLpsMempmMacoffPgFtcR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - MAC OFF SNPS memory power mode when enable LPS memory power mode(txllt/rc4/txoqt/keysrch/rxbacam/dmafifo/txpwr) 0 : normal 1 : NAP"]
    #[inline(always)]
    pub fn bit_lps_mempm_macoff_snps(&self) -> BitLpsMempmMacoffSnpsR {
        BitLpsMempmMacoffSnpsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MAC OFF FTC/SNPS memory power mode when enable LPS memory power mode (for rxfifo/txfifo) 0 : normal 1 : NAP/LS"]
    #[inline(always)]
    pub fn bit_lps_mempm_macoff_ftc(&self) -> BitLpsMempmMacoffFtcR {
        BitLpsMempmMacoffFtcR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - MAC ON FTC memory power mode when enable LPS memory power mode (for txpktbuf high 8k) 00 : normal 01 : NAP 10 : RET 11 : SLP"]
    #[inline(always)]
    pub fn bit_lps_mempm_macon_ftc_txpkt(&self) -> BitLpsMempmMaconFtcTxpktR {
        BitLpsMempmMaconFtcTxpktR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:22 - MAC OFF FTC memory power mode when enable LPS memory power mode (for txrptbuf) 00 : normal 01 : NAP 10 : RET 11 : SLP"]
    #[inline(always)]
    pub fn bit_lps_mempm_macoff_ftc_txrpt(&self) -> BitLpsMempmMacoffFtcTxrptR {
        BitLpsMempmMacoffFtcTxrptR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - MAC OFF FTC memory power mode when enable LPS memory power mode(for rxpktbuf/ txpktbuf low 24k\\[SHARE with system\\]) 00 : normal 01 : NAP 10 : RET 11 : SLP"]
    #[inline(always)]
    pub fn bit_lps_mempm_macoff_ftc_shr(&self) -> BitLpsMempmMacoffFtcShrR {
        BitLpsMempmMacoffFtcShrR::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 29 - LPS option to save retention FF 1: enable request to save retention FF 0: disable request to save retention FF"]
    #[inline(always)]
    pub fn bit_wl_lps_opt_save_reg(&self) -> BitWlLpsOptSaveRegR {
        BitWlLpsOptSaveRegR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - LPS option to gate tsf_off clock 1:enable tsf_off clock when enter LPS CG 0:disable tsf_off clock when enter LPS CG"]
    #[inline(always)]
    pub fn bit_lps_opt_gtsfoff(&self) -> BitLpsOptGtsfoffR {
        BitLpsOptGtsfoffR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MAC memory power mode when BIT_WL_LPS_MEMPM_EN is 0, 0 : Normal 1 : SD"]
    #[inline(always)]
    pub fn bit_wl_nrm_mempm_sl(&mut self) -> BitWlNrmMempmSlW<'_, RegLpsCtrlSpec> {
        BitWlNrmMempmSlW::new(self, 0)
    }
    #[doc = "Bit 1 - enable LPS memory power mode control"]
    #[inline(always)]
    pub fn bit_wl_lps_mempm_en(&mut self) -> BitWlLpsMempmEnW<'_, RegLpsCtrlSpec> {
        BitWlLpsMempmEnW::new(self, 1)
    }
    #[doc = "Bits 2:7 - rsvd"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegLpsCtrlSpec> {
        RsvdW::new(self, 2)
    }
    #[doc = "Bit 8 - LPS option to enable WL MAC OFF block power 1: Enable WL MAC OFF block power 0: Disable WL MAC OFF block power"]
    #[inline(always)]
    pub fn bit_wl_lps_opt_emacpwr(&mut self) -> BitWlLpsOptEmacpwrW<'_, RegLpsCtrlSpec> {
        BitWlLpsOptEmacpwrW::new(self, 8)
    }
    #[doc = "Bit 10 - LPS option to reset WL MAC OFF block when enable WL MAC OFF power 1: not reset WL MAC OFF block 0: reset WL MAC OFF block"]
    #[inline(always)]
    pub fn bit_wl_lps_opt_emacfun(&mut self) -> BitWlLpsOptEmacfunW<'_, RegLpsCtrlSpec> {
        BitWlLpsOptEmacfunW::new(self, 10)
    }
    #[doc = "Bits 15:16 - MAC OFF FTC memory power mode when enable LPS memory power mode(for rxcsibuf) 00 : normal 01 : NAP 10 : RET 11 : SLP"]
    #[inline(always)]
    pub fn bit_lps_mempm_macoff_pg_ftc(&mut self) -> BitLpsMempmMacoffPgFtcW<'_, RegLpsCtrlSpec> {
        BitLpsMempmMacoffPgFtcW::new(self, 15)
    }
    #[doc = "Bit 17 - MAC OFF SNPS memory power mode when enable LPS memory power mode(txllt/rc4/txoqt/keysrch/rxbacam/dmafifo/txpwr) 0 : normal 1 : NAP"]
    #[inline(always)]
    pub fn bit_lps_mempm_macoff_snps(&mut self) -> BitLpsMempmMacoffSnpsW<'_, RegLpsCtrlSpec> {
        BitLpsMempmMacoffSnpsW::new(self, 17)
    }
    #[doc = "Bit 18 - MAC OFF FTC/SNPS memory power mode when enable LPS memory power mode (for rxfifo/txfifo) 0 : normal 1 : NAP/LS"]
    #[inline(always)]
    pub fn bit_lps_mempm_macoff_ftc(&mut self) -> BitLpsMempmMacoffFtcW<'_, RegLpsCtrlSpec> {
        BitLpsMempmMacoffFtcW::new(self, 18)
    }
    #[doc = "Bits 19:20 - MAC ON FTC memory power mode when enable LPS memory power mode (for txpktbuf high 8k) 00 : normal 01 : NAP 10 : RET 11 : SLP"]
    #[inline(always)]
    pub fn bit_lps_mempm_macon_ftc_txpkt(
        &mut self,
    ) -> BitLpsMempmMaconFtcTxpktW<'_, RegLpsCtrlSpec> {
        BitLpsMempmMaconFtcTxpktW::new(self, 19)
    }
    #[doc = "Bits 21:22 - MAC OFF FTC memory power mode when enable LPS memory power mode (for txrptbuf) 00 : normal 01 : NAP 10 : RET 11 : SLP"]
    #[inline(always)]
    pub fn bit_lps_mempm_macoff_ftc_txrpt(
        &mut self,
    ) -> BitLpsMempmMacoffFtcTxrptW<'_, RegLpsCtrlSpec> {
        BitLpsMempmMacoffFtcTxrptW::new(self, 21)
    }
    #[doc = "Bits 23:24 - MAC OFF FTC memory power mode when enable LPS memory power mode(for rxpktbuf/ txpktbuf low 24k\\[SHARE with system\\]) 00 : normal 01 : NAP 10 : RET 11 : SLP"]
    #[inline(always)]
    pub fn bit_lps_mempm_macoff_ftc_shr(&mut self) -> BitLpsMempmMacoffFtcShrW<'_, RegLpsCtrlSpec> {
        BitLpsMempmMacoffFtcShrW::new(self, 23)
    }
    #[doc = "Bit 29 - LPS option to save retention FF 1: enable request to save retention FF 0: disable request to save retention FF"]
    #[inline(always)]
    pub fn bit_wl_lps_opt_save_reg(&mut self) -> BitWlLpsOptSaveRegW<'_, RegLpsCtrlSpec> {
        BitWlLpsOptSaveRegW::new(self, 29)
    }
    #[doc = "Bit 30 - LPS option to gate tsf_off clock 1:enable tsf_off clock when enter LPS CG 0:disable tsf_off clock when enter LPS CG"]
    #[inline(always)]
    pub fn bit_lps_opt_gtsfoff(&mut self) -> BitLpsOptGtsfoffW<'_, RegLpsCtrlSpec> {
        BitLpsOptGtsfoffW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_lps_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_lps_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegLpsCtrlSpec;
impl crate::RegisterSpec for RegLpsCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_lps_ctrl::R`](R) reader structure"]
impl crate::Readable for RegLpsCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_lps_ctrl::W`](W) writer structure"]
impl crate::Writable for RegLpsCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_LPS_CTRL to value 0x21ff_8000"]
impl crate::Resettable for RegLpsCtrlSpec {
    const RESET_VALUE: u32 = 0x21ff_8000;
}
