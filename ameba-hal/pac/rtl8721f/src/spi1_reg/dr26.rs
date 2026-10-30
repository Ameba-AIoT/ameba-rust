#[doc = "Register `DR26` reader"]
pub type R = crate::R<Dr26Spec>;
#[doc = "Register `DR26` writer"]
pub type W = crate::W<Dr26Spec>;
#[doc = "Field `DR26` reader - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr26R = crate::FieldReader<u16>;
#[doc = "Field `DR26` writer - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr26W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr26(&self) -> Dr26R {
        Dr26R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr26(&mut self) -> Dr26W<'_, Dr26Spec> {
        Dr26W::new(self, 0)
    }
}
#[doc = "Data Register 26\n\nYou can [`read`](crate::Reg::read) this register and get [`dr26::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr26::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr26Spec;
impl crate::RegisterSpec for Dr26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr26::R`](R) reader structure"]
impl crate::Readable for Dr26Spec {}
#[doc = "`write(|w| ..)` method takes [`dr26::W`](W) writer structure"]
impl crate::Writable for Dr26Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR26 to value 0"]
impl crate::Resettable for Dr26Spec {}
