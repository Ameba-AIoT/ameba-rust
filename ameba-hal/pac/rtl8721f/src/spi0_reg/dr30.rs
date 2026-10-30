#[doc = "Register `DR30` reader"]
pub type R = crate::R<Dr30Spec>;
#[doc = "Register `DR30` writer"]
pub type W = crate::W<Dr30Spec>;
#[doc = "Field `DR30` reader - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr30R = crate::FieldReader<u16>;
#[doc = "Field `DR30` writer - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr30W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr30(&self) -> Dr30R {
        Dr30R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr30(&mut self) -> Dr30W<'_, Dr30Spec> {
        Dr30W::new(self, 0)
    }
}
#[doc = "Data Register 30\n\nYou can [`read`](crate::Reg::read) this register and get [`dr30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr30Spec;
impl crate::RegisterSpec for Dr30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr30::R`](R) reader structure"]
impl crate::Readable for Dr30Spec {}
#[doc = "`write(|w| ..)` method takes [`dr30::W`](W) writer structure"]
impl crate::Writable for Dr30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR30 to value 0"]
impl crate::Resettable for Dr30Spec {}
