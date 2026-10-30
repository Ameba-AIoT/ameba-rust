#[doc = "Register `RxFDP3` reader"]
pub type R = crate::R<RxFdp3Spec>;
#[doc = "Register `RxFDP3` writer"]
pub type W = crate::W<RxFdp3Spec>;
#[doc = "Field `RxFDP3` reader - Rx queue Descriptor Pointer to the Rx Ring"]
pub type RxFdp3R = crate::FieldReader<u32>;
#[doc = "Field `RxFDP3` writer - Rx queue Descriptor Pointer to the Rx Ring"]
pub type RxFdp3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Rx queue Descriptor Pointer to the Rx Ring"]
    #[inline(always)]
    pub fn rx_fdp3(&self) -> RxFdp3R {
        RxFdp3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Rx queue Descriptor Pointer to the Rx Ring"]
    #[inline(always)]
    pub fn rx_fdp3(&mut self) -> RxFdp3W<'_, RxFdp3Spec> {
        RxFdp3W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fdp3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_fdp3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxFdp3Spec;
impl crate::RegisterSpec for RxFdp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_fdp3::R`](R) reader structure"]
impl crate::Readable for RxFdp3Spec {}
#[doc = "`write(|w| ..)` method takes [`rx_fdp3::W`](W) writer structure"]
impl crate::Writable for RxFdp3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RxFDP3 to value 0"]
impl crate::Resettable for RxFdp3Spec {}
