#[doc = "Register `REG_DUMMY_AIP_INFO0` reader"]
pub type R = crate::R<RegDummyAipInfo0Spec>;
#[doc = "Field `aip_info_rsvd0` reader - reserved for aip info indication"]
pub type AipInfoRsvd0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - reserved for aip info indication"]
    #[inline(always)]
    pub fn aip_info_rsvd0(&self) -> AipInfoRsvd0R {
        AipInfoRsvd0R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_dummy_aip_info0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDummyAipInfo0Spec;
impl crate::RegisterSpec for RegDummyAipInfo0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_dummy_aip_info0::R`](R) reader structure"]
impl crate::Readable for RegDummyAipInfo0Spec {}
#[doc = "`reset()` method sets REG_DUMMY_AIP_INFO0 to value 0"]
impl crate::Resettable for RegDummyAipInfo0Spec {}
