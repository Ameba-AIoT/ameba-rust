#[doc = "Register `REG_AX_ARFR_WEIGHTING_TABLE` reader"]
pub type R = crate::R<RegAxArfrWeightingTableSpec>;
#[doc = "Register `REG_AX_ARFR_WEIGHTING_TABLE` writer"]
pub type W = crate::W<RegAxArfrWeightingTableSpec>;
#[doc = "Field `BIT_rate0_weighting` reader - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is CCK1M"]
pub type BitRate0WeightingR = crate::FieldReader;
#[doc = "Field `BIT_rate0_weighting` writer - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is CCK1M"]
pub type BitRate0WeightingW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_rate1_weighting` reader - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is CCK2M"]
pub type BitRate1WeightingR = crate::FieldReader;
#[doc = "Field `BIT_rate1_weighting` writer - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is CCK2M"]
pub type BitRate1WeightingW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_rate2_weighting` reader - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is CCK5_5M"]
pub type BitRate2WeightingR = crate::FieldReader;
#[doc = "Field `BIT_rate2_weighting` writer - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is CCK5_5M"]
pub type BitRate2WeightingW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_rate3_weighting` reader - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is CCK11M"]
pub type BitRate3WeightingR = crate::FieldReader;
#[doc = "Field `BIT_rate3_weighting` writer - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is CCK11M"]
pub type BitRate3WeightingW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_rate4_weighting` reader - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is OFDM6M"]
pub type BitRate4WeightingR = crate::FieldReader;
#[doc = "Field `BIT_rate4_weighting` writer - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is OFDM6M"]
pub type BitRate4WeightingW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_rate5_weighting` reader - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is OFDM9M"]
pub type BitRate5WeightingR = crate::FieldReader;
#[doc = "Field `BIT_rate5_weighting` writer - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is OFDM9M"]
pub type BitRate5WeightingW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_rate6_weighting` reader - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is OFDM12M"]
pub type BitRate6WeightingR = crate::FieldReader;
#[doc = "Field `BIT_rate6_weighting` writer - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is OFDM12M"]
pub type BitRate6WeightingW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_rate7_weighting` reader - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is OFDM18M"]
pub type BitRate7WeightingR = crate::FieldReader;
#[doc = "Field `BIT_rate7_weighting` writer - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is OFDM18M"]
pub type BitRate7WeightingW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is CCK1M"]
    #[inline(always)]
    pub fn bit_rate0_weighting(&self) -> BitRate0WeightingR {
        BitRate0WeightingR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is CCK2M"]
    #[inline(always)]
    pub fn bit_rate1_weighting(&self) -> BitRate1WeightingR {
        BitRate1WeightingR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is CCK5_5M"]
    #[inline(always)]
    pub fn bit_rate2_weighting(&self) -> BitRate2WeightingR {
        BitRate2WeightingR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is CCK11M"]
    #[inline(always)]
    pub fn bit_rate3_weighting(&self) -> BitRate3WeightingR {
        BitRate3WeightingR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is OFDM6M"]
    #[inline(always)]
    pub fn bit_rate4_weighting(&self) -> BitRate4WeightingR {
        BitRate4WeightingR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is OFDM9M"]
    #[inline(always)]
    pub fn bit_rate5_weighting(&self) -> BitRate5WeightingR {
        BitRate5WeightingR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is OFDM12M"]
    #[inline(always)]
    pub fn bit_rate6_weighting(&self) -> BitRate6WeightingR {
        BitRate6WeightingR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is OFDM18M"]
    #[inline(always)]
    pub fn bit_rate7_weighting(&self) -> BitRate7WeightingR {
        BitRate7WeightingR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is CCK1M"]
    #[inline(always)]
    pub fn bit_rate0_weighting(&mut self) -> BitRate0WeightingW<'_, RegAxArfrWeightingTableSpec> {
        BitRate0WeightingW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is CCK2M"]
    #[inline(always)]
    pub fn bit_rate1_weighting(&mut self) -> BitRate1WeightingW<'_, RegAxArfrWeightingTableSpec> {
        BitRate1WeightingW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is CCK5_5M"]
    #[inline(always)]
    pub fn bit_rate2_weighting(&mut self) -> BitRate2WeightingW<'_, RegAxArfrWeightingTableSpec> {
        BitRate2WeightingW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is CCK11M"]
    #[inline(always)]
    pub fn bit_rate3_weighting(&mut self) -> BitRate3WeightingW<'_, RegAxArfrWeightingTableSpec> {
        BitRate3WeightingW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is OFDM6M"]
    #[inline(always)]
    pub fn bit_rate4_weighting(&mut self) -> BitRate4WeightingW<'_, RegAxArfrWeightingTableSpec> {
        BitRate4WeightingW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is OFDM9M"]
    #[inline(always)]
    pub fn bit_rate5_weighting(&mut self) -> BitRate5WeightingW<'_, RegAxArfrWeightingTableSpec> {
        BitRate5WeightingW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is OFDM12M"]
    #[inline(always)]
    pub fn bit_rate6_weighting(&mut self) -> BitRate6WeightingW<'_, RegAxArfrWeightingTableSpec> {
        BitRate6WeightingW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Weighting value of the corresponding data rate with respect to rate_sel in reg 0x1480\\[2:0\\] 0: disable fallback to the rate 1~15: valid weighting value default is OFDM18M"]
    #[inline(always)]
    pub fn bit_rate7_weighting(&mut self) -> BitRate7WeightingW<'_, RegAxArfrWeightingTableSpec> {
        BitRate7WeightingW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_arfr_weighting_table::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_arfr_weighting_table::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxArfrWeightingTableSpec;
impl crate::RegisterSpec for RegAxArfrWeightingTableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_arfr_weighting_table::R`](R) reader structure"]
impl crate::Readable for RegAxArfrWeightingTableSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_arfr_weighting_table::W`](W) writer structure"]
impl crate::Writable for RegAxArfrWeightingTableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_ARFR_WEIGHTING_TABLE to value 0x3211_2111"]
impl crate::Resettable for RegAxArfrWeightingTableSpec {
    const RESET_VALUE: u32 = 0x3211_2111;
}
