#[doc = "Register `REG_TSF_CLK_STATE_RXBCN_TIME` reader"]
pub type R = crate::R<RegTsfClkStateRxbcnTimeSpec>;
#[doc = "Register `REG_TSF_CLK_STATE_RXBCN_TIME` writer"]
pub type W = crate::W<RegTsfClkStateRxbcnTimeSpec>;
#[doc = "Field `BIT_CLK_TSF_IS32K` reader - Which Clock is Used by TSF Clock 1: 32K Clock 0: MAC Clock"]
pub type BitClkTsfIs32kR = crate::BitReader;
#[doc = "Field `BIT_RXBCN_TIMEUP_COUNT1_THRES` reader - threshold for rxbcn timeup counter,when 0x10A\\[14:8\\] equals to this value,generate interrupt"]
pub type BitRxbcnTimeupCount1ThresR = crate::FieldReader;
#[doc = "Field `BIT_RXBCN_TIMEUP_COUNT1_THRES` writer - threshold for rxbcn timeup counter,when 0x10A\\[14:8\\] equals to this value,generate interrupt"]
pub type BitRxbcnTimeupCount1ThresW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BIT_RXBCN_TIMEUP_COUNT1_CLR` reader - clear rxbcn timeup counter1"]
pub type BitRxbcnTimeupCount1ClrR = crate::BitReader;
#[doc = "Field `BIT_RXBCN_TIMEUP_COUNT1_CLR` writer - clear rxbcn timeup counter1"]
pub type BitRxbcnTimeupCount1ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXBCN_TIMEUP_COUNT1` reader - rxbcn timeup counter1"]
pub type BitRxbcnTimeupCount1R = crate::FieldReader;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::BitReader;
impl R {
    #[doc = "Bit 15 - Which Clock is Used by TSF Clock 1: 32K Clock 0: MAC Clock"]
    #[inline(always)]
    pub fn bit_clk_tsf_is32k(&self) -> BitClkTsfIs32kR {
        BitClkTsfIs32kR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - threshold for rxbcn timeup counter,when 0x10A\\[14:8\\] equals to this value,generate interrupt"]
    #[inline(always)]
    pub fn bit_rxbcn_timeup_count1_thres(&self) -> BitRxbcnTimeupCount1ThresR {
        BitRxbcnTimeupCount1ThresR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - clear rxbcn timeup counter1"]
    #[inline(always)]
    pub fn bit_rxbcn_timeup_count1_clr(&self) -> BitRxbcnTimeupCount1ClrR {
        BitRxbcnTimeupCount1ClrR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - rxbcn timeup counter1"]
    #[inline(always)]
    pub fn bit_rxbcn_timeup_count1(&self) -> BitRxbcnTimeupCount1R {
        BitRxbcnTimeupCount1R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:22 - threshold for rxbcn timeup counter,when 0x10A\\[14:8\\] equals to this value,generate interrupt"]
    #[inline(always)]
    pub fn bit_rxbcn_timeup_count1_thres(
        &mut self,
    ) -> BitRxbcnTimeupCount1ThresW<'_, RegTsfClkStateRxbcnTimeSpec> {
        BitRxbcnTimeupCount1ThresW::new(self, 16)
    }
    #[doc = "Bit 23 - clear rxbcn timeup counter1"]
    #[inline(always)]
    pub fn bit_rxbcn_timeup_count1_clr(
        &mut self,
    ) -> BitRxbcnTimeupCount1ClrW<'_, RegTsfClkStateRxbcnTimeSpec> {
        BitRxbcnTimeupCount1ClrW::new(self, 23)
    }
}
#[doc = "TSF CLK state and Beacon timeup counter1\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tsf_clk_state_rxbcn_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tsf_clk_state_rxbcn_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTsfClkStateRxbcnTimeSpec;
impl crate::RegisterSpec for RegTsfClkStateRxbcnTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tsf_clk_state_rxbcn_time::R`](R) reader structure"]
impl crate::Readable for RegTsfClkStateRxbcnTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tsf_clk_state_rxbcn_time::W`](W) writer structure"]
impl crate::Writable for RegTsfClkStateRxbcnTimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TSF_CLK_STATE_RXBCN_TIME to value 0x001e_0000"]
impl crate::Resettable for RegTsfClkStateRxbcnTimeSpec {
    const RESET_VALUE: u32 = 0x001e_0000;
}
