#[doc = "Register `RxFDP5` reader"]
pub type R = crate::R<RxFdp5Spec>;
#[doc = "Register `RxFDP5` writer"]
pub type W = crate::W<RxFdp5Spec>;
#[doc = "Field `RxFDP5` reader - Rx queue Descriptor Pointer to the Rx Ring"]
pub type RxFdp5R = crate::FieldReader<u32>;
#[doc = "Field `RxFDP5` writer - Rx queue Descriptor Pointer to the Rx Ring"]
pub type RxFdp5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Rx queue Descriptor Pointer to the Rx Ring"]
    #[inline(always)]
    pub fn rx_fdp5(&self) -> RxFdp5R {
        RxFdp5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Rx queue Descriptor Pointer to the Rx Ring"]
    #[inline(always)]
    pub fn rx_fdp5(&mut self) -> RxFdp5W<'_, RxFdp5Spec> {
        RxFdp5W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fdp5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_fdp5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxFdp5Spec;
impl crate::RegisterSpec for RxFdp5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_fdp5::R`](R) reader structure"]
impl crate::Readable for RxFdp5Spec {}
#[doc = "`write(|w| ..)` method takes [`rx_fdp5::W`](W) writer structure"]
impl crate::Writable for RxFdp5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RxFDP5 to value 0"]
impl crate::Resettable for RxFdp5Spec {}
