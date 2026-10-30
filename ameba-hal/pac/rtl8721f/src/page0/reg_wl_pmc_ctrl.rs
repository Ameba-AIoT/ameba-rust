#[doc = "Register `REG_WL_PMC_CTRL` reader"]
pub type R = crate::R<RegWlPmcCtrlSpec>;
#[doc = "Register `REG_WL_PMC_CTRL` writer"]
pub type W = crate::W<RegWlPmcCtrlSpec>;
#[doc = "Field `BIT_WL_PMC_ONMAC` reader - Inform WLAN PMC FSM to power On mode, include clock, isolation, power control for MAC only (clear when Power Ready)"]
pub type BitWlPmcOnmacR = crate::BitReader;
#[doc = "Field `BIT_WL_PMC_ONMAC` writer - Inform WLAN PMC FSM to power On mode, include clock, isolation, power control for MAC only (clear when Power Ready)"]
pub type BitWlPmcOnmacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WL_PMC_OFFMAC` reader - Inform WLAN PMC FSM to power OFF mode, include clock, isolation, power control for MAC only (clear when Power Ready)"]
pub type BitWlPmcOffmacR = crate::BitReader;
#[doc = "Field `BIT_WL_PMC_OFFMAC` writer - Inform WLAN PMC FSM to power OFF mode, include clock, isolation, power control for MAC only (clear when Power Ready)"]
pub type BitWlPmcOffmacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WL_PMC_LPS_EN` reader - Inform WLAN PMC FSM to enter LPS mode. (auto clear when ready)"]
pub type BitWlPmcLpsEnR = crate::BitReader;
#[doc = "Field `BIT_WL_PMC_LPS_EN` writer - Inform WLAN PMC FSM to enter LPS mode. (auto clear when ready)"]
pub type BitWlPmcLpsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CUR_PST_IS_LPS` reader - indicate current WL power status is LPS mode"]
pub type BitCurPstIsLpsR = crate::BitReader;
#[doc = "Field `BIT_WL_PMC_CNT_PAR` reader - WL PMC FSM counting unit."]
pub type BitWlPmcCntParR = crate::FieldReader;
#[doc = "Field `BIT_WL_PMC_CNT_PAR` writer - WL PMC FSM counting unit."]
pub type BitWlPmcCntParW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WL_retention_cell_sel` reader - 1:choose retention cell type I(U22 ULP),which is controlled by 0x20\\[15\\] 2:choose retention cell type II(RTKM),which is controlled by 0x20\\[17:16\\]"]
pub type WlRetentionCellSelR = crate::BitReader;
#[doc = "Field `WL_retention_cell_sel` writer - 1:choose retention cell type I(U22 ULP),which is controlled by 0x20\\[15\\] 2:choose retention cell type II(RTKM),which is controlled by 0x20\\[17:16\\]"]
pub type WlRetentionCellSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WL_retain_en_type_1` reader - 1 to 0:gen req to restore register in power on flow 0 to 1:gen req to backup register in power on flow default should be 1,otherwise there be unexpected leakage power when VCC is power off and VCCG is still on"]
pub type WlRetainEnType1R = crate::BitReader;
#[doc = "Field `WL_retain_en_type_1` writer - 1 to 0:gen req to restore register in power on flow 0 to 1:gen req to backup register in power on flow default should be 1,otherwise there be unexpected leakage power when VCC is power off and VCCG is still on"]
pub type WlRetainEnType1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WL_restore_en_type_2` writer - enable restoring value of retention DFF when wlmac power on,should be assert when reset is high"]
pub type WlRestoreEnType2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WL_save_en_type_2` writer - enable saving value of retention DFF when wlmac power off or pg,should be assert when reset is high"]
pub type WlSaveEnType2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Inform WLAN PMC FSM to power On mode, include clock, isolation, power control for MAC only (clear when Power Ready)"]
    #[inline(always)]
    pub fn bit_wl_pmc_onmac(&self) -> BitWlPmcOnmacR {
        BitWlPmcOnmacR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Inform WLAN PMC FSM to power OFF mode, include clock, isolation, power control for MAC only (clear when Power Ready)"]
    #[inline(always)]
    pub fn bit_wl_pmc_offmac(&self) -> BitWlPmcOffmacR {
        BitWlPmcOffmacR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Inform WLAN PMC FSM to enter LPS mode. (auto clear when ready)"]
    #[inline(always)]
    pub fn bit_wl_pmc_lps_en(&self) -> BitWlPmcLpsEnR {
        BitWlPmcLpsEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - indicate current WL power status is LPS mode"]
    #[inline(always)]
    pub fn bit_cur_pst_is_lps(&self) -> BitCurPstIsLpsR {
        BitCurPstIsLpsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:13 - WL PMC FSM counting unit."]
    #[inline(always)]
    pub fn bit_wl_pmc_cnt_par(&self) -> BitWlPmcCntParR {
        BitWlPmcCntParR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - 1:choose retention cell type I(U22 ULP),which is controlled by 0x20\\[15\\] 2:choose retention cell type II(RTKM),which is controlled by 0x20\\[17:16\\]"]
    #[inline(always)]
    pub fn wl_retention_cell_sel(&self) -> WlRetentionCellSelR {
        WlRetentionCellSelR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1 to 0:gen req to restore register in power on flow 0 to 1:gen req to backup register in power on flow default should be 1,otherwise there be unexpected leakage power when VCC is power off and VCCG is still on"]
    #[inline(always)]
    pub fn wl_retain_en_type_1(&self) -> WlRetainEnType1R {
        WlRetainEnType1R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Inform WLAN PMC FSM to power On mode, include clock, isolation, power control for MAC only (clear when Power Ready)"]
    #[inline(always)]
    pub fn bit_wl_pmc_onmac(&mut self) -> BitWlPmcOnmacW<'_, RegWlPmcCtrlSpec> {
        BitWlPmcOnmacW::new(self, 0)
    }
    #[doc = "Bit 1 - Inform WLAN PMC FSM to power OFF mode, include clock, isolation, power control for MAC only (clear when Power Ready)"]
    #[inline(always)]
    pub fn bit_wl_pmc_offmac(&mut self) -> BitWlPmcOffmacW<'_, RegWlPmcCtrlSpec> {
        BitWlPmcOffmacW::new(self, 1)
    }
    #[doc = "Bit 2 - Inform WLAN PMC FSM to enter LPS mode. (auto clear when ready)"]
    #[inline(always)]
    pub fn bit_wl_pmc_lps_en(&mut self) -> BitWlPmcLpsEnW<'_, RegWlPmcCtrlSpec> {
        BitWlPmcLpsEnW::new(self, 2)
    }
    #[doc = "Bits 8:13 - WL PMC FSM counting unit."]
    #[inline(always)]
    pub fn bit_wl_pmc_cnt_par(&mut self) -> BitWlPmcCntParW<'_, RegWlPmcCtrlSpec> {
        BitWlPmcCntParW::new(self, 8)
    }
    #[doc = "Bit 14 - 1:choose retention cell type I(U22 ULP),which is controlled by 0x20\\[15\\] 2:choose retention cell type II(RTKM),which is controlled by 0x20\\[17:16\\]"]
    #[inline(always)]
    pub fn wl_retention_cell_sel(&mut self) -> WlRetentionCellSelW<'_, RegWlPmcCtrlSpec> {
        WlRetentionCellSelW::new(self, 14)
    }
    #[doc = "Bit 15 - 1 to 0:gen req to restore register in power on flow 0 to 1:gen req to backup register in power on flow default should be 1,otherwise there be unexpected leakage power when VCC is power off and VCCG is still on"]
    #[inline(always)]
    pub fn wl_retain_en_type_1(&mut self) -> WlRetainEnType1W<'_, RegWlPmcCtrlSpec> {
        WlRetainEnType1W::new(self, 15)
    }
    #[doc = "Bit 16 - enable restoring value of retention DFF when wlmac power on,should be assert when reset is high"]
    #[inline(always)]
    pub fn wl_restore_en_type_2(&mut self) -> WlRestoreEnType2W<'_, RegWlPmcCtrlSpec> {
        WlRestoreEnType2W::new(self, 16)
    }
    #[doc = "Bit 17 - enable saving value of retention DFF when wlmac power off or pg,should be assert when reset is high"]
    #[inline(always)]
    pub fn wl_save_en_type_2(&mut self) -> WlSaveEnType2W<'_, RegWlPmcCtrlSpec> {
        WlSaveEnType2W::new(self, 17)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wl_pmc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wl_pmc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWlPmcCtrlSpec;
impl crate::RegisterSpec for RegWlPmcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wl_pmc_ctrl::R`](R) reader structure"]
impl crate::Readable for RegWlPmcCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wl_pmc_ctrl::W`](W) writer structure"]
impl crate::Writable for RegWlPmcCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WL_PMC_CTRL to value 0x9f00"]
impl crate::Resettable for RegWlPmcCtrlSpec {
    const RESET_VALUE: u32 = 0x9f00;
}
