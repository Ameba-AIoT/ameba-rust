#[doc = "Register `DR19` reader"]
pub type R = crate::R<Dr19Spec>;
#[doc = "Register `DR19` writer"]
pub type W = crate::W<Dr19Spec>;
#[doc = "Field `DR19` reader - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr19R = crate::FieldReader<u16>;
#[doc = "Field `DR19` writer - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr19W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr19(&self) -> Dr19R {
        Dr19R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr19(&mut self) -> Dr19W<'_, Dr19Spec> {
        Dr19W::new(self, 0)
    }
}
#[doc = "Data Register 19\n\nYou can [`read`](crate::Reg::read) this register and get [`dr19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr19Spec;
impl crate::RegisterSpec for Dr19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr19::R`](R) reader structure"]
impl crate::Readable for Dr19Spec {}
#[doc = "`write(|w| ..)` method takes [`dr19::W`](W) writer structure"]
impl crate::Writable for Dr19Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR19 to value 0"]
impl crate::Resettable for Dr19Spec {}
