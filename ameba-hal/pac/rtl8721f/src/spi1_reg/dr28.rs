#[doc = "Register `DR28` reader"]
pub type R = crate::R<Dr28Spec>;
#[doc = "Register `DR28` writer"]
pub type W = crate::W<Dr28Spec>;
#[doc = "Field `DR28` reader - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr28R = crate::FieldReader<u16>;
#[doc = "Field `DR28` writer - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr28W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr28(&self) -> Dr28R {
        Dr28R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr28(&mut self) -> Dr28W<'_, Dr28Spec> {
        Dr28W::new(self, 0)
    }
}
#[doc = "Data Register 28\n\nYou can [`read`](crate::Reg::read) this register and get [`dr28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr28Spec;
impl crate::RegisterSpec for Dr28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr28::R`](R) reader structure"]
impl crate::Readable for Dr28Spec {}
#[doc = "`write(|w| ..)` method takes [`dr28::W`](W) writer structure"]
impl crate::Writable for Dr28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR28 to value 0"]
impl crate::Resettable for Dr28Spec {}
