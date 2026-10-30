#[doc = "Register `DR29` reader"]
pub type R = crate::R<Dr29Spec>;
#[doc = "Register `DR29` writer"]
pub type W = crate::W<Dr29Spec>;
#[doc = "Field `DR29` reader - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr29R = crate::FieldReader<u16>;
#[doc = "Field `DR29` writer - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr29W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr29(&self) -> Dr29R {
        Dr29R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr29(&mut self) -> Dr29W<'_, Dr29Spec> {
        Dr29W::new(self, 0)
    }
}
#[doc = "Data Register 29\n\nYou can [`read`](crate::Reg::read) this register and get [`dr29::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr29::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr29Spec;
impl crate::RegisterSpec for Dr29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr29::R`](R) reader structure"]
impl crate::Readable for Dr29Spec {}
#[doc = "`write(|w| ..)` method takes [`dr29::W`](W) writer structure"]
impl crate::Writable for Dr29Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR29 to value 0"]
impl crate::Resettable for Dr29Spec {}
