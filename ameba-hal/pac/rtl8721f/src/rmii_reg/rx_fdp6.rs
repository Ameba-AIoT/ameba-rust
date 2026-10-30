#[doc = "Register `RxFDP6` reader"]
pub type R = crate::R<RxFdp6Spec>;
#[doc = "Register `RxFDP6` writer"]
pub type W = crate::W<RxFdp6Spec>;
#[doc = "Field `RxFDP6` reader - Rx queue Descriptor Pointer to the Rx Ring"]
pub type RxFdp6R = crate::FieldReader<u32>;
#[doc = "Field `RxFDP6` writer - Rx queue Descriptor Pointer to the Rx Ring"]
pub type RxFdp6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Rx queue Descriptor Pointer to the Rx Ring"]
    #[inline(always)]
    pub fn rx_fdp6(&self) -> RxFdp6R {
        RxFdp6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Rx queue Descriptor Pointer to the Rx Ring"]
    #[inline(always)]
    pub fn rx_fdp6(&mut self) -> RxFdp6W<'_, RxFdp6Spec> {
        RxFdp6W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fdp6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_fdp6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxFdp6Spec;
impl crate::RegisterSpec for RxFdp6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_fdp6::R`](R) reader structure"]
impl crate::Readable for RxFdp6Spec {}
#[doc = "`write(|w| ..)` method takes [`rx_fdp6::W`](W) writer structure"]
impl crate::Writable for RxFdp6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RxFDP6 to value 0"]
impl crate::Resettable for RxFdp6Spec {}
