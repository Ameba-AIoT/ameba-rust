#[doc = "Register `DR9` reader"]
pub type R = crate::R<Dr9Spec>;
#[doc = "Register `DR9` writer"]
pub type W = crate::W<Dr9Spec>;
#[doc = "Field `DR9` reader - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr9R = crate::FieldReader<u16>;
#[doc = "Field `DR9` writer - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr9W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr9(&self) -> Dr9R {
        Dr9R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr9(&mut self) -> Dr9W<'_, Dr9Spec> {
        Dr9W::new(self, 0)
    }
}
#[doc = "Data Register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`dr9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr9Spec;
impl crate::RegisterSpec for Dr9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr9::R`](R) reader structure"]
impl crate::Readable for Dr9Spec {}
#[doc = "`write(|w| ..)` method takes [`dr9::W`](W) writer structure"]
impl crate::Writable for Dr9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR9 to value 0"]
impl crate::Resettable for Dr9Spec {}
