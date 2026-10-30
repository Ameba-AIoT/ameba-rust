#[doc = "Register `DR16` reader"]
pub type R = crate::R<Dr16Spec>;
#[doc = "Register `DR16` writer"]
pub type W = crate::W<Dr16Spec>;
#[doc = "Field `DR16` reader - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr16R = crate::FieldReader<u16>;
#[doc = "Field `DR16` writer - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
pub type Dr16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr16(&self) -> Dr16R {
        Dr16R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read: Receive FIFO buffer Write: Transmit FIFO buffer"]
    #[inline(always)]
    pub fn dr16(&mut self) -> Dr16W<'_, Dr16Spec> {
        Dr16W::new(self, 0)
    }
}
#[doc = "Data Register 16\n\nYou can [`read`](crate::Reg::read) this register and get [`dr16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr16Spec;
impl crate::RegisterSpec for Dr16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr16::R`](R) reader structure"]
impl crate::Readable for Dr16Spec {}
#[doc = "`write(|w| ..)` method takes [`dr16::W`](W) writer structure"]
impl crate::Writable for Dr16Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR16 to value 0"]
impl crate::Resettable for Dr16Spec {}
