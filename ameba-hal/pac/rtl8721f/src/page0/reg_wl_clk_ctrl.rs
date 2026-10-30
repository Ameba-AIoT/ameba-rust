#[doc = "Register `REG_WL_CLK_CTRL` reader"]
pub type R = crate::R<RegWlClkCtrlSpec>;
#[doc = "Register `REG_WL_CLK_CTRL` writer"]
pub type W = crate::W<RegWlClkCtrlSpec>;
#[doc = "Field `BIT_WL_CKMAC_EN` reader - 1: enable MAC clock; 0: gated"]
pub type BitWlCkmacEnR = crate::BitReader;
#[doc = "Field `BIT_WL_CKMAC_EN` writer - 1: enable MAC clock; 0: gated"]
pub type BitWlCkmacEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WL_CKSEC_EN` reader - 1: enable MAC security engine clock; 0: gated"]
pub type BitWlCksecEnR = crate::BitReader;
#[doc = "Field `BIT_WL_CKSEC_EN` writer - 1: enable MAC security engine clock; 0: gated"]
pub type BitWlCksecEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WL_CK32K_EN` reader - 1: enable 32k clock; 0: gated"]
pub type BitWlCk32kEnR = crate::BitReader;
#[doc = "Field `BIT_WL_CK32K_EN` writer - 1: enable 32k clock; 0: gated"]
pub type BitWlCk32kEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WL_FQSEL_CK32K` reader - 1:32K frequency select 32.768KHz 0:32K frequency select 32KHz"]
pub type BitWlFqselCk32kR = crate::BitReader;
#[doc = "Field `BIT_WL_FQSEL_CK32K` writer - 1:32K frequency select 32.768KHz 0:32K frequency select 32KHz"]
pub type BitWlFqselCk32kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TSF_CKSLP_SEL` reader - 1: TSF clock switch to 32.768KHz/32kHz; 0: TSF clock switch to MAC clock"]
pub type BitTsfCkslpSelR = crate::BitReader;
#[doc = "Field `BIT_TSF_CKSLP_SEL` writer - 1: TSF clock switch to 32.768KHz/32kHz; 0: TSF clock switch to MAC clock"]
pub type BitTsfCkslpSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_IS_SCH_CLK_INSLP_V2` reader - WL TSF clock status: 0: MAC clock 1: 32K clock"]
pub type BitIsSchClkInslpV2R = crate::BitReader;
#[doc = "Field `BIT_WL_CKEN_IQDUMP` reader - 0: disable clock for iqdump (LA MODE) 1: enable clock for iqdump (LA MODE) same clock source as security clock"]
pub type BitWlCkenIqdumpR = crate::BitReader;
#[doc = "Field `BIT_WL_CKEN_IQDUMP` writer - 0: disable clock for iqdump (LA MODE) 1: enable clock for iqdump (LA MODE) same clock source as security clock"]
pub type BitWlCkenIqdumpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WLOFF_SWITCH_TSF_EN` reader - enable wlpmc switch TSF clock to 32K clock"]
pub type BitWloffSwitchTsfEnR = crate::BitReader;
#[doc = "Field `BIT_WLOFF_SWITCH_TSF_EN` writer - enable wlpmc switch TSF clock to 32K clock"]
pub type BitWloffSwitchTsfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WLON_SWITCH_TSF_EN` reader - enable wlpmc switch TSF clock to MAC clock"]
pub type BitWlonSwitchTsfEnR = crate::BitReader;
#[doc = "Field `BIT_WLON_SWITCH_TSF_EN` writer - enable wlpmc switch TSF clock to MAC clock"]
pub type BitWlonSwitchTsfEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: enable MAC clock; 0: gated"]
    #[inline(always)]
    pub fn bit_wl_ckmac_en(&self) -> BitWlCkmacEnR {
        BitWlCkmacEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: enable MAC security engine clock; 0: gated"]
    #[inline(always)]
    pub fn bit_wl_cksec_en(&self) -> BitWlCksecEnR {
        BitWlCksecEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: enable 32k clock; 0: gated"]
    #[inline(always)]
    pub fn bit_wl_ck32k_en(&self) -> BitWlCk32kEnR {
        BitWlCk32kEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - 1:32K frequency select 32.768KHz 0:32K frequency select 32KHz"]
    #[inline(always)]
    pub fn bit_wl_fqsel_ck32k(&self) -> BitWlFqselCk32kR {
        BitWlFqselCk32kR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: TSF clock switch to 32.768KHz/32kHz; 0: TSF clock switch to MAC clock"]
    #[inline(always)]
    pub fn bit_tsf_ckslp_sel(&self) -> BitTsfCkslpSelR {
        BitTsfCkslpSelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WL TSF clock status: 0: MAC clock 1: 32K clock"]
    #[inline(always)]
    pub fn bit_is_sch_clk_inslp_v2(&self) -> BitIsSchClkInslpV2R {
        BitIsSchClkInslpV2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 0: disable clock for iqdump (LA MODE) 1: enable clock for iqdump (LA MODE) same clock source as security clock"]
    #[inline(always)]
    pub fn bit_wl_cken_iqdump(&self) -> BitWlCkenIqdumpR {
        BitWlCkenIqdumpR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - enable wlpmc switch TSF clock to 32K clock"]
    #[inline(always)]
    pub fn bit_wloff_switch_tsf_en(&self) -> BitWloffSwitchTsfEnR {
        BitWloffSwitchTsfEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - enable wlpmc switch TSF clock to MAC clock"]
    #[inline(always)]
    pub fn bit_wlon_switch_tsf_en(&self) -> BitWlonSwitchTsfEnR {
        BitWlonSwitchTsfEnR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: enable MAC clock; 0: gated"]
    #[inline(always)]
    pub fn bit_wl_ckmac_en(&mut self) -> BitWlCkmacEnW<'_, RegWlClkCtrlSpec> {
        BitWlCkmacEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: enable MAC security engine clock; 0: gated"]
    #[inline(always)]
    pub fn bit_wl_cksec_en(&mut self) -> BitWlCksecEnW<'_, RegWlClkCtrlSpec> {
        BitWlCksecEnW::new(self, 1)
    }
    #[doc = "Bit 3 - 1: enable 32k clock; 0: gated"]
    #[inline(always)]
    pub fn bit_wl_ck32k_en(&mut self) -> BitWlCk32kEnW<'_, RegWlClkCtrlSpec> {
        BitWlCk32kEnW::new(self, 3)
    }
    #[doc = "Bit 5 - 1:32K frequency select 32.768KHz 0:32K frequency select 32KHz"]
    #[inline(always)]
    pub fn bit_wl_fqsel_ck32k(&mut self) -> BitWlFqselCk32kW<'_, RegWlClkCtrlSpec> {
        BitWlFqselCk32kW::new(self, 5)
    }
    #[doc = "Bit 8 - 1: TSF clock switch to 32.768KHz/32kHz; 0: TSF clock switch to MAC clock"]
    #[inline(always)]
    pub fn bit_tsf_ckslp_sel(&mut self) -> BitTsfCkslpSelW<'_, RegWlClkCtrlSpec> {
        BitTsfCkslpSelW::new(self, 8)
    }
    #[doc = "Bit 10 - 0: disable clock for iqdump (LA MODE) 1: enable clock for iqdump (LA MODE) same clock source as security clock"]
    #[inline(always)]
    pub fn bit_wl_cken_iqdump(&mut self) -> BitWlCkenIqdumpW<'_, RegWlClkCtrlSpec> {
        BitWlCkenIqdumpW::new(self, 10)
    }
    #[doc = "Bit 11 - enable wlpmc switch TSF clock to 32K clock"]
    #[inline(always)]
    pub fn bit_wloff_switch_tsf_en(&mut self) -> BitWloffSwitchTsfEnW<'_, RegWlClkCtrlSpec> {
        BitWloffSwitchTsfEnW::new(self, 11)
    }
    #[doc = "Bit 12 - enable wlpmc switch TSF clock to MAC clock"]
    #[inline(always)]
    pub fn bit_wlon_switch_tsf_en(&mut self) -> BitWlonSwitchTsfEnW<'_, RegWlClkCtrlSpec> {
        BitWlonSwitchTsfEnW::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wl_clk_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wl_clk_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWlClkCtrlSpec;
impl crate::RegisterSpec for RegWlClkCtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`reg_wl_clk_ctrl::R`](R) reader structure"]
impl crate::Readable for RegWlClkCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wl_clk_ctrl::W`](W) writer structure"]
impl crate::Writable for RegWlClkCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WL_CLK_CTRL to value 0"]
impl crate::Resettable for RegWlClkCtrlSpec {}
