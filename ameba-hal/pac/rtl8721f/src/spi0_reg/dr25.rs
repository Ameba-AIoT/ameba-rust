#[doc = "Register `DR25` reader"]
pub type R = crate::R<Dr25Spec>;
#[doc = "Register `DR25` writer"]
pub type W = crate::W<Dr25Spec>;
#[doc = "Field `DR25` reader - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr25R = crate::FieldReader<u16>;
#[doc = "Field `DR25` writer - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr25W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr25(&self) -> Dr25R {
        Dr25R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr25(&mut self) -> Dr25W<'_, Dr25Spec> {
        Dr25W::new(self, 0)
    }
}
#[doc = "Data Register 25\n\nYou can [`read`](crate::Reg::read) this register and get [`dr25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr25Spec;
impl crate::RegisterSpec for Dr25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr25::R`](R) reader structure"]
impl crate::Readable for Dr25Spec {}
#[doc = "`write(|w| ..)` method takes [`dr25::W`](W) writer structure"]
impl crate::Writable for Dr25Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR25 to value 0"]
impl crate::Resettable for Dr25Spec {}
