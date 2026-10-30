#[doc = "Register `DR6` reader"]
pub type R = crate::R<Dr6Spec>;
#[doc = "Register `DR6` writer"]
pub type W = crate::W<Dr6Spec>;
#[doc = "Field `DR6` reader - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr6R = crate::FieldReader<u16>;
#[doc = "Field `DR6` writer - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr6(&self) -> Dr6R {
        Dr6R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr6(&mut self) -> Dr6W<'_, Dr6Spec> {
        Dr6W::new(self, 0)
    }
}
#[doc = "Data Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`dr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr6Spec;
impl crate::RegisterSpec for Dr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr6::R`](R) reader structure"]
impl crate::Readable for Dr6Spec {}
#[doc = "`write(|w| ..)` method takes [`dr6::W`](W) writer structure"]
impl crate::Writable for Dr6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR6 to value 0"]
impl crate::Resettable for Dr6Spec {}
