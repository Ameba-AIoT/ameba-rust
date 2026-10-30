#[doc = "Register `REG_FSIMR_V1` reader"]
pub type R = crate::R<RegFsimrV1Spec>;
#[doc = "Register `REG_FSIMR_V1` writer"]
pub type W = crate::W<RegFsimrV1Spec>;
#[doc = "Field `BIT_FS_BTON_STS_UPDATE_INT_EN` reader - BTON status update interrupt mask When 0x78\\[31\\] toggled, this interrupt is issue to indicate the status updated."]
pub type BitFsBtonStsUpdateIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_BTON_STS_UPDATE_INT_EN` writer - BTON status update interrupt mask When 0x78\\[31\\] toggled, this interrupt is issue to indicate the status updated."]
pub type BitFsBtonStsUpdateIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PWMERR_INT_EN_V1` reader - access page0 timeout error mask"]
pub type BitFsPwmerrIntEnV1R = crate::BitReader;
#[doc = "Field `BIT_FS_PWMERR_INT_EN_V1` writer - access page0 timeout error mask"]
pub type BitFsPwmerrIntEnV1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BTON status update interrupt mask When 0x78\\[31\\] toggled, this interrupt is issue to indicate the status updated."]
    #[inline(always)]
    pub fn bit_fs_bton_sts_update_int_en(&self) -> BitFsBtonStsUpdateIntEnR {
        BitFsBtonStsUpdateIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - access page0 timeout error mask"]
    #[inline(always)]
    pub fn bit_fs_pwmerr_int_en_v1(&self) -> BitFsPwmerrIntEnV1R {
        BitFsPwmerrIntEnV1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BTON status update interrupt mask When 0x78\\[31\\] toggled, this interrupt is issue to indicate the status updated."]
    #[inline(always)]
    pub fn bit_fs_bton_sts_update_int_en(
        &mut self,
    ) -> BitFsBtonStsUpdateIntEnW<'_, RegFsimrV1Spec> {
        BitFsBtonStsUpdateIntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - access page0 timeout error mask"]
    #[inline(always)]
    pub fn bit_fs_pwmerr_int_en_v1(&mut self) -> BitFsPwmerrIntEnV1W<'_, RegFsimrV1Spec> {
        BitFsPwmerrIntEnV1W::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fsimr_v1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fsimr_v1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFsimrV1Spec;
impl crate::RegisterSpec for RegFsimrV1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fsimr_v1::R`](R) reader structure"]
impl crate::Readable for RegFsimrV1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_fsimr_v1::W`](W) writer structure"]
impl crate::Writable for RegFsimrV1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FSIMR_V1 to value 0"]
impl crate::Resettable for RegFsimrV1Spec {}
