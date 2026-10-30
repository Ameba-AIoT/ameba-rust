#[doc = "Register `Rx_FDP1` reader"]
pub type R = crate::R<RxFdp1Spec>;
#[doc = "Register `Rx_FDP1` writer"]
pub type W = crate::W<RxFdp1Spec>;
#[doc = "Field `RxFDP1` reader - Rx 1st queue Descriptor Pointer to the Rx Ring"]
pub type RxFdp1R = crate::FieldReader<u32>;
#[doc = "Field `RxFDP1` writer - Rx 1st queue Descriptor Pointer to the Rx Ring"]
pub type RxFdp1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Rx 1st queue Descriptor Pointer to the Rx Ring"]
    #[inline(always)]
    pub fn rx_fdp1(&self) -> RxFdp1R {
        RxFdp1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Rx 1st queue Descriptor Pointer to the Rx Ring"]
    #[inline(always)]
    pub fn rx_fdp1(&mut self) -> RxFdp1W<'_, RxFdp1Spec> {
        RxFdp1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fdp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_fdp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxFdp1Spec;
impl crate::RegisterSpec for RxFdp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_fdp1::R`](R) reader structure"]
impl crate::Readable for RxFdp1Spec {}
#[doc = "`write(|w| ..)` method takes [`rx_fdp1::W`](W) writer structure"]
impl crate::Writable for RxFdp1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Rx_FDP1 to value 0"]
impl crate::Resettable for RxFdp1Spec {}
