#[doc = "Register `AX_CLIENT_OM_CTRL` reader"]
pub type R = crate::R<AxClientOmCtrlSpec>;
#[doc = "Register `AX_CLIENT_OM_CTRL` writer"]
pub type W = crate::W<AxClientOmCtrlSpec>;
#[doc = "Field `BIT_UL_MU_DATA_DIS` reader - Disable to respond UL data triggered by basic trigger frame. Per port config. (BA/ACK/BSR/BQR are still allowed to Tx)"]
pub type BitUlMuDataDisR = crate::FieldReader;
#[doc = "Field `BIT_UL_MU_DATA_DIS` writer - Disable to respond UL data triggered by basic trigger frame. Per port config. (BA/ACK/BSR/BQR are still allowed to Tx)"]
pub type BitUlMuDataDisW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_UL_MU_DIS` reader - Disable to respond UL response triggered by any trigger frame. Per port config."]
pub type BitUlMuDisR = crate::FieldReader;
#[doc = "Field `BIT_UL_MU_DIS` writer - Disable to respond UL response triggered by any trigger frame. Per port config."]
pub type BitUlMuDisW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_WMAC_RXTB_IGNOREA2` reader - will not check a2 when receive tb ppdu"]
pub type BitWmacRxtbIgnorea2R = crate::BitReader;
#[doc = "Field `BIT_WMAC_RXTB_IGNOREA2` writer - will not check a2 when receive tb ppdu"]
pub type BitWmacRxtbIgnorea2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_CSI_ERR_FLAG` reader - clear csi_length error flag"]
pub type BitClrCsiErrFlagR = crate::BitReader;
#[doc = "Field `BIT_CLR_CSI_ERR_FLAG` writer - clear csi_length error flag"]
pub type BitClrCsiErrFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CSI_LEN_ERR_FLAG` reader - csi_length error flag"]
pub type BitCsiLenErrFlagR = crate::BitReader;
#[doc = "Field `BIT_RXTRIG_RU26_DIS` reader - disable response to trigger frame when ru_allocation addressed to me is 26-tone ru"]
pub type BitRxtrigRu26DisR = crate::BitReader;
#[doc = "Field `BIT_RXTRIG_RU26_DIS` writer - disable response to trigger frame when ru_allocation addressed to me is 26-tone ru"]
pub type BitRxtrigRu26DisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Disable to respond UL data triggered by basic trigger frame. Per port config. (BA/ACK/BSR/BQR are still allowed to Tx)"]
    #[inline(always)]
    pub fn bit_ul_mu_data_dis(&self) -> BitUlMuDataDisR {
        BitUlMuDataDisR::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Disable to respond UL response triggered by any trigger frame. Per port config."]
    #[inline(always)]
    pub fn bit_ul_mu_dis(&self) -> BitUlMuDisR {
        BitUlMuDisR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - will not check a2 when receive tb ppdu"]
    #[inline(always)]
    pub fn bit_wmac_rxtb_ignorea2(&self) -> BitWmacRxtbIgnorea2R {
        BitWmacRxtbIgnorea2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - clear csi_length error flag"]
    #[inline(always)]
    pub fn bit_clr_csi_err_flag(&self) -> BitClrCsiErrFlagR {
        BitClrCsiErrFlagR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - csi_length error flag"]
    #[inline(always)]
    pub fn bit_csi_len_err_flag(&self) -> BitCsiLenErrFlagR {
        BitCsiLenErrFlagR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - disable response to trigger frame when ru_allocation addressed to me is 26-tone ru"]
    #[inline(always)]
    pub fn bit_rxtrig_ru26_dis(&self) -> BitRxtrigRu26DisR {
        BitRxtrigRu26DisR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Disable to respond UL data triggered by basic trigger frame. Per port config. (BA/ACK/BSR/BQR are still allowed to Tx)"]
    #[inline(always)]
    pub fn bit_ul_mu_data_dis(&mut self) -> BitUlMuDataDisW<'_, AxClientOmCtrlSpec> {
        BitUlMuDataDisW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Disable to respond UL response triggered by any trigger frame. Per port config."]
    #[inline(always)]
    pub fn bit_ul_mu_dis(&mut self) -> BitUlMuDisW<'_, AxClientOmCtrlSpec> {
        BitUlMuDisW::new(self, 2)
    }
    #[doc = "Bit 4 - will not check a2 when receive tb ppdu"]
    #[inline(always)]
    pub fn bit_wmac_rxtb_ignorea2(&mut self) -> BitWmacRxtbIgnorea2W<'_, AxClientOmCtrlSpec> {
        BitWmacRxtbIgnorea2W::new(self, 4)
    }
    #[doc = "Bit 5 - clear csi_length error flag"]
    #[inline(always)]
    pub fn bit_clr_csi_err_flag(&mut self) -> BitClrCsiErrFlagW<'_, AxClientOmCtrlSpec> {
        BitClrCsiErrFlagW::new(self, 5)
    }
    #[doc = "Bit 7 - disable response to trigger frame when ru_allocation addressed to me is 26-tone ru"]
    #[inline(always)]
    pub fn bit_rxtrig_ru26_dis(&mut self) -> BitRxtrigRu26DisW<'_, AxClientOmCtrlSpec> {
        BitRxtrigRu26DisW::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ax_client_om_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ax_client_om_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxClientOmCtrlSpec;
impl crate::RegisterSpec for AxClientOmCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ax_client_om_ctrl::R`](R) reader structure"]
impl crate::Readable for AxClientOmCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ax_client_om_ctrl::W`](W) writer structure"]
impl crate::Writable for AxClientOmCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AX_CLIENT_OM_CTRL to value 0x10"]
impl crate::Resettable for AxClientOmCtrlSpec {
    const RESET_VALUE: u8 = 0x10;
}
