#[doc = "Register `DR21` reader"]
pub type R = crate::R<Dr21Spec>;
#[doc = "Register `DR21` writer"]
pub type W = crate::W<Dr21Spec>;
#[doc = "Field `DR21` reader - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr21R = crate::FieldReader<u16>;
#[doc = "Field `DR21` writer - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr21W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr21(&self) -> Dr21R {
        Dr21R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr21(&mut self) -> Dr21W<'_, Dr21Spec> {
        Dr21W::new(self, 0)
    }
}
#[doc = "Data Register 21\n\nYou can [`read`](crate::Reg::read) this register and get [`dr21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr21Spec;
impl crate::RegisterSpec for Dr21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr21::R`](R) reader structure"]
impl crate::Readable for Dr21Spec {}
#[doc = "`write(|w| ..)` method takes [`dr21::W`](W) writer structure"]
impl crate::Writable for Dr21Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR21 to value 0"]
impl crate::Resettable for Dr21Spec {}
