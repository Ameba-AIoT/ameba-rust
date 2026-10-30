#[doc = "Register `REG_DUMMY_AIP_CTRL0` reader"]
pub type R = crate::R<RegDummyAipCtrl0Spec>;
#[doc = "Register `REG_DUMMY_AIP_CTRL0` writer"]
pub type W = crate::W<RegDummyAipCtrl0Spec>;
#[doc = "Field `aip_ctrl_rsvd0` reader - reserved for aip control"]
pub type AipCtrlRsvd0R = crate::FieldReader;
#[doc = "Field `aip_ctrl_rsvd0` writer - reserved for aip control"]
pub type AipCtrlRsvd0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - reserved for aip control"]
    #[inline(always)]
    pub fn aip_ctrl_rsvd0(&self) -> AipCtrlRsvd0R {
        AipCtrlRsvd0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - reserved for aip control"]
    #[inline(always)]
    pub fn aip_ctrl_rsvd0(&mut self) -> AipCtrlRsvd0W<'_, RegDummyAipCtrl0Spec> {
        AipCtrlRsvd0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_dummy_aip_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_dummy_aip_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDummyAipCtrl0Spec;
impl crate::RegisterSpec for RegDummyAipCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_dummy_aip_ctrl0::R`](R) reader structure"]
impl crate::Readable for RegDummyAipCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_dummy_aip_ctrl0::W`](W) writer structure"]
impl crate::Writable for RegDummyAipCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DUMMY_AIP_CTRL0 to value 0x0f"]
impl crate::Resettable for RegDummyAipCtrl0Spec {
    const RESET_VALUE: u32 = 0x0f;
}
