#[doc = "Register `DR4` reader"]
pub type R = crate::R<Dr4Spec>;
#[doc = "Register `DR4` writer"]
pub type W = crate::W<Dr4Spec>;
#[doc = "Field `DR4` reader - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr4R = crate::FieldReader<u16>;
#[doc = "Field `DR4` writer - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr4(&self) -> Dr4R {
        Dr4R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr4(&mut self) -> Dr4W<'_, Dr4Spec> {
        Dr4W::new(self, 0)
    }
}
#[doc = "Data Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`dr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr4Spec;
impl crate::RegisterSpec for Dr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr4::R`](R) reader structure"]
impl crate::Readable for Dr4Spec {}
#[doc = "`write(|w| ..)` method takes [`dr4::W`](W) writer structure"]
impl crate::Writable for Dr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR4 to value 0"]
impl crate::Resettable for Dr4Spec {}
