#[doc = "Register `REG_RXBCN_COUNTER_CTRL2` reader"]
pub type R = crate::R<RegRxbcnCounterCtrl2Spec>;
#[doc = "Register `REG_RXBCN_COUNTER_CTRL2` writer"]
pub type W = crate::W<RegRxbcnCounterCtrl2Spec>;
#[doc = "Field `BIT_RXBCN_TIMRUP_MAX` reader - upper limit of bcn_timeup. If the bcn_timeup greater than BIT_RXBCN_TIMRUP_MAX, use the BIT_RXBCN_TIMRUP_MAX as bcn_timeup. UNIT: 32US. Default is 15ms"]
pub type BitRxbcnTimrupMaxR = crate::FieldReader<u16>;
#[doc = "Field `BIT_RXBCN_TIMRUP_MAX` writer - upper limit of bcn_timeup. If the bcn_timeup greater than BIT_RXBCN_TIMRUP_MAX, use the BIT_RXBCN_TIMRUP_MAX as bcn_timeup. UNIT: 32US. Default is 15ms"]
pub type BitRxbcnTimrupMaxW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_RXBCN_TIMEUP_MIN` reader - lower limit of bcn_timeup. If the bcn_timeup less than BIT_RXBCN_TIMRUP_MIN, use the BIT_RXBCN_TIMRUP_MIN as bcn_timeup. UNIT: 32US. Default is 2ms"]
pub type BitRxbcnTimeupMinR = crate::FieldReader<u16>;
#[doc = "Field `BIT_RXBCN_TIMEUP_MIN` writer - lower limit of bcn_timeup. If the bcn_timeup less than BIT_RXBCN_TIMRUP_MIN, use the BIT_RXBCN_TIMRUP_MIN as bcn_timeup. UNIT: 32US. Default is 2ms"]
pub type BitRxbcnTimeupMinW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_RXBCN_CNT` reader - statistic the times of RXBCN in RXBCN_CNT_PRD"]
pub type BitRxbcnCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11 - upper limit of bcn_timeup. If the bcn_timeup greater than BIT_RXBCN_TIMRUP_MAX, use the BIT_RXBCN_TIMRUP_MAX as bcn_timeup. UNIT: 32US. Default is 15ms"]
    #[inline(always)]
    pub fn bit_rxbcn_timrup_max(&self) -> BitRxbcnTimrupMaxR {
        BitRxbcnTimrupMaxR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - lower limit of bcn_timeup. If the bcn_timeup less than BIT_RXBCN_TIMRUP_MIN, use the BIT_RXBCN_TIMRUP_MIN as bcn_timeup. UNIT: 32US. Default is 2ms"]
    #[inline(always)]
    pub fn bit_rxbcn_timeup_min(&self) -> BitRxbcnTimeupMinR {
        BitRxbcnTimeupMinR::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bits 24:31 - statistic the times of RXBCN in RXBCN_CNT_PRD"]
    #[inline(always)]
    pub fn bit_rxbcn_cnt(&self) -> BitRxbcnCntR {
        BitRxbcnCntR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - upper limit of bcn_timeup. If the bcn_timeup greater than BIT_RXBCN_TIMRUP_MAX, use the BIT_RXBCN_TIMRUP_MAX as bcn_timeup. UNIT: 32US. Default is 15ms"]
    #[inline(always)]
    pub fn bit_rxbcn_timrup_max(&mut self) -> BitRxbcnTimrupMaxW<'_, RegRxbcnCounterCtrl2Spec> {
        BitRxbcnTimrupMaxW::new(self, 0)
    }
    #[doc = "Bits 12:23 - lower limit of bcn_timeup. If the bcn_timeup less than BIT_RXBCN_TIMRUP_MIN, use the BIT_RXBCN_TIMRUP_MIN as bcn_timeup. UNIT: 32US. Default is 2ms"]
    #[inline(always)]
    pub fn bit_rxbcn_timeup_min(&mut self) -> BitRxbcnTimeupMinW<'_, RegRxbcnCounterCtrl2Spec> {
        BitRxbcnTimeupMinW::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rxbcn_counter_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rxbcn_counter_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxbcnCounterCtrl2Spec;
impl crate::RegisterSpec for RegRxbcnCounterCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rxbcn_counter_ctrl2::R`](R) reader structure"]
impl crate::Readable for RegRxbcnCounterCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_rxbcn_counter_ctrl2::W`](W) writer structure"]
impl crate::Writable for RegRxbcnCounterCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RXBCN_COUNTER_CTRL2 to value 0x0004_01e0"]
impl crate::Resettable for RegRxbcnCounterCtrl2Spec {
    const RESET_VALUE: u32 = 0x0004_01e0;
}
