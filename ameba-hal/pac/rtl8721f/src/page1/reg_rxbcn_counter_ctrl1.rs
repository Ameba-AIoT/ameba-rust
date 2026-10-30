#[doc = "Register `REG_RXBCN_COUNTER_CTRL1` reader"]
pub type R = crate::R<RegRxbcnCounterCtrl1Spec>;
#[doc = "Register `REG_RXBCN_COUNTER_CTRL1` writer"]
pub type W = crate::W<RegRxbcnCounterCtrl1Spec>;
#[doc = "Field `BIT_RXBCN_THRES_HIGH` reader - upper limit of rxbcn. If the RXBCN.COUNTER > THRES_HIGH, RXBCN timeup period - (2^BIT_RXBCN_TIMEUP_ADJSP_DOWN)*32US"]
pub type BitRxbcnThresHighR = crate::FieldReader;
#[doc = "Field `BIT_RXBCN_THRES_HIGH` writer - upper limit of rxbcn. If the RXBCN.COUNTER > THRES_HIGH, RXBCN timeup period - (2^BIT_RXBCN_TIMEUP_ADJSP_DOWN)*32US"]
pub type BitRxbcnThresHighW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_RXBCN_THRES_LOW` reader - lower limit of rxbcn. If the RXBCN.COUNTER < THRES_LOW, RXBCN timeup period + (2^BIT_RXBCN_TIMEUP_ADJSP_UP)*32US"]
pub type BitRxbcnThresLowR = crate::FieldReader;
#[doc = "Field `BIT_RXBCN_THRES_LOW` writer - lower limit of rxbcn. If the RXBCN.COUNTER < THRES_LOW, RXBCN timeup period + (2^BIT_RXBCN_TIMEUP_ADJSP_UP)*32US"]
pub type BitRxbcnThresLowW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_RXBCN_TIMEUP_ADJSP_UP` reader - the adjust step of the rxbcn timeup period adjustment when increase the period. UNIT: (2^BIT_RXBCN_TIMEUP_ADJSP_UP)*32US"]
pub type BitRxbcnTimeupAdjspUpR = crate::FieldReader;
#[doc = "Field `BIT_RXBCN_TIMEUP_ADJSP_UP` writer - the adjust step of the rxbcn timeup period adjustment when increase the period. UNIT: (2^BIT_RXBCN_TIMEUP_ADJSP_UP)*32US"]
pub type BitRxbcnTimeupAdjspUpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_RXBCN_TIMEUP_ADJSP_DOWN` reader - the adjust step of the rxbcn timeup period adjustment when decrease the period. UNIT: (2^BIT_RXBCN_TIMEUP_ADJSP_DOWN)*32US"]
pub type BitRxbcnTimeupAdjspDownR = crate::FieldReader;
#[doc = "Field `BIT_RXBCN_TIMEUP_ADJSP_DOWN` writer - the adjust step of the rxbcn timeup period adjustment when decrease the period. UNIT: (2^BIT_RXBCN_TIMEUP_ADJSP_DOWN)*32US"]
pub type BitRxbcnTimeupAdjspDownW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_RXBCN_CNT_PRD` reader - 2^(BIT_RXBCN_CNT_PRD), default is 128 times"]
pub type BitRxbcnCntPrdR = crate::FieldReader;
#[doc = "Field `BIT_RXBCN_CNT_PRD` writer - 2^(BIT_RXBCN_CNT_PRD), default is 128 times"]
pub type BitRxbcnCntPrdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_EN_RXBCN_CNT` reader - 1: statistic the number of RXBCN in specified period"]
pub type BitEnRxbcnCntR = crate::BitReader;
#[doc = "Field `BIT_EN_RXBCN_CNT` writer - 1: statistic the number of RXBCN in specified period"]
pub type BitEnRxbcnCntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - upper limit of rxbcn. If the RXBCN.COUNTER > THRES_HIGH, RXBCN timeup period - (2^BIT_RXBCN_TIMEUP_ADJSP_DOWN)*32US"]
    #[inline(always)]
    pub fn bit_rxbcn_thres_high(&self) -> BitRxbcnThresHighR {
        BitRxbcnThresHighR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - lower limit of rxbcn. If the RXBCN.COUNTER < THRES_LOW, RXBCN timeup period + (2^BIT_RXBCN_TIMEUP_ADJSP_UP)*32US"]
    #[inline(always)]
    pub fn bit_rxbcn_thres_low(&self) -> BitRxbcnThresLowR {
        BitRxbcnThresLowR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - the adjust step of the rxbcn timeup period adjustment when increase the period. UNIT: (2^BIT_RXBCN_TIMEUP_ADJSP_UP)*32US"]
    #[inline(always)]
    pub fn bit_rxbcn_timeup_adjsp_up(&self) -> BitRxbcnTimeupAdjspUpR {
        BitRxbcnTimeupAdjspUpR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - the adjust step of the rxbcn timeup period adjustment when decrease the period. UNIT: (2^BIT_RXBCN_TIMEUP_ADJSP_DOWN)*32US"]
    #[inline(always)]
    pub fn bit_rxbcn_timeup_adjsp_down(&self) -> BitRxbcnTimeupAdjspDownR {
        BitRxbcnTimeupAdjspDownR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:27 - 2^(BIT_RXBCN_CNT_PRD), default is 128 times"]
    #[inline(always)]
    pub fn bit_rxbcn_cnt_prd(&self) -> BitRxbcnCntPrdR {
        BitRxbcnCntPrdR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - 1: statistic the number of RXBCN in specified period"]
    #[inline(always)]
    pub fn bit_en_rxbcn_cnt(&self) -> BitEnRxbcnCntR {
        BitEnRxbcnCntR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - upper limit of rxbcn. If the RXBCN.COUNTER > THRES_HIGH, RXBCN timeup period - (2^BIT_RXBCN_TIMEUP_ADJSP_DOWN)*32US"]
    #[inline(always)]
    pub fn bit_rxbcn_thres_high(&mut self) -> BitRxbcnThresHighW<'_, RegRxbcnCounterCtrl1Spec> {
        BitRxbcnThresHighW::new(self, 0)
    }
    #[doc = "Bits 8:15 - lower limit of rxbcn. If the RXBCN.COUNTER < THRES_LOW, RXBCN timeup period + (2^BIT_RXBCN_TIMEUP_ADJSP_UP)*32US"]
    #[inline(always)]
    pub fn bit_rxbcn_thres_low(&mut self) -> BitRxbcnThresLowW<'_, RegRxbcnCounterCtrl1Spec> {
        BitRxbcnThresLowW::new(self, 8)
    }
    #[doc = "Bits 16:18 - the adjust step of the rxbcn timeup period adjustment when increase the period. UNIT: (2^BIT_RXBCN_TIMEUP_ADJSP_UP)*32US"]
    #[inline(always)]
    pub fn bit_rxbcn_timeup_adjsp_up(
        &mut self,
    ) -> BitRxbcnTimeupAdjspUpW<'_, RegRxbcnCounterCtrl1Spec> {
        BitRxbcnTimeupAdjspUpW::new(self, 16)
    }
    #[doc = "Bits 20:22 - the adjust step of the rxbcn timeup period adjustment when decrease the period. UNIT: (2^BIT_RXBCN_TIMEUP_ADJSP_DOWN)*32US"]
    #[inline(always)]
    pub fn bit_rxbcn_timeup_adjsp_down(
        &mut self,
    ) -> BitRxbcnTimeupAdjspDownW<'_, RegRxbcnCounterCtrl1Spec> {
        BitRxbcnTimeupAdjspDownW::new(self, 20)
    }
    #[doc = "Bits 24:27 - 2^(BIT_RXBCN_CNT_PRD), default is 128 times"]
    #[inline(always)]
    pub fn bit_rxbcn_cnt_prd(&mut self) -> BitRxbcnCntPrdW<'_, RegRxbcnCounterCtrl1Spec> {
        BitRxbcnCntPrdW::new(self, 24)
    }
    #[doc = "Bit 31 - 1: statistic the number of RXBCN in specified period"]
    #[inline(always)]
    pub fn bit_en_rxbcn_cnt(&mut self) -> BitEnRxbcnCntW<'_, RegRxbcnCounterCtrl1Spec> {
        BitEnRxbcnCntW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rxbcn_counter_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rxbcn_counter_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxbcnCounterCtrl1Spec;
impl crate::RegisterSpec for RegRxbcnCounterCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rxbcn_counter_ctrl1::R`](R) reader structure"]
impl crate::Readable for RegRxbcnCounterCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_rxbcn_counter_ctrl1::W`](W) writer structure"]
impl crate::Writable for RegRxbcnCounterCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RXBCN_COUNTER_CTRL1 to value 0x0711_0000"]
impl crate::Resettable for RegRxbcnCounterCtrl1Spec {
    const RESET_VALUE: u32 = 0x0711_0000;
}
