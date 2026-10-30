#[doc = "Register `DR17` reader"]
pub type R = crate::R<Dr17Spec>;
#[doc = "Register `DR17` writer"]
pub type W = crate::W<Dr17Spec>;
#[doc = "Field `DR17` reader - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr17R = crate::FieldReader<u16>;
#[doc = "Field `DR17` writer - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr17W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr17(&self) -> Dr17R {
        Dr17R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr17(&mut self) -> Dr17W<'_, Dr17Spec> {
        Dr17W::new(self, 0)
    }
}
#[doc = "Data Register 17\n\nYou can [`read`](crate::Reg::read) this register and get [`dr17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr17Spec;
impl crate::RegisterSpec for Dr17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr17::R`](R) reader structure"]
impl crate::Readable for Dr17Spec {}
#[doc = "`write(|w| ..)` method takes [`dr17::W`](W) writer structure"]
impl crate::Writable for Dr17Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR17 to value 0"]
impl crate::Resettable for Dr17Spec {}
