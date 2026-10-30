#[doc = "Register `REG_CLKDIV_CKSEL` reader"]
pub type R = crate::R<RegClkdivCkselSpec>;
#[doc = "Register `REG_CLKDIV_CKSEL` writer"]
pub type W = crate::W<RegClkdivCkselSpec>;
#[doc = "Field `BIT_WL_80M_PHASE` reader - 80M security clock phase control"]
pub type BitWl80mPhaseR = crate::FieldReader;
#[doc = "Field `BIT_WL_80M_PHASE` writer - 80M security clock phase control"]
pub type BitWl80mPhaseW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_WL_40M_PHASE` reader - 40M MAC clock phase control"]
pub type BitWl40mPhaseR = crate::FieldReader;
#[doc = "Field `BIT_WL_40M_PHASE` writer - 40M MAC clock phase control"]
pub type BitWl40mPhaseW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_WL_RFC_PHASE` reader - 80M WLRFC clock phase control"]
pub type BitWlRfcPhaseR = crate::FieldReader;
#[doc = "Field `BIT_WL_RFC_PHASE` writer - 80M WLRFC clock phase control"]
pub type BitWlRfcPhaseW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_WL_CLK_SYNC` reader - 1: sync 40M/80M clock phase with PLL"]
pub type BitWlClkSyncR = crate::BitReader;
#[doc = "Field `BIT_WL_CLK_SYNC` writer - 1: sync 40M/80M clock phase with PLL"]
pub type BitWlClkSyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MAC_CLK_SEL` reader - 11/10:20M 01:40M 00:80M"]
pub type BitMacClkSelR = crate::FieldReader;
#[doc = "Field `BIT_MAC_CLK_SEL` writer - 11/10:20M 01:40M 00:80M"]
pub type BitMacClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_SEC_CLK_SEL` reader - 11/10:40M 01:80M 00:160M"]
pub type BitSecClkSelR = crate::FieldReader;
#[doc = "Field `BIT_SEC_CLK_SEL` writer - 11/10:40M 01:80M 00:160M"]
pub type BitSecClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - 80M security clock phase control"]
    #[inline(always)]
    pub fn bit_wl_80m_phase(&self) -> BitWl80mPhaseR {
        BitWl80mPhaseR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 40M MAC clock phase control"]
    #[inline(always)]
    pub fn bit_wl_40m_phase(&self) -> BitWl40mPhaseR {
        BitWl40mPhaseR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 80M WLRFC clock phase control"]
    #[inline(always)]
    pub fn bit_wl_rfc_phase(&self) -> BitWlRfcPhaseR {
        BitWlRfcPhaseR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - 1: sync 40M/80M clock phase with PLL"]
    #[inline(always)]
    pub fn bit_wl_clk_sync(&self) -> BitWlClkSyncR {
        BitWlClkSyncR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 18:19 - 11/10:20M 01:40M 00:80M"]
    #[inline(always)]
    pub fn bit_mac_clk_sel(&self) -> BitMacClkSelR {
        BitMacClkSelR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - 11/10:40M 01:80M 00:160M"]
    #[inline(always)]
    pub fn bit_sec_clk_sel(&self) -> BitSecClkSelR {
        BitSecClkSelR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 80M security clock phase control"]
    #[inline(always)]
    pub fn bit_wl_80m_phase(&mut self) -> BitWl80mPhaseW<'_, RegClkdivCkselSpec> {
        BitWl80mPhaseW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 40M MAC clock phase control"]
    #[inline(always)]
    pub fn bit_wl_40m_phase(&mut self) -> BitWl40mPhaseW<'_, RegClkdivCkselSpec> {
        BitWl40mPhaseW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 80M WLRFC clock phase control"]
    #[inline(always)]
    pub fn bit_wl_rfc_phase(&mut self) -> BitWlRfcPhaseW<'_, RegClkdivCkselSpec> {
        BitWlRfcPhaseW::new(self, 8)
    }
    #[doc = "Bit 15 - 1: sync 40M/80M clock phase with PLL"]
    #[inline(always)]
    pub fn bit_wl_clk_sync(&mut self) -> BitWlClkSyncW<'_, RegClkdivCkselSpec> {
        BitWlClkSyncW::new(self, 15)
    }
    #[doc = "Bits 18:19 - 11/10:20M 01:40M 00:80M"]
    #[inline(always)]
    pub fn bit_mac_clk_sel(&mut self) -> BitMacClkSelW<'_, RegClkdivCkselSpec> {
        BitMacClkSelW::new(self, 18)
    }
    #[doc = "Bits 20:21 - 11/10:40M 01:80M 00:160M"]
    #[inline(always)]
    pub fn bit_sec_clk_sel(&mut self) -> BitSecClkSelW<'_, RegClkdivCkselSpec> {
        BitSecClkSelW::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_clkdiv_cksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_clkdiv_cksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegClkdivCkselSpec;
impl crate::RegisterSpec for RegClkdivCkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_clkdiv_cksel::R`](R) reader structure"]
impl crate::Readable for RegClkdivCkselSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_clkdiv_cksel::W`](W) writer structure"]
impl crate::Writable for RegClkdivCkselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CLKDIV_CKSEL to value 0x0014_0000"]
impl crate::Resettable for RegClkdivCkselSpec {
    const RESET_VALUE: u32 = 0x0014_0000;
}
