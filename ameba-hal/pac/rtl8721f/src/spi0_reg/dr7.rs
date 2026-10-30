#[doc = "Register `DR7` reader"]
pub type R = crate::R<Dr7Spec>;
#[doc = "Register `DR7` writer"]
pub type W = crate::W<Dr7Spec>;
#[doc = "Field `DR7` reader - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr7R = crate::FieldReader<u16>;
#[doc = "Field `DR7` writer - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr7W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr7(&self) -> Dr7R {
        Dr7R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr7(&mut self) -> Dr7W<'_, Dr7Spec> {
        Dr7W::new(self, 0)
    }
}
#[doc = "Data Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`dr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr7Spec;
impl crate::RegisterSpec for Dr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr7::R`](R) reader structure"]
impl crate::Readable for Dr7Spec {}
#[doc = "`write(|w| ..)` method takes [`dr7::W`](W) writer structure"]
impl crate::Writable for Dr7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR7 to value 0"]
impl crate::Resettable for Dr7Spec {}
