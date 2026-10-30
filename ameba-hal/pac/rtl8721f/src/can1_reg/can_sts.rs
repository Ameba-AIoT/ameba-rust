#[doc = "Register `CAN_STS` reader"]
pub type R = crate::R<CanStsSpec>;
#[doc = "Field `bus_on_state` reader - bus state * 1 means the can module is already bus on * 0 means the can module is busoff"]
pub type BusOnStateR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - bus state * 1 means the can module is already bus on * 0 means the can module is busoff"]
    #[inline(always)]
    pub fn bus_on_state(&self) -> BusOnStateR {
        BusOnStateR::new((self.bits & 1) != 0)
    }
}
#[doc = "CAN bus status\n\nYou can [`read`](crate::Reg::read) this register and get [`can_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanStsSpec;
impl crate::RegisterSpec for CanStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_sts::R`](R) reader structure"]
impl crate::Readable for CanStsSpec {}
#[doc = "`reset()` method sets CAN_STS to value 0"]
impl crate::Resettable for CanStsSpec {}
