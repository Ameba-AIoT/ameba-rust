#[doc = "Register `REG_AON_BUS_ST` reader"]
pub type R = crate::R<RegAonBusStSpec>;
#[doc = "Field `aon_resp_timeout` reader - This flag indicate system access aon domain timeout , access maybe failed. Active 1. Write 1 will clear the status."]
pub type AonRespTimeoutR = crate::BitReader;
#[doc = "Field `rtc_resp_timeout` reader - This flag indicate system access rtc domain timeout , access maybe failed. Active 1. Write 1 will clear the status."]
pub type RtcRespTimeoutR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This flag indicate system access aon domain timeout , access maybe failed. Active 1. Write 1 will clear the status."]
    #[inline(always)]
    pub fn aon_resp_timeout(&self) -> AonRespTimeoutR {
        AonRespTimeoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - This flag indicate system access rtc domain timeout , access maybe failed. Active 1. Write 1 will clear the status."]
    #[inline(always)]
    pub fn rtc_resp_timeout(&self) -> RtcRespTimeoutR {
        RtcRespTimeoutR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_aon_bus_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAonBusStSpec;
impl crate::RegisterSpec for RegAonBusStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_aon_bus_st::R`](R) reader structure"]
impl crate::Readable for RegAonBusStSpec {}
#[doc = "`reset()` method sets REG_AON_BUS_ST to value 0"]
impl crate::Resettable for RegAonBusStSpec {}
