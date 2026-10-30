#[doc = "Register `RxRingSize2` reader"]
pub type R = crate::R<RxRingSize2Spec>;
#[doc = "Register `RxRingSize2` writer"]
pub type W = crate::W<RxRingSize2Spec>;
#[doc = "Field `RxRingSize2` reader - The total number of descriptors in the Rx descriptor rings of queue. Act as bit mask, eg. RxRingSize \\[11:0\\]: 0000_0000_1111: 16 descriptors 0000_0001_1111: 32 descriptors 0000_0011_1111: 64 descriptors 0000_0111_1111: 128 descriptors 0000_1111_1111: 256 descriptors 0001_1111_1111: 512 descriptors 0011_1111_1111: 1024 descriptors 0111_1111_1111: 2048 descriptors 1111_1111_1111: 4096 descriptors Any other value in this register yields undefined results."]
pub type RxRingSize2R = crate::FieldReader<u16>;
#[doc = "Field `RxRingSize2` writer - The total number of descriptors in the Rx descriptor rings of queue. Act as bit mask, eg. RxRingSize \\[11:0\\]: 0000_0000_1111: 16 descriptors 0000_0001_1111: 32 descriptors 0000_0011_1111: 64 descriptors 0000_0111_1111: 128 descriptors 0000_1111_1111: 256 descriptors 0001_1111_1111: 512 descriptors 0011_1111_1111: 1024 descriptors 0111_1111_1111: 2048 descriptors 1111_1111_1111: 4096 descriptors Any other value in this register yields undefined results."]
pub type RxRingSize2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - The total number of descriptors in the Rx descriptor rings of queue. Act as bit mask, eg. RxRingSize \\[11:0\\]: 0000_0000_1111: 16 descriptors 0000_0001_1111: 32 descriptors 0000_0011_1111: 64 descriptors 0000_0111_1111: 128 descriptors 0000_1111_1111: 256 descriptors 0001_1111_1111: 512 descriptors 0011_1111_1111: 1024 descriptors 0111_1111_1111: 2048 descriptors 1111_1111_1111: 4096 descriptors Any other value in this register yields undefined results."]
    #[inline(always)]
    pub fn rx_ring_size2(&self) -> RxRingSize2R {
        RxRingSize2R::new(self.bits & 0x0fff)
    }
}
impl W {
    #[doc = "Bits 0:11 - The total number of descriptors in the Rx descriptor rings of queue. Act as bit mask, eg. RxRingSize \\[11:0\\]: 0000_0000_1111: 16 descriptors 0000_0001_1111: 32 descriptors 0000_0011_1111: 64 descriptors 0000_0111_1111: 128 descriptors 0000_1111_1111: 256 descriptors 0001_1111_1111: 512 descriptors 0011_1111_1111: 1024 descriptors 0111_1111_1111: 2048 descriptors 1111_1111_1111: 4096 descriptors Any other value in this register yields undefined results."]
    #[inline(always)]
    pub fn rx_ring_size2(&mut self) -> RxRingSize2W<'_, RxRingSize2Spec> {
        RxRingSize2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ring_size2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ring_size2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxRingSize2Spec;
impl crate::RegisterSpec for RxRingSize2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rx_ring_size2::R`](R) reader structure"]
impl crate::Readable for RxRingSize2Spec {}
#[doc = "`write(|w| ..)` method takes [`rx_ring_size2::W`](W) writer structure"]
impl crate::Writable for RxRingSize2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RxRingSize2 to value 0"]
impl crate::Resettable for RxRingSize2Spec {}
