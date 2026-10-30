#[doc = "Register `REG_SR_CONTROL_DBG` reader"]
pub type R = crate::R<RegSrControlDbgSpec>;
#[doc = "Field `BIT_SR_TIMER_TX` reader - time can be used for spatial reuse"]
pub type BitSrTimerTxR = crate::FieldReader<u16>;
#[doc = "Field `BIT_SR_PD_TX` reader - the value of received power level minus obss_pd_min"]
pub type BitSrPdTxR = crate::FieldReader<u16>;
#[doc = "Field `BIT_SR_RESTRICTED` reader - the spatial reuse type is restricted"]
pub type BitSrRestrictedR = crate::BitReader;
#[doc = "Field `BIT_SR_UPD_EN` reader - the received pkt is allowed to update basic nav"]
pub type BitSrUpdEnR = crate::BitReader;
#[doc = "Field `BIT_RXPPDU_SRG` reader - the received pkt is an srg ppdu"]
pub type BitRxppduSrgR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - time can be used for spatial reuse"]
    #[inline(always)]
    pub fn bit_sr_timer_tx(&self) -> BitSrTimerTxR {
        BitSrTimerTxR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:24 - the value of received power level minus obss_pd_min"]
    #[inline(always)]
    pub fn bit_sr_pd_tx(&self) -> BitSrPdTxR {
        BitSrPdTxR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 25 - the spatial reuse type is restricted"]
    #[inline(always)]
    pub fn bit_sr_restricted(&self) -> BitSrRestrictedR {
        BitSrRestrictedR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - the received pkt is allowed to update basic nav"]
    #[inline(always)]
    pub fn bit_sr_upd_en(&self) -> BitSrUpdEnR {
        BitSrUpdEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - the received pkt is an srg ppdu"]
    #[inline(always)]
    pub fn bit_rxppdu_srg(&self) -> BitRxppduSrgR {
        BitRxppduSrgR::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sr_control_dbg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSrControlDbgSpec;
impl crate::RegisterSpec for RegSrControlDbgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sr_control_dbg::R`](R) reader structure"]
impl crate::Readable for RegSrControlDbgSpec {}
#[doc = "`reset()` method sets REG_SR_CONTROL_DBG to value 0"]
impl crate::Resettable for RegSrControlDbgSpec {}
