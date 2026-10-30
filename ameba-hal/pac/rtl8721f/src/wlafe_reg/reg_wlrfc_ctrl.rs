#[doc = "Register `REG_WLRFC_CTRL` reader"]
pub type R = crate::R<RegWlrfcCtrlSpec>;
#[doc = "Register `REG_WLRFC_CTRL` writer"]
pub type W = crate::W<RegWlrfcCtrlSpec>;
#[doc = "Field `wlrfc_pwc_en` reader - 1: gen request of wlan rfc power cut 0: gen request of turn off wlan rfc power cut"]
pub type WlrfcPwcEnR = crate::BitReader;
#[doc = "Field `wlrfc_pwc_en` writer - 1: gen request of wlan rfc power cut 0: gen request of turn off wlan rfc power cut"]
pub type WlrfcPwcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `iso_wlrfc_en` reader - Isolation control for wlan rfc 1: isolation 0: attach"]
pub type IsoWlrfcEnR = crate::BitReader;
#[doc = "Field `iso_wlrfc_en` writer - Isolation control for wlan rfc 1: isolation 0: attach"]
pub type IsoWlrfcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wlrfc_retain_en_type_1` reader - 1 to 0:gen req to restore register in power on flow 0 to 1:gen req to backup register in power on flow default should be 1,otherwise there be unexpected leakage power when VCC is power off and VCCG is still on"]
pub type WlrfcRetainEnType1R = crate::BitReader;
#[doc = "Field `wlrfc_retain_en_type_1` writer - 1 to 0:gen req to restore register in power on flow 0 to 1:gen req to backup register in power on flow default should be 1,otherwise there be unexpected leakage power when VCC is power off and VCCG is still on"]
pub type WlrfcRetainEnType1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wlrfc_pwc_sel` reader - Option to turn off wlrfc pwr cut 1: enable 0: disable RL6973: RFC without retention FF. default is 0"]
pub type WlrfcPwcSelR = crate::BitReader;
#[doc = "Field `wlrfc_pwc_sel` writer - Option to turn off wlrfc pwr cut 1: enable 0: disable RL6973: RFC without retention FF. default is 0"]
pub type WlrfcPwcSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wlafe_ckrfc_en` reader - 1: enable WLRFC clock; 0: gated"]
pub type WlafeCkrfcEnR = crate::BitReader;
#[doc = "Field `wlafe_ckrfc_en` writer - 1: enable WLRFC clock; 0: gated"]
pub type WlafeCkrfcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wlrfc_clk_sel` reader - rfc clock divider, it is based on 320m wlpll 0: div1 1: div2 2: div3 3/other: divider by this value + 1,(default is 3:80M)"]
pub type WlrfcClkSelR = crate::FieldReader;
#[doc = "Field `wlrfc_clk_sel` writer - rfc clock divider, it is based on 320m wlpll 0: div1 1: div2 2: div3 3/other: divider by this value + 1,(default is 3:80M)"]
pub type WlrfcClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `wlrfc_save_en_type_2` writer - enable saving value of retention DFF when wlmac power off or pg,should be assert when reset is high"]
pub type WlrfcSaveEnType2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wlrfc_restore_en_type_2` writer - enable restoring value of retention DFF when wlmac power on,should be assert when reset is high"]
pub type WlrfcRestoreEnType2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wlrfc_retention_cell_sel` reader - 1:choose retention cell type I(U22 ULP),which is controlled by 0x28\\[2\\] 0:choose retention cell type II(RTKM),which is controlled by 0x28\\[9:8\\]"]
pub type WlrfcRetentionCellSelR = crate::BitReader;
#[doc = "Field `wlrfc_retention_cell_sel` writer - 1:choose retention cell type I(U22 ULP),which is controlled by 0x28\\[2\\] 0:choose retention cell type II(RTKM),which is controlled by 0x28\\[9:8\\]"]
pub type WlrfcRetentionCellSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dis_wlbb_ckrfc_en` reader - the gating control of rfc clk is pta_ckrfc_en|wlbb_ckrfc_en|wlafe_ckrfc_en,and wlbb_ckrfc_en can not be control by register directly in RL6678,add option for top to control wlbb_ckrfc_en 1: disable wlbb_ckrfc_en; 0: enable wlbb_ckrfc_en"]
pub type DisWlbbCkrfcEnR = crate::BitReader;
#[doc = "Field `dis_wlbb_ckrfc_en` writer - the gating control of rfc clk is pta_ckrfc_en|wlbb_ckrfc_en|wlafe_ckrfc_en,and wlbb_ckrfc_en can not be control by register directly in RL6678,add option for top to control wlbb_ckrfc_en 1: disable wlbb_ckrfc_en; 0: enable wlbb_ckrfc_en"]
pub type DisWlbbCkrfcEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: gen request of wlan rfc power cut 0: gen request of turn off wlan rfc power cut"]
    #[inline(always)]
    pub fn wlrfc_pwc_en(&self) -> WlrfcPwcEnR {
        WlrfcPwcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Isolation control for wlan rfc 1: isolation 0: attach"]
    #[inline(always)]
    pub fn iso_wlrfc_en(&self) -> IsoWlrfcEnR {
        IsoWlrfcEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1 to 0:gen req to restore register in power on flow 0 to 1:gen req to backup register in power on flow default should be 1,otherwise there be unexpected leakage power when VCC is power off and VCCG is still on"]
    #[inline(always)]
    pub fn wlrfc_retain_en_type_1(&self) -> WlrfcRetainEnType1R {
        WlrfcRetainEnType1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Option to turn off wlrfc pwr cut 1: enable 0: disable RL6973: RFC without retention FF. default is 0"]
    #[inline(always)]
    pub fn wlrfc_pwc_sel(&self) -> WlrfcPwcSelR {
        WlrfcPwcSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: enable WLRFC clock; 0: gated"]
    #[inline(always)]
    pub fn wlafe_ckrfc_en(&self) -> WlafeCkrfcEnR {
        WlafeCkrfcEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - rfc clock divider, it is based on 320m wlpll 0: div1 1: div2 2: div3 3/other: divider by this value + 1,(default is 3:80M)"]
    #[inline(always)]
    pub fn wlrfc_clk_sel(&self) -> WlrfcClkSelR {
        WlrfcClkSelR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 10 - 1:choose retention cell type I(U22 ULP),which is controlled by 0x28\\[2\\] 0:choose retention cell type II(RTKM),which is controlled by 0x28\\[9:8\\]"]
    #[inline(always)]
    pub fn wlrfc_retention_cell_sel(&self) -> WlrfcRetentionCellSelR {
        WlrfcRetentionCellSelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - the gating control of rfc clk is pta_ckrfc_en|wlbb_ckrfc_en|wlafe_ckrfc_en,and wlbb_ckrfc_en can not be control by register directly in RL6678,add option for top to control wlbb_ckrfc_en 1: disable wlbb_ckrfc_en; 0: enable wlbb_ckrfc_en"]
    #[inline(always)]
    pub fn dis_wlbb_ckrfc_en(&self) -> DisWlbbCkrfcEnR {
        DisWlbbCkrfcEnR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: gen request of wlan rfc power cut 0: gen request of turn off wlan rfc power cut"]
    #[inline(always)]
    pub fn wlrfc_pwc_en(&mut self) -> WlrfcPwcEnW<'_, RegWlrfcCtrlSpec> {
        WlrfcPwcEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Isolation control for wlan rfc 1: isolation 0: attach"]
    #[inline(always)]
    pub fn iso_wlrfc_en(&mut self) -> IsoWlrfcEnW<'_, RegWlrfcCtrlSpec> {
        IsoWlrfcEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 1 to 0:gen req to restore register in power on flow 0 to 1:gen req to backup register in power on flow default should be 1,otherwise there be unexpected leakage power when VCC is power off and VCCG is still on"]
    #[inline(always)]
    pub fn wlrfc_retain_en_type_1(&mut self) -> WlrfcRetainEnType1W<'_, RegWlrfcCtrlSpec> {
        WlrfcRetainEnType1W::new(self, 2)
    }
    #[doc = "Bit 3 - Option to turn off wlrfc pwr cut 1: enable 0: disable RL6973: RFC without retention FF. default is 0"]
    #[inline(always)]
    pub fn wlrfc_pwc_sel(&mut self) -> WlrfcPwcSelW<'_, RegWlrfcCtrlSpec> {
        WlrfcPwcSelW::new(self, 3)
    }
    #[doc = "Bit 4 - 1: enable WLRFC clock; 0: gated"]
    #[inline(always)]
    pub fn wlafe_ckrfc_en(&mut self) -> WlafeCkrfcEnW<'_, RegWlrfcCtrlSpec> {
        WlafeCkrfcEnW::new(self, 4)
    }
    #[doc = "Bits 5:7 - rfc clock divider, it is based on 320m wlpll 0: div1 1: div2 2: div3 3/other: divider by this value + 1,(default is 3:80M)"]
    #[inline(always)]
    pub fn wlrfc_clk_sel(&mut self) -> WlrfcClkSelW<'_, RegWlrfcCtrlSpec> {
        WlrfcClkSelW::new(self, 5)
    }
    #[doc = "Bit 8 - enable saving value of retention DFF when wlmac power off or pg,should be assert when reset is high"]
    #[inline(always)]
    pub fn wlrfc_save_en_type_2(&mut self) -> WlrfcSaveEnType2W<'_, RegWlrfcCtrlSpec> {
        WlrfcSaveEnType2W::new(self, 8)
    }
    #[doc = "Bit 9 - enable restoring value of retention DFF when wlmac power on,should be assert when reset is high"]
    #[inline(always)]
    pub fn wlrfc_restore_en_type_2(&mut self) -> WlrfcRestoreEnType2W<'_, RegWlrfcCtrlSpec> {
        WlrfcRestoreEnType2W::new(self, 9)
    }
    #[doc = "Bit 10 - 1:choose retention cell type I(U22 ULP),which is controlled by 0x28\\[2\\] 0:choose retention cell type II(RTKM),which is controlled by 0x28\\[9:8\\]"]
    #[inline(always)]
    pub fn wlrfc_retention_cell_sel(&mut self) -> WlrfcRetentionCellSelW<'_, RegWlrfcCtrlSpec> {
        WlrfcRetentionCellSelW::new(self, 10)
    }
    #[doc = "Bit 11 - the gating control of rfc clk is pta_ckrfc_en|wlbb_ckrfc_en|wlafe_ckrfc_en,and wlbb_ckrfc_en can not be control by register directly in RL6678,add option for top to control wlbb_ckrfc_en 1: disable wlbb_ckrfc_en; 0: enable wlbb_ckrfc_en"]
    #[inline(always)]
    pub fn dis_wlbb_ckrfc_en(&mut self) -> DisWlbbCkrfcEnW<'_, RegWlrfcCtrlSpec> {
        DisWlbbCkrfcEnW::new(self, 11)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wlrfc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wlrfc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWlrfcCtrlSpec;
impl crate::RegisterSpec for RegWlrfcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wlrfc_ctrl::R`](R) reader structure"]
impl crate::Readable for RegWlrfcCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wlrfc_ctrl::W`](W) writer structure"]
impl crate::Writable for RegWlrfcCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WLRFC_CTRL to value 0x36"]
impl crate::Resettable for RegWlrfcCtrlSpec {
    const RESET_VALUE: u32 = 0x36;
}
