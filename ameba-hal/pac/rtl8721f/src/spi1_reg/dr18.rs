#[doc = "Register `DR18` reader"]
pub type R = crate::R<Dr18Spec>;
#[doc = "Register `DR18` writer"]
pub type W = crate::W<Dr18Spec>;
#[doc = "Field `DR18` reader - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr18R = crate::FieldReader<u16>;
#[doc = "Field `DR18` writer - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr18W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr18(&self) -> Dr18R {
        Dr18R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr18(&mut self) -> Dr18W<'_, Dr18Spec> {
        Dr18W::new(self, 0)
    }
}
#[doc = "Data Register 18\n\nYou can [`read`](crate::Reg::read) this register and get [`dr18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr18Spec;
impl crate::RegisterSpec for Dr18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr18::R`](R) reader structure"]
impl crate::Readable for Dr18Spec {}
#[doc = "`write(|w| ..)` method takes [`dr18::W`](W) writer structure"]
impl crate::Writable for Dr18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR18 to value 0"]
impl crate::Resettable for Dr18Spec {}
