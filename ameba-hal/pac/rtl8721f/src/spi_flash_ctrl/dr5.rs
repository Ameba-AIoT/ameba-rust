#[doc = "Register `DR5` reader"]
pub type R = crate::R<Dr5Spec>;
#[doc = "Register `DR5` writer"]
pub type W = crate::W<Dr5Spec>;
#[doc = "Field `DR5` reader - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
pub type Dr5R = crate::FieldReader<u32>;
#[doc = "Field `DR5` writer - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
pub type Dr5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
    #[inline(always)]
    pub fn dr5(&self) -> Dr5R {
        Dr5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - It is a data buffer with 8-bit width FIFO. If accessing in word data byte, it would read/write 4 entries of FIFO. SPIC will set SSIENR.SPIC_EN=1 automatically if user continues to push DR when DR is full in user write mode."]
    #[inline(always)]
    pub fn dr5(&mut self) -> Dr5W<'_, Dr5Spec> {
        Dr5W::new(self, 0)
    }
}
#[doc = "Data Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`dr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr5Spec;
impl crate::RegisterSpec for Dr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr5::R`](R) reader structure"]
impl crate::Readable for Dr5Spec {}
#[doc = "`write(|w| ..)` method takes [`dr5::W`](W) writer structure"]
impl crate::Writable for Dr5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR5 to value 0"]
impl crate::Resettable for Dr5Spec {}
