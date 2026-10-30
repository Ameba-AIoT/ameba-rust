#[doc = "Register `REG_WAK_STATUS1` reader"]
pub type R = crate::R<RegWakStatus1Spec>;
#[doc = "Field `wak_evt0_sta_59_30` reader - Acoording chip_arch spec"]
pub type WakEvt0Sta59_30R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29 - Acoording chip_arch spec"]
    #[inline(always)]
    pub fn wak_evt0_sta_59_30(&self) -> WakEvt0Sta59_30R {
        WakEvt0Sta59_30R::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wak_status1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWakStatus1Spec;
impl crate::RegisterSpec for RegWakStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wak_status1::R`](R) reader structure"]
impl crate::Readable for RegWakStatus1Spec {}
#[doc = "`reset()` method sets REG_WAK_STATUS1 to value 0"]
impl crate::Resettable for RegWakStatus1Spec {}
