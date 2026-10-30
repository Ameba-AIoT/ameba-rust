#[doc = "Register `REG_BCNDROP_TO_MISC_CTRL` reader"]
pub type R = crate::R<RegBcndropToMiscCtrlSpec>;
#[doc = "Register `REG_BCNDROP_TO_MISC_CTRL` writer"]
pub type W = crate::W<RegBcndropToMiscCtrlSpec>;
#[doc = "Field `BIT_BEACON_DROP_IVL` reader - indicate the timeout of beacon drop"]
pub type BitBeaconDropIvlR = crate::FieldReader;
#[doc = "Field `BIT_BEACON_DROP_IVL` writer - indicate the timeout of beacon drop"]
pub type BitBeaconDropIvlW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BIT_BEACON_DROP_EN` reader - enable beacon drop function"]
pub type BitBeaconDropEnR = crate::BitReader;
#[doc = "Field `BIT_BEACON_DROP_EN` writer - enable beacon drop function"]
pub type BitBeaconDropEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HGQ_TIMEOUT_PERIOD` reader - When hgq empty for this period, close atim window. And this will block hgq dma for pcie, unless 0x524\\[12\\] set enable. TU: 32us"]
pub type BitHgqTimeoutPeriodR = crate::FieldReader;
#[doc = "Field `BIT_HGQ_TIMEOUT_PERIOD` writer - When hgq empty for this period, close atim window. And this will block hgq dma for pcie, unless 0x524\\[12\\] set enable. TU: 32us"]
pub type BitHgqTimeoutPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_TXCMD_TIMEOUT_PERIOD` reader - When 40M/80M TX CMD ready and secondary_CCA_20/secondary_CCA_40 set for this period, then the TX CMD will be cleared. TU: 1us"]
pub type BitTxcmdTimeoutPeriodR = crate::FieldReader;
#[doc = "Field `BIT_TXCMD_TIMEOUT_PERIOD` writer - When 40M/80M TX CMD ready and secondary_CCA_20/secondary_CCA_40 set for this period, then the TX CMD will be cleared. TU: 1us"]
pub type BitTxcmdTimeoutPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_TSF_TIMER_2_SEL` reader - genarete tsfime_int\\[2\\] 00: PORT0's tsft 01: PORT1's tsft 10:PORT2's tsft 11: FREERUN CUT"]
pub type BitTsfTimer2SelR = crate::FieldReader;
#[doc = "Field `BIT_TSF_TIMER_2_SEL` writer - genarete tsfime_int\\[2\\] 00: PORT0's tsft 01: PORT1's tsft 10:PORT2's tsft 11: FREERUN CUT"]
pub type BitTsfTimer2SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_TSF_TIMER_1_SEL` reader - genarete tsfime_int\\[1\\] 00: PORT0's tsft 01: PORT1's tsft 10:PORT2's tsft 11: FREERUN CUT"]
pub type BitTsfTimer1SelR = crate::FieldReader;
#[doc = "Field `BIT_TSF_TIMER_1_SEL` writer - genarete tsfime_int\\[1\\] 00: PORT0's tsft 01: PORT1's tsft 10:PORT2's tsft 11: FREERUN CUT"]
pub type BitTsfTimer1SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_TSF_TIMER_0_SEL` reader - genarete tsfime_int\\[0\\] 00: PORT0's tsft 01: PORT1's tsft 10:PORT2's tsft 11: FREERUN CUT"]
pub type BitTsfTimer0SelR = crate::FieldReader;
#[doc = "Field `BIT_TSF_TIMER_0_SEL` writer - genarete tsfime_int\\[0\\] 00: PORT0's tsft 01: PORT1's tsft 10:PORT2's tsft 11: FREERUN CUT"]
pub type BitTsfTimer0SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - indicate the timeout of beacon drop"]
    #[inline(always)]
    pub fn bit_beacon_drop_ivl(&self) -> BitBeaconDropIvlR {
        BitBeaconDropIvlR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - enable beacon drop function"]
    #[inline(always)]
    pub fn bit_beacon_drop_en(&self) -> BitBeaconDropEnR {
        BitBeaconDropEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - When hgq empty for this period, close atim window. And this will block hgq dma for pcie, unless 0x524\\[12\\] set enable. TU: 32us"]
    #[inline(always)]
    pub fn bit_hgq_timeout_period(&self) -> BitHgqTimeoutPeriodR {
        BitHgqTimeoutPeriodR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - When 40M/80M TX CMD ready and secondary_CCA_20/secondary_CCA_40 set for this period, then the TX CMD will be cleared. TU: 1us"]
    #[inline(always)]
    pub fn bit_txcmd_timeout_period(&self) -> BitTxcmdTimeoutPeriodR {
        BitTxcmdTimeoutPeriodR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 26:27 - genarete tsfime_int\\[2\\] 00: PORT0's tsft 01: PORT1's tsft 10:PORT2's tsft 11: FREERUN CUT"]
    #[inline(always)]
    pub fn bit_tsf_timer_2_sel(&self) -> BitTsfTimer2SelR {
        BitTsfTimer2SelR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - genarete tsfime_int\\[1\\] 00: PORT0's tsft 01: PORT1's tsft 10:PORT2's tsft 11: FREERUN CUT"]
    #[inline(always)]
    pub fn bit_tsf_timer_1_sel(&self) -> BitTsfTimer1SelR {
        BitTsfTimer1SelR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - genarete tsfime_int\\[0\\] 00: PORT0's tsft 01: PORT1's tsft 10:PORT2's tsft 11: FREERUN CUT"]
    #[inline(always)]
    pub fn bit_tsf_timer_0_sel(&self) -> BitTsfTimer0SelR {
        BitTsfTimer0SelR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - indicate the timeout of beacon drop"]
    #[inline(always)]
    pub fn bit_beacon_drop_ivl(&mut self) -> BitBeaconDropIvlW<'_, RegBcndropToMiscCtrlSpec> {
        BitBeaconDropIvlW::new(self, 0)
    }
    #[doc = "Bit 7 - enable beacon drop function"]
    #[inline(always)]
    pub fn bit_beacon_drop_en(&mut self) -> BitBeaconDropEnW<'_, RegBcndropToMiscCtrlSpec> {
        BitBeaconDropEnW::new(self, 7)
    }
    #[doc = "Bits 8:15 - When hgq empty for this period, close atim window. And this will block hgq dma for pcie, unless 0x524\\[12\\] set enable. TU: 32us"]
    #[inline(always)]
    pub fn bit_hgq_timeout_period(&mut self) -> BitHgqTimeoutPeriodW<'_, RegBcndropToMiscCtrlSpec> {
        BitHgqTimeoutPeriodW::new(self, 8)
    }
    #[doc = "Bits 16:23 - When 40M/80M TX CMD ready and secondary_CCA_20/secondary_CCA_40 set for this period, then the TX CMD will be cleared. TU: 1us"]
    #[inline(always)]
    pub fn bit_txcmd_timeout_period(
        &mut self,
    ) -> BitTxcmdTimeoutPeriodW<'_, RegBcndropToMiscCtrlSpec> {
        BitTxcmdTimeoutPeriodW::new(self, 16)
    }
    #[doc = "Bits 26:27 - genarete tsfime_int\\[2\\] 00: PORT0's tsft 01: PORT1's tsft 10:PORT2's tsft 11: FREERUN CUT"]
    #[inline(always)]
    pub fn bit_tsf_timer_2_sel(&mut self) -> BitTsfTimer2SelW<'_, RegBcndropToMiscCtrlSpec> {
        BitTsfTimer2SelW::new(self, 26)
    }
    #[doc = "Bits 28:29 - genarete tsfime_int\\[1\\] 00: PORT0's tsft 01: PORT1's tsft 10:PORT2's tsft 11: FREERUN CUT"]
    #[inline(always)]
    pub fn bit_tsf_timer_1_sel(&mut self) -> BitTsfTimer1SelW<'_, RegBcndropToMiscCtrlSpec> {
        BitTsfTimer1SelW::new(self, 28)
    }
    #[doc = "Bits 30:31 - genarete tsfime_int\\[0\\] 00: PORT0's tsft 01: PORT1's tsft 10:PORT2's tsft 11: FREERUN CUT"]
    #[inline(always)]
    pub fn bit_tsf_timer_0_sel(&mut self) -> BitTsfTimer0SelW<'_, RegBcndropToMiscCtrlSpec> {
        BitTsfTimer0SelW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bcndrop_to_misc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bcndrop_to_misc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBcndropToMiscCtrlSpec;
impl crate::RegisterSpec for RegBcndropToMiscCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bcndrop_to_misc_ctrl::R`](R) reader structure"]
impl crate::Readable for RegBcndropToMiscCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bcndrop_to_misc_ctrl::W`](W) writer structure"]
impl crate::Writable for RegBcndropToMiscCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BCNDROP_TO_MISC_CTRL to value 0x00ff_1000"]
impl crate::Resettable for RegBcndropToMiscCtrlSpec {
    const RESET_VALUE: u32 = 0x00ff_1000;
}
