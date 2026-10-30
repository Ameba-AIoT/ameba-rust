#[doc = "Register `DR13` reader"]
pub type R = crate::R<Dr13Spec>;
#[doc = "Register `DR13` writer"]
pub type W = crate::W<Dr13Spec>;
#[doc = "Field `DR13` reader - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr13R = crate::FieldReader<u16>;
#[doc = "Field `DR13` writer - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr13W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr13(&self) -> Dr13R {
        Dr13R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr13(&mut self) -> Dr13W<'_, Dr13Spec> {
        Dr13W::new(self, 0)
    }
}
#[doc = "Data Register 13\n\nYou can [`read`](crate::Reg::read) this register and get [`dr13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr13Spec;
impl crate::RegisterSpec for Dr13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr13::R`](R) reader structure"]
impl crate::Readable for Dr13Spec {}
#[doc = "`write(|w| ..)` method takes [`dr13::W`](W) writer structure"]
impl crate::Writable for Dr13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR13 to value 0"]
impl crate::Resettable for Dr13Spec {}
