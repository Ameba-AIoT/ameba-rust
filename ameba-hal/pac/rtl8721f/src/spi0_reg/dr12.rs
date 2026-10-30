#[doc = "Register `DR12` reader"]
pub type R = crate::R<Dr12Spec>;
#[doc = "Register `DR12` writer"]
pub type W = crate::W<Dr12Spec>;
#[doc = "Field `DR12` reader - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr12R = crate::FieldReader<u16>;
#[doc = "Field `DR12` writer - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr12W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr12(&self) -> Dr12R {
        Dr12R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr12(&mut self) -> Dr12W<'_, Dr12Spec> {
        Dr12W::new(self, 0)
    }
}
#[doc = "Data Register 12\n\nYou can [`read`](crate::Reg::read) this register and get [`dr12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr12Spec;
impl crate::RegisterSpec for Dr12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr12::R`](R) reader structure"]
impl crate::Readable for Dr12Spec {}
#[doc = "`write(|w| ..)` method takes [`dr12::W`](W) writer structure"]
impl crate::Writable for Dr12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR12 to value 0"]
impl crate::Resettable for Dr12Spec {}
