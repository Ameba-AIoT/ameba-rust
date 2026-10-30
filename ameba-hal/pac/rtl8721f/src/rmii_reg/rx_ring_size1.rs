#[doc = "Register `Rx_RingSize1` reader"]
pub type R = crate::R<RxRingSize1Spec>;
#[doc = "Register `Rx_RingSize1` writer"]
pub type W = crate::W<RxRingSize1Spec>;
#[doc = "Field `RxRingSize_1_high` reader - This is the bits11-8 of RxRingSize1."]
pub type RxRingSize1HighR = crate::FieldReader;
#[doc = "Field `RxRingSize_1_high` writer - This is the bits11-8 of RxRingSize1."]
pub type RxRingSize1HighW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RxRingSize_1_low` reader - This is the total number of descriptors in the Rx descriptor rings of 1st queue. Act as bit mask, eg. RxRingSize \\[11:0\\]: 0000_0000_1111: 16 descriptors 0000_0001_1111: 32 descriptors 0000_0011_1111: 64 descriptors 0000_0111_1111: 128 descriptors 0000_1111_1111: 256 descriptors 0001_1111_1111: 512 descriptors 0011_1111_1111: 1024 descriptors 0111_1111_1111: 2048 descriptors 1111_1111_1111: 4096 descriptors Any other value in this register yields undefined results."]
pub type RxRingSize1LowR = crate::FieldReader;
#[doc = "Field `RxRingSize_1_low` writer - This is the total number of descriptors in the Rx descriptor rings of 1st queue. Act as bit mask, eg. RxRingSize \\[11:0\\]: 0000_0000_1111: 16 descriptors 0000_0001_1111: 32 descriptors 0000_0011_1111: 64 descriptors 0000_0111_1111: 128 descriptors 0000_1111_1111: 256 descriptors 0001_1111_1111: 512 descriptors 0011_1111_1111: 1024 descriptors 0111_1111_1111: 2048 descriptors 1111_1111_1111: 4096 descriptors Any other value in this register yields undefined results."]
pub type RxRingSize1LowW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - This is the bits11-8 of RxRingSize1."]
    #[inline(always)]
    pub fn rx_ring_size_1_high(&self) -> RxRingSize1HighR {
        RxRingSize1HighR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - This is the total number of descriptors in the Rx descriptor rings of 1st queue. Act as bit mask, eg. RxRingSize \\[11:0\\]: 0000_0000_1111: 16 descriptors 0000_0001_1111: 32 descriptors 0000_0011_1111: 64 descriptors 0000_0111_1111: 128 descriptors 0000_1111_1111: 256 descriptors 0001_1111_1111: 512 descriptors 0011_1111_1111: 1024 descriptors 0111_1111_1111: 2048 descriptors 1111_1111_1111: 4096 descriptors Any other value in this register yields undefined results."]
    #[inline(always)]
    pub fn rx_ring_size_1_low(&self) -> RxRingSize1LowR {
        RxRingSize1LowR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - This is the bits11-8 of RxRingSize1."]
    #[inline(always)]
    pub fn rx_ring_size_1_high(&mut self) -> RxRingSize1HighW<'_, RxRingSize1Spec> {
        RxRingSize1HighW::new(self, 0)
    }
    #[doc = "Bits 8:15 - This is the total number of descriptors in the Rx descriptor rings of 1st queue. Act as bit mask, eg. RxRingSize \\[11:0\\]: 0000_0000_1111: 16 descriptors 0000_0001_1111: 32 descriptors 0000_0011_1111: 64 descriptors 0000_0111_1111: 128 descriptors 0000_1111_1111: 256 descriptors 0001_1111_1111: 512 descriptors 0011_1111_1111: 1024 descriptors 0111_1111_1111: 2048 descriptors 1111_1111_1111: 4096 descriptors Any other value in this register yields undefined results."]
    #[inline(always)]
    pub fn rx_ring_size_1_low(&mut self) -> RxRingSize1LowW<'_, RxRingSize1Spec> {
        RxRingSize1LowW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ring_size1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ring_size1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxRingSize1Spec;
impl crate::RegisterSpec for RxRingSize1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rx_ring_size1::R`](R) reader structure"]
impl crate::Readable for RxRingSize1Spec {}
#[doc = "`write(|w| ..)` method takes [`rx_ring_size1::W`](W) writer structure"]
impl crate::Writable for RxRingSize1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Rx_RingSize1 to value 0"]
impl crate::Resettable for RxRingSize1Spec {}
