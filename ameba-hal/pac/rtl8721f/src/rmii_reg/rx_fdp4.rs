#[doc = "Register `RxFDP4` reader"]
pub type R = crate::R<RxFdp4Spec>;
#[doc = "Register `RxFDP4` writer"]
pub type W = crate::W<RxFdp4Spec>;
#[doc = "Field `RxFDP4` reader - Rx queue Descriptor Pointer to the Rx Ring"]
pub type RxFdp4R = crate::FieldReader<u32>;
#[doc = "Field `RxFDP4` writer - Rx queue Descriptor Pointer to the Rx Ring"]
pub type RxFdp4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Rx queue Descriptor Pointer to the Rx Ring"]
    #[inline(always)]
    pub fn rx_fdp4(&self) -> RxFdp4R {
        RxFdp4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Rx queue Descriptor Pointer to the Rx Ring"]
    #[inline(always)]
    pub fn rx_fdp4(&mut self) -> RxFdp4W<'_, RxFdp4Spec> {
        RxFdp4W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fdp4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_fdp4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxFdp4Spec;
impl crate::RegisterSpec for RxFdp4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_fdp4::R`](R) reader structure"]
impl crate::Readable for RxFdp4Spec {}
#[doc = "`write(|w| ..)` method takes [`rx_fdp4::W`](W) writer structure"]
impl crate::Writable for RxFdp4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RxFDP4 to value 0"]
impl crate::Resettable for RxFdp4Spec {}
