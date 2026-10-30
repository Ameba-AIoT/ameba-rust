#[doc = "Register `BT_VIR_E0_BIST_DRF_PAUSE0` reader"]
pub type R = crate::R<BtVirE0BistDrfPause0Spec>;
#[doc = "Field `bt_vir_e0_mbist_drf_pause_19_0` reader - "]
pub type BtVirE0MbistDrfPause19_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn bt_vir_e0_mbist_drf_pause_19_0(&self) -> BtVirE0MbistDrfPause19_0R {
        BtVirE0MbistDrfPause19_0R::new(self.bits & 0x000f_ffff)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`bt_vir_e0_bist_drf_pause0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BtVirE0BistDrfPause0Spec;
impl crate::RegisterSpec for BtVirE0BistDrfPause0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bt_vir_e0_bist_drf_pause0::R`](R) reader structure"]
impl crate::Readable for BtVirE0BistDrfPause0Spec {}
#[doc = "`reset()` method sets BT_VIR_E0_BIST_DRF_PAUSE0 to value 0"]
impl crate::Resettable for BtVirE0BistDrfPause0Spec {}
