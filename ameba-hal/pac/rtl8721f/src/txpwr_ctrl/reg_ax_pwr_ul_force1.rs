#[doc = "Register `REG_AX_PWR_UL_FORCE1` reader"]
pub type R = crate::R<RegAxPwrUlForce1Spec>;
#[doc = "Register `REG_AX_PWR_UL_FORCE1` writer"]
pub type W = crate::W<RegAxPwrUlForce1Spec>;
#[doc = "Field `r_force_out_STA_txpwr_bb` reader - (RTL_only) Force output to MAC"]
pub type RForceOutStaTxpwrBbR = crate::FieldReader<u16>;
#[doc = "Field `r_force_out_STA_txpwr_bb` writer - (RTL_only) Force output to MAC"]
pub type RForceOutStaTxpwrBbW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `r_force_out_STA_txpwr_bb_on` reader - (RTL_only) Force output to MAC en"]
pub type RForceOutStaTxpwrBbOnR = crate::BitReader;
#[doc = "Field `r_force_out_STA_txpwr_bb_on` writer - (RTL_only) Force output to MAC en"]
pub type RForceOutStaTxpwrBbOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_out_STA_txpwr_mac` reader - (RTL_only) Force output to MAC"]
pub type RForceOutStaTxpwrMacR = crate::FieldReader;
#[doc = "Field `r_force_out_STA_txpwr_mac` writer - (RTL_only) Force output to MAC"]
pub type RForceOutStaTxpwrMacW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `r_force_out_STA_txpwr_mac_on` reader - (RTL_only) Force output to MAC en"]
pub type RForceOutStaTxpwrMacOnR = crate::BitReader;
#[doc = "Field `r_force_out_STA_txpwr_mac_on` writer - (RTL_only) Force output to MAC en"]
pub type RForceOutStaTxpwrMacOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_out_UPH` reader - (RTL_only) Force output to MAC"]
pub type RForceOutUphR = crate::FieldReader;
#[doc = "Field `r_force_out_UPH` writer - (RTL_only) Force output to MAC"]
pub type RForceOutUphW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `r_force_out_UPH_on` reader - (RTL_only) Force output to MAC en"]
pub type RForceOutUphOnR = crate::BitReader;
#[doc = "Field `r_force_out_UPH_on` writer - (RTL_only) Force output to MAC en"]
pub type RForceOutUphOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_out_min_tx_pwr_flag` reader - (RTL_only) Force output to MAC"]
pub type RForceOutMinTxPwrFlagR = crate::BitReader;
#[doc = "Field `r_force_out_min_tx_pwr_flag` writer - (RTL_only) Force output to MAC"]
pub type RForceOutMinTxPwrFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_out_min_tx_pwr_flag_on` reader - (RTL_only) Force output to MAC en"]
pub type RForceOutMinTxPwrFlagOnR = crate::BitReader;
#[doc = "Field `r_force_out_min_tx_pwr_flag_on` writer - (RTL_only) Force output to MAC en"]
pub type RForceOutMinTxPwrFlagOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_out_abort_tx_idx` reader - (RTL_only) Force output to MAC"]
pub type RForceOutAbortTxIdxR = crate::FieldReader;
#[doc = "Field `r_force_out_abort_tx_idx` writer - (RTL_only) Force output to MAC"]
pub type RForceOutAbortTxIdxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_force_out_abort_tx_idx_on` reader - (RTL_only) Force output to MAC en"]
pub type RForceOutAbortTxIdxOnR = crate::BitReader;
#[doc = "Field `r_force_out_abort_tx_idx_on` writer - (RTL_only) Force output to MAC en"]
pub type RForceOutAbortTxIdxOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_out_cfo_comp` reader - (RTL_only) Force output to MAC"]
pub type RForceOutCfoCompR = crate::FieldReader;
#[doc = "Field `r_force_out_cfo_comp` writer - (RTL_only) Force output to MAC"]
pub type RForceOutCfoCompW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_force_out_cfo_comp_on` reader - (RTL_only) Force output to MAC en"]
pub type RForceOutCfoCompOnR = crate::BitReader;
#[doc = "Field `r_force_out_cfo_comp_on` writer - (RTL_only) Force output to MAC en"]
pub type RForceOutCfoCompOnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_sta_txpwr_bb(&self) -> RForceOutStaTxpwrBbR {
        RForceOutStaTxpwrBbR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_sta_txpwr_bb_on(&self) -> RForceOutStaTxpwrBbOnR {
        RForceOutStaTxpwrBbOnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_sta_txpwr_mac(&self) -> RForceOutStaTxpwrMacR {
        RForceOutStaTxpwrMacR::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_sta_txpwr_mac_on(&self) -> RForceOutStaTxpwrMacOnR {
        RForceOutStaTxpwrMacOnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_uph(&self) -> RForceOutUphR {
        RForceOutUphR::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_uph_on(&self) -> RForceOutUphOnR {
        RForceOutUphOnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_min_tx_pwr_flag(&self) -> RForceOutMinTxPwrFlagR {
        RForceOutMinTxPwrFlagR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_min_tx_pwr_flag_on(&self) -> RForceOutMinTxPwrFlagOnR {
        RForceOutMinTxPwrFlagOnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_abort_tx_idx(&self) -> RForceOutAbortTxIdxR {
        RForceOutAbortTxIdxR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_abort_tx_idx_on(&self) -> RForceOutAbortTxIdxOnR {
        RForceOutAbortTxIdxOnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_cfo_comp(&self) -> RForceOutCfoCompR {
        RForceOutCfoCompR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_cfo_comp_on(&self) -> RForceOutCfoCompOnR {
        RForceOutCfoCompOnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_sta_txpwr_bb(&mut self) -> RForceOutStaTxpwrBbW<'_, RegAxPwrUlForce1Spec> {
        RForceOutStaTxpwrBbW::new(self, 0)
    }
    #[doc = "Bit 9 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_sta_txpwr_bb_on(
        &mut self,
    ) -> RForceOutStaTxpwrBbOnW<'_, RegAxPwrUlForce1Spec> {
        RForceOutStaTxpwrBbOnW::new(self, 9)
    }
    #[doc = "Bits 10:15 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_sta_txpwr_mac(&mut self) -> RForceOutStaTxpwrMacW<'_, RegAxPwrUlForce1Spec> {
        RForceOutStaTxpwrMacW::new(self, 10)
    }
    #[doc = "Bit 16 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_sta_txpwr_mac_on(
        &mut self,
    ) -> RForceOutStaTxpwrMacOnW<'_, RegAxPwrUlForce1Spec> {
        RForceOutStaTxpwrMacOnW::new(self, 16)
    }
    #[doc = "Bits 17:21 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_uph(&mut self) -> RForceOutUphW<'_, RegAxPwrUlForce1Spec> {
        RForceOutUphW::new(self, 17)
    }
    #[doc = "Bit 22 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_uph_on(&mut self) -> RForceOutUphOnW<'_, RegAxPwrUlForce1Spec> {
        RForceOutUphOnW::new(self, 22)
    }
    #[doc = "Bit 23 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_min_tx_pwr_flag(
        &mut self,
    ) -> RForceOutMinTxPwrFlagW<'_, RegAxPwrUlForce1Spec> {
        RForceOutMinTxPwrFlagW::new(self, 23)
    }
    #[doc = "Bit 24 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_min_tx_pwr_flag_on(
        &mut self,
    ) -> RForceOutMinTxPwrFlagOnW<'_, RegAxPwrUlForce1Spec> {
        RForceOutMinTxPwrFlagOnW::new(self, 24)
    }
    #[doc = "Bits 25:26 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_abort_tx_idx(&mut self) -> RForceOutAbortTxIdxW<'_, RegAxPwrUlForce1Spec> {
        RForceOutAbortTxIdxW::new(self, 25)
    }
    #[doc = "Bit 27 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_abort_tx_idx_on(
        &mut self,
    ) -> RForceOutAbortTxIdxOnW<'_, RegAxPwrUlForce1Spec> {
        RForceOutAbortTxIdxOnW::new(self, 27)
    }
    #[doc = "Bits 28:30 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_cfo_comp(&mut self) -> RForceOutCfoCompW<'_, RegAxPwrUlForce1Spec> {
        RForceOutCfoCompW::new(self, 28)
    }
    #[doc = "Bit 31 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_cfo_comp_on(&mut self) -> RForceOutCfoCompOnW<'_, RegAxPwrUlForce1Spec> {
        RForceOutCfoCompOnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_ul_force1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_ul_force1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrUlForce1Spec;
impl crate::RegisterSpec for RegAxPwrUlForce1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_ul_force1::R`](R) reader structure"]
impl crate::Readable for RegAxPwrUlForce1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_ul_force1::W`](W) writer structure"]
impl crate::Writable for RegAxPwrUlForce1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_UL_FORCE1 to value 0"]
impl crate::Resettable for RegAxPwrUlForce1Spec {}
