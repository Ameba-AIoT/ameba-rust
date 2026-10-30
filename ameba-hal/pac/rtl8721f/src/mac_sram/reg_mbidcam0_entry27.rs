#[doc = "Register `REG_MBIDCAM0_ENTRY27` reader"]
pub type R = crate::R<RegMbidcam0Entry27Spec>;
#[doc = "Register `REG_MBIDCAM0_ENTRY27` writer"]
pub type W = crate::W<RegMbidcam0Entry27Spec>;
#[doc = "Field `MBIDCAM0_ENTRY27_BSSID_MACID0` reader - "]
pub type Mbidcam0Entry27BssidMacid0R = crate::FieldReader<u32>;
#[doc = "Field `MBIDCAM0_ENTRY27_BSSID_MACID0` writer - "]
pub type Mbidcam0Entry27BssidMacid0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mbidcam0_entry27_bssid_macid0(&self) -> Mbidcam0Entry27BssidMacid0R {
        Mbidcam0Entry27BssidMacid0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mbidcam0_entry27_bssid_macid0(
        &mut self,
    ) -> Mbidcam0Entry27BssidMacid0W<'_, RegMbidcam0Entry27Spec> {
        Mbidcam0Entry27BssidMacid0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mbidcam0_entry27::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mbidcam0_entry27::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMbidcam0Entry27Spec;
impl crate::RegisterSpec for RegMbidcam0Entry27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mbidcam0_entry27::R`](R) reader structure"]
impl crate::Readable for RegMbidcam0Entry27Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_mbidcam0_entry27::W`](W) writer structure"]
impl crate::Writable for RegMbidcam0Entry27Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MBIDCAM0_ENTRY27 to value 0"]
impl crate::Resettable for RegMbidcam0Entry27Spec {}
