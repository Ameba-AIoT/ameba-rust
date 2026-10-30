#[doc = "Register `DR31` reader"]
pub type R = crate::R<Dr31Spec>;
#[doc = "Register `DR31` writer"]
pub type W = crate::W<Dr31Spec>;
#[doc = "Field `DR31` reader - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr31R = crate::FieldReader<u16>;
#[doc = "Field `DR31` writer - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr31W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr31(&self) -> Dr31R {
        Dr31R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr31(&mut self) -> Dr31W<'_, Dr31Spec> {
        Dr31W::new(self, 0)
    }
}
#[doc = "Data Register 31\n\nYou can [`read`](crate::Reg::read) this register and get [`dr31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr31Spec;
impl crate::RegisterSpec for Dr31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr31::R`](R) reader structure"]
impl crate::Readable for Dr31Spec {}
#[doc = "`write(|w| ..)` method takes [`dr31::W`](W) writer structure"]
impl crate::Writable for Dr31Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR31 to value 0"]
impl crate::Resettable for Dr31Spec {}
