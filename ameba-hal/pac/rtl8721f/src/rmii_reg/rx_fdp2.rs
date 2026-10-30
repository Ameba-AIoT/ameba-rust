#[doc = "Register `RxFDP2` reader"]
pub type R = crate::R<RxFdp2Spec>;
#[doc = "Register `RxFDP2` writer"]
pub type W = crate::W<RxFdp2Spec>;
#[doc = "Field `RxFDP2` reader - Rx queue Descriptor Pointer to the Rx Ring"]
pub type RxFdp2R = crate::FieldReader<u32>;
#[doc = "Field `RxFDP2` writer - Rx queue Descriptor Pointer to the Rx Ring"]
pub type RxFdp2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Rx queue Descriptor Pointer to the Rx Ring"]
    #[inline(always)]
    pub fn rx_fdp2(&self) -> RxFdp2R {
        RxFdp2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Rx queue Descriptor Pointer to the Rx Ring"]
    #[inline(always)]
    pub fn rx_fdp2(&mut self) -> RxFdp2W<'_, RxFdp2Spec> {
        RxFdp2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fdp2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_fdp2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxFdp2Spec;
impl crate::RegisterSpec for RxFdp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_fdp2::R`](R) reader structure"]
impl crate::Readable for RxFdp2Spec {}
#[doc = "`write(|w| ..)` method takes [`rx_fdp2::W`](W) writer structure"]
impl crate::Writable for RxFdp2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RxFDP2 to value 0"]
impl crate::Resettable for RxFdp2Spec {}
