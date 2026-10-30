#[doc = "Register `REG_WAK_STATUS0` reader"]
pub type R = crate::R<RegWakStatus0Spec>;
#[doc = "Field `wak_evt0_sta_29_0` reader - Acoording chip_arch spec"]
pub type WakEvt0Sta29_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29 - Acoording chip_arch spec"]
    #[inline(always)]
    pub fn wak_evt0_sta_29_0(&self) -> WakEvt0Sta29_0R {
        WakEvt0Sta29_0R::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wak_status0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWakStatus0Spec;
impl crate::RegisterSpec for RegWakStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wak_status0::R`](R) reader structure"]
impl crate::Readable for RegWakStatus0Spec {}
#[doc = "`reset()` method sets REG_WAK_STATUS0 to value 0"]
impl crate::Resettable for RegWakStatus0Spec {}
