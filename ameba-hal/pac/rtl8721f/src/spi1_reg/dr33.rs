#[doc = "Register `DR33` reader"]
pub type R = crate::R<Dr33Spec>;
#[doc = "Register `DR33` writer"]
pub type W = crate::W<Dr33Spec>;
#[doc = "Field `DR33` reader - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr33R = crate::FieldReader<u16>;
#[doc = "Field `DR33` writer - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr33W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr33(&self) -> Dr33R {
        Dr33R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr33(&mut self) -> Dr33W<'_, Dr33Spec> {
        Dr33W::new(self, 0)
    }
}
#[doc = "Data Register 33\n\nYou can [`read`](crate::Reg::read) this register and get [`dr33::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr33::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr33Spec;
impl crate::RegisterSpec for Dr33Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr33::R`](R) reader structure"]
impl crate::Readable for Dr33Spec {}
#[doc = "`write(|w| ..)` method takes [`dr33::W`](W) writer structure"]
impl crate::Writable for Dr33Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR33 to value 0"]
impl crate::Resettable for Dr33Spec {}
