#[doc = "Register `MBIST_GRP_EN_BT_VIR_E00` reader"]
pub type R = crate::R<MbistGrpEnBtVirE00Spec>;
#[doc = "Register `MBIST_GRP_EN_BT_VIR_E00` writer"]
pub type W = crate::W<MbistGrpEnBtVirE00Spec>;
#[doc = "Field `bt_vir_e0_bist_en_19_0` reader - "]
pub type BtVirE0BistEn19_0R = crate::FieldReader<u32>;
#[doc = "Field `bt_vir_e0_bist_en_19_0` writer - "]
pub type BtVirE0BistEn19_0W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn bt_vir_e0_bist_en_19_0(&self) -> BtVirE0BistEn19_0R {
        BtVirE0BistEn19_0R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn bt_vir_e0_bist_en_19_0(&mut self) -> BtVirE0BistEn19_0W<'_, MbistGrpEnBtVirE00Spec> {
        BtVirE0BistEn19_0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mbist_grp_en_bt_vir_e00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbist_grp_en_bt_vir_e00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbistGrpEnBtVirE00Spec;
impl crate::RegisterSpec for MbistGrpEnBtVirE00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbist_grp_en_bt_vir_e00::R`](R) reader structure"]
impl crate::Readable for MbistGrpEnBtVirE00Spec {}
#[doc = "`write(|w| ..)` method takes [`mbist_grp_en_bt_vir_e00::W`](W) writer structure"]
impl crate::Writable for MbistGrpEnBtVirE00Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MBIST_GRP_EN_BT_VIR_E00 to value 0x000f_ffff"]
impl crate::Resettable for MbistGrpEnBtVirE00Spec {
    const RESET_VALUE: u32 = 0x000f_ffff;
}
