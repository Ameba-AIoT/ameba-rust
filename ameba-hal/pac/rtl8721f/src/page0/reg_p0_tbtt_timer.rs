#[doc = "Register `REG_P0_TBTT_TIMER` reader"]
pub type R = crate::R<RegP0TbttTimerSpec>;
#[doc = "Field `P0_128US_CNT` reader - Unit: 128us"]
pub type P0_128usCntR = crate::FieldReader;
#[doc = "Field `P0_BCN_CNT_DOWN` reader - Unit: 1ms beacon_cnt_down - 1"]
pub type P0BcnCntDownR = crate::FieldReader<u16>;
#[doc = "Field `P0_ELY_BCNINT_CNT` reader - Unit: bcn_space the total time to next specified TBTT is (new_ely_agg_cnt - 1) * r_bcn_space(0x554) + (beacon_cnt_down - 1)* 1ms + (cnt_128us + 1)* 128us"]
pub type P0ElyBcnintCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Unit: 128us"]
    #[inline(always)]
    pub fn p0_128us_cnt(&self) -> P0_128usCntR {
        P0_128usCntR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:18 - Unit: 1ms beacon_cnt_down - 1"]
    #[inline(always)]
    pub fn p0_bcn_cnt_down(&self) -> P0BcnCntDownR {
        P0BcnCntDownR::new(((self.bits >> 3) & 0xffff) as u16)
    }
    #[doc = "Bits 24:30 - Unit: bcn_space the total time to next specified TBTT is (new_ely_agg_cnt - 1) * r_bcn_space(0x554) + (beacon_cnt_down - 1)* 1ms + (cnt_128us + 1)* 128us"]
    #[inline(always)]
    pub fn p0_ely_bcnint_cnt(&self) -> P0ElyBcnintCntR {
        P0ElyBcnintCntR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_p0_tbtt_timer::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegP0TbttTimerSpec;
impl crate::RegisterSpec for RegP0TbttTimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_p0_tbtt_timer::R`](R) reader structure"]
impl crate::Readable for RegP0TbttTimerSpec {}
#[doc = "`reset()` method sets REG_P0_TBTT_TIMER to value 0"]
impl crate::Resettable for RegP0TbttTimerSpec {}
