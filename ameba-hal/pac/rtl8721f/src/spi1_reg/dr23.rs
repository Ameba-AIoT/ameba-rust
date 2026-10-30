#[doc = "Register `DR23` reader"]
pub type R = crate::R<Dr23Spec>;
#[doc = "Register `DR23` writer"]
pub type W = crate::W<Dr23Spec>;
#[doc = "Field `DR23` reader - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr23R = crate::FieldReader<u16>;
#[doc = "Field `DR23` writer - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr23W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr23(&self) -> Dr23R {
        Dr23R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr23(&mut self) -> Dr23W<'_, Dr23Spec> {
        Dr23W::new(self, 0)
    }
}
#[doc = "Data Register 23\n\nYou can [`read`](crate::Reg::read) this register and get [`dr23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr23Spec;
impl crate::RegisterSpec for Dr23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr23::R`](R) reader structure"]
impl crate::Readable for Dr23Spec {}
#[doc = "`write(|w| ..)` method takes [`dr23::W`](W) writer structure"]
impl crate::Writable for Dr23Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR23 to value 0"]
impl crate::Resettable for Dr23Spec {}
