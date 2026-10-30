#[doc = "Register `DR34` reader"]
pub type R = crate::R<Dr34Spec>;
#[doc = "Register `DR34` writer"]
pub type W = crate::W<Dr34Spec>;
#[doc = "Field `DR34` reader - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr34R = crate::FieldReader<u16>;
#[doc = "Field `DR34` writer - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr34W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr34(&self) -> Dr34R {
        Dr34R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr34(&mut self) -> Dr34W<'_, Dr34Spec> {
        Dr34W::new(self, 0)
    }
}
#[doc = "Data Register 34\n\nYou can [`read`](crate::Reg::read) this register and get [`dr34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr34Spec;
impl crate::RegisterSpec for Dr34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr34::R`](R) reader structure"]
impl crate::Readable for Dr34Spec {}
#[doc = "`write(|w| ..)` method takes [`dr34::W`](W) writer structure"]
impl crate::Writable for Dr34Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR34 to value 0"]
impl crate::Resettable for Dr34Spec {}
