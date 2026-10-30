#[doc = "Register `DR35` reader"]
pub type R = crate::R<Dr35Spec>;
#[doc = "Register `DR35` writer"]
pub type W = crate::W<Dr35Spec>;
#[doc = "Field `DR35` reader - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr35R = crate::FieldReader<u16>;
#[doc = "Field `DR35` writer - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr35W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr35(&self) -> Dr35R {
        Dr35R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr35(&mut self) -> Dr35W<'_, Dr35Spec> {
        Dr35W::new(self, 0)
    }
}
#[doc = "Data Register 35\n\nYou can [`read`](crate::Reg::read) this register and get [`dr35::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr35::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr35Spec;
impl crate::RegisterSpec for Dr35Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr35::R`](R) reader structure"]
impl crate::Readable for Dr35Spec {}
#[doc = "`write(|w| ..)` method takes [`dr35::W`](W) writer structure"]
impl crate::Writable for Dr35Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR35 to value 0"]
impl crate::Resettable for Dr35Spec {}
